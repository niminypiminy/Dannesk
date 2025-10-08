use egui::{Ui, RichText, Color32, Stroke};
use crate::ui::managexrp::trade::buffers::{TradeState, update_buffers};
use crate::ui::managexrp::trade::styles::text_color;

pub fn render_trade_flags(ui: &mut Ui, trade_state: &mut TradeState, buffer_id: &str, is_dark_mode: bool) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        let available_width = ui.available_width();
        let max_width = (available_width * 0.8).clamp(300.0, 600.0);
        ui.set_max_width(max_width);

        let label_font_size = (available_width * 0.04).clamp(12.0, 16.0);
        let spacing = (available_width * 0.015).clamp(8.0, 16.0);

        let button_width = (max_width / 2.0 - spacing).clamp(120.0, 280.0);
        let total_content_width = 2.0 * button_width + spacing;
        let padding = (max_width - total_content_width).max(0.0) / 2.0;

        ui.horizontal(|ui| {
            ui.add_space(padding);

            let original_visuals = ui.visuals().clone();
            if !is_dark_mode {
                ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, text_color(is_dark_mode));
                ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(2.0, text_color(is_dark_mode));
                ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(210, 210, 210); // Match white_theme widgets.hovered.bg_fill
            }

            for (flag, _desc) in [
                ("FillOrKill", "Cancel if not filled completely"),
                ("ImmediateOrCancel", "Cancel if not filled immediately"),
            ] {
                let flag_value = format!("tf{}", flag);
                let is_selected = trade_state.flags.contains(&flag_value);

                let label = format!("{}", flag);
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new(label)
                                .size(label_font_size)
                                .color(text_color(is_dark_mode)),
                        )
                        .min_size(egui::vec2(button_width, 24.0))
                        .fill(if is_selected {
                            if is_dark_mode {
                                Color32::from_rgb(50, 50, 50) // Match dark theme and render_asset_selector
                            } else {
                                Color32::from_rgb(200, 200, 200) // Match white_theme widgets.active.bg_fill
                            }
                        } else {
                            if is_dark_mode {
                                Color32::from_rgba_premultiplied(50, 50, 50, 200) // Keep for dark mode
                            } else {
                                Color32::from_rgb(220, 220, 220) // Match white_theme widgets.inactive.bg_fill
                            }
                        })
                        .stroke(if is_selected {
                            Stroke::new(1.0, if is_dark_mode {
                                Color32::from_rgb(180, 180, 180) // Match render_asset_selector dark mode
                            } else {
                                Color32::from_rgb(130, 130, 130) // Match white_theme widgets.active.bg_stroke
                            })
                        } else {
                            Stroke::new(0.5, if is_dark_mode {
                                Color32::from_rgb(180, 180, 180)
                            } else {
                                Color32::from_rgb(130, 130, 130) // Match white_theme widgets.inactive.bg_stroke
                            })
                        }),
                    )
                    .clicked()
                {
                    if is_selected {
                        trade_state.flags.retain(|x| x != &flag_value);
                    } else {
                        let other_flag = if flag == "FillOrKill" {
                            "tfImmediateOrCancel"
                        } else {
                            "tfFillOrKill"
                        };
                        trade_state.flags.retain(|x| x != other_flag);
                        trade_state.flags.push(flag_value.clone());
                    }
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
                }
                ui.add_space(spacing);
            }

            ui.visuals_mut().widgets = original_visuals.widgets;
            ui.add_space(padding);
        });
        ui.add_space(spacing);
    });
}