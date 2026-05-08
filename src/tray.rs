use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIcon, TrayIconBuilder,
};
use image::RgbaImage;

pub struct TrayManager {
    pub _tray: TrayIcon,
    pub show_hide_id: tray_icon::menu::MenuId,
    pub new_note_id: tray_icon::menu::MenuId,
    pub quit_id: tray_icon::menu::MenuId,
}

impl TrayManager {
    pub fn new() -> anyhow::Result<Self> {
        let icon = make_icon();

        let show_hide = MenuItem::new("表示/非表示", true, None);
        let new_note = MenuItem::new("新規付箋", true, None);
        let quit = MenuItem::new("終了", true, None);

        let show_hide_id = show_hide.id().clone();
        let new_note_id = new_note.id().clone();
        let quit_id = quit.id().clone();

        let menu = Menu::new();
        menu.append(&show_hide)?;
        menu.append(&new_note)?;
        menu.append(&quit)?;

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Sticky Notes")
            .with_icon(icon)
            .build()?;

        Ok(Self {
            _tray: tray,
            show_hide_id,
            new_note_id,
            quit_id,
        })
    }

    pub fn poll_event(&self) -> Option<TrayMenuAction> {
        if let Ok(ev) = MenuEvent::receiver().try_recv() {
            if ev.id == self.quit_id {
                return Some(TrayMenuAction::Quit);
            } else if ev.id == self.show_hide_id {
                return Some(TrayMenuAction::ShowHide);
            } else if ev.id == self.new_note_id {
                return Some(TrayMenuAction::NewNote);
            }
        }
        None
    }
}

pub enum TrayMenuAction {
    Quit,
    ShowHide,
    NewNote,
}

fn make_icon() -> tray_icon::Icon {
    // 32x32 の黄色い付箋風アイコンを生成
    let size = 32u32;
    let mut img = RgbaImage::new(size, size);

    for y in 0..size {
        for x in 0..size {
            let px = if x == 0 || x == size - 1 || y == 0 || y == size - 1 {
                [180, 130, 0, 255] // 枠線 (濃い黄色)
            } else if y < 6 {
                [251, 191, 36, 255] // 上部タブ (黄色)
            } else {
                [254, 252, 232, 255] // 本文エリア (薄黄)
            };
            img.put_pixel(x, y, image::Rgba(px));
        }
    }

    // 罫線を描く
    for line in [12u32, 18, 24] {
        for x in 4..28 {
            img.put_pixel(x, line, image::Rgba([200, 200, 150, 255]));
        }
    }

    let rgba = img.into_raw();
    tray_icon::Icon::from_rgba(rgba, size, size).expect("icon")
}
