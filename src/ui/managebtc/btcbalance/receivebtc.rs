use egui::{Ui, RichText, Area, Pos2, Vec2, Color32, Frame, Align2, Margin};
use crate::channel::{CHANNEL, BTCModalState, BTCActiveView};

pub fn render(ui: &mut Ui, wallet_address: &Option<String>) -> bool {
    let mut should_close = false;

    // Calculate overlay position (centered)
    let screen_size = ui.ctx().input(|i| i.screen_rect.size());
    let modal_size = Vec2::new(400.0, 60.0); // Increased height for better spacing
    let pos = Pos2::new(
        (screen_size.x - modal_size.x) / 2.0,
        (screen_size.y - modal_size.y) / 2.0,
    );

    // Render overlay using Area
    Area::new(egui::Id::new("receive_overlay"))
        .fixed_pos(pos)
        .anchor(Align2::CENTER_CENTER, Vec2::splat(0.0))
        .show(ui.ctx(), |ui| {
            // Semi-transparent background
            ui.painter().rect_filled(
                ui.ctx().input(|i| i.screen_rect),
                0.0,
                Color32::from_black_alpha(200),
            );

            // Overlay content frame
            Frame::popup(ui.style())
                .fill(ui.style().visuals.panel_fill)
                .stroke(egui::Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
                .outer_margin(0.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_min_size(modal_size);
                    ui.set_max_size(modal_size);

                    // Close button in top-right corner
                    Area::new(egui::Id::new("receive_close_button"))
                        .anchor(Align2::RIGHT_TOP, Vec2::new(-5.0, 5.0))
                        .show(ui.ctx(), |ui| {
                            if ui.button(RichText::new("X").size(14.0)).clicked() {
                                should_close = true;
                            }
                        });

                    // Main content with centered layout
                    ui.allocate_ui_with_layout(
                        modal_size,
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            ui.add_space(10.0);
                            match wallet_address {
                                Some(address) => {
                                    let text_color = ui.style().visuals.text_color();
                                    ui.label(RichText::new(address).size(14.0).color(text_color));
                                    ui.add_space(12.0); // Increased spacing for breathing room
                                    // Modernized copy button
                                    ui.vertical_centered(|ui| {
                                        let original_visuals = ui.visuals().clone();
                                        if !ui.style().visuals.dark_mode {
                                            ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
                                            ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
                                        }
                                        Frame::new() // egui 0.31.1, no ID argument
                                            .inner_margin(Margin::symmetric(8, 4)) // Integer margins
                                            .show(ui, |ui| {
                                                if ui
                                                    .add(
                                                        egui::Button::new(RichText::new("Copy Address").size(14.0).color(text_color))
                                                            .min_size(egui::Vec2::new(100.0, 28.0)), // Slightly larger button
                                                    )
                                                    .clicked()
                                                {
                                                    ui.ctx().copy_text(address.clone());
                                                }
                                            });
                                        ui.visuals_mut().widgets = original_visuals.widgets;
                                    });
                                }
                                None => {
                                    ui.label(RichText::new("No wallet address available")
                                        .size(14.0)
                                        .color(Color32::from_rgb(255, 100, 100)));
                                    ui.add_space(10.0);
                                }
                            }
                            ui.add_space(10.0);
                        },
                    );
                });
        });

    // If the modal should close, update state via channel
    if should_close {
        let _ = CHANNEL.btc_modal_tx.send(BTCModalState {
            import_wallet: None,
            create_wallet: None,
            view_type: BTCActiveView::BTC,
        });
    }

    should_close
}