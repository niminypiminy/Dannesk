// src/ui/managexrp/euro/send/step3.rs
use egui::{Ui, Margin, Color32, RichText, Frame, Grid};
use crate::channel::{CHANNEL, SendEuroTransaction, SendEuroTransactionState, ProgressState, WSCommand};
use super::buffer_manager::{BufferManager, InputMode};
use super::styles::styled_text_edit;
use tokio::sync::mpsc;

pub fn validate_and_submit_transaction(
    ui: &mut Ui,
    local_state: &mut SendEuroTransaction,
    buffer_manager: &BufferManager,
    wallet_address: Option<String>,
    commands_tx: mpsc::Sender<WSCommand>,
    send_euro_tx: &tokio::sync::watch::Sender<SendEuroTransactionState>,
) {
    let trimmed_passphrase = buffer_manager.passphrase_buffer().trim();
    let trimmed_seed = buffer_manager.seed_buffer().trim();
    let trimmed_address = buffer_manager.address_buffer().trim();
    let trimmed_euro_amount = buffer_manager.euro_amount_buffer().trim();

    // Ensure exactly one input is provided
    let (passphrase, seed) = match (trimmed_passphrase.is_empty(), trimmed_seed.is_empty()) {
        (true, true) => {
            local_state.error = Some("Either passphrase or seed must be provided.".to_string());
            return;
        }
        (false, false) => {
            local_state.error = Some("Provide only one: passphrase or seed.".to_string());
            return;
        }
        (false, true) => (Some(trimmed_passphrase.to_string()), None),
        (true, false) => (None, Some(trimmed_seed.to_string())),
    };

    if !trimmed_address.starts_with('r') || trimmed_address.len() != 34 {
        local_state.error = Some("Invalid address.".to_string());
    } else if let Ok(amount) = trimmed_euro_amount.parse::<f64>() {
        if amount <= 0.0 {
            local_state.error = Some("Amount must be greater than zero.".to_string());
        } else if wallet_address.is_none() {
            local_state.error = Some("No wallet address found.".to_string());
        } else {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 0.0,
                message: "Starting transaction".to_string(),
            }));
            ui.ctx().request_repaint();

            let command = WSCommand {
                command: "submit_transaction".to_string(),
                wallet: Some(wallet_address.unwrap()),
                recipient: Some(trimmed_address.to_string()),
                amount: Some(trimmed_euro_amount.to_string()),
                passphrase,
                seed,
                trustline_limit: None,
                tx_type: Some("payment".to_string()),
                taker_pays: None,
                taker_gets: None,
                flags: None,
                wallet_type: Some("EURO".to_string()),
            };
            let _ = commands_tx.try_send(command);
            local_state.loading = true;

            let _ = send_euro_tx.send(SendEuroTransactionState {
                send_euro: Some(local_state.clone()),
            });
            ui.ctx().request_repaint();
        }
    } else {
        local_state.error = Some("Invalid amount format.".to_string());
    }
}

pub fn render_step3(
    ui: &mut Ui,
    local_state: &mut SendEuroTransaction,
    buffer_manager: &mut BufferManager,
    wallet_address: Option<String>,
    is_dark_mode: bool,
    text_color: Color32,
    send_euro_tx: &tokio::sync::watch::Sender<SendEuroTransactionState>,
    commands_tx: mpsc::Sender<WSCommand>,
) {
    Frame::default()
        .outer_margin(Margin {
            left: 36,
            right: 0,
            top: 8,
            bottom: 8,
        })
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.heading(RichText::new("Confirm Transaction").size(18.0).color(text_color));
                ui.add_space(8.0);

                Grid::new("transaction_details_grid")
                    .striped(true)
                    .num_columns(2)
                    .spacing([10.0, 5.0])
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        ui.label(RichText::new("Item").size(14.0).strong().color(text_color));
                        ui.label(RichText::new("Value").size(14.0).strong().color(text_color));
                        ui.end_row();

                        ui.label(RichText::new("Recipient").size(14.0).color(text_color));
                        ui.label(
                            RichText::new(buffer_manager.address_buffer())
                                .size(14.0)
                                .color(text_color),
                        );
                        ui.end_row();

                        ui.label(RichText::new("Amount").size(14.0).color(text_color));
                        ui.label(
                            RichText::new(format!("€{:.2}", buffer_manager.euro_amount_buffer()))
                                .size(14.0)
                                .color(text_color),
                        );
                        ui.end_row();
                    });

                ui.add_space(12.0);

                let mut current_mode = InputMode::from_string(buffer_manager.input_mode());

                ui.horizontal(|ui| {
                    if ui.radio_value(&mut current_mode, InputMode::Passphrase, "Passphrase").clicked() {
                        buffer_manager.set_input_mode(InputMode::Passphrase);
                    }
                    if ui.radio_value(&mut current_mode, InputMode::Seed, "Seed").clicked() {
                        buffer_manager.set_input_mode(InputMode::Seed);
                    }
                });

                ui.add_space(8.0);

                match current_mode {
                    InputMode::Passphrase => {
                        ui.label(RichText::new("Passphrase").size(16.0).color(text_color))
                            .on_hover_text("Enter your passphrase (if stored in keyring) to sign the transaction.");
                        ui.add_space(4.0);
                        let mut temp_passphrase = buffer_manager.passphrase_buffer().to_string();
                        let passphrase_edit = styled_text_edit(ui, &mut temp_passphrase, 250.0, is_dark_mode, true);
                        if passphrase_edit.changed() {
                            local_state.error = None;
                            buffer_manager.update_passphrase(&temp_passphrase);
                        }
                    }
                    InputMode::Seed => {
                        ui.label(RichText::new("Seed").size(16.0).color(text_color))
                            .on_hover_text("Enter your seed (starts with 's', 29+ characters) to sign the transaction.");
                        ui.add_space(4.0);
                        let mut temp_seed = buffer_manager.seed_buffer().to_string();
                        let seed_edit = styled_text_edit(ui, &mut temp_seed, 250.0, is_dark_mode, true);
                        if seed_edit.changed() {
                            local_state.error = None;
                            buffer_manager.update_seed(&temp_seed);
                        }
                    }
                }

                if let Some(error) = &local_state.error {
                    ui.add_space(8.0);
                    ui.colored_label(Color32::RED, error);
                }

                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    let original_visuals = ui.visuals().clone();
                    if !is_dark_mode {
                        ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
                        ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
                    }
                    Frame::default()
                        .inner_margin(Margin::symmetric(0, 4))
                        .show(ui, |ui| {
                            let submit_button = ui.add(
                                egui::Button::new(RichText::new("Submit").size(14.0).color(text_color))
                                    .min_size(egui::Vec2::new(80.0, 28.0)),
                            );
                            if submit_button.clicked() {
                                validate_and_submit_transaction(
                                    ui,
                                    local_state,
                                    buffer_manager,
                                    wallet_address,
                                    commands_tx.clone(),
                                    send_euro_tx,
                                );
                            }
                        });
                    ui.visuals_mut().widgets = original_visuals.widgets;
                });
            });
        });
}