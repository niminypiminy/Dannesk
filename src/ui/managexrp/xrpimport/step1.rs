use egui::{Ui, RichText, Color32, Frame, Margin};
use crate::channel::{CHANNEL, XRPModalState, XRPImport, ActiveView};
use super::{buffers, styles};

pub fn render(ui: &mut Ui, import_state: &mut XRPImport, buffer_id: &str) {
    let xrp_modal_tx = CHANNEL.xrp_modal_tx.clone();
    let is_dark_mode = CHANNEL.theme_user_rx.borrow().0;
    let (mut seed_buffer, passphrase_buffer) = buffers::get_buffer(buffer_id);

    ui.label(RichText::new("Enter your XRP seed (private key).").size(16.0).color(styles::text_color(is_dark_mode)));
    ui.add_space(5.0);

    let seed_edit = styles::styled_text_edit(ui, &mut seed_buffer, is_dark_mode, false);
    if seed_edit.changed() {
        buffers::update_buffer(buffer_id, seed_buffer.clone(), passphrase_buffer.clone());
        import_state.error = None;
    }
    ui.add_space(5.0);

    if let Some(error) = &import_state.error {
        ui.colored_label(Color32::RED, error);
        ui.add_space(5.0);
    }

    ui.add_space(5.0);
    // Modernized Continue Button
    ui.vertical_centered(|ui| {
        let original_visuals = ui.visuals().clone();
        let text_color = ui.style().visuals.text_color();
        if !is_dark_mode {
            ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
            ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
        }
        Frame::new() // egui 0.31.1, no ID argument
            .inner_margin(Margin::symmetric(8, 4))
            .show(ui, |ui| {
                let continue_button = ui.add(
                    egui::Button::new(RichText::new("Continue").size(14.0).color(text_color))
                        .min_size(egui::Vec2::new(100.0, 28.0)),
                );
                if continue_button.clicked() {
                    let trimmed_seed = seed_buffer.trim();
                    if trimmed_seed.len() < 29 || !trimmed_seed.starts_with('s') {
                        import_state.error = Some("Invalid seed format. Must start with 's' and be at least 29 characters.".to_string());
                    } else {
                        import_state.step = 2;
                    }
                    buffers::update_buffer(buffer_id, seed_buffer.clone(), passphrase_buffer);
                    let _ = xrp_modal_tx.send(XRPModalState {
                        import_wallet: Some(import_state.clone()),
                        create_wallet: None,
                        view_type: ActiveView::XRP,
                    });
                    ui.ctx().request_repaint();
                }
            });
        ui.visuals_mut().widgets = original_visuals.widgets;
    });
}