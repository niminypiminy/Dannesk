use egui::{Ui, Align, Visuals, Color32, RichText, Sense};
use chrono::{Local, Timelike};
use crate::channel::{CHANNEL, Tab, WSCommand};
use crate::ui::{balance, managexrp, managebtc, banner};
use crate::ui::progressbar::ProgressBarState;
use crate::utils::svg_render::SvgCanvas;
use tokio::sync::mpsc;

pub fn render_dashboard(ui: &mut Ui, commands_tx: mpsc::Sender<WSCommand>) -> Option<(bool, String, bool)> {
    let mut action = None;

    let (is_dark_mode, user_name, hide_balance) = CHANNEL.theme_user_rx.borrow().clone();
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
    if progress_bar.render_progress_bar(ui, 300.0) {
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
        // Render WebSocket banner if WebSockets are down
        if modal_state.websocket {
            banner::render_websocket_banner(ui);
        }

        // Scope for header with tabs, icons, and greeting
        ui.scope(|ui| {
            // Set button padding for consistent spacing
            ui.style_mut().spacing.button_padding = egui::vec2(10.0, 5.0);

            ui.horizontal(|ui| {
                // Left side: Tabs (Balance, XRP, BTC)
                ui.horizontal(|ui| {
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
                });

                // Center: Icon buttons (theme toggle, settings, name, exchange, hide balance)
                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                    // Calculate available width and allocate space to center icons
                    let available_width = ui.available_width();
                    let icon_group_width = 120.0; // 5 icons (16.0 each) + 4 spaces (10.0 each) = 80 + 40 = 120
                    ui.add_space(((available_width - icon_group_width) / 2.0) - 160.0); // Adjusted to shift left

                    ui.horizontal(|ui| {
                        // Theme toggle button (Sun/Moon)
                        let icon = if is_dark_mode { "☀" } else { "🌙" }; // Sun (U+2600), Moon (U+1F319)
                        if ui.button(icon).clicked() {
                            let new_dark_mode = !is_dark_mode;
                            action = Some((new_dark_mode, user_name.clone(), hide_balance));
                            ui.ctx().set_visuals(if new_dark_mode { Visuals::dark() } else { Visuals::light() });
                        }

                        ui.add_space(10.0);
                        // Settings button (Pin icon)
                        if ui
                            .add(
                                egui::Button::image(
                                    SvgCanvas::paint_svg("pin.svg")
                                        .fit_to_exact_size(egui::vec2(16.0, 16.0))
                                        .tint(text_color),
                                )
                                .sense(Sense::click())
                            )
                            .clicked()
                        {
                            let mut new_state = CHANNEL.modal_rx.borrow().clone();
                            new_state.settings = true;
                            let _ = CHANNEL.modal_tx.send(new_state);
                        }

                        ui.add_space(10.0);
                        // Name button (Unicode person icon)
                        if ui
                            .add(
                                egui::Button::new(RichText::new("👤").size(15.0).color(text_color))
                                    .sense(Sense::click())
                            )
                            .clicked()
                        {
                            let mut new_state = CHANNEL.modal_rx.borrow().clone();
                            new_state.name = true;
                            let _ = CHANNEL.modal_tx.send(new_state);
                        }

                        ui.add_space(10.0);
                        // Exchange button (Exchange icon)
                        if ui
                            .add(
                                egui::Button::image(
                                    SvgCanvas::paint_svg("exchange.svg")
                                        .fit_to_exact_size(egui::vec2(16.0, 16.0))
                                        .tint(text_color),
                                )
                                .sense(Sense::click())
                            )
                            .clicked()
                        {
                            let mut new_state = CHANNEL.modal_rx.borrow().clone();
                            new_state.exchange = true;
                            let _ = CHANNEL.modal_tx.send(new_state);
                        }

                        ui.add_space(10.0);
                        // Hide balance toggle button (Eyeball icons)
                        let eye_icon = if hide_balance {
                            SvgCanvas::paint_svg("hidden.svg")
                        } else {
                            SvgCanvas::paint_svg("nothidden.svg")
                        };
                        let lock_color = if hide_balance {
                            Color32::from_rgb(180, 180, 180) // Gray when balance is hidden
                        } else {
                            text_color // Same color as other icons
                        };
                        if ui
                            .add(
                                egui::Button::image(
                                    eye_icon
                                        .fit_to_exact_size(egui::vec2(16.0, 16.0))
                                        .tint(lock_color),
                                )
                                .sense(Sense::click())
                            )
                            .clicked()
                        {
                            let new_hide_balance = !hide_balance;
                            action = Some((is_dark_mode, user_name.clone(), new_hide_balance));
                        }
                    });
                });

                // Right side: Greeting with non-clickable name
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(10.0);
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
                    ui.add(egui::Label::new(ui.fonts(|f| f.layout_job(job))));
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

    action
}