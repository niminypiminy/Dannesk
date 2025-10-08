// src/ui/managebtc/btcsend/step2.rs
use egui::{Ui, Margin, Color32, RichText, Frame, Grid};
use crate::channel::{SignTransaction, SignTransactionState, WSCommand};
use super::buffers::{get_buffers, update_buffers};
use super::styles::styled_text_edit;
use super::step3::render_step3;
use tokio::sync::mpsc;

pub fn render_step2(
    ui: &mut Ui,
    local_state: &mut SignTransaction,
    address_buffer: &mut String,
    btc_amount_buffer: &mut String,
    usd_amount_buffer: &mut String,
    passphrase_buffer: &String,
    buffer_id: &str,
    balance: f64,
    exchange_rate: f64,
    btc_address: Option<String>, // Added
    is_dark_mode: bool,
    text_color: Color32,
    sign_transaction_tx: &tokio::sync::watch::Sender<SignTransactionState>,
    commands_tx: mpsc::Sender<WSCommand>, // Added
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
                ui.add_space(8.0);

                // BTC and USD Amount Inputs
                Grid::new("amount_grid")
                    .num_columns(2)
                    .spacing([5.0, 5.0])
                    .min_col_width(150.0)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new("Amount (BTC)").size(16.0).color(text_color));
                            let btc_edit = styled_text_edit(ui, btc_amount_buffer, 100.0, is_dark_mode, false);
                            if btc_edit.changed() {
                                if let Ok(btc) = btc_amount_buffer.parse::<f64>() {
                                    let usd = btc * exchange_rate;
                                    *usd_amount_buffer = format!("{:.5}", usd);
                                } else {
                                    usd_amount_buffer.clear();
                                }
                                let (_addr, _btc, _usd, _pass, _custom_fee, _seed_words, _input_mode) = get_buffers(buffer_id); // Fix typo and destructure 7-tuple
                                update_buffers(
                                    buffer_id,
                                    address_buffer.clone(),
                                    btc_amount_buffer.clone(),
                                    usd_amount_buffer.clone(),
                                    passphrase_buffer.clone(),
                                    custom_fee_buffer.clone(), // Use parameter
                                    seed_words.clone(),
                                    input_mode.clone(),
                                );
                                local_state.error = None;
                                ui.ctx().request_repaint();
                            }
                        });

                        ui.vertical(|ui| {
                            ui.label(RichText::new("Amount (USD)").size(16.0).color(text_color));
                            let usd_edit = styled_text_edit(ui, usd_amount_buffer, 100.0, is_dark_mode, false);
                            if usd_edit.changed() {
                                if let Ok(usd) = usd_amount_buffer.parse::<f64>() {
                                    let btc = usd / exchange_rate;
                                    *btc_amount_buffer = format!("{:.6}", btc);
                                } else {
                                    btc_amount_buffer.clear();
                                }
                                let (_addr, _btc, _usd, _pass, _custom_fee, _seed_words, _input_mode) = get_buffers(buffer_id); // Fix typo and destructure 7-tuple
                                update_buffers(
                                    buffer_id,
                                    address_buffer.clone(),
                                    btc_amount_buffer.clone(),
                                    usd_amount_buffer.clone(),
                                    passphrase_buffer.clone(),
                                    custom_fee_buffer.clone(), // Use parameter
                                    seed_words.clone(),
                                    input_mode.clone(),
                                );
                                local_state.error = None;
                                ui.ctx().request_repaint();
                            }
                        });
                        ui.end_row();
                    });

                ui.add_space(12.0);

                // Balance and Exchange Rate Grid
                Grid::new("balance_info_grid")
                    .striped(true)
                    .num_columns(2)
                    .spacing([10.0, 5.0])
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        ui.label(RichText::new("Item").size(14.0).strong().color(text_color));
                        ui.label(RichText::new("Value").size(14.0).strong().color(text_color));
                        ui.end_row();

                        ui.label(RichText::new("Balance").size(14.0).color(text_color));
                        ui.label(
                            RichText::new(format!("{:.6} BTC", balance))
                                .size(14.0)
                                .color(text_color),
                        );
                        ui.end_row();

                        ui.label(RichText::new("Exchange Rate").size(14.0).color(text_color));
                        ui.label(
                            RichText::new(format!("${:.5}", exchange_rate))
                                .size(14.0)
                                .color(text_color),
                        );
                        ui.end_row();
                    });

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
                            let next_button = ui.add(
                                egui::Button::new(RichText::new("Next").size(14.0).color(text_color))
                                    .min_size(egui::Vec2::new(80.0, 28.0)),
                            );
                            if next_button.clicked() {
                                let trimmed_btc_amount = btc_amount_buffer.trim();
                                if trimmed_btc_amount.is_empty() {
                                    local_state.error = Some("Amount cannot be empty.".to_string());
                                } else if let Ok(amount) = trimmed_btc_amount.parse::<f64>() {
                                    if amount <= 0.0 {
                                        local_state.error = Some("Amount must be greater than zero.".to_string());
                                    } else {
                                        local_state.step = 3;
                                        let _ = sign_transaction_tx.send(SignTransactionState {
                                            send_transaction: Some(local_state.clone()),
                                        });
                                        // Call step3 with correct arguments
                                        render_step3(
                                            ui,
                                            local_state,
                                            address_buffer,
                                            btc_amount_buffer,
                                            usd_amount_buffer,
                                            &mut passphrase_buffer.clone(), // Clone to mutable
                                            buffer_id,
                                            balance,
                                            exchange_rate,
                                            btc_address.clone(),
                                            is_dark_mode,
                                            text_color,
                                            sign_transaction_tx,
                                            commands_tx.clone(),
                                            custom_fee_buffer,
                                            seed_words,
                                            input_mode,
                                        );
                                    }
                                } else {
                                    local_state.error = Some("Invalid amount format.".to_string());
                                }
                                let (_addr, _btc, _usd, _pass, _custom_fee, _seed_words, _input_mode) = get_buffers(buffer_id); // Fix typo and destructure 7-tuple
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
                        });
                    ui.visuals_mut().widgets = original_visuals.widgets;
                });
            });
        });
}