// src/ui/managexrp/xrpsend/step1.rs

use egui::{Ui, Color32, RichText, Frame, Margin};
use crate::channel::{SignTransaction, SignTransactionState};
use super::buffer_manager::BufferManager; // Import BufferManager
use super::styles::styled_text_edit;

pub fn render_step1(
    ui: &mut Ui,
    local_state: &mut SignTransaction,
    buffer_manager: &mut BufferManager, // Replace individual buffers with BufferManager
    _balance: f64,
    _exchange_rate: f64,
    is_dark_mode: bool,
    text_color: Color32,
    sign_transaction_tx: &tokio::sync::watch::Sender<SignTransactionState>,
) {
    ui.add_space(20.0);
    ui.heading(RichText::new("Recipient Address").size(18.0).color(text_color));
    ui.add_space(20.0);
    let mut temp_address = buffer_manager.address_buffer().to_string();
    let address_edit = styled_text_edit(ui, &mut temp_address, 275.0, is_dark_mode, false);
    if address_edit.changed() {
        buffer_manager.update_address(&temp_address); // Update address via BufferManager
        local_state.error = None;
    }

    if let Some(error) = &local_state.error {
        ui.add_space(10.0);
        ui.colored_label(Color32::RED, error);
    }

    ui.add_space(10.0);
    // Center the button using a layout similar to the receive example
    ui.allocate_ui_with_layout(
        egui::Vec2::new(150.0, 40.0), // Size for button area, scaled relative to modal
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            // Modernized Next button with retro vibe
            let original_visuals = ui.visuals().clone();
            if !is_dark_mode {
                ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
                ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
            }
            Frame::new() // egui 0.31.1, no ID argument
                .inner_margin(Margin::symmetric(8, 4)) // Matches Copy Address button
                .show(ui, |ui| {
                    // Dynamic button size based on modal width (350.0 from mod.rs)
                    let button_width = 100.0 * (ui.available_width() / 350.0).min(1.2).max(0.8); // Scale between 80% and 120%
                    let next_button = ui.add(
                        egui::Button::new(RichText::new("Next").size(14.0).color(text_color))
                            .min_size(egui::Vec2::new(button_width, 28.0)),
                    );
                    if next_button.clicked() {
                        let trimmed_address = buffer_manager.address_buffer().trim();
                        if trimmed_address.is_empty() {
                            local_state.error = Some("Recipient address cannot be empty.".to_string());
                        } else if !trimmed_address.starts_with('r') || trimmed_address.len() != 34 {
                            local_state.error = Some("Invalid XRP address: Must start with 'r' and be 34 characters.".to_string());
                        } else {
                            local_state.step = 2;
                            let _ = sign_transaction_tx.send(SignTransactionState {
                                send_transaction: Some(local_state.clone()),
                            });
                        }
                        buffer_manager.update_buffers(); // Update buffers on button click
                        ui.ctx().request_repaint();
                    }
                });
            ui.visuals_mut().widgets = original_visuals.widgets;
        },
    );
}