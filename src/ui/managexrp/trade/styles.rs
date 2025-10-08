use egui::{Ui, RichText, Color32, Stroke, Margin, TextEdit, WidgetText, Frame};

pub fn text_color(is_dark_mode: bool) -> Color32 {
    if is_dark_mode {
        Color32::from_rgb(255, 254, 250) // #fffefa
    } else {
        Color32::from_rgb(34, 34, 34) // #222222
    }
}

// Removed unused button_text_color()

pub fn modal_fill(is_dark_mode: bool) -> Color32 {
    if is_dark_mode {
        Color32::from_rgb(30, 30, 30)
    } else {
        Color32::WHITE
    }
}

pub fn modal_stroke() -> Stroke {
    Stroke::new(1.0, Color32::from_rgb(200, 200, 200))
}

pub fn styled_text_edit(
    ui: &mut Ui,
    buffer: &mut String,
    is_dark_mode: bool,
    is_password: bool,
    hint_text: impl Into<WidgetText>,
    desired_width: Option<f32>,
) -> egui::Response {
    let original_visuals = ui.visuals().clone();
    ui.visuals_mut().widgets.inactive.bg_fill = if is_dark_mode {
        Color32::from_rgba_premultiplied(50, 50, 50, 200)
    } else {
        Color32::from_rgb(220, 220, 220) // Match white_theme widgets.inactive.bg_fill
    };
    ui.visuals_mut().widgets.active.bg_stroke = Stroke::new(1.0, text_color(is_dark_mode));
    ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::new(0.5, text_color(is_dark_mode));
    ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::new(1.5, text_color(is_dark_mode));
    ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, text_color(is_dark_mode));
    ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(2.0, text_color(is_dark_mode));

    let response = Frame::new()
        .inner_margin(Margin::symmetric(8, 4))
        .corner_radius(4.0)
        .show(ui, |ui| {
            ui.add(
                TextEdit::singleline(buffer)
                    .password(is_password)
                    .desired_width(desired_width.unwrap_or(135.0))
                    .hint_text(hint_text)
                    .margin(Margin::symmetric(4, 2))
                    .min_size(egui::vec2(0.0, 24.0))
                    .frame(true),
            )
        })
        .inner;
    ui.visuals_mut().widgets = original_visuals.widgets;
    response
}

pub fn close_button(ui: &mut Ui, buffer_id: &str, should_close: &mut bool, is_dark_mode: bool) {
    egui::Area::new(egui::Id::new(format!("close_button_{}", buffer_id)))
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-5.0, 5.0))
        .show(ui.ctx(), |ui| {
            if ui
                .button(RichText::new("X").size(14.0).color(text_color(is_dark_mode)))
                .clicked()
            {
                *should_close = true;
            }
        });
}