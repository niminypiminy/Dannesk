use egui::{Ui, RichText, Color32, Stroke, Margin, Frame, TextEdit, Area, Align2, Vec2};

pub fn text_color(is_dark_mode: bool) -> Color32 {
    if is_dark_mode {
        Color32::from_rgb(255, 254, 250) 
    } else {
        Color32::from_rgb(34, 34, 34) 
    }
}

pub fn button_text_color() -> Color32 {
    Color32::from_rgb(255, 254, 250) // Always light (#fffefa) for buttons
}

pub fn modal_fill(is_dark_mode: bool) -> Color32 {
    if is_dark_mode { Color32::from_rgb(30, 30, 30) } else { Color32::WHITE }
}

pub fn modal_stroke() -> Stroke {
    Stroke::new(1.0, Color32::from_rgb(200, 200, 200))
}

pub fn seed_frame(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let is_dark_mode = crate::channel::CHANNEL.theme_user_rx.borrow().0;
    Frame::group(ui.style())
        .fill(if is_dark_mode { Color32::from_rgb(50, 50, 50) } else { Color32::from_rgb(200, 200, 200) })
        .stroke(Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
        .inner_margin(Margin::same(8))
        .show(ui, add_contents);
}

pub fn styled_text_edit(ui: &mut Ui, buffer: &mut String, is_dark_mode: bool) -> egui::Response {
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
    ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, text_color(is_dark_mode));
    ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(2.0, text_color(is_dark_mode));
    let response = ui.add(
        TextEdit::singleline(buffer)
            .password(true)
            .desired_width(280.0)
            .margin(Margin::same(4))
            .min_size(egui::vec2(0.0, 18.0))
            .frame(true),
    );
    ui.visuals_mut().widgets = original_visuals.widgets;
    response
}

pub fn close_button(ui: &mut Ui, buffer_id: &str, should_close: &mut bool) {
    Area::new(egui::Id::new(format!("close_button_{}", buffer_id)))
        .anchor(Align2::RIGHT_TOP, Vec2::new(-5.0, 5.0))
        .show(ui.ctx(), |ui| {
            if ui.button(RichText::new("X").size(14.0).color(button_text_color())).clicked() {
                *should_close = true;
            }
        });
}