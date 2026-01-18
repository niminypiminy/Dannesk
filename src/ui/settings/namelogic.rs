use crate::channel::{CHANNEL, ProgressState, SettingsState, SettingsView};
use crate::utils::json_storage;
use std::time::Duration;
use tokio::time::sleep;

pub struct NameLogic;

impl NameLogic {
    pub async fn update_name(new_name: String, is_dark: bool, hide_balance: bool) {
        // 1. Initial State
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.5,
            message: "Saving new name...".to_string(),
        }));

        // 2. Blocking I/O for JSON storage
        let json_data = serde_json::json!({ "saved_name": new_name });
        let write_result = tokio::task::spawn_blocking(move || {
            json_storage::write_json("settings.json", &json_data)
        }).await;

        // 3. Handle Result and Channel Update
       match write_result {
    Ok(Ok(())) => {
        let _ = CHANNEL.theme_user_tx.send((is_dark, new_name, hide_balance));

        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "Name updated successfully!".to_string(),
        }));

        // 1. Close settings by setting last_view to None
        let _ = CHANNEL.settings_modal_tx.send(SettingsState {
            view_type: SettingsView::Name, // Reset to default for next open
            last_view: None,               // This triggers the Gate in balance.rs to close
        });

        // 2. Explicitly ensure we are on the Balance tab
        let _ = CHANNEL.selected_tab_tx.send(crate::channel::Tab::Balance);
    }
            _ => {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Error: Failed to save name.".to_string(),
                }));
            }
        }

        // Cleanup progress bar
        sleep(Duration::from_secs(2)).await;
        let _ = CHANNEL.progress_tx.send(None);
    }
}