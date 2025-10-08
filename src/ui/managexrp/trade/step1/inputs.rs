use egui::{Ui, RichText, Frame, Color32, Margin, Stroke};
use crate::ui::managexrp::trade::buffers::{TradeState, update_buffers};
use crate::ui::managexrp::trade::styles::{text_color, styled_text_edit};
use crate::channel::CHANNEL;

pub fn render_amount_and_price_inputs(ui: &mut Ui, trade_state: &mut TradeState, buffer_id: &str, is_dark_mode: bool) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        let available_width = ui.available_width();
        let max_width = (available_width * 0.8).clamp(300.0, 600.0);
        ui.set_max_width(max_width);

        // Dynamic font sizes and spacing
        let label_font_size = (available_width * 0.04).clamp(12.0, 16.0);
        let hint_font_size = (available_width * 0.035).clamp(10.0, 14.0);
        let spacing = (available_width * 0.015).clamp(8.0, 16.0);

        // Calculate rates and total cost
        let rates_rx = CHANNEL.rates_rx.clone();
        let rates = rates_rx.borrow().clone();
        let xrp_usd = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;
        let eur_usd = rates.get("EUR/USD").copied().unwrap_or(0.0) as f64;
        let xrp_eur = rates.get("XRP/EUR").copied().unwrap_or(0.0) as f64;

        let rate = match (trade_state.base_asset.as_str(), trade_state.quote_asset.as_str()) {
            ("XRP", "RLUSD") => xrp_usd,
            ("RLUSD", "XRP") => if xrp_usd > 0.0 { 1.0 / xrp_usd } else { 0.0 },
            ("XRP", "EUROP") => xrp_eur,
            ("EUROP", "XRP") => if xrp_eur > 0.0 { 1.0 / xrp_eur } else { 0.0 },
            ("RLUSD", "EUROP") => if eur_usd > 0.0 { 1.0 / eur_usd } else { 0.0 },
            ("EUROP", "RLUSD") => eur_usd,
            _ => 0.0,
        };

        let amount: f64 = trade_state.amount.parse().unwrap_or(0.0);
        let fee_multiplier = 1.0 + (trade_state.fee_percentage / 100.0);
        let total_cost = amount * rate * fee_multiplier;

        // Horizontal layout for inputs
        ui.horizontal(|ui| {
            ui.add_space(spacing / 2.0); // Adjusted left spacing

            ui.vertical(|ui| {
                let input_width = (max_width / 2.0 - spacing).clamp(120.0, 280.0);
                ui.set_max_width(input_width);
                ui.add_space(spacing);
                Frame::new()
                    .inner_margin(Margin::symmetric(8, 4))
                    .show(ui, |ui| {
                        ui.label(RichText::new(format!("Amount ({})", trade_state.base_asset))
                            .size(label_font_size)
                            .color(text_color(is_dark_mode)));
                    });
                ui.add_space(spacing / 2.0);
                let response = styled_text_edit(
                    ui,
                    &mut trade_state.amount,
                    is_dark_mode,
                    false,
                    RichText::new(format!("Enter amount in {}", trade_state.base_asset))
                        .size(hint_font_size)
                        .color(Color32::from_gray(100)),
                    Some(input_width),
                );
                if response.changed() {
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
            });

            ui.add_space(spacing);

            ui.vertical(|ui| {
                let input_width = (max_width / 2.0 - spacing).clamp(120.0, 280.0);
                ui.set_max_width(input_width);
                ui.add_space(spacing);
                Frame::new()
                    .inner_margin(Margin::symmetric(8, 4))
                    .show(ui, |ui| {
                        ui.label(RichText::new(format!("Price ({})", trade_state.quote_asset))
                            .size(label_font_size)
                            .color(text_color(is_dark_mode)));
                    });
                ui.add_space(spacing / 2.0);
                let response = styled_text_edit(
                    ui,
                    &mut trade_state.limit_price,
                    is_dark_mode,
                    false,
                    RichText::new(format!("Enter price in {}", trade_state.quote_asset))
                        .size(hint_font_size)
                        .color(Color32::from_gray(100)),
                    Some(input_width),
                );
                if response.changed() {
                    trade_state.fee_percentage = 0.0; // Reset fee_percentage when price is manually edited
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
                        trade_state.fee_percentage, // Use updated fee_percentage
                        trade_state.search_query.clone(),
                        trade_state.input_mode.clone(),
                    );
                }
            });
        });

        // Expected cost label
        let expect_text = format!(
            "Expect to pay {:.4} {} (incl. {:.2}% fee) for {} {}",
            total_cost, trade_state.quote_asset, trade_state.fee_percentage, amount, trade_state.base_asset
        );

        if !expect_text.is_empty() && amount > 0.0 {
            ui.add_space(spacing);
            ui.label(
                RichText::new(expect_text)
                    .size(hint_font_size)
                    .color(text_color(is_dark_mode))
                    .italics(),
            );
        }
        ui.add_space(spacing);
    });
}

