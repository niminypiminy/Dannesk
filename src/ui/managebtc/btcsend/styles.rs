// src/ui/managexrp/xrpsend/styles.rs

use egui::{Color32, Stroke, Ui, TextEdit, Margin, Area, Align2, Vec2, RichText};

pub fn get_text_color(is_dark_mode: bool) -> Color32 {
    if is_dark_mode {
        Color32::from_rgb(255, 254, 250)
    } else {
        Color32::from_rgb(34, 34, 34)
    }
}

pub fn modal_fill(is_dark_mode: bool) -> Color32 {
    if is_dark_mode { Color32::from_rgb(30, 30, 30) } else { Color32::WHITE }
}

pub fn modal_stroke() -> Stroke {
    Stroke::new(1.0, Color32::from_rgb(200, 200, 200))
}

pub fn close_button(ui: &mut Ui, buffer_id: &str, should_close: &mut bool) {
    Area::new(egui::Id::new(format!("close_button_{}", buffer_id)))
        .anchor(Align2::RIGHT_TOP, Vec2::new(-5.0, 5.0))
        .show(ui.ctx(), |ui| {
            if ui.button(RichText::new("X").size(14.0)).clicked() {
                *should_close = true;
            }
        });
}

pub fn styled_text_edit(
    ui: &mut Ui,
    buffer: &mut String,
    width: f32,
    is_dark_mode: bool,
    is_password: bool,
) -> egui::Response {
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
    if !is_dark_mode {
        ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(30, 29, 27));
        ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(2.0, Color32::from_rgb(30, 29, 27));
    }
    let response = ui.add(
        TextEdit::singleline(buffer)
            .password(is_password)
            .desired_width(width)
            .margin(Margin::same(4))
            .min_size(egui::vec2(0.0, 18.0))
            .frame(true),
    );
    ui.visuals_mut().widgets = original_visuals.widgets;
    response
}