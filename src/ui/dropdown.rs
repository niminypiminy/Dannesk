use egui::{Context, Ui, Color32, RichText, Sense, vec2, Id, pos2, Area, Frame};
use crate::channel::CHANNEL;
use crate::utils::svg_render::SvgCanvas;

pub fn render(ctx: &Context) -> Option<(bool, String, bool)> {
    let settings_open = *CHANNEL.settings_dropdown_rx.borrow();
    if !settings_open {
        return None;
    }

    // Retrieve button rect (stored by dashboard.rs)
    let button_rect = ctx.data_mut(|d| 
        d.get_temp(Id::new("settings_button_rect"))
            .map(|r: &egui::Rect| r.clone())
            .unwrap_or_else(|| egui::Rect::from_min_size(pos2(0.0, 0.0), vec2(0.0, 0.0)))
    );

    let (is_dark_mode, _, _) = CHANNEL.theme_user_rx.borrow().clone();
    let text_color = if is_dark_mode {
        Color32::from_rgb(255, 254, 250)
    } else {
        Color32::from_rgb(34, 34, 34)
    };

    let dropdown_size = vec2(250.0, 150.0);

    let mut action = None;
    Area::new(Id::new("settings_dropdown"))
        .fixed_pos(button_rect.right_top() + vec2(0.0, 5.0))
        .show(ctx, |ui| {
            Frame::default()
                .fill(ui.style().visuals.panel_fill)
                .stroke(egui::Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_min_size(dropdown_size);
                    ui.set_max_size(dropdown_size);

                    // Render content and capture action (no auto-close here—let user control)
                    if let Some(settings_action) = render_settings_dropdown(ui) {
                        action = Some(settings_action);
                        // Removed: let _ = CHANNEL.settings_dropdown_tx.send(false);
                    }
                });
        });

    // Apply theme change immediately if action involves it
    if let Some((new_dark_mode, _, _)) = &action {
        if *new_dark_mode != is_dark_mode {
            ctx.set_visuals(if *new_dark_mode { egui::Visuals::dark() } else { egui::Visuals::light() });
        }
    }

    action
}

