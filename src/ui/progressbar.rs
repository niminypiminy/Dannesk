// src/ui/progressbar.rs
use dioxus_native::prelude::*;
use std::time::{Instant, Duration};
use crate::channel::{CHANNEL, ProgressState};
use crate::context::GlobalContext;

#[component]
pub fn ProgressBar(operation_name: String) -> Element {
    let global = use_context::<GlobalContext>();
    let progress_state = global.progress;

    let Some(ref state) = *progress_state.read() else { return rsx! {} };

    let mut pulse_opacity = use_signal(|| 1.0f32);

    // Coroutine for terminal-style pulse (breathing opacity)
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        let start = Instant::now();
        loop {
            let elapsed = start.elapsed().as_secs_f32();
            // Faster, sharper pulse for a "system" feel
            let anim_value = (elapsed * 2.0).sin();
            pulse_opacity.set(0.6 + 0.4 * anim_value);
            tokio::time::sleep(Duration::from_millis(16)).await;
        }
    });

    // Timeout and Dismiss logic (Kept from your original logic)
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        let mut ts = Instant::now();
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;
            let current = (*progress_state.read()).clone();
            let Some(ref current_state) = current else { break; };

            let is_in_flight = current_state.progress < 1.0
                && !current_state.message.to_lowercase().contains("error")
                && !current_state.message.to_lowercase().contains("failed");

            if is_in_flight {
                if ts.elapsed().as_secs() >= 15 {
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: "ERR_TIMEOUT // PLEASE_RETRY".to_string(),
                    }));
                    ts = Instant::now();
                }
            } else {
                ts = Instant::now();
            }

            if current_state.progress >= 1.0
                || current_state.message.to_lowercase().contains("error")
                || current_state.message.to_lowercase().contains("failed")
            {
                tokio::time::sleep(Duration::from_millis(1200)).await;
                let _ = CHANNEL.progress_tx.send(None);
                break;
            }
        }
    });

    let progress = state.progress.clamp(0.0, 1.0);
    let progress_width = (progress * 100.0).round();
    
    // Terminal styling mapping
    let status_color = if state.message.to_lowercase().contains("error") || state.message.to_lowercase().contains("failed") {
        "var(--status-warn)"
    } else if progress >= 1.0 {
        "var(--status-ok)"
    } else {
        "var(--accent)"
    };

    let op_name_upper = operation_name.to_uppercase();
    let msg_upper = state.message.to_uppercase();

    rsx! {
        style { {r#"
            .terminal-overlay {
                position: fixed;
                top: 0; left: 0;
                height: 100%; width: 100%;
                background-color: rgba(0, 0, 0, 0.9);
                display: flex;
                justify-content: center;
                align-items: center;
                z-index: 9999;
                font-family: 'JetBrains Mono', monospace;
            }
            .terminal-box {
                width: 450px;
                padding: 24px;
                background: var(--bg-secondary);
                border: 1px solid var(--border);
                position: relative;
                box-shadow: 0 10px 30px rgba(0,0,0,0.5);
            }
            .terminal-header {
                display: flex;
                justify-content: space-between;
                margin-bottom: 12px;
                font-size: 0.7rem;
                letter-spacing: 1px;
            }
            .progress-container {
                width: 100%;
                height: 12px;
                background: var(--bg-faint);
                border: 1px solid var(--border);
                padding: 2px;
                margin-bottom: 12px;
            }
            .progress-fill {
                height: 100%;
                background: var(--accent);
                transition: width 0.2s ease-out;
            }
            .status-line {
                font-size: 0.65rem;
                color: var(--text-secondary);
                display: flex;
                gap: 8px;
            }
            .scanline {
                width: 100%;
                height: 1px;
                background: var(--accent);
                opacity: 0.1;
                margin-top: 10px;
            }
        "#} }

        div { class: "terminal-overlay",
            div { class: "terminal-box",
                // Top info row
                div { class: "terminal-header",
                    span { style: "color: var(--text-secondary);", "PROCESS // {op_name_upper}" }
                    span { style: "color: {status_color}; font-weight: bold;", "{progress_width}%" }
                }

                // Main Progress Bar
                div { class: "progress-container",
                    div { 
                        class: "progress-fill",
                        style: "
                            width: {progress_width}%; 
                            background-color: {status_color};
                            opacity: {pulse_opacity};
                        "
                    }
                }

                // Subtext / Message
                div { class: "status-line",
                    span { style: "color: {status_color}", ">" }
                    span { "{msg_upper}" }
                }

                // Aesthetic "System Decor"
                div { class: "scanline" }
            }
        }
    }
}