use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyNote {
    pub id: String,
    #[serde(default)]
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl StickyNote {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: String::new(),
            content: String::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn display_title(&self) -> String {
        if !self.title.is_empty() {
            let s: String = self.title.chars().take(20).collect();
            return if self.title.chars().count() > 20 { format!("{}…", s) } else { s };
        }
        let first = self.content.lines().next().unwrap_or("").trim();
        if first.is_empty() { return "(無題)".to_string(); }
        let s: String = first.chars().take(20).collect();
        if first.chars().count() > 20 { format!("{}…", s) } else { s }
    }

    pub fn preview(&self) -> String {
        let text = self.content.trim();
        if text.is_empty() { return String::new(); }
        let s: String = text.chars().take(40).collect();
        if text.chars().count() > 40 { format!("{}…", s) } else { s }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub position: SidebarPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SidebarPosition {
    Right,
    Left,
}

impl Default for Config {
    fn default() -> Self {
        Self { position: SidebarPosition::Right }
    }
}