pub fn render_fee_percentage_input(ui: &mut Ui, trade_state: &mut TradeState, buffer_id: &str, is_dark_mode: bool) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        let available_width = ui.available_width();
        let max_width = (available_width * 0.8).clamp(300.0, 600.0);
        ui.set_max_width(max_width);
        let spacing = (available_width * 0.015).clamp(8.0, 16.0);
        ui.add_space(spacing);

        ui.horizontal(|ui| {
            let button_width = (max_width / 4.0).clamp(60.0, 100.0);
            let button_height = (available_width * 0.05).clamp(28.0, 36.0);
            let total_button_width = 3.0 * button_width + 2.0 * spacing;
            ui.add_space((max_width - total_button_width) / 2.0);

            let percentages = [0.10, 0.15, 0.20];
            for &percentage in percentages.iter() {
                let label = format!("{:.2}%", percentage);
                let is_selected = (trade_state.fee_percentage != 0.0) && (trade_state.fee_percentage - percentage).abs() < 0.001;
                let original_visuals = ui.visuals().clone();

                if !is_dark_mode {
                    ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, text_color(is_dark_mode));
                    ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(2.0, text_color(is_dark_mode));
                    ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(210, 210, 210); // Match white_theme hovered
                }

                if ui
                    .add(
                        egui::Button::new(RichText::new(label).size(14.0).color(text_color(is_dark_mode)))
                            .min_size(egui::Vec2::new(button_width, button_height))
                            .fill(if is_selected {
                                if is_dark_mode {
                                    Color32::from_rgb(50, 50, 50) // Matches dark theme
                                } else {
                                    Color32::from_rgb(200, 200, 200) // Matches white_theme active
                                }
                            } else {
                                if is_dark_mode {
                                    Color32::from_rgba_premultiplied(50, 50, 50, 200) // Keep for dark mode
                                } else {
                                    Color32::from_rgb(220, 220, 220) // Matches white_theme inactive
                                }
                            })
                            .stroke(if is_selected {
                                Stroke::new(1.0, if is_dark_mode {
                                    Color32::from_rgb(180, 180, 180)
                                } else {
                                    Color32::from_rgb(130, 130, 130) // Matches white_theme active
                                })
                            } else {
                                Stroke::new(0.5, if is_dark_mode {
                                    Color32::from_rgb(180, 180, 180)
                                } else {
                                    Color32::from_rgb(130, 130, 130) // Matches white_theme inactive
                                })
                            })
                    )
                    .clicked()
                {
                    trade_state.fee_percentage = percentage;
                    let amount: f64 = trade_state.amount.parse().unwrap_or(0.0);
                    let rates_rx = CHANNEL.rates_rx.clone();
                    let rates = rates_rx.borrow().clone();
                    let xrp_usd = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;
                    let eur_usd = rates.get("EUR/USD").copied().unwrap_or(0.0) as f64;
                    let xrp_eur = rates.get("XRP/EUR").copied().unwrap_or(0.0) as f64;
                    let rate = match (trade_state.base_asset.as_str(), trade_state.quote_asset.as_str()) {
                        ("XRP", "RLUSD") => xrp_usd,
                        ("RLUSD", "XRP") => if xrp_usd > 0.0 { 1.0 / xrp_usd } else { 0.0 },
                        ("XRP", "EUROP") => xrp_eur,
                        ("EUROP", "XRP") => if xrp_eur > 0.0 { 1.0 / xrp_eur } else { 0.0 },
                        ("RLUSD", "EUROP") => if eur_usd > 0.0 { 1.0 / eur_usd } else { 0.0 },
                        ("EUROP", "RLUSD") => eur_usd,
                        _ => 0.0,
                    };
                    let fee_multiplier = 1.0 + (percentage / 100.0);
                    trade_state.limit_price = format!("{:.4}", amount * rate * fee_multiplier);
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

                ui.visuals_mut().widgets = original_visuals.widgets;
                ui.add_space(spacing);
            }

            ui.add_space((max_width - total_button_width) / 2.0);
        });
        ui.add_space(spacing);
    });
}