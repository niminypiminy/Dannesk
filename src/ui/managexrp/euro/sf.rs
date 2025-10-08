use egui::{Ui, RichText, Area, Pos2, Vec2, Color32, Frame, Align2};
use crate::channel::{CHANNEL, XRPModalState, ActiveView};

pub fn render(ui: &mut Ui) -> bool {
    let mut should_close = false;

    // Get theme information
    let (is_dark_mode, _, _) = CHANNEL.theme_user_rx.borrow().clone();

    // Define text color based on theme
    let text_color = if is_dark_mode {
        egui::Color32::from_rgb(255, 254, 250) // #fffefa for dark theme
    } else {
        egui::Color32::from_rgb(61, 12, 60) // #2d3a4b for light theme
    };

    // Calculate overlay position (centered)
    let screen_size = ui.ctx().input(|i| i.screen_rect.size());
    let modal_size = Vec2::new(300.0, 100.0); // Slightly taller to fit text
    let pos = Pos2::new(
        (screen_size.x - modal_size.x) / 2.0,
        (screen_size.y - modal_size.y) / 2.0,
    );

    // Render overlay using Area
    Area::new(egui::Id::new("schuman_financial_overlay"))
        .fixed_pos(pos)
        .anchor(Align2::CENTER_CENTER, Vec2::splat(0.0))
        .show(ui.ctx(), |ui| {
            // Semi-transparent background
            ui.painter().rect_filled(
                ui.ctx().input(|i| i.screen_rect),
                0.0,
                Color32::from_black_alpha(200),
            );

            // Overlay content frame
            Frame::popup(ui.style())
                .fill(ui.style().visuals.panel_fill)
                .stroke(egui::Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
                .outer_margin(0.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_min_size(modal_size);
                    ui.set_max_size(modal_size);

                    // Close button in top-right corner
                    Area::new(egui::Id::new("sf_close_button"))
                        .anchor(Align2::RIGHT_TOP, Vec2::new(-5.0, 5.0))
                        .show(ui.ctx(), |ui| {
                            if ui.button(RichText::new("X")).clicked() {
                                should_close = true;
                            }
                        });

                    // Main content with centered layout
                    ui.allocate_ui_with_layout(
                        modal_size,
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            ui.add_space(10.0);
                            ui.label(
                                RichText::new("EUROP")
                                    .size(16.0)
                                    .strong()
                                    .color(text_color),
                            );
                            ui.add_space(5.0);
                            ui.label(
                                RichText::new("The EUROP token is pegged to the euro. It is owned and managed by Schuman Financial, and is audited by the accounting firm KPMG.")
                                    .size(14.0)
                                    .color(text_color),
                            );
                            ui.add_space(10.0);
                        },
                    );
                });
        });

    // If the modal should close, update state via channel
    if should_close {
        let _ = CHANNEL.xrp_modal_tx.send(XRPModalState {
            import_wallet: None,
            create_wallet: None,
            view_type: ActiveView::EURO,
        });
    }

    should_close
}