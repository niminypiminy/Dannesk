// src/ui/managexrp/xrpsend/step2.rs

use egui::{Ui, Margin, Color32, RichText, Frame, Grid};
use crate::channel::{SignTransaction, SignTransactionState};
use super::buffer_manager::BufferManager;
use super::styles::styled_text_edit;

pub fn render_step2(
    ui: &mut Ui,
    local_state: &mut SignTransaction,
    buffer_manager: &mut BufferManager, // Replace individual buffers with BufferManager
    balance: f64,
    exchange_rate: f64,
    is_dark_mode: bool,
    text_color: Color32,
    sign_transaction_tx: &tokio::sync::watch::Sender<SignTransactionState>,
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

                // XRP and USD Amount Inputs
                Grid::new("amount_grid")
                    .num_columns(2)
                    .spacing([5.0, 5.0])
                    .min_col_width(150.0)
                    .show(ui, |ui| {
                        // XRP Input
                        ui.vertical(|ui| {
                            ui.label(RichText::new("Amount (XRP)").size(16.0).color(text_color));
                            let mut temp_xrp_amount = buffer_manager.xrp_amount_buffer().to_string();
                            let xrp_edit = styled_text_edit(ui, &mut temp_xrp_amount, 100.0, is_dark_mode, false);
                            if xrp_edit.changed() {
                                if let Ok(xrp) = temp_xrp_amount.parse::<f64>() {
                                    let usd = xrp * exchange_rate;
                                    buffer_manager.update_usd_amount(&format!("{:.5}", usd));
                                } else {
                                    buffer_manager.update_usd_amount("");
                                }
                                buffer_manager.update_xrp_amount(&temp_xrp_amount);
                                local_state.error = None;
                            }
                        });

                        // USD Input
                        ui.vertical(|ui| {
                            ui.label(RichText::new("Amount (USD)").size(16.0).color(text_color));
                            let mut temp_usd_amount = buffer_manager.usd_amount_buffer().to_string();
                            let usd_edit = styled_text_edit(ui, &mut temp_usd_amount, 100.0, is_dark_mode, false);
                            if usd_edit.changed() {
                                if let Ok(usd) = temp_usd_amount.parse::<f64>() {
                                    let xrp = usd / exchange_rate;
                                    buffer_manager.update_xrp_amount(&format!("{:.6}", xrp));
                                } else {
                                    buffer_manager.update_xrp_amount("");
                                }
                                buffer_manager.update_usd_amount(&temp_usd_amount);
                                local_state.error = None;
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
                            RichText::new(format!("{:.6} XRP", balance))
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
                    // Modernized Next button with step1 style, keeping exact left alignment
                    let original_visuals = ui.visuals().clone();
                    if !is_dark_mode {
                        ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
                        ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
                    }
                    Frame::new() // egui 0.31.1, no ID argument
                        .inner_margin(Margin::symmetric(0, 4)) // Minimal horizontal margin to avoid rightward shift
                        .show(ui, |ui| {
                            let next_button = ui.add(
                                egui::Button::new(RichText::new("Next").size(14.0).color(text_color))
                                    .min_size(egui::Vec2::new(80.0, 28.0)), // Fixed width to match default button
                            );
                            if next_button.clicked() {
                                let trimmed_xrp_amount = buffer_manager.xrp_amount_buffer().trim();
                                if trimmed_xrp_amount.is_empty() {
                                    local_state.error = Some("Amount cannot be empty.".to_string());
                                } else if let Ok(amount) = trimmed_xrp_amount.parse::<f64>() {
                                    if amount <= 0.0 {
                                        local_state.error = Some("Amount must be greater than zero.".to_string());
                                    } else if amount > balance - 1.0 {
                                        local_state.error = Some("Insufficient funds: 1 XRP reserve required.".to_string());
                                    } else {
                                        local_state.step = 3;
                                        let _ = sign_transaction_tx.send(SignTransactionState {
                                            send_transaction: Some(local_state.clone()),
                                        });
                                    }
                                } else {
                                    local_state.error = Some("Invalid amount format.".to_string());
                                }
                                buffer_manager.update_buffers(); // Update buffers on button click
                                ui.ctx().request_repaint();
                            }
                        });
                    ui.visuals_mut().widgets = original_visuals.widgets;
                });
            });
        });
}