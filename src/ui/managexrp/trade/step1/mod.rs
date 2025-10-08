use egui::{Ui, RichText, Color32};
use crate::channel::CHANNEL;
use super::buffers::TradeState;
use super::step1::{
    asset_selector::render_asset_selector,
    inputs::{
        render_amount_and_price_inputs, // Only this input function
        render_fee_percentage_input,
    },
    trade_flags::{render_trade_flags},
    next_button::render_next_button,
};

pub fn render(ui: &mut Ui, trade_state: &mut TradeState, buffer_id: &str) {
    let xrp_modal_tx = CHANNEL.xrp_modal_tx.clone();
    let is_dark_mode = CHANNEL.theme_user_rx.borrow().0;

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        let available_width = ui.available_width();
        // Dynamic max width: 80% of available width, clamped
        let max_width = (available_width * 0.8).clamp(300.0, 600.0);
        ui.set_max_width(max_width);

        // Dynamic spacing and font sizes
        let spacing = (available_width * 0.015).clamp(10.0, 20.0);
        let label_font_size = (available_width * 0.04).clamp(12.0, 16.0);

        ui.add_space(spacing);
        ui.add_space(spacing * 1.5);

        render_asset_selector(ui, trade_state, buffer_id, is_dark_mode);
        ui.add_space(spacing);

        render_amount_and_price_inputs(ui, trade_state, buffer_id, is_dark_mode);
        ui.add_space(spacing);

        render_fee_percentage_input(ui, trade_state, buffer_id, is_dark_mode);
        ui.add_space(spacing);

        render_trade_flags(ui, trade_state, buffer_id, is_dark_mode);
        ui.add_space(spacing);

        if let Some(error) = &trade_state.error {
            ui.add_space(spacing);
            ui.colored_label(Color32::RED, error);
        }
        ui.add_space(spacing);

        render_next_button(ui, trade_state, buffer_id, &xrp_modal_tx);

        ui.add_space(spacing * 1.5);
        let rates_rx = CHANNEL.rates_rx.clone();
        let rates = rates_rx.borrow().clone();
        let xrp_usd = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;
        let eur_usd = rates.get("EUR/USD").copied().unwrap_or(0.0) as f64;
        let xrp_eur = rates.get("XRP/EUR").copied().unwrap_or(0.0) as f64;

        let (_rate, rate_text) = match (trade_state.base_asset.as_str(), trade_state.quote_asset.as_str()) {
            ("XRP", "RLUSD") => (xrp_usd, format!("1 XRP = {:.4} RLUSD", xrp_usd)),
            ("RLUSD", "XRP") => {
                let rate = if xrp_usd > 0.0 { 1.0 / xrp_usd } else { 0.0 };
                (rate, format!("1 RLUSD = {:.4} XRP", rate))
            }
            ("XRP", "EUROP") => (xrp_eur, format!("1 XRP = {:.4} EUROP", xrp_eur)),
            ("EUROP", "XRP") => {
                let rate = if xrp_eur > 0.0 { 1.0 / xrp_eur } else { 0.0 };
                (rate, format!("1 EUROP = {:.4} XRP", rate))
            }
            ("EUROP", "RLUSD") => {
                let rate = if eur_usd > 0.0 { 1.0 / eur_usd } else { 0.0 };
                (rate, format!("1 EUROP = {:.4} RLUSD", rate))
            }
            ("RLUSD", "EUROP") => (eur_usd, format!("1 RLUSD = {:.4} EUROP", eur_usd)),
            _ => (0.0, String::from("Select a valid pair")),
        };

        ui.label(
            RichText::new(rate_text)
                .size(label_font_size)
                .color(super::styles::text_color(is_dark_mode))
                .strong(),
        );
    });
}
  
pub mod asset_selector;
pub mod inputs;
pub mod trade_flags;
pub mod next_button;