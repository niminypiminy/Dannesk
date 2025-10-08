// src/ui/managexrp/rlusd/step2.rs
use egui::{Ui, Color32, RichText, Frame, Margin};
use crate::channel::{SendRLUSDTransaction, SendRLUSDTransactionState};
use super::buffer_manager::BufferManager;
use super::styles::styled_text_edit;

pub fn render_step2(
    ui: &mut Ui,
    local_state: &mut SendRLUSDTransaction,
    buffer_manager: &mut BufferManager,
    is_dark_mode: bool,
    text_color: Color32,
    send_rlusd_tx: &tokio::sync::watch::Sender<SendRLUSDTransactionState>,
) {
    ui.add_space(20.0);
    ui.heading(RichText::new("Amount (USD)").size(18.0).color(text_color));
    ui.add_space(20.0);
    let mut temp_amount = buffer_manager.usd_amount_buffer().to_string();
    let usd_edit = styled_text_edit(ui, &mut temp_amount, 100.0, is_dark_mode, false);
    if usd_edit.changed() {
        if temp_amount.parse::<f64>().is_ok() {
            buffer_manager.update_usd_amount(&temp_amount);
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
    ui.allocate_ui_with_layout(
        egui::Vec2::new(150.0, 40.0),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            let original_visuals = ui.visuals().clone();
            if !is_dark_mode {
                ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
                ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
            }
            Frame::default()
                .inner_margin(Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    let button_width = 100.0 * (ui.available_width() / 350.0).min(1.2).max(0.8);
                    let next_button = ui.add(
                        egui::Button::new(RichText::new("Next").size(14.0).color(text_color))
                            .min_size(egui::Vec2::new(button_width, 28.0)),
                    );
                    if next_button.clicked() {
                        let trimmed_usd_amount = buffer_manager.usd_amount_buffer().trim();
                        if trimmed_usd_amount.is_empty() {
                            local_state.error = Some("Amount cannot be empty.".to_string());
                        } else if let Ok(usd) = trimmed_usd_amount.parse::<f64>() {
                            if usd <= 0.0 {
                                local_state.error = Some("Amount must be greater than zero.".to_string());
                            } else {
                                local_state.step = 3;
                                let _ = send_rlusd_tx.send(SendRLUSDTransactionState {
                                    send_rlusd: Some(local_state.clone()),
                                });
                                ui.ctx().request_repaint();
                            }
                        } else {
                            local_state.error = Some("Invalid amount format.".to_string());
                        }
                    }
                });
            ui.visuals_mut().widgets = original_visuals.widgets;
        },
    );
}