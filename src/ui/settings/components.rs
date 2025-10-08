use serde::{Serialize, Deserialize};
use egui::Ui;
use crate::utils::json_storage;

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    // Empty enum, kept for compatibility with NameComponent
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SharedSettingsState {
    pub saved_name: String,
}

impl SharedSettingsState {
    pub fn save_to_file(name: &str) -> bool {
        let json = serde_json::json!({
            "saved_name": name,
        });
        json_storage::write_json("settings.json", &json).is_ok()
    }
}

pub trait SettingComponent: std::fmt::Debug {
    fn render(&mut self, ui: &mut Ui, is_dark_mode: bool, current_name: String) -> Option<SettingsMessage>;
}