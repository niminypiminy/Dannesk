// src/ui/managexrp/euro/enable/styles.rs
use egui::{Ui, RichText, Area, Vec2, Margin, Color32, Frame, Align2, Stroke};
use super::buffer_manager::BufferManager;
use super::validate_and_submit_transaction;
use crate::channel::WSCommand;
use tokio::sync::mpsc;

pub fn get_text_color(is_dark_mode: bool) -> Color32 {
    if is_dark_mode {
        Color32::from_rgb(255, 254, 250)
    } else {
        Color32::from_rgb(34, 34, 34)
    }
}

pub fn modal_fill(is_dark_mode: bool) -> Color32 {
    if is_dark_mode {
        Color32::from_rgb(30, 30, 30)
    } else {
        Color32::from_rgb(240, 240, 240)
    }
}

pub fn modal_stroke() -> Stroke {
    Stroke::new(1.0, Color32::from_rgb(200, 200, 200))
}

pub fn close_button(ui: &mut Ui, buffer_id: &str, should_close: &mut bool, text_color: Color32, is_dark_mode: bool) {
    Area::new(egui::Id::new(format!("enable_close_button_{}", buffer_id)))
        .anchor(Align2::RIGHT_TOP, Vec2::new(-5.0, 5.0))
        .show(ui.ctx(), |ui| {
            let original_visuals = ui.visuals().clone();
            ui.visuals_mut().widgets.noninteractive.bg_stroke = Stroke::new(
                1.0,
                if is_dark_mode { Color32::from_rgb(180, 180, 180) } else { Color32::from_rgb(100, 100, 100) },
            );
            ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::new(
                1.0,
                if is_dark_mode { Color32::from_rgb(180, 180, 180) } else { Color32::from_rgb(100, 100, 100) },
            );
            ui.visuals_mut().widgets.active.bg_stroke = Stroke::new(
                1.0,
                if is_dark_mode { Color32::from_rgb(220, 220, 220) } else { Color32::from_rgb(150, 150, 150) },
            );
            ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(
                1.0,
                if is_dark_mode { Color32::from_rgb(255, 254, 250) } else { Color32::from_rgb(30, 29, 27) },
            );
            ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(
                2.0,
                if is_dark_mode { Color32::from_rgb(255, 254, 250) } else { Color32::from_rgb(30, 29, 27) },
            );
            ui.visuals_mut().widgets.inactive.bg_fill = if is_dark_mode {
                Color32::from_rgb(50, 50, 50)
            } else {
                Color32::from_rgb(200, 200, 200)
            };
            ui.visuals_mut().widgets.hovered.bg_fill = if is_dark_mode {
                Color32::from_rgb(70, 70, 70)
            } else {
                Color32::from_rgb(210, 210, 210)
            };
            ui.visuals_mut().widgets.active.bg_fill = if is_dark_mode {
                Color32::from_rgb(90, 90, 90)
            } else {
                Color32::from_rgb(180, 180, 180)
            };

            if ui
                .button(RichText::new("X").size(14.0).color(text_color))
                .clicked()
            {
                *should_close = true;
            }
            ui.visuals_mut().widgets = original_visuals.widgets;
        });
}

pub fn styled_submit_button(
    ui: &mut Ui,
    text: &str,
    buffer_manager: &mut BufferManager,
    wallet: Option<String>,
    commands_tx: mpsc::Sender<WSCommand>,
    is_dark_mode: bool,
    text_color: Color32,
) -> bool {
    let available_width = ui.available_width();
    let button_text_size = (available_width * 0.04).clamp(14.0, 16.0);
    let button_width = (available_width * 0.2).clamp(100.0, 200.0);

    let original_visuals = ui.visuals().clone();
    if !is_dark_mode {
        ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, text_color);
        ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(2.0, text_color);
        ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(210, 210, 210);
    }

    let response = Frame::NONE
        .inner_margin(Margin {
            left: 8,
            right: 8,
            top: 4,
            bottom: 4,
        })
        .corner_radius(4.0)
        .show(ui, |ui| {
            ui.add(
                egui::Button::new(RichText::new(text).size(button_text_size).color(text_color))
                    .min_size(Vec2::new(button_width, button_text_size * 2.0))
                    .fill(if is_dark_mode {
                        Color32::from_rgb(50, 50, 50)
                    } else {
                        Color32::from_rgb(200, 200, 200)
                    })
                    .stroke(if is_dark_mode {
                        Stroke::new(1.0, Color32::from_rgb(180, 180, 180))
                    } else {
                        Stroke::new(1.0, Color32::from_rgb(130, 130, 130))
                    }),
            )
        })
        .inner;

    let clicked = response.clicked();
    if clicked {
        validate_and_submit_transaction(buffer_manager, wallet, commands_tx);
    }

    ui.visuals_mut().widgets = original_visuals.widgets;
    clicked
}

pub fn styled_text_edit(
    ui: &mut Ui,
    text: &mut String,
    desired_width: f32,
    is_dark_mode: bool,
    is_password: bool,
    text_color: Color32,
) -> egui::Response {
    let available_width = ui.available_width();
    let label_size = (available_width * 0.035).clamp(12.0, 14.0);
    ui.visuals_mut().widgets.inactive.bg_fill = if is_dark_mode {
        Color32::from_rgba_premultiplied(50, 50, 50, 200)
    } else {
        Color32::from_rgb(220, 220, 220)
    };
    ui.visuals_mut().widgets.active.bg_stroke = Stroke::new(1.0, text_color);
    ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::new(0.5, text_color);
    ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::new(1.5, text_color);
    ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, text_color);
    ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(1.0, text_color);

    Frame::NONE
        .inner_margin(Margin {
            left: 8,
            right: 8,
            top: 4,
            bottom: 4,
        })
        .corner_radius(4.0)
        .show(ui, |ui| {
            ui.add(
                egui::TextEdit::singleline(text)
                    .desired_width(desired_width)
                    .text_color(text_color)
                    .hint_text(RichText::new(if is_password { "Enter passphrase or seed" } else { "Enter text" }).size(label_size).color(Color32::from_gray(100)))
                    .password(is_password)
                    .margin(Margin {
                        left: 4,
                        right: 4,
                        top: 2,
                        bottom: 2,
                    }),
            )
        }).inner
}