// Existing content renderer (unchanged—modals auto-close only)
pub fn render_settings_dropdown(ui: &mut Ui) -> Option<(bool, String, bool)> {
    let mut action = None;

    let (is_dark_mode, user_name, hide_balance) = CHANNEL.theme_user_rx.borrow().clone();

    // Fetch WebSocket status for potential use in sidebar (e.g., status indicator)
    let exchange_connected = *CHANNEL.exchange_ws_status_rx.borrow();
    let crypto_connected = *CHANNEL.crypto_ws_status_rx.borrow();

    // Define text color based on theme (match dashboard)
    let text_color = if is_dark_mode {
        egui::Color32::from_rgb(255, 254, 250) // #fffefa for dark mode
    } else {
        egui::Color32::from_rgb(34, 34, 34) // #222222 for light mode
    };

    ui.vertical(|ui| {
        // Close button at top-right
        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() - 20.0);
            if ui.button(RichText::new("×").size(14.0).color(text_color)).on_hover_text("Close settings").clicked() {
                let _ = CHANNEL.settings_dropdown_tx.send(false);
            }
        });
        ui.add_space(5.0);

        // Theme toggle (Sun/Moon with label) - as a button in dropdown
        if ui.button(RichText::new(format!("{} Theme", if is_dark_mode { "☀" } else { "🌙" })).size(14.0).color(text_color)).clicked() {
            let new_dark_mode = !is_dark_mode;
            action = Some((new_dark_mode, user_name.clone(), hide_balance));
        }
        ui.add_space(2.0);

        // Settings button (Pin icon with label) - as a button in dropdown
        ui.horizontal(|ui| {
            let resp = ui.add(
                egui::Button::image(
                    SvgCanvas::paint_svg("pin.svg")
                        .fit_to_exact_size(egui::vec2(16.0, 16.0))
                        .tint(text_color),
                )
                .sense(Sense::click())
                .min_size(egui::vec2(20.0, 20.0)),
            );
            ui.label(RichText::new("Settings").size(14.0).color(text_color));
            if resp.clicked() {
                let mut new_state = CHANNEL.modal_rx.borrow().clone();
                new_state.settings = true;
                let _ = CHANNEL.modal_tx.send(new_state);
                // Auto-close dropdown when opening modal
                let _ = CHANNEL.settings_dropdown_tx.send(false);
            }
        });
        ui.add_space(2.0);

        // Name button (Person icon with name) - as a button in dropdown
        ui.horizontal(|ui| {
            let resp = ui.add(
                egui::Button::new(RichText::new("👤").size(15.0).color(text_color))
                    .sense(Sense::click())
                    .min_size(egui::vec2(20.0, 20.0)),
            );
            ui.label(RichText::new(format!("Account: {}", user_name)).size(14.0).color(text_color));
            if resp.clicked() {
                let mut new_state = CHANNEL.modal_rx.borrow().clone();
                new_state.name = true;
                let _ = CHANNEL.modal_tx.send(new_state);
                // Auto-close dropdown when opening modal
                let _ = CHANNEL.settings_dropdown_tx.send(false);
            }
        });
        ui.add_space(2.0);

        // Exchange button (Exchange icon with label) - as a button in dropdown
        ui.horizontal(|ui| {
            let resp = ui.add(
                egui::Button::image(
                    SvgCanvas::paint_svg("exchange.svg")
                        .fit_to_exact_size(egui::vec2(16.0, 16.0))
                        .tint(text_color),
                )
                .sense(Sense::click())
                .min_size(egui::vec2(20.0, 20.0)),
            );
            ui.label(RichText::new("Exchange").size(14.0).color(text_color));
            if resp.clicked() {
                let mut new_state = CHANNEL.modal_rx.borrow().clone();
                new_state.exchange = true;
                let _ = CHANNEL.modal_tx.send(new_state);
                // Auto-close dropdown when opening modal
                let _ = CHANNEL.settings_dropdown_tx.send(false);
            }
        });
        ui.add_space(2.0);

        // Hide balance toggle (Eye icons with label) - as a button in dropdown
        ui.horizontal(|ui| {
            let eye_icon = if hide_balance {
                SvgCanvas::paint_svg("hidden.svg")
            } else {
                SvgCanvas::paint_svg("nothidden.svg")
            };
            let lock_color = if hide_balance {
                Color32::from_rgb(180, 180, 180) // Gray when hidden
            } else {
                text_color
            };
            let resp = ui.add(
                egui::Button::image(
                    eye_icon
                        .fit_to_exact_size(egui::vec2(16.0, 16.0))
                        .tint(lock_color),
                )
                .sense(Sense::click())
                .min_size(egui::vec2(20.0, 20.0)),
            );
            let label_text = if hide_balance { "Balance: Hidden" } else { "Balance: Visible" };
            ui.label(RichText::new(label_text).size(14.0).color(text_color));
            if resp.clicked() {
                let new_hide_balance = !hide_balance;
                action = Some((is_dark_mode, user_name.clone(), new_hide_balance));
            }
        });
        ui.add_space(5.0);

        // WebSocket status indicator (small section at bottom of dropdown - separate for exchange and crypto)
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(RichText::new("WS Status:").size(12.0).color(text_color));
            ui.add_space(5.0);

            // Exchange WS circle
            let exchange_tooltip = if exchange_connected {
                "Exchange WS: Connected"
            } else {
                "Exchange WS: Disconnected"
            };
            let exchange_size = vec2(12.0, 16.0);
            let (exchange_rect, exchange_resp) = ui.allocate_exact_size(exchange_size, egui::Sense::hover());
            let exchange_color = if exchange_connected {
                Color32::from_rgb(0, 255, 0)
            } else {
                Color32::from_rgb(255, 0, 0)
            };
            ui.painter().circle_filled(exchange_rect.center(), 5.0, exchange_color);
            exchange_resp.on_hover_text(exchange_tooltip);
            ui.label(RichText::new("Exchange").size(10.0).color(text_color));

            ui.add_space(10.0);

            // Crypto WS circle
            let crypto_tooltip = if crypto_connected {
                "Crypto WS: Connected"
            } else {
                "Crypto WS: Disconnected"
            };
            let crypto_size = vec2(12.0, 16.0);
            let (crypto_rect, crypto_resp) = ui.allocate_exact_size(crypto_size, egui::Sense::hover());
            let crypto_color = if crypto_connected {
                Color32::from_rgb(0, 255, 0)
            } else {
                Color32::from_rgb(255, 0, 0)
            };
            ui.painter().circle_filled(crypto_rect.center(), 5.0, crypto_color);
            crypto_resp.on_hover_text(crypto_tooltip);
            ui.label(RichText::new("Crypto").size(10.0).color(text_color));
        });
    });

    action
}