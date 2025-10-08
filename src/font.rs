use egui::{FontDefinitions, FontData, FontFamily};
use std::sync::Arc;

pub fn setup_custom_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    let dejavu_regular = include_bytes!("fonts/DejaVuSansMono.ttf");
    fonts.font_data.insert(
        "DejaVuSansMono".to_owned(),
        Arc::new(FontData::from_static(dejavu_regular)),
    );

    let dejavu_bold = include_bytes!("fonts/DejaVuSansMono-Bold.ttf");
    fonts.font_data.insert(
        "DejaVuSansMonoBold".to_owned(),
        Arc::new(FontData::from_static(dejavu_bold)),
    );

    fonts
        .families
        .entry(FontFamily::Name("DejaVuSansMono".into()))
        .or_insert_with(Vec::new)
        .push("DejaVuSansMono".to_owned());

    fonts
        .families
        .entry(FontFamily::Name("DejaVuSansMonoBold".into()))
        .or_insert_with(Vec::new)
        .push("DejaVuSansMonoBold".to_owned());

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_insert_with(Vec::new)
        .extend(vec![
            "DejaVuSansMono".to_owned(),
            "DejaVuSansMonoBold".to_owned(),
        ]);

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_insert_with(Vec::new)
        .extend(vec![
            "DejaVuSansMono".to_owned(),
            "DejaVuSansMonoBold".to_owned(),
        ]);

    ctx.set_fonts(fonts);
}