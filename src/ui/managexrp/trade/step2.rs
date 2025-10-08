use egui::{Ui, RichText, Color32, Stroke, Vec2};
use crate::channel::{CHANNEL, WSCommand, XRPModalState, ActiveView, ProgressState};
use super::buffers::{TradeState, update_buffers, clear_buffer};

// Local enum for UI control
#[derive(Clone, PartialEq, Debug)]
enum InputMode {
    Passphrase,
    Seed,
}

impl InputMode {
    fn to_string(&self) -> String {
        match self {
            InputMode::Passphrase => "Passphrase".to_string(),
            InputMode::Seed => "Seed".to_string(),
        }
    }

    fn from_string(s: &str) -> Self {
        match s {
            "Seed" => InputMode::Seed,
            _ => InputMode::Passphrase,
        }
    }
}

pub fn render(ui: &mut Ui, trade_state: &mut TradeState, buffer_id: &str, commands_tx: tokio::sync::mpsc::Sender<WSCommand>) {
    let xrp_modal_tx = CHANNEL.xrp_modal_tx.clone();
    let wallet_balance_rx = CHANNEL.wallet_balance_rx.clone();
    let progress_tx = CHANNEL.progress_tx.clone();
    let is_dark_mode = CHANNEL.theme_user_rx.borrow().0;

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        let available_width = ui.available_width();
        let max_width = (available_width * 0.8).clamp(300.0, 600.0);
        ui.set_max_width(max_width);
        let spacing = (available_width * 0.015).clamp(8.0, 16.0);
        let label_font_size = (available_width * 0.04).clamp(12.0, 16.0);
        let button_width = (max_width / 4.0).clamp(60.0, 100.0);
        let button_height = (available_width * 0.05).clamp(28.0, 36.0);

ui.add_space(spacing * 3.0); // Increased spacing before "Confirm Trade"
        ui.label(
            RichText::new("Confirm Trade")
                .size(label_font_size + 2.0)
                .color(super::styles::text_color(is_dark_mode)),
        );
        ui.add_space(spacing * 3.0);

        // Input mode selection
        let mut current_mode = InputMode::from_string(&trade_state.input_mode);
       ui.horizontal(|ui| {
    let radio_button_width = (max_width / 4.0).clamp(60.0, 120.0);
    let total_content_width = 2.0 * radio_button_width + spacing;
    let padding = (max_width - total_content_width).max(0.0) / 2.0;
    ui.add_space(padding + 25.0); // Shift right by ~thumb size (25px)

    if ui.radio_value(&mut current_mode, InputMode::Passphrase, "Passphrase").clicked() {
        trade_state.seed.clear();
        trade_state.input_mode = InputMode::Passphrase.to_string();
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

    if ui.radio_value(&mut current_mode, InputMode::Seed, "Seed").clicked() {
        trade_state.passphrase.clear();
        trade_state.input_mode = InputMode::Seed.to_string();
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

    ui.add_space(padding);
});
ui.add_space(spacing * 3.0); // Increased spacing after radio buttons




        // Conditional input field
        let input_width = (max_width * 0.8).clamp(200.0, 400.0);
        match current_mode {
            InputMode::Passphrase => {
                ui.label(
                    RichText::new("Passphrase")
                        .size(label_font_size)
                        .color(super::styles::text_color(is_dark_mode))
                );
                ui.add_space(spacing / 2.0);
                if super::styles::styled_text_edit(
                    ui,
                    &mut trade_state.passphrase,
                    is_dark_mode,
                    true,
                    RichText::new("Enter passphrase").size(label_font_size - 2.0).color(Color32::from_gray(100)),
                    Some(input_width),
                )
                .changed()
                {
                    trade_state.error = None;
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
            }
            InputMode::Seed => {
                ui.label(
                    RichText::new("Seed")
                        .size(label_font_size)
                        .color(super::styles::text_color(is_dark_mode))
                );
                ui.add_space(spacing / 2.0);
                if super::styles::styled_text_edit(
                    ui,
                    &mut trade_state.seed,
                    is_dark_mode,
                    true,
                    RichText::new("Enter seed").size(label_font_size - 2.0).color(Color32::from_gray(100)),
                    Some(input_width),
                )
                .changed()
                {
                    trade_state.error = None;
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
            }
        }
ui.add_space(spacing * 3.0); // Increased spacing after radio buttons

        if let Some(error) = &trade_state.error {
            ui.add_space(spacing / 2.0);
            ui.colored_label(Color32::RED, error);
            ui.add_space(spacing);
        }

        // Buttons
        ui.horizontal(|ui| {
            let total_button_width = 2.0 * button_width + spacing;
            ui.add_space((max_width - total_button_width) / 2.0);

            // Back button
            if ui
                .add(
                    egui::Button::new(RichText::new("←").size(label_font_size).color(super::styles::text_color(is_dark_mode)))
                        .min_size(Vec2::new(button_width, button_height))
                        .fill(if is_dark_mode {
                            Color32::from_rgba_premultiplied(50, 50, 50, 200)
                        } else {
                            Color32::from_rgba_premultiplied(200, 200, 200, 200)
                        })
                        .stroke(Stroke::new(0.5, super::styles::text_color(is_dark_mode))),
                )
                .clicked()
            {
                trade_state.step = 1;
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
                let _ = xrp_modal_tx.send(XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: ActiveView::Trade,
                });
                ui.ctx().request_repaint();
            }

            ui.add_space(spacing);

            // Submit button
            if ui
                .add(
                    egui::Button::new(RichText::new("Submit").size(label_font_size).color(super::styles::text_color(is_dark_mode)))
                        .min_size(Vec2::new(button_width, button_height))
                        .fill(if is_dark_mode {
                            Color32::from_rgba_premultiplied(50, 50, 50, 200)
                        } else {
                            Color32::from_rgba_premultiplied(0, 0, 255, 200)
                        })
                        .stroke(Stroke::new(1.0, super::styles::text_color(is_dark_mode))),
                )
                .clicked()
            {
                let trimmed_passphrase = trade_state.passphrase.trim();
                let trimmed_seed = trade_state.seed.trim();

                // Ensure exactly one input is provided
                let (passphrase, seed) = match (trimmed_passphrase.is_empty(), trimmed_seed.is_empty()) {
                    (true, true) => {
                        trade_state.error = Some("Either passphrase or seed must be provided.".to_string());
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
                        return;
                    }
                    (false, false) => {
                        trade_state.error = Some("Provide only one: passphrase or seed.".to_string());
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
                        return;
                    }
                    (false, true) => (Some(trimmed_passphrase.to_string()), None),
                    (true, false) => (None, Some(trimmed_seed.to_string())),
                };

                if trade_state.base_asset == trade_state.quote_asset {
                    trade_state.error = Some("Buy and Sell assets cannot be the same".to_string());
                } else if trade_state.amount.parse::<f64>().unwrap_or(0.0) <= 0.0 {
                    trade_state.error = Some("Amount must be greater than zero.".to_string());
                } else if trade_state.limit_price.parse::<f64>().unwrap_or(0.0) <= 0.0 {
                    trade_state.error = Some("Limit price must be greater than zero.".to_string());
                } else {
                    let (_xrp_balance, wallet_address, _is_active, _privatekey) = wallet_balance_rx.borrow().clone();
                    let wallet = wallet_address.unwrap_or_default();
                    if wallet.is_empty() {
                        trade_state.error = Some("No wallet address found".to_string());
                    } else {
                        let _ = progress_tx.send(Some(ProgressState {
                            progress: 0.0,
                            message: "Submitting trade".to_string(),
                        }));

                        // Calculate taker_pays and taker_gets
                        let amount = trade_state.amount.parse::<f64>().unwrap_or(0.0);
                        let limit_price = trade_state.limit_price.parse::<f64>().unwrap_or(1.0);
                        let offer_amount = (amount * limit_price).to_string();

                        let (taker_pays, taker_gets) = (
                            (trade_state.amount.clone(), trade_state.base_asset.clone()),
                            (offer_amount, trade_state.quote_asset.clone()),
                        );

                        let command = WSCommand {
                            command: "submit_transaction".to_string(),
                            wallet: Some(wallet),
                            recipient: None,
                            amount: None,
                            passphrase,
                            seed,
                            trustline_limit: None,
                            tx_type: Some("offer_create".to_string()),
                            taker_pays: Some(taker_pays),
                            taker_gets: Some(taker_gets),
                            flags: Some(trade_state.flags.clone()),
                            wallet_type: None,
                        };

                        let _ = commands_tx.try_send(command);
                        trade_state.error = Some("Trade submitted successfully".to_string());
                        trade_state.done = true;
                        clear_buffer(buffer_id);
                    }
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
                let _ = xrp_modal_tx.send(XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: ActiveView::Trade,
                });
                ui.ctx().request_repaint();
            }

            ui.add_space((max_width - total_button_width) / 2.0);
        });
        ui.add_space(spacing);
    });
}