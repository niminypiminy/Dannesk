// src/ui/managexrp/euro/enable/mod.rs
use egui::{Ui, RichText, Area, Pos2, Vec2, Color32, Frame, Align2, Align, Layout};
use crate::channel::{CHANNEL, XRPModalState, ActiveView, WSCommand};
use crate::ui::managexrp::euro::shared_utils::{TrustlineStateBuffer, get_or_init_buffer_id, clear_buffer};
use self::buffer_manager::{BufferManager, InputMode};
use self::styles::{get_text_color, modal_fill, modal_stroke, close_button, styled_text_edit, styled_submit_button};
use tokio::sync::mpsc;

pub mod buffer_manager;
pub mod styles;

// Constants
const MODAL_SIZE: Vec2 = Vec2::new(350.0, 180.0);
const TITLE_SIZE: f32 = 18.0;
const SUBTEXT_SIZE: f32 = 14.0;
const LABEL_SIZE: f32 = 16.0;

// Helper function to initialize or retrieve state
fn get_state(ui: &mut Ui) -> TrustlineStateBuffer {
    ui.data_mut(|d| {
        d.get_persisted(egui::Id::new("enable_trustline_state"))
            .unwrap_or_else(|| {
                let new_state = TrustlineStateBuffer::default();
                d.insert_persisted(egui::Id::new("enable_trustline_state"), new_state.clone());
                new_state
            })
    })
}

// Helper function to render styled label
fn styled_label(ui: &mut Ui, text: &str, size: f32, strong: bool, color: Color32) {
    let mut rich_text = RichText::new(text).size(size).color(color);
    if strong {
        rich_text = rich_text.strong();
    }
    ui.label(rich_text);
}

pub fn validate_and_submit_transaction(
    buffer_manager: &mut BufferManager,
    wallet: Option<String>,
    commands_tx: mpsc::Sender<WSCommand>,
) {
    let trimmed_passphrase = buffer_manager.passphrase().trim();
    let trimmed_seed = buffer_manager.seed().trim();

    let (passphrase, seed) = match (trimmed_passphrase.is_empty(), trimmed_seed.is_empty()) {
        (true, true) => {
            buffer_manager.set_error(Some("Either passphrase or seed must be provided.".to_string()));
            return;
        }
        (false, false) => {
            buffer_manager.set_error(Some("Provide only one: passphrase or seed.".to_string()));
            return;
        }
        (false, true) => (Some(trimmed_passphrase.to_string()), None),
        (true, false) => (None, Some(trimmed_seed.to_string())),
    };

    if wallet.is_none() {
        buffer_manager.set_error(Some("No wallet selected".to_string()));
    } else {
        let (xrp_balance, _wallet_address, _is_active, _privatekey) = CHANNEL.wallet_balance_rx.borrow().clone();
        if xrp_balance < 0.20 + 0.00001 {
            buffer_manager.set_error(Some("Insufficient XRP balance (0.20 XRP reserve + fee required)".to_string()));
        } else {
            let command = WSCommand {
                command: "submit_transaction".to_string(),
                wallet,
                recipient: None,
                amount: None,
                passphrase,
                seed,
                trustline_limit: None,
                tx_type: Some("trustset_euro".to_string()),
                taker_pays: None,
                taker_gets: None,
                flags: None,
                wallet_type: None,
            };

            if let Err(e) = commands_tx.try_send(command) {
                buffer_manager.set_error(Some(format!("Failed to send command: {}", e)));
            } else {
                buffer_manager.set_done(true);
                buffer_manager.update_passphrase("");
                buffer_manager.update_seed("");
                buffer_manager.set_error(None);
            }
        }
    }
}

