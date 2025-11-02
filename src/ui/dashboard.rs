use egui::{Ui, RichText};
use chrono::{Local, Timelike};
use crate::channel::{CHANNEL, Tab, WSCommand};
use crate::ui::{balance, managexrp, managebtc, dropdown};
use crate::ui::progressbar::ProgressBarState;
use tokio::sync::mpsc;

pub fn render_dashboard(ui: &mut Ui, commands_tx: mpsc::Sender<WSCommand>) -> Option<(bool, String, bool)> {
    let mut action = None;

    let (is_dark_mode, user_name, _hide_balance) = CHANNEL.theme_user_rx.borrow().clone();
    let selected_tab = *CHANNEL.selected_tab_rx.borrow();
    let mut modal_state = CHANNEL.modal_rx.borrow().clone();

    // Fetch WebSocket status directly from watch channels
    let exchange_connected = *CHANNEL.exchange_ws_status_rx.borrow();
    let crypto_connected = *CHANNEL.crypto_ws_status_rx.borrow();
    modal_state.websocket = !exchange_connected || !crypto_connected;
    let _ = CHANNEL.modal_tx.send(modal_state.clone());

    // Initialize progress bar
    let mut progress_bar = ProgressBarState::new("processing...".to_string());

    // Render progress bar first to prioritize overlay
    if progress_bar.render_progress_bar(ui, 400.0) {
        return None;
    }

    // Define text color based on theme
    let text_color = if is_dark_mode {
        egui::Color32::from_rgb(255, 254, 250) // #fffefa for dark mode
    } else {
        egui::Color32::from_rgb(34, 34, 34) // #222222 for light mode
    };

    // Override text color for tab labels in light theme only
    let tab_text_color = if is_dark_mode {
        None // Use theme's text color (#fffefa) for dark mode
    } else {
        Some(egui::Color32::from_rgb(34, 34, 34)) // #222222 for light theme
    };

    ui.vertical(|ui| {
        // Scope for header with tabs and greeting (center icons removed)
        ui.scope(|ui| {
            // Set button padding for consistent spacing
            ui.style_mut().spacing.button_padding = egui::vec2(10.0, 5.0);

            ui.horizontal(|ui| {
                // Left side: Tabs (Balance, XRP, BTC) + Settings dropdown
                ui.horizontal(|ui| {
                    let settings_label = RichText::new("Settings").color(tab_text_color.unwrap_or(ui.style().visuals.widgets.active.fg_stroke.color));

                    // Balance tab
                    if ui
                        .selectable_label(
                            selected_tab == Tab::Balance,
                            RichText::new("Balance").color(tab_text_color.unwrap_or(ui.style().visuals.widgets.active.fg_stroke.color)),
                        )
                        .clicked()
                    {
                        let _ = CHANNEL.selected_tab_tx.send(Tab::Balance);
                    }

                    // XRP tab
                    if ui
                        .selectable_label(
                            selected_tab == Tab::XRP,
                            RichText::new("XRP").color(tab_text_color.unwrap_or(ui.style().visuals.widgets.active.fg_stroke.color)),
                        )
                        .clicked()
                    {
                        let _ = CHANNEL.selected_tab_tx.send(Tab::XRP);
                    }

                    // BTC tab
                    if ui
                        .selectable_label(
                            selected_tab == Tab::BTC,
                            RichText::new("BTC").color(tab_text_color.unwrap_or(ui.style().visuals.widgets.active.fg_stroke.color)),
                        )
                        .clicked()
                    {
                        let _ = CHANNEL.selected_tab_tx.send(Tab::BTC);
                    }

                    // Settings button (toggle)
                    let settings_response = ui.selectable_label(false, settings_label);
                    if settings_response.clicked() {
                        let new_open = !*CHANNEL.settings_dropdown_rx.borrow();
                        let _ = CHANNEL.settings_dropdown_tx.send(new_open);
                    }

                    // Store the button rect for positioning the area (temp data)
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(egui::Id::new("settings_button_rect"), settings_response.rect);
                    });
                });

                // Right side: Greeting with non-clickable name (WebSocket status indicator removed to avoid clutter)
                // (Pushed to right with add_space)
                ui.horizontal(|ui| {
                    // Compute the greeting layout job and galley for width measurement
                    let now = Local::now().hour();
                    let greeting = match now {
                        5..=11 => "Good morning",
                        12..=16 => "Good afternoon",
                        _ => "Good evening",
                    };
                    let mut job = egui::text::LayoutJob::default();
                    job.append(
                        &format!("{}, ", greeting),
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::new(16.0, egui::FontFamily::Name("DejaVuSansMono".into())),
                            color: text_color,
                            ..Default::default()
                        },
                    );
                    let name_format = egui::TextFormat {
                        font_id: egui::FontId::new(16.0, egui::FontFamily::Name("DejaVuSansMonoBold".into())),
                        color: text_color,
                        ..Default::default()
                    };
                    job.append(&user_name, 0.0, name_format);
                    let galley = ui.fonts(|f| f.layout_job(job));

                    // Dimensions
                    let greeting_width = galley.size().x;
                    let margin_to_edge = 10.0;

                    let elements_width = greeting_width;
                    ui.add_space(ui.available_width() - elements_width - margin_to_edge);

                    // Add greeting
                    ui.add(egui::Label::new(galley));
                });
            });
        });
    });

    // Render tab content below
    match selected_tab {
        Tab::Balance => {
            balance::render_balance(ui);
        }
        Tab::XRP => {
            managexrp::render_manage_xrp(ui, commands_tx.clone());
        }
        Tab::BTC => {
            managebtc::render_manage_btc(ui, commands_tx.clone());
        }
    }

    // Delegate dropdown rendering/actions to dropdown.rs (forward any action for caller)
    if let Some(dropdown_action) = dropdown::render(ui.ctx()) {
        action = Some(dropdown_action);
    }

    action
}