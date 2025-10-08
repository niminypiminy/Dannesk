use eframe::egui;

pub fn render_websocket_banner(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(10.0);

        egui::Frame::default()
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                    ui.label(
                        egui::RichText::new("Data temporarily unavailable. Please try again later.")
                            .color(ui.visuals().strong_text_color())
                            .size(14.0),
                    );
                });
            });
        ui.add_space(10.0);
    });
}