pub fn view(ui: &mut Ui, commands_tx: mpsc::Sender<WSCommand>) -> bool {
    let wallet = CHANNEL.wallet_balance_rx.borrow().1.clone();
    let (is_dark_mode, _, _) = CHANNEL.theme_user_rx.borrow().clone();
    let mut should_close = false;

    let text_color = get_text_color(is_dark_mode);
    let state = get_state(ui);
    let buffer_id = get_or_init_buffer_id(&mut state.clone());
    let mut buffer_manager = BufferManager::from_state_buffer(&buffer_id, state);

    // Modal positioning
    let screen_size = ui.ctx().input(|i| i.screen_rect.size());
    let pos = Pos2::new(
        (screen_size.x - MODAL_SIZE.x) / 2.0,
        (screen_size.y - MODAL_SIZE.y) / 2.0,
    );

    // Overlay
    Area::new(egui::Id::new("enable_overlay"))
        .fixed_pos(pos)
        .anchor(Align2::CENTER_CENTER, Vec2::splat(0.0))
        .show(ui.ctx(), |ui| {
            // Semi-transparent background
            ui.painter().rect_filled(
                ui.ctx().input(|i| i.screen_rect),
                0.0,
                Color32::from_black_alpha(200),
            );

            // Modal frame
            Frame::popup(ui.style())
                .fill(modal_fill(is_dark_mode))
                .stroke(modal_stroke())
                .outer_margin(0.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_min_size(MODAL_SIZE);
                    ui.set_max_size(MODAL_SIZE);

                    // Close button
                    close_button(ui, &buffer_id, &mut should_close, text_color, is_dark_mode);
                    if should_close {
                        buffer_manager.update_passphrase("");
                        buffer_manager.update_seed("");
                        buffer_manager.set_error(None);
                    }

                    // Enable trustline UI
                    ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                        styled_label(ui, "Enable EURO Trustline", TITLE_SIZE, true, text_color);
                        ui.add_space(10.0);
                        styled_label(
                            ui,
                            "This will create a EUROP trustline with a default limit of 1,000,000 EUROP. The xrpl blockchain will reserve 0.20 XRP for the trustline. You can adjust the trustline limit after activation.",
                            SUBTEXT_SIZE,
                            false,
                            text_color,
                        );
                        ui.add_space(10.0);

                        // Input mode selection
let mut current_mode = InputMode::from_string(buffer_manager.input_mode());
ui.with_layout(Layout::top_down(Align::Center), |ui| {
    let available_width = ui.available_width();
    let max_width = (available_width * 0.8).clamp(200.0, 300.0);
    ui.set_max_width(max_width);

    let button_width = (max_width / 2.0 - 8.0).clamp(80.0, 140.0);
    let total_content_width = 2.0 * button_width + 8.0;
    let base_padding = (max_width - total_content_width).max(0.0) / 2.0;
    let left_padding = base_padding + 60.0; // Add offset to push right
    let right_padding = base_padding.max(0.0); // Maintain or reduce right padding

    ui.horizontal(|ui| {
        ui.add_space(left_padding);
        if ui.radio_value(&mut current_mode, InputMode::Passphrase, "Passphrase").clicked() {
            buffer_manager.set_input_mode(InputMode::Passphrase.clone());
        }
        ui.add_space(8.0);
        if ui.radio_value(&mut current_mode, InputMode::Seed, "Seed").clicked() {
            buffer_manager.set_input_mode(InputMode::Seed.clone());
        }
        ui.add_space(right_padding);
    });
});
ui.add_space(8.0);

                        // Input field
                        ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                            let (label, mut value, update_fn) = match current_mode {
                                InputMode::Passphrase => (
                                    "Passphrase",
                                    buffer_manager.passphrase().to_string(),
                                    BufferManager::update_passphrase as fn(&mut BufferManager, &str),
                                ),
                                InputMode::Seed => (
                                    "Seed",
                                    buffer_manager.seed().to_string(),
                                    BufferManager::update_seed as fn(&mut BufferManager, &str),
                                ),
                            };
                            styled_label(ui, label, LABEL_SIZE, false, text_color);
                            ui.add_space(5.0);
                            let response = styled_text_edit(ui, &mut value, 250.0, is_dark_mode, true, text_color);
                            if response.changed() {
                                update_fn(&mut buffer_manager, &value);
                                buffer_manager.set_error(None);
                            }
                        });

                        // Error display
                        if let Some(error) = buffer_manager.error() {
                            ui.add_space(10.0);
                            styled_label(ui, error, SUBTEXT_SIZE, false, Color32::from_rgb(200, 100, 100));
                        }

                        // Submit button
                        ui.add_space(10.0);
                        ui.vertical_centered(|ui| {
                            styled_submit_button(
                                ui,
                                "Confirm",
                                &mut buffer_manager,
                                wallet.clone(),
                                commands_tx.clone(),
                                is_dark_mode,
                                text_color,
                            );
                        });
                    });
                });
        });

    // Persist state
    ui.data_mut(|d| d.insert_persisted(egui::Id::new("enable_trustline_state"), buffer_manager.to_state_buffer()));

    // Clear buffer and ui.data_mut on close
    if should_close || buffer_manager.done() {
        clear_buffer(&buffer_id);
        ui.data_mut(|d| d.remove::<TrustlineStateBuffer>(egui::Id::new("enable_trustline_state")));
        let _ = CHANNEL.xrp_modal_tx.send(XRPModalState {
            import_wallet: None,
            create_wallet: None,
            view_type: ActiveView::EURO,
        });
    }

    should_close || buffer_manager.done()
}