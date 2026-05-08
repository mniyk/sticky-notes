#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod storage;
mod tray;
mod window;

use std::sync::{Arc, Mutex};
use slint::{ModelRc, VecModel};
use models::StickyNote;
use window::{Screen, SIDEBAR_W, EDITOR_W, EDITOR_H};

slint::include_modules!();

fn notes_to_model(notes: &[StickyNote]) -> ModelRc<NoteItem> {
    ModelRc::new(VecModel::from(
        notes.iter().map(|n| NoteItem {
            id: n.id.clone().into(),
            title: n.display_title().into(),
            preview: n.preview().into(),
        }).collect::<Vec<_>>()
    ))
}

fn open_editor(
    sidebar: &SidebarWindow,
    editor: &EditorWindow,
    note: &StickyNote,
    screen: &Screen,
) {
    let (ex, ey) = screen.editor_center_pos();
    editor.set_edit_id(note.id.clone().into());
    editor.set_edit_title(note.title.clone().into());
    editor.set_edit_content(note.content.clone().into());
    editor.set_confirm_delete(false);
    editor.window().set_size(slint::PhysicalSize::new(EDITOR_W, EDITOR_H));
    editor.window().set_position(slint::PhysicalPosition::new(ex, ey));
    editor.window().show().ok();
    sidebar.set_active_id(note.id.clone().into());
}

fn close_editor(sidebar: &SidebarWindow, editor: &EditorWindow) {
    editor.window().hide().ok();
    sidebar.set_active_id("".into());
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::new()
        .parse_filters("warn,icu_provider=off,icu_segmenter=off")
        .init();

    let tray = tray::TrayManager::new()?;

    let notes = Arc::new(Mutex::new({
        let mut v = storage::load_notes();
        v.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        v
    }));

    let screen = Screen::get();
    let sidebar = SidebarWindow::new()?;
    let editor  = EditorWindow::new()?;

    // サイドバー配置（右端・フル高さ）
    let (sx, sy) = screen.sidebar_pos();
    sidebar.window().set_size(slint::PhysicalSize::new(SIDEBAR_W, screen.h as u32));
    sidebar.window().set_position(slint::PhysicalPosition::new(sx, sy));
    sidebar.window().show()?;

    sidebar.set_notes(notes_to_model(&notes.lock().unwrap()));

    // 新規作成
    sidebar.on_new_note({
        let sidebar = sidebar.as_weak();
        let editor  = editor.as_weak();
        let notes   = Arc::clone(&notes);
        let screen  = screen.clone();
        move || {
            let sidebar = sidebar.unwrap();
            let editor  = editor.unwrap();
            let note = StickyNote::new();
            {
                let mut v = notes.lock().unwrap();
                v.insert(0, note.clone());
                storage::save_notes(&v);
                sidebar.set_notes(notes_to_model(&v));
            }
            open_editor(&sidebar, &editor, &note, &screen);
        }
    });

    // 既存ノートを開く
    sidebar.on_open_note({
        let sidebar = sidebar.as_weak();
        let editor  = editor.as_weak();
        let notes   = Arc::clone(&notes);
        let screen  = screen.clone();
        move |id| {
            let sidebar = sidebar.unwrap();
            let editor  = editor.unwrap();
            if let Some(note) = notes.lock().unwrap().iter().find(|n| n.id == id.as_str()).cloned() {
                open_editor(&sidebar, &editor, &note, &screen);
            }
        }
    });

    // 保存
    editor.on_save_note({
        let sidebar = sidebar.as_weak();
        let notes   = Arc::clone(&notes);
        move |id, title, content| {
            let mut v = notes.lock().unwrap();
            if let Some(n) = v.iter_mut().find(|n| n.id == id.as_str()) {
                n.title   = title.to_string();
                n.content = content.to_string();
                n.updated_at = chrono::Utc::now();
            }
            v.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
            storage::save_notes(&v);
            if let Some(s) = sidebar.upgrade() {
                s.set_notes(notes_to_model(&v));
            }
        }
    });

    // 削除
    editor.on_delete_note({
        let sidebar = sidebar.as_weak();
        let editor  = editor.as_weak();
        let notes   = Arc::clone(&notes);
        move |id| {
            let sidebar = sidebar.unwrap();
            let editor  = editor.unwrap();
            {
                let mut v = notes.lock().unwrap();
                v.retain(|n| n.id != id.as_str());
                storage::save_notes(&v);
                sidebar.set_notes(notes_to_model(&v));
            }
            close_editor(&sidebar, &editor);
        }
    });

    // エディタを閉じる → 保存してハイライト解除
    {
        let sidebar_w = sidebar.as_weak();
        let editor_w  = editor.as_weak();
        editor.window().on_close_requested(move || {
            if let (Some(s), Some(e)) = (sidebar_w.upgrade(), editor_w.upgrade()) {
                e.invoke_save_note(e.get_edit_id(), e.get_edit_title(), e.get_edit_content());
                close_editor(&s, &e);
            }
            slint::CloseRequestResponse::KeepWindowShown
        });
    }

    // サイドバーを閉じる → 隠すだけ
    {
        let sidebar_w = sidebar.as_weak();
        let editor_w  = editor.as_weak();
        sidebar.window().on_close_requested(move || {
            if let (Some(s), Some(e)) = (sidebar_w.upgrade(), editor_w.upgrade()) {
                e.window().hide().ok();
                s.window().hide().ok();
            }
            slint::CloseRequestResponse::KeepWindowShown
        });
    }

    // トレイ
    let _tray_timer = {
        let sidebar_w = sidebar.as_weak();
        let editor_w  = editor.as_weak();
        let notes     = Arc::clone(&notes);
        let screen    = screen.clone();
        let t = slint::Timer::default();
        t.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(200), move || {
            match tray.poll_event() {
                Some(tray::TrayMenuAction::Quit) => {
                    slint::quit_event_loop().ok();
                }
                Some(tray::TrayMenuAction::ShowHide) => {
                    if let Some(s) = sidebar_w.upgrade() {
                        if s.window().is_visible() {
                            s.window().hide().ok();
                            editor_w.upgrade().map(|e| e.window().hide().ok());
                        } else {
                            s.window().show().ok();
                        }
                    }
                }
                Some(tray::TrayMenuAction::NewNote) => {
                    if let (Some(s), Some(e)) = (sidebar_w.upgrade(), editor_w.upgrade()) {
                        s.window().show().ok();
                        let note = StickyNote::new();
                        {
                            let mut v = notes.lock().unwrap();
                            v.insert(0, note.clone());
                            storage::save_notes(&v);
                            s.set_notes(notes_to_model(&v));
                        }
                        open_editor(&s, &e, &note, &screen);
                    }
                }
                None => {}
            }
        });
        t
    };

    slint::run_event_loop()?;
    Ok(())
}
