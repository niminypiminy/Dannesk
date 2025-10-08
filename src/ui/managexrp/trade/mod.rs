use egui::{Ui, Vec2, Pos2, Align2, Area, Frame, Color32, RichText};
use crate::channel::{CHANNEL, XRPModalState, ActiveView, WSCommand};
use crate::ui::managexrp::trade::buffers::{get_or_init_buffer_id, get_buffer, clear_buffer, clear_all_buffers, update_buffers};
use crate::ui::managexrp::trade::styles::{text_color, modal_fill, modal_stroke, close_button};

pub mod buffers;
pub mod step1;
pub mod step2;
pub mod styles;

pub fn view(ui: &mut Ui, commands_tx: tokio::sync::mpsc::Sender<WSCommand>) -> bool {
    let theme_rx = CHANNEL.theme_user_rx.clone();
    let xrp_modal_tx = CHANNEL.xrp_modal_tx.clone();
    let is_dark_mode = theme_rx.borrow().0;

    let buffer_id = get_or_init_buffer_id();
    let mut state = get_buffer(&buffer_id).unwrap_or_default();

    if state.step == 0 || state.step > 2 {
        state.step = 1;
        update_buffers(
            &buffer_id,
            state.base_asset.clone(),
            state.quote_asset.clone(),
            state.amount.clone(),
            state.limit_price.clone(),
            state.flags.clone(),
            state.passphrase.clone(),
            state.seed.clone(),
            state.step,
            state.done,
            state.error.clone(),
            state.fee_percentage,
            state.search_query.clone(),
            state.input_mode.clone(),
        );
    }

    let mut should_close = false;

    let screen_size = ui.ctx().input(|i| i.screen_rect.size());
    let modal_size = Vec2::new(
        (screen_size.x * 0.6).clamp(400.0, 800.0),
        (screen_size.y * 0.5).clamp(300.0, 600.0),
    );
    let pos = Pos2::new(
        (screen_size.x - modal_size.x) / 2.0,
        (screen_size.y - modal_size.y) / 2.0,
    );

    Area::new(egui::Id::new(format!("trade_overlay_{}", buffer_id)))
        .fixed_pos(pos)
        .anchor(Align2::CENTER_CENTER, Vec2::splat(0.0))
        .show(ui.ctx(), |ui| {
            ui.painter().rect_filled(
                ui.ctx().input(|i| i.screen_rect),
                0.0,
                Color32::from_black_alpha(200),
            );

            Frame::group(ui.style())
                .fill(modal_fill(is_dark_mode))
                .stroke(modal_stroke())
                .outer_margin(0.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_min_size(modal_size);
                    ui.set_max_size(modal_size);

                    close_button(ui, &buffer_id, &mut should_close, is_dark_mode);

                    ui.allocate_ui_with_layout(
                        modal_size,
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            let title_font_size = (modal_size.x * 0.05).clamp(16.0, 20.0);
                            ui.label(
                                RichText::new(format!("Trade Modal - Step {}", state.step))
                                    .size(title_font_size)
                                    .color(text_color(is_dark_mode)),
                            );
                            match state.step {
                                1 => step1::render(ui, &mut state, &buffer_id),
                                2 => step2::render(ui, &mut state, &buffer_id, commands_tx.clone()),
                                _ => {
                                    ui.label(
                                        RichText::new("Invalid step")
                                            .color(text_color(is_dark_mode))
                                            .size(title_font_size),
                                    );
                                }
                            }
                        },
                    );
                });
        });

    let is_done = state.done;
    if should_close || is_done {
        clear_all_buffers();
        clear_buffer(&buffer_id);
        let _ = xrp_modal_tx.send(XRPModalState {
            import_wallet: None,
            create_wallet: None,
            view_type: ActiveView::XRP,
        });
    } else {
        update_buffers(
            &buffer_id,
            state.base_asset,
            state.quote_asset,
            state.amount,
            state.limit_price,
            state.flags,
            state.passphrase,
            state.seed,
            state.step,
            state.done,
            state.error,
            state.fee_percentage,
            state.search_query,
            state.input_mode,
        );
    }

    should_close || is_done
}