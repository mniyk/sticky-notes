use std::path::PathBuf;
use crate::models::{Config, StickyNote};

fn notes_path() -> PathBuf {
    let dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("sticky-notes");
    std::fs::create_dir_all(&dir).ok();
    dir.join("notes.json")
}

fn config_path() -> PathBuf {
    let dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("sticky-notes");
    std::fs::create_dir_all(&dir).ok();
    dir.join("config.json")
}

pub fn load_notes() -> Vec<StickyNote> {
    let path = notes_path();
    if !path.exists() {
        return vec![];
    }
    let data = std::fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn save_notes(notes: &[StickyNote]) {
    let path = notes_path();
    if let Ok(data) = serde_json::to_string_pretty(notes) {
        std::fs::write(path, data).ok();
    }
}

#[allow(dead_code)]
pub fn load_config() -> Config {
    let path = config_path();
    if !path.exists() {
        return Config::default();
    }
    let data = std::fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

#[allow(dead_code)]
pub fn save_config(config: &Config) {
    let path = config_path();
    if let Ok(data) = serde_json::to_string_pretty(config) {
        std::fs::write(path, data).ok();
    }
}
