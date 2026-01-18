// src/ui/progressbar.rs (Fixed Dioxus version - corrected for Dioxus 0.7 use_context and coroutine)

use dioxus::prelude::*;
use std::time::{Instant, Duration};
use crate::channel::{CHANNEL, ProgressState};
use crate::context::GlobalContext;

#[component]
pub fn ProgressBar(operation_name: String) -> Element {
    let global = use_context::<GlobalContext>();
    let progress_state = global.progress;

    let Some(ref state) = *progress_state.read() else { return rsx! {} };

    let mut pulse_factor = use_signal(|| 0.5f32);

    // Coroutine for animation (breathing effect) - only pulses when in progress
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        let start = Instant::now();
        loop {
            let elapsed = start.elapsed().as_secs_f32();
            let anim_value = elapsed.sin();
            pulse_factor.set(0.5 + 0.5 * anim_value);
            tokio::time::sleep(Duration::from_millis(16)).await;
        }
    });

    // Coroutine for monitoring timeout and auto-dismiss
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        let mut ts = Instant::now(); // Set on mount (when progress becomes Some)
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await; // Check every 500ms for responsiveness

            let current = (*progress_state.read()).clone();
            let Some(ref current_state) = current else {
                // If became None, unmounting
                break;
            };

            let is_in_flight = current_state.progress < 1.0
                && !current_state.message.to_lowercase().contains("error")
                && !current_state.message.to_lowercase().contains("failed");

            if is_in_flight {
                if ts.elapsed().as_secs() >= 15 {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: "Operation timed out—please try again".to_string(),
                    }));
                    ts = Instant::now(); // Reset after sending
                }
            } else {
                ts = Instant::now(); // Reset timer when not in flight
            }

            // Check for auto-dismiss conditions
            if current_state.progress >= 1.0
                || current_state.message.to_lowercase().contains("error")
                || current_state.message.to_lowercase().contains("failed")
            {
                tokio::time::sleep(Duration::from_millis(1000)).await;
                let _ = CHANNEL.progress_tx.send(None);
                break; // Stop after dismiss
            }
        }
    });

    let progress = state.progress.clamp(0.0, 1.0);
    let status_text = format!("{}: {}", operation_name, state.message);

    let (fill_color, text_color) = if state.message.to_lowercase().contains("error") || state.message.to_lowercase().contains("failed") {
        ("#dc2626".to_string(), "#ffffff".to_string())
    } else if state.message.to_lowercase().contains("timed out") {
        ("#eab308".to_string(), "#000000".to_string())
    } else if progress >= 1.0 {
        ("#16a34a".to_string(), "#ffffff".to_string())
    } else {
        // In-progress: Breathing green instead of grey
        let pf = *pulse_factor.read();
        let green_val = (160.0 + (220.0 - 160.0) * pf).round() as u8;
        let fill_color_str = format!("rgb(22, {}, 74)", green_val);
        (fill_color_str, "#ffffff".to_string())
    };

    let progress_width = (progress * 100.0).round();

    rsx! {
        div {
            style: "
                position: fixed;
                top: 0;
                left: 0;
                height: 100%; 
                width: 100%;
                background-color: rgba(0, 0, 0, 0.8);
                display: flex;
                justify-content: center;
                align-items: center;
                z-index: 9999;
                cursor: wait;
            ",
            div {
                style: "
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    gap: 8px;
                    padding: 20px;
                    background: rgba(255, 255, 255, 0.1);
                    border-radius: 12px;
                    backdrop-filter: blur(10px);
                ",
                div {
                    style: "color: {text_color}; font-size: 14px; font-weight: bold; text-align: center; max-width: 400px;",
                    "{status_text}"
                }
                div {
                    style: "
                        width: 400px;
                        height: 24px;
                        background-color: #e5e7eb;
                        border-radius: 12px;
                        overflow: hidden;
                    ",
                    div {
                        style: "
                            width: {progress_width}%;
                            height: 100%;
                            background-color: {fill_color};
                            border-radius: 12px;
                            transition: width 0.3s ease-in-out;
                        ",
                    }
                }
            }
        }
    }
}