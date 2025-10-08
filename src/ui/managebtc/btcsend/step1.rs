// src/ui/managebtc/btcsend/step1.rs
use egui::{Ui, Color32, RichText, Frame, Margin};
use crate::channel::{SignTransaction, SignTransactionState};
use super::buffers::{get_buffers, update_buffers};
use super::styles::styled_text_edit;

pub fn render_step1(
    ui: &mut Ui,
    local_state: &mut SignTransaction,
    address_buffer: &mut String,
    btc_amount_buffer: &mut String,
    usd_amount_buffer: &mut String,
    passphrase_buffer: &String,
    buffer_id: &str,
    _balance: f64,
    _exchange_rate: f64,
    is_dark_mode: bool,
    text_color: Color32,
    sign_transaction_tx: &tokio::sync::watch::Sender<SignTransactionState>,
    custom_fee_buffer: &mut String,
    seed_words: &mut [String; 24],
    input_mode: &mut String,
) {
    ui.add_space(20.0);
    ui.label(RichText::new("Recipient Address").size(16.0).color(text_color));
    ui.add_space(20.0);
    let address_edit = styled_text_edit(ui, address_buffer, 275.0, is_dark_mode, false);
    if address_edit.changed() {
        let (_addr, _btc, _usd, _pass, _custom_fee, _seed_words, _input_mode) = get_buffers(buffer_id); // Destructure 7-tuple
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

    if let Some(error) = &local_state.error {
        ui.add_space(10.0);
        ui.colored_label(Color32::RED, error);
    }

    ui.add_space(10.0);
    ui.allocate_ui_with_layout(
        egui::Vec2::new(150.0, 40.0),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            let original_visuals = ui.visuals().clone();
            if !is_dark_mode {
                ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
                ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
            }
            Frame::new()
                .inner_margin(Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    let button_width = 100.0 * (ui.available_width() / 350.0).min(1.2).max(0.8);
                    let next_button = ui.add(
                        egui::Button::new(RichText::new("Next").size(14.0).color(text_color))
                            .min_size(egui::Vec2::new(button_width, 28.0)),
                    );
                    if next_button.clicked() {
                        let trimmed_address = address_buffer.trim();
                        if trimmed_address.is_empty() {
                            local_state.error = Some("Recipient address cannot be empty.".to_string());
                        } else if !trimmed_address.starts_with('b') {
                            local_state.error = Some("Invalid BTC address: Must start with 'b'".to_string());
                        } else {
                            local_state.step = 2;
                            let _ = sign_transaction_tx.send(SignTransactionState {
                                send_transaction: Some(local_state.clone()),
                            });
                        }
                        let (_addr, _btc, _usd, _pass, _custom_fee, _seed_words, _input_mode) = get_buffers(buffer_id); // Destructure 7-tuple
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
                        ui.ctx().request_repaint();
                    }
                });
            ui.visuals_mut().widgets = original_visuals.widgets;
        },
    );
}