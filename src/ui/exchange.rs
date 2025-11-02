use egui::{Ui, RichText, Grid, Frame, Color32, Area, Pos2, Vec2, Align2};
use crate::channel::CHANNEL;
use crate::utils::svg_render::SvgCanvas; // Import SvgCanvas
use std::collections::HashMap;

pub fn render(ui: &mut Ui, rates: &HashMap<String, f32>, text_color: Color32, is_dark_mode: bool) -> bool {
    let mut should_close = false;

    let screen_size = ui.ctx().input(|i| i.screen_rect.size());
    // Dynamic modal size: 50% of screen width, 50% of screen height, clamped
    let modal_size = Vec2::new(
        (screen_size.x * 0.5).clamp(400.0, 750.0),
        (screen_size.y * 0.5).clamp(300.0, 600.0),
    );
    let pos = Pos2::new(
        (screen_size.x - modal_size.x) / 2.0,
        (screen_size.y - modal_size.y) / 2.0,
    );

    Area::new(egui::Id::new("exchange_rates_overlay"))
        .fixed_pos(pos)
        .anchor(Align2::CENTER_CENTER, Vec2::splat(0.0))
        .show(ui.ctx(), |ui| {
            ui.painter().rect_filled(
                ui.ctx().input(|i| i.screen_rect),
                0.0,
                Color32::from_black_alpha(200),
            );

            Frame::popup(ui.style())
                .fill(ui.style().visuals.panel_fill)
                .stroke(egui::Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
                .outer_margin(0.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.set_min_size(modal_size);
                    ui.set_max_size(modal_size);

                    Area::new(egui::Id::new("exchange_rates_close_button"))
                        .anchor(Align2::RIGHT_TOP, Vec2::new(-5.0, 5.0))
                        .show(ui.ctx(), |ui| {
                            let button_size = (modal_size.x * 0.05).clamp(16.0, 18.0);
                            if ui.button(RichText::new("X").size(button_size).color(text_color)).clicked() {
                                should_close = true;
                            }
                        });

                    ui.allocate_ui_with_layout(
                        modal_size,
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            let available_width = ui.available_width();
                            // Dynamic font sizes, increased by ~25%
                            let title_size = (available_width * 0.06).clamp(22.0, 30.0);
                            let text_size = (available_width * 0.045).clamp(15.0, 18.0);
                            let spacing = (available_width * 0.04).clamp(10.0, 15.0);

                            ui.add_space(spacing);
                            ui.label(RichText::new("Exchange Rates").size(title_size).strong().color(text_color));
                            ui.add_space(spacing);

                            // Center the grid
                            let total_grid_width = (available_width * 0.85).min(600.0);
                            if available_width > total_grid_width {
                                ui.add_space((available_width - total_grid_width) / 2.0);
                            }

                            Frame::NONE
                                .outer_margin(egui::Margin {
                                    left: (40.0 * (available_width / 1000.0).clamp(0.5, 1.0)) as i8,
                                    right: 0,
                                    top: 0,
                                    bottom: 0,
                                })
                                .show(ui, |ui| {
                                    let col_width = (total_grid_width - 20.0) / 2.0;
                                    Grid::new("exchange_rates_grid")
                                        .striped(true)
                                        .num_columns(2)
                                        .spacing([12.0 * (available_width / 1000.0).clamp(0.5, 1.0), 6.0])
                                        .min_col_width(col_width)
                                        .show(ui, |ui| {
                                            ui.label(RichText::new("Pair").size(text_size).strong().color(text_color));
                                            ui.label(RichText::new("Rate").size(text_size).strong().color(text_color));
                                            ui.end_row();

                                            // Icon size scales with screen width, increased by 25%
                                            let icon_size = 20.0 * (available_width / 1000.0).clamp(0.5, 1.0);

                                            // XRP/USD
                                            if let Some(xrp_usd) = rates.get("XRP/USD") {
                                                ui.horizontal(|ui| {
                                                    ui.add(
    SvgCanvas::paint_svg(if is_dark_mode { "xrp_white.svg" } else { "xrp_dark.svg" })
        .fit_to_exact_size(egui::vec2(icon_size, icon_size)),
);
                                                    ui.add_space(5.0 * (available_width / 1000.0).clamp(0.5, 1.0));
                                                    ui.label(RichText::new("XRP/USD").size(text_size).color(text_color));
                                                });
                                                ui.label(RichText::new(format!("${:.4}", xrp_usd)).size(text_size).color(text_color));
                                                ui.end_row();
                                            }

                                            // BTC/USD
                                            if let Some(btc_usd) = rates.get("BTC/USD") {
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        SvgCanvas::paint_svg("btc.svg")
                                                            .fit_to_exact_size(egui::vec2(icon_size, icon_size))
                                                            
                                                    );
                                                    ui.add_space(5.0 * (available_width / 1000.0).clamp(0.5, 1.0));
                                                    ui.label(RichText::new("BTC/USD").size(text_size).color(text_color));
                                                });
                                                ui.label(RichText::new(format!("${:.2}", btc_usd)).size(text_size).color(text_color));
                                                ui.end_row();
                                            }

                                            // EUR/USD
                                            if let Some(eur_usd) = rates.get("EUR/USD") {
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        SvgCanvas::paint_svg("europ.svg")
                                                            .fit_to_exact_size(egui::vec2(icon_size, icon_size))
                                                            
                                                    );
                                                    ui.add_space(5.0 * (available_width / 1000.0).clamp(0.5, 1.0));
                                                    ui.label(RichText::new("EUR/USD").size(text_size).color(text_color));
                                                });
                                                ui.label(RichText::new(format!("${:.2}", eur_usd)).size(text_size).color(text_color));
                                                ui.end_row();
                                            }

                                            // XRP/EUR
                                            if let Some(xrp_eur) = rates.get("XRP/EUR") {
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        SvgCanvas::paint_svg(if is_dark_mode { "xrp_white.svg" } else { "xrp_dark.svg" })
                                                            .fit_to_exact_size(egui::vec2(icon_size, icon_size))
                                                          
                                                    );
                                                    ui.add_space(5.0 * (available_width / 1000.0).clamp(0.5, 1.0));
                                                    ui.label(RichText::new("XRP/EUR").size(text_size).color(text_color));
                                                });
                                                ui.label(RichText::new(format!("€{:.4}", xrp_eur)).size(text_size).color(text_color));
                                                ui.end_row();
                                            }

                                            // BTC/EUR
                                            if let Some(btc_eur) = rates.get("BTC/EUR") {
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        SvgCanvas::paint_svg("btc.svg")
                                                            .fit_to_exact_size(egui::vec2(icon_size, icon_size))
                                                    );
                                                    ui.add_space(5.0 * (available_width / 1000.0).clamp(0.5, 1.0));
                                                    ui.label(RichText::new("BTC/EUR").size(text_size).color(text_color));
                                                });
                                                ui.label(RichText::new(format!("€{:.2}", btc_eur)).size(text_size).color(text_color));
                                                ui.end_row();
                                            }

                                            // RLUSD/USD
                                            ui.horizontal(|ui| {
                                                ui.add(
                                                    SvgCanvas::paint_svg("rlusd.svg")
                                                        .fit_to_exact_size(egui::vec2(icon_size, icon_size))
                                                );
                                                ui.add_space(5.0 * (available_width / 1000.0).clamp(0.5, 1.0));
                                                ui.label(RichText::new("RLUSD/USD").size(text_size).color(text_color));
                                            });
                                            ui.label(RichText::new("$1.00").size(text_size).color(text_color));
                                            ui.end_row();

                                            // EUROP/EUR
                                            ui.horizontal(|ui| {
                                                ui.add(
                                                    SvgCanvas::paint_svg("europ.svg")
                                                        .fit_to_exact_size(egui::vec2(icon_size, icon_size))
                                                );
                                                ui.add_space(5.0 * (available_width / 1000.0).clamp(0.5, 1.0));
                                                ui.label(RichText::new("EUROP/EUR").size(text_size).color(text_color));
                                            });
                                            ui.label(RichText::new("€1.00").size(text_size).color(text_color));
                                            ui.end_row();
                                        });
                                });
                            ui.add_space(spacing);
                        },
                    );
                });
        });

    if should_close {
        let mut new_state = CHANNEL.modal_rx.borrow().clone();
        new_state.exchange = false;
        let _ = CHANNEL.modal_tx.send(new_state);
    }

    should_close
}