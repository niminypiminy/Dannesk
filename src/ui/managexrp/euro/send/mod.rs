// src/ui/managexrp/euro/send/mod.rs
use egui::{Ui, Color32, Vec2, Pos2, Align2, Area, Frame};
use crate::channel::{CHANNEL, SendEuroTransaction, SendEuroTransactionState, WSCommand};
use uuid::Uuid;
use tokio::sync::mpsc;

mod step1;
mod step2;
mod step3;
mod buffer_manager;
mod buffers;
mod styles;

pub use step1::render_step1;
pub use step2::render_step2;
pub use step3::render_step3;
pub use buffers::{get_buffers, clear_buffer};
pub use buffer_manager::BufferManager;

pub fn view(ui: &mut Ui, send_state: &mut SendEuroTransaction, commands_tx: mpsc::Sender<WSCommand>) -> bool {
    let theme_rx = CHANNEL.theme_user_rx.clone();
    let send_euro_tx = CHANNEL.send_euro_tx.clone();
    let wallet_balance_rx = CHANNEL.wallet_balance_rx.clone();
    let is_dark_mode = theme_rx.borrow().0;

    // Clone inner data
    let (_balance, wallet_address, _xrp_active, _private_key_deleted) = wallet_balance_rx.borrow().clone();

    // Local state from caller
    let mut local_state = send_state.clone();
    let mut should_close = false;

    // Get or initialize buffer_id
    let buffer_id = if let Some(id) = local_state.buffer_id.clone() {
        id
    } else {
        let new_id = Uuid::new_v4().to_string();
        local_state.buffer_id = Some(new_id.clone());
        new_id
    };

    // Initialize BufferManager with existing buffers
    let (address_buffer, euro_amount_buffer, passphrase_buffer, seed_buffer, input_mode) = get_buffers(&buffer_id);
    let mut buffer_manager = BufferManager::new(
        &buffer_id,
        address_buffer,
        euro_amount_buffer,
        passphrase_buffer,
        seed_buffer,
        input_mode,
    );

    // Update modal state with buffer_id if changed
    if local_state.buffer_id != send_state.buffer_id {
        let _ = send_euro_tx.send(SendEuroTransactionState {
            send_euro: Some(local_state.clone()),
        });
    }

    // Define text color
    let text_color = styles::get_text_color(is_dark_mode);

    // Define modal size
    let modal_size = Vec2::new(350.0, 150.0);
    let screen_size = ui.ctx().input(|i| i.screen_rect.size());
    let pos = Pos2::new(
        (screen_size.x - modal_size.x) / 2.0,
        (screen_size.y - modal_size.y) / 2.0,
    );

    // Create overlay area
    Area::new(egui::Id::new(format!("send_euro_overlay_{}", buffer_id)))
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
            Frame::group(ui.style())
                .fill(styles::modal_fill(is_dark_mode))
                .stroke(styles::modal_stroke())
                .outer_margin(0.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_min_size(modal_size);
                    ui.set_max_size(modal_size);

                    // Add close button
                    styles::close_button(ui, &buffer_id, &mut should_close);

                    // Render content based on step
                    ui.allocate_ui_with_layout(
                        modal_size,
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            match local_state.step {
                                1 => render_step1(
                                    ui,
                                    &mut local_state,
                                    &mut buffer_manager,
                                    is_dark_mode,
                                    text_color,
                                    &send_euro_tx,
                                ),
                                2 => render_step2(
                                    ui,
                                    &mut local_state,
                                    &mut buffer_manager,
                                    is_dark_mode,
                                    text_color,
                                    &send_euro_tx,
                                ),
                                3 => render_step3(
                                    ui,
                                    &mut local_state,
                                    &mut buffer_manager,
                                    wallet_address,
                                    is_dark_mode,
                                    text_color,
                                    &send_euro_tx,
                                    commands_tx.clone(),
                                ),
                                _ => {}
                            }
                        },
                    );
                });
        });

    // Handle close
    if should_close {
        let _ = send_euro_tx.send(SendEuroTransactionState { send_euro: None });
        clear_buffer(&buffer_id);
    }

    // Update caller’s state
    *send_state = local_state;
    should_close
}