use crate::channel::{CHANNEL, ProgressState, Tab}; 
use std::time::Duration;
use tokio::time::sleep;

pub struct PinLogic;

impl PinLogic {
    pub async fn change_pin(old_pin: String, new_pin: String) {
        // 1. Verification State
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.5,
            message: "Verifying credentials...".to_string(),
        }));

        // 2. Perform the actual PIN change
        let result = tokio::task::spawn_blocking(move || {
            crate::pin::change_pin(&old_pin, &new_pin)
        }).await;

        match result {
            Ok(Ok(())) => {
                // Success: Update progress bar
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "PIN updated successfully!".to_string(),
                }));

                // 3. Ensure global navigation is set to Balance
                // (The local overlay in balance.rs handles its own closing)
                let _ = CHANNEL.selected_tab_tx.send(Tab::Balance);
            }
            _ => {
                // Failure: Notify user
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "SIGNAL_INTERRUPT: PIN update failed.".to_string(),
                }));
            }
        }
        
        // Let the user read the status message before clearing the bar
        sleep(Duration::from_secs(2)).await;
        let _ = CHANNEL.progress_tx.send(None);
    }
}