// src/ui/managexrp/xrpsend/step3.rs
use egui::{Ui, Margin, Color32, RichText, Frame, Grid, Vec2, Layout, Align};
use crate::channel::{CHANNEL, SignTransaction, SignTransactionState, ProgressState, WSCommand};
use super::buffers::{update_buffers};
use super::styles::styled_text_edit;
use tokio::sync::mpsc;

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

pub fn render_step3(
    ui: &mut Ui,
    local_state: &mut SignTransaction,
    address_buffer: &mut String,
    btc_amount_buffer: &mut String,
    usd_amount_buffer: &mut String,
    passphrase_buffer: &mut String,
    buffer_id: &str,
    _balance: f64,
    _exchange_rate: f64,
    btc_address: Option<String>,
    is_dark_mode: bool,
    text_color: Color32,
    sign_transaction_tx: &tokio::sync::watch::Sender<SignTransactionState>,
    commands_tx: mpsc::Sender<WSCommand>,
    custom_fee_buffer: &mut String,
    seed_words: &mut [String; 24],
    input_mode: &mut String,
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

                // Display transaction details
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
                        ui.label(RichText::new(address_buffer.clone()).size(14.0).color(text_color));
                        ui.end_row();
                        ui.label(RichText::new("Amount").size(14.0).color(text_color));
                        ui.label(RichText::new(format!("{} BTC", btc_amount_buffer)).size(14.0).color(text_color));
                        ui.end_row();
                        ui.label(RichText::new("Exchange").size(14.0).color(text_color));
                        ui.label(RichText::new(format!("${}", usd_amount_buffer)).size(14.0).color(text_color));
                        ui.end_row();
                    });

                ui.add_space(12.0);

                // Fee input
                ui.label(RichText::new("Transaction Fee (in satoshis)").size(16.0).color(text_color))
                    .on_hover_text("Enter the total transaction fee in satoshis. Higher fees prioritize faster confirmation. Check mempool.space for current fee rates.");
                ui.add_space(4.0);
                let fee_edit = styled_text_edit(ui, custom_fee_buffer, 300.0, is_dark_mode, false);
                if fee_edit.changed() {
                    local_state.error = None;
                    update_buffers(
                        buffer_id,
                        address_buffer.clone(),
                        btc_amount_buffer.clone(),
                        usd_amount_buffer.clone(),
                        passphrase_buffer.clone(),
                        custom_fee_buffer.clone(),
                        seed_words.clone(),
                        input_mode.clone(),
                    );
                    ui.ctx().request_repaint();
                }

                ui.add_space(12.0);

                // Input mode selection
                let mut current_mode = InputMode::from_string(input_mode);
                ui.horizontal(|ui| {
                    if ui.radio_value(&mut current_mode, InputMode::Passphrase, "Passphrase").clicked() {
*seed_words = vec![String::new(); 24].try_into().unwrap();
                        *input_mode = InputMode::Passphrase.to_string();
                        update_buffers(
                            buffer_id,
                            address_buffer.clone(),
                            btc_amount_buffer.clone(),
                            usd_amount_buffer.clone(),
                            passphrase_buffer.clone(),
                            custom_fee_buffer.clone(),
                            seed_words.clone(),
                            input_mode.clone(),
                        );
                    }
                    if ui.radio_value(&mut current_mode, InputMode::Seed, "Seed").clicked() {
                        passphrase_buffer.clear();
                        *input_mode = InputMode::Seed.to_string();
                        update_buffers(
                            buffer_id,
                            address_buffer.clone(),
                            btc_amount_buffer.clone(),
                            usd_amount_buffer.clone(),
                            passphrase_buffer.clone(),
                            custom_fee_buffer.clone(),
                            seed_words.clone(),
                            input_mode.clone(),
                        );
                    }
                });
                ui.add_space(10.0);

                // Conditional input: Passphrase or Seed grid
                match current_mode {
                    InputMode::Passphrase => {
                        ui.label(RichText::new("Passphrase").size(16.0).color(text_color))
                            .on_hover_text("Enter your passphrase (if stored in keyring) to sign the transaction.");
                        ui.add_space(4.0);
                        let input_edit = styled_text_edit(ui, passphrase_buffer, 300.0, is_dark_mode, true);
                        if input_edit.changed() {
                            local_state.error = None;
                            update_buffers(
                                buffer_id,
                                address_buffer.clone(),
                                btc_amount_buffer.clone(),
                                usd_amount_buffer.clone(),
                                passphrase_buffer.clone(),
                                custom_fee_buffer.clone(),
                                seed_words.clone(),
                                input_mode.clone(),
                            );
                            ui.ctx().request_repaint();
                        }
                    }
                    InputMode::Seed => {
                        ui.label(RichText::new("24-Word Mnemonic").size(16.0).color(text_color))
                            .on_hover_text("Enter your 24-word mnemonic phrase to sign the transaction.");
                        ui.add_space(4.0);

                        let box_width = 80.0;
                        let spacing = 6.0;
                        let columns = 4;
                        let total_boxes = 24;
                        let content_width = (box_width * columns as f32) + (spacing * (columns - 1) as f32);
                        ui.allocate_ui_with_layout(
                            Vec2::new(content_width, 200.0),
                            Layout::top_down(Align::Center),
                            |ui| {
                                ui.style_mut().spacing.item_spacing = Vec2::new(spacing, spacing);
                                let mut pasted = None;
                                egui::Grid::new("mnemonic_grid")
                                    .num_columns(columns)
                                    .spacing([spacing, spacing])
                                    .show(ui, |ui| {
                                        for i in 0..total_boxes {
                                            let mut word = seed_words[i].clone();
                                            let response = ui.add(
                                                egui::TextEdit::singleline(&mut word)
                                                    .desired_width(box_width)
                                                    .min_size(Vec2::new(box_width, 24.0))
                                                    .hint_text(format!("{}", i + 1))
                                                    .id_source(format!("seed_word_{}", i)),
                                            );

                                            if response.changed() {
                                                if word.contains(' ') {
                                                    pasted = Some((i, word.clone()));
                                                } else {
                                                    seed_words[i] = word.clone();
                                                    update_buffers(
                                                        buffer_id,
                                                        address_buffer.clone(),
                                                        btc_amount_buffer.clone(),
                                                        usd_amount_buffer.clone(),
                                                        passphrase_buffer.clone(),
                                                        custom_fee_buffer.clone(),
                                                        seed_words.clone(),
                                                        input_mode.clone(),
                                                    );
                                                }
                                            }

                                            if (i + 1) % columns == 0 {
                                                ui.end_row();
                                            }
                                        }
                                    });

                                if let Some((start_index, pasted_text)) = pasted {
                                    let words: Vec<&str> = pasted_text.trim().split_whitespace().collect();
                                    for (j, w) in words.iter().enumerate().take(24 - start_index) {
                                        seed_words[start_index + j] = w.to_string();
                                    }
                                    update_buffers(
                                        buffer_id,
                                        address_buffer.clone(),
                                        btc_amount_buffer.clone(),
                                        usd_amount_buffer.clone(),
                                        passphrase_buffer.clone(),
                                        custom_fee_buffer.clone(),
                                        seed_words.clone(),
                                        input_mode.clone(),
                                    );
                                }
                            },
                        );
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
                    Frame::new()
                        .inner_margin(Margin::symmetric(0, 4))
                        .show(ui, |ui| {
                            let submit_button = ui.add(
                                egui::Button::new(RichText::new("Submit").size(14.0).color(text_color))
                                    .min_size(egui::Vec2::new(80.0, 28.0)),
                            );
                            if submit_button.clicked() {
                                let trimmed_input = passphrase_buffer.trim();
                                let trimmed_address = address_buffer.trim();
                                let trimmed_btc_amount = btc_amount_buffer.trim();
                                let trimmed_fee = custom_fee_buffer.trim();
                                let seed_phrase = seed_words.iter().filter(|w| !w.is_empty()).map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
                                let word_count = seed_words.iter().filter(|w| !w.is_empty()).count();

                                let (passphrase, seed) = match (trimmed_input.is_empty(), word_count == 0) {
                                    (true, true) => {
                                        local_state.error = Some("Either passphrase or 24-word mnemonic must be provided.".to_string());
                                        return;
                                    }
                                    (false, false) => {
                                        local_state.error = Some("Provide only one: passphrase or mnemonic.".to_string());
                                        return;
                                    }
                                    (false, true) => (Some(trimmed_input.to_string()), None),
                                    (true, false) => {
                                        if word_count != 24 {
                                            local_state.error = Some("Mnemonic must be 24 words.".to_string());
                                            return;
                                        }
                                        (None, Some(seed_phrase))
                                    }
                                };

                                if let Ok(amount) = trimmed_btc_amount.parse::<f64>() {
                                    if amount <= 0.0 {
                                        local_state.error = Some("Amount must be greater than zero.".to_string());
                                    } else if trimmed_fee.is_empty() {
                                        local_state.error = Some("Please enter a transaction fee.".to_string());
                                    } else if let Ok(fee_sats) = trimmed_fee.parse::<u64>() {
                                        if fee_sats < 200 {
                                            local_state.error = Some("Fee must be at least 200 satoshis.".to_string());
                                        } else if btc_address.is_none() {
                                            local_state.error = Some("No wallet address found.".to_string());
                                        } else {
                                            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                                                progress: 0.0,
                                                message: "Starting transaction".to_string(),
                                            }));
                                            ui.ctx().request_repaint();

                                            let command = WSCommand {
                                                command: "bitcoin_submit_transaction".to_string(),
                                                wallet: Some(btc_address.unwrap()),
                                                recipient: Some(trimmed_address.to_string()),
                                                amount: Some(trimmed_btc_amount.to_string()),
                                                passphrase,
                                                seed,
                                                trustline_limit: Some(fee_sats.to_string()),
                                                tx_type: Some("BTC".to_string()),
                                                taker_pays: None,
                                                taker_gets: None,
                                                flags: None,
                                                wallet_type: None,
                                            };
                                            let _ = commands_tx.try_send(command);
                                            local_state.loading = true;
                                        }
                                    } else {
                                        local_state.error = Some("Invalid fee format. Enter a number in satoshis.".to_string());
                                    }
                                } else {
                                    local_state.error = Some("Invalid amount format.".to_string());
                                }

                                update_buffers(
                                    buffer_id,
                                    address_buffer.clone(),
                                    btc_amount_buffer.clone(),
                                    usd_amount_buffer.clone(),
                                    passphrase_buffer.clone(),
                                    custom_fee_buffer.clone(),
                                    seed_words.clone(),
                                    input_mode.clone(),
                                );
                                let _ = sign_transaction_tx.send(SignTransactionState {
                                    send_transaction: Some(local_state.clone()),
                                });
                                ui.ctx().request_repaint();
                            }
                        });
                    ui.visuals_mut().widgets = original_visuals.widgets;
                });
            });
        });
}