// src/ui/managexrp/euro/send/step2.rs
use egui::{Ui, Color32, Frame, Margin, RichText};
use crate::channel::{SendEuroTransaction, SendEuroTransactionState};
use super::buffer_manager::BufferManager;
use super::styles::styled_text_edit;

pub fn render_step2(
    ui: &mut Ui,
    local_state: &mut SendEuroTransaction,
    buffer_manager: &mut BufferManager,
    is_dark_mode: bool,
    text_color: Color32,
    send_euro_tx: &tokio::sync::watch::Sender<SendEuroTransactionState>,
) {
    ui.add_space(20.0);
    ui.heading(RichText::new("Amount (EURO)").size(18.0).color(text_color));
    ui.add_space(20.0);
    let mut temp_amount = buffer_manager.euro_amount_buffer().to_string();
    let euro_edit = styled_text_edit(ui, &mut temp_amount, 100.0, is_dark_mode, false);
    if euro_edit.changed() {
        if temp_amount.parse::<f64>().is_ok() {
            buffer_manager.update_euro_amount(&temp_amount);
            local_state.error = None;
        } else {
            local_state.error = Some("Invalid amount format.".to_string());
        }
    }

    if let Some(error) = &local_state.error {
        ui.add_space(10.0);
        ui.colored_label(Color32::RED, error);
    }

    ui.add_space(10.0);
    // Center the button to match rlusd/step2.rs
    ui.allocate_ui_with_layout(
        egui::Vec2::new(150.0, 40.0), // Size for button area, matches rlusd/step2
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            // Modernized Next button to match rlusd/step2
            let original_visuals = ui.visuals().clone();
            if !is_dark_mode {
                ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
                ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
            }
            Frame::default() // egui 0.31.1, no ID argument
                .inner_margin(Margin::symmetric(8, 4)) // Matches rlusd/step2
                .show(ui, |ui| {
                    // Dynamic button size based on modal width (350.0 from mod.rs)
                    let button_width = 100.0 * (ui.available_width() / 350.0).min(1.2).max(0.8); // Matches rlusd/step2
                    let next_button = ui.add(
                        egui::Button::new(RichText::new("Next").size(14.0).color(text_color))
                            .min_size(egui::Vec2::new(button_width, 28.0)),
                    );
                    if next_button.clicked() {
                        let trimmed_euro_amount = buffer_manager.euro_amount_buffer().trim();
                        if trimmed_euro_amount.is_empty() {
                            local_state.error = Some("Amount cannot be empty.".to_string());
                        } else if let Ok(euro) = trimmed_euro_amount.parse::<f64>() {
                            if euro <= 0.0 {
                                local_state.error = Some("Amount must be greater than zero.".to_string());
                            } else {
                                local_state.step = 3;
                                let _ = send_euro_tx.send(SendEuroTransactionState {
                                    send_euro: Some(local_state.clone()),
                                });
                            }
                        } else {
                            local_state.error = Some("Invalid amount format.".to_string());
                        }
                        ui.ctx().request_repaint();
                    }
                });
            ui.visuals_mut().widgets = original_visuals.widgets;
        },
    );
}