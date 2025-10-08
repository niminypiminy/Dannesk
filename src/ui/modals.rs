use egui::{Context, Color32, RichText, Area, Pos2, Vec2, Frame, Align2, Ui};
use crate::channel::CHANNEL;
use crate::ui::settings::settings::SettingsState;
use crate::ui::exchange;
use crate::ui::name::NameComponent;
use crate::ui::settings::components::SettingComponent;
use std::cell::RefCell;

thread_local! {
    pub static SETTINGS_STATE: RefCell<SettingsState> = RefCell::new(SettingsState::new());
    pub static NAME_STATE: RefCell<NameComponent> = RefCell::new(NameComponent::new());
}

pub fn render_modals(ctx: &Context) {
    let modal_state = CHANNEL.modal_rx.borrow().clone();

    if modal_state.settings {
        render_settings_modal(ctx);
    }

    if modal_state.exchange {
        render_exchange_modal(ctx);
    }

    if modal_state.name {
        render_name_modal(ctx);
    }
}

pub fn render_modal_frame(
    ctx: &Context,
    id: &str,
    modal_size: Vec2,
    content: impl FnOnce(&mut Ui, Color32),
    close_on_click: bool,
) -> bool {
    let mut should_close = false;
    let screen_size = ctx.input(|i| i.screen_rect.size());
    let pos = Pos2::new(
        (screen_size.x - modal_size.x) / 2.0,
        (screen_size.y - modal_size.y) / 2.0,
    );
    let (is_dark_mode, _, _) = CHANNEL.theme_user_rx.borrow().clone();
    let text_color = if is_dark_mode {
        Color32::from_rgb(255, 254, 250)
    } else {
        Color32::from_rgb(34, 34, 34)
    };

    Area::new(egui::Id::new(id))
        .fixed_pos(pos)
        .anchor(Align2::CENTER_CENTER, Vec2::splat(0.0))
        .show(ctx, |ui| {
            // Semi-transparent background
            ui.painter().rect_filled(
                ctx.input(|i| i.screen_rect),
                0.0,
                Color32::from_black_alpha(200),
            );

            ui.allocate_ui_with_layout(
                modal_size,
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    Frame::default()
                        .fill(ui.style().visuals.panel_fill)
                        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.set_min_size(modal_size);
                            ui.set_max_size(modal_size);

                            // Close button
                            Area::new(egui::Id::new(format!("{}_close_button", id)))
                                .anchor(Align2::RIGHT_TOP, Vec2::new(-5.0, 5.0))
                                .show(ctx, |ui| {
                                    if ui.button(RichText::new("X").size(14.0).color(text_color)).clicked() {
                                        should_close = true;
                                    }
                                });

                            // Render modal-specific content
                            content(ui, text_color);
                        });
                });
        });

    if should_close && close_on_click {
        let mut new_state = CHANNEL.modal_rx.borrow().clone();
        match id {
            "name_overlay" => new_state.name = false,
            "settings_overlay" => new_state.settings = false,
            _ => {}
        }
        let _ = CHANNEL.modal_tx.send(new_state);
    }

    should_close
}

fn render_name_modal(ctx: &Context) {
    let (is_dark_mode, current_name, _hide_balance) = CHANNEL.theme_user_rx.borrow().clone();
    let screen_size = ctx.input(|i| i.screen_rect.size());
    // Dynamic modal size: 40% of screen width, 30% of screen height, clamped
    let modal_size = Vec2::new(
        (screen_size.x * 0.4).clamp(300.0, 600.0),
        (screen_size.y * 0.3).clamp(200.0, 400.0),
    );

    render_modal_frame(
        ctx,
        "name_overlay",
        modal_size,
        |ui, text_color| {
            let available_width = ui.available_width();
            let title_size = (available_width * 0.05).clamp(18.0, 24.0);
            let spacing = (available_width * 0.03).clamp(8.0, 12.0);
            ui.add_space(spacing);
            ui.label(RichText::new("Change Name").size(title_size).strong().color(text_color));
            ui.add_space(spacing);
            NAME_STATE.with(|state| {
                let mut state = state.borrow_mut();
                state.render(ui, is_dark_mode, current_name);
            });
            ui.add_space(spacing);
        },
        true,
    );
}

fn render_settings_modal(ctx: &Context) {
    let screen_size = ctx.input(|i| i.screen_rect.size());
    // Dynamic modal size: 40% of screen width, 40% of screen height, clamped
    let modal_size = Vec2::new(
        (screen_size.x * 0.4).clamp(300.0, 600.0),
        (screen_size.y * 0.4).clamp(300.0, 500.0),
    );

    render_modal_frame(
        ctx,
        "settings_overlay",
        modal_size,
        |ui, text_color| {
            ui.add_space(8.0);
            // Dynamic title size based on modal width
            let title_size = (modal_size.x * 0.05).clamp(18.0, 24.0);
            ui.label(RichText::new("Change PIN").size(title_size).strong().color(text_color));
            ui.add_space(8.0);
            SETTINGS_STATE.with(|state| {
                let mut state = state.borrow_mut();
                state.render(ui);
            });
            ui.add_space(8.0);
        },
        true,
    );
}

fn render_exchange_modal(ctx: &Context) {
    let (is_dark_mode, _, _) = CHANNEL.theme_user_rx.borrow().clone();
    let text_color = if is_dark_mode {
        Color32::from_rgb(255, 254, 250)
    } else {
        Color32::from_rgb(34, 34, 34)
    };
    let rates = CHANNEL.rates_rx.borrow().clone();

    Area::new(egui::Id::new("exchange_rates_modal"))
        .fixed_pos(Pos2::new(0.0, 0.0))
        .show(ctx, |ui| {
            if exchange::render(ui, &rates, text_color, is_dark_mode) {
                let mut new_state = CHANNEL.modal_rx.borrow().clone();
                new_state.exchange = false;
                let _ = CHANNEL.modal_tx.send(new_state);
            }
        });
}