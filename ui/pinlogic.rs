use crate::channel::{CHANNEL, ProgressState, SideBarView}; 

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
                // Success
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "PIN updated successfully!".to_string(),
                }));

                // ← EXACTLY like XRPCreateLogic
                let _ = CHANNEL.sidebar_view_tx.send(SideBarView::None);
            }
            _ => {
                // Failure
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "SIGNAL_INTERRUPT: PIN update failed.".to_string(),
                }));
            }
        }
    }
}