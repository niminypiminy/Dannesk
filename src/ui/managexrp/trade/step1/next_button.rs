use egui::{Ui, RichText, Vec2};
use crate::channel::{XRPModalState, ActiveView};
use crate::ui::managexrp::trade::buffers::{TradeState, update_buffers};

pub fn render_next_button(ui: &mut Ui, trade_state: &mut TradeState, buffer_id: &str, xrp_modal_tx: &tokio::sync::watch::Sender<XRPModalState>) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        // Get available width and calculate dynamic sizes
        let available_width = ui.available_width();
        let max_width = (available_width * 0.8).clamp(300.0, 600.0); // Match input.rs and trade_flags.rs
        ui.set_max_width(max_width);

        // Dynamic font sizes and spacing
        let button_font_size = (available_width * 0.04).clamp(16.0, 20.0); // Slightly larger than inputs
        let spacing = (available_width * 0.015).clamp(8.0, 16.0);

        // Calculate button width and height
        let button_width = (max_width / 4.0).clamp(60.0, 100.0); // Match percentage button sizing
        let button_height = (available_width * 0.05).clamp(28.0, 36.0); // Match percentage button height
        let total_content_width = button_width; // Single button
        let padding = (max_width - total_content_width).max(0.0) / 2.0; // Center the button


        ui.horizontal(|ui| {
            ui.add_space(padding); // Left padding to center

            if ui
                .add(
                    egui::Button::new(RichText::new("→").size(button_font_size))
                        .min_size(Vec2::new(button_width, button_height)),
                )
                .clicked()
            {
                trade_state.error = None;
                if trade_state.base_asset.is_empty() || trade_state.quote_asset.is_empty() {
                    trade_state.error = Some("Please select a valid trading pair".to_string());
                } else if trade_state.base_asset == trade_state.quote_asset {
                    trade_state.error = Some("Base and quote assets cannot be the same".to_string());
                } else if trade_state.amount.is_empty() || trade_state.amount.parse::<f64>().is_err() {
                    trade_state.error = Some("Please enter a valid amount".to_string());
                } else if trade_state.limit_price.is_empty() || trade_state.limit_price.parse::<f64>().is_err() {
                    trade_state.error = Some("Please enter a valid price".to_string());
                } else {
                    trade_state.step = 2;
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

            ui.add_space(padding); // Right padding to center
        });
        ui.add_space(spacing); // Vertical spacing
    });
}