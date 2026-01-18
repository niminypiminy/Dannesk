use crate::channel::{CHANNEL, ProgressState, SettingsState, SettingsView}; // Updated imports
use std::time::Duration;
use tokio::time::sleep;

pub struct PinLogic;

impl PinLogic {
    pub async fn change_pin(old_pin: String, new_pin: String) {
        // 1. Initial State
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.5,
            message: "Verifying credentials...".to_string(),
        }));

        // 2. Blocking operation
        let result = tokio::task::spawn_blocking(move || {
            crate::pin::change_pin(&old_pin, &new_pin)
        }).await;

        match result {
             Ok(Ok(())) => {

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
                    message: "Error: PIN update failed. Check old PIN.".to_string(),
                }));
            }
        }
        
        // Optional: wait a bit before hiding the progress bar
        sleep(Duration::from_secs(2)).await;
        let _ = CHANNEL.progress_tx.send(None);
    }
}