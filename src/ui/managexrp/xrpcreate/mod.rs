use egui::{Ui, Vec2, Pos2, Align2, Area, Frame, Color32};
use tokio::sync::mpsc;
use crate::channel::{XRPImport, XRPModalState, ActiveView, WSCommand};

pub mod buffers;
pub mod step1;
pub mod step2;
pub mod styles;

pub fn view(ui: &mut Ui, create_state: &mut XRPImport, commands_tx: mpsc::Sender<WSCommand>) -> bool {
    let theme_rx = crate::channel::CHANNEL.theme_user_rx.clone();
    let xrp_modal_tx = crate::channel::CHANNEL.xrp_modal_tx.clone();
    let is_dark_mode = theme_rx.borrow().0;

    let buffer_id = buffers::get_or_init_buffer_id(create_state);

    if create_state.buffer_id != Some(buffer_id.clone()) {
        create_state.buffer_id = Some(buffer_id.clone());
        let _ = xrp_modal_tx.send(XRPModalState {
            create_wallet: Some(create_state.clone()),
            import_wallet: None,
            view_type: ActiveView::XRP,
        });
    }

    let mut should_close = false;

    let screen_size = ui.ctx().input(|i| i.screen_rect.size());
    let modal_size = Vec2::new(350.0, 50.0);
    let pos = Pos2::new(
        (screen_size.x - modal_size.x) / 2.0,
        (screen_size.y - modal_size.y) / 2.0,
    );

    Area::new(egui::Id::new(format!("create_wallet_overlay_{}", buffer_id)))
        .fixed_pos(pos)
        .anchor(Align2::CENTER_CENTER, Vec2::splat(0.0))
        .show(ui.ctx(), |ui| {
            ui.painter().rect_filled(
                ui.ctx().input(|i| i.screen_rect),
                0.0,
                Color32::from_black_alpha(200),
            );

            Frame::group(ui.style())
                .fill(styles::modal_fill(is_dark_mode))
                .stroke(styles::modal_stroke())
                .outer_margin(0.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_min_size(modal_size);
                    ui.set_max_size(modal_size);

                    styles::close_button(ui, &buffer_id, &mut should_close);

                    ui.allocate_ui_with_layout(
                        modal_size,
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            if create_state.step == 1 {
                                step1::render(ui, create_state, &buffer_id);
                            } else if create_state.step == 2 {
                                // Pass commands_tx to step2::render
                                step2::render(ui, create_state, &buffer_id, commands_tx.clone());
                            }
                        },
                    );
                });
        });

    if should_close || create_state.done {
        if let Some(mut seed) = create_state.seed.take() {
            use zeroize::Zeroize; // Import here
            seed.zeroize(); // Overwrite with zeros
        }
        let _ = xrp_modal_tx.send(XRPModalState {
            create_wallet: None,
            import_wallet: None,
            view_type: ActiveView::XRP,
        });
        buffers::clear_buffer(&buffer_id); // Clear passphrase buffer
    }

    should_close || create_state.done
}