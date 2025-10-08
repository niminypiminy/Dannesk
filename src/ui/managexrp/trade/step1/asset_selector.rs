use egui::{Ui, Color32, Stroke, Margin, RichText, Frame, Button};
use crate::ui::managexrp::trade::buffers::{TradeState, update_buffers};
use crate::ui::managexrp::trade::styles::text_color;

pub fn render_asset_selector(ui: &mut Ui, trade_state: &mut TradeState, buffer_id: &str, is_dark_mode: bool) {
    let pairs = vec![
        ("XRP/RLUSD", "XRP", "RLUSD"),
        ("RLUSD/XRP", "RLUSD", "XRP"),
        ("XRP/EUROP", "XRP", "EUROP"),
        ("EUROP/XRP", "EUROP", "XRP"),
        ("RLUSD/EUROP", "RLUSD", "EUROP"),
        ("EUROP/RLUSD", "EUROP", "RLUSD"),
    ];

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        let available_width = ui.available_width();
        let max_width = (available_width * 0.8).clamp(300.0, 600.0);
        ui.set_max_width(max_width);

        let label_font_size = (available_width * 0.04).clamp(12.0, 16.0);
        let button_font_size = (available_width * 0.035).clamp(11.0, 14.0);
        let spacing = (available_width * 0.015).clamp(8.0, 16.0);
        let button_width = (max_width / 2.0 - spacing).clamp(120.0, 280.0);
        let button_height = (available_width * 0.05).clamp(28.0, 36.0);

        ui.add_space(spacing);

        ui.vertical(|ui| {
            ui.set_max_width(max_width);
            Frame::new()
                .inner_margin(Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(RichText::new("Select Trading Pair").size(label_font_size).color(text_color(is_dark_mode)));
                    });
                });

            ui.add_space(spacing);

            let total_grid_width = 2.0 * button_width + spacing;
            ui.add_space((max_width - total_grid_width) / 2.0);

            egui::Grid::new("trading_pairs_grid")
                .num_columns(2)
                .spacing([spacing, spacing / 2.0])
                .show(ui, |ui| {
                    for (pair, base, quote) in pairs.iter() {
                        let is_selected = trade_state.search_query == *pair;
                        let button = Button::new(
                            RichText::new(*pair)
                                .size(button_font_size)
                                .color(if is_selected {
                                    text_color(is_dark_mode) // #222222 in light mode
                                } else {
                                    if is_dark_mode {
                                        Color32::from_rgb(150, 150, 150) // Keep for dark mode
                                    } else {
                                        Color32::from_rgb(30, 30, 30) // Match white_theme fg_stroke
                                    }
                                }),
                        )
                        .min_size(egui::Vec2::new(button_width, button_height))
                        .fill(if is_selected {
                            if is_dark_mode {
                                Color32::from_rgb(50, 50, 50) // Matches dark theme
                            } else {
                                Color32::from_rgb(200, 200, 200) // Matches white_theme active
                            }
                        } else {
                            if is_dark_mode {
                                Color32::TRANSPARENT // Keep for dark mode
                            } else {
                                Color32::from_rgb(220, 220, 220) // Matches white_theme inactive
                            }
                        })
                        .stroke(Stroke::new(
                            1.0,
                            if is_dark_mode {
                                Color32::from_rgb(180, 180, 180) // Matches dark theme
                            } else {
                                Color32::from_rgb(130, 130, 130) // Matches white_theme active/inactive stroke
                            },
                        ));

                        if ui.add(button).clicked() {
                            trade_state.base_asset = base.to_string();
                            trade_state.quote_asset = quote.to_string();
                            trade_state.search_query = pair.to_string();
                            trade_state.error = None;
                            trade_state.amount = String::new();
                            trade_state.limit_price = String::new();
                            update_buffers(
                                buffer_id,
                                trade_state.base_asset.clone(),
                                trade_state.quote_asset.clone(),
                                trade_state.amount.clone(),
                                trade_state.limit_price.clone(),
                                trade_state.flags.clone(),
                                trade_state.passphrase.clone(),
                                trade_state.seed.clone(),
                                trade_state.step,
                                trade_state.done,
                                trade_state.error.clone(),
                                trade_state.fee_percentage,
                                trade_state.search_query.clone(),
                                trade_state.input_mode.clone(),
                            );
                            ui.ctx().request_repaint();
                        }

                        if pairs.iter().position(|p| p.0 == *pair).unwrap() % 2 == 1 {
                            ui.end_row();
                        }
                    }
                });

            ui.add_space((max_width - total_grid_width) / 2.0);
            ui.add_space(spacing);
        });
    });
}