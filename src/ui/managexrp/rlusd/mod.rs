use egui::{Ui, RichText, Frame, Margin, Color32, Stroke};
use crate::channel::{CHANNEL, XRPModalState, ActiveView, WSCommand};
use tokio::sync::mpsc;

pub mod rlusd;
pub mod enable;
pub mod shared_utils;
pub mod receiverlusd;
pub mod send;
pub mod modify;
pub mod ripple;

pub fn render_rlusd_balance(ui: &mut Ui, _commands_tx: mpsc::Sender<WSCommand>) {
    let rlusd_rx = CHANNEL.rlusd_rx.clone();
    let xrp_modal_tx = CHANNEL.xrp_modal_tx.clone();
    let (_balance, has_rlusd, _trust_line_limit) = rlusd_rx.borrow().clone();
    let (is_dark_mode, _, _) = CHANNEL.theme_user_rx.borrow().clone();

    // Define text color based on theme
    let text_color = if is_dark_mode {
        Color32::from_rgb(255, 254, 250)
    } else {
        Color32::from_rgb(34, 34, 34)
    };

    ui.vertical(|ui| {
        if has_rlusd {
            // Render the RLUSD balance view if has_rlusd is true
            rlusd::render_rlusd_balance(ui);
        } else {
            // Render an enable button if has_rlusd is false
            ui.set_min_height(ui.available_height());
            let available_size = ui.available_size();
            let button_height = 36.0; // Kept for layout consistency
            let total_content_height = 30.0 + 20.0 + button_height;

            ui.add_space((available_size.y - total_content_height) / 2.0);

            ui.vertical_centered(|ui| {
                ui.label(RichText::new("RLUSD").size(30.0).color(text_color));
                ui.add_space(20.0);

                let available_width = ui.available_width();
                let button_text_size = (available_width * 0.04).clamp(14.0, 16.0);
                let button_width = (available_width * 0.2).clamp(100.0, 200.0);

                let original_visuals = ui.visuals().clone();
                if !is_dark_mode {
                    ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, text_color);
                    ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(2.0, text_color);
                    ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(210, 210, 210);
                }

                Frame::NONE
                    .inner_margin(Margin {
                        left: 8,
                        right: 8,
                        top: 4,
                        bottom: 4,
                    })
                    .corner_radius(4.0)
                    .show(ui, |ui| {
                        if ui
                            .add(
                                egui::Button::new(RichText::new("Enable RLUSD").size(button_text_size).color(text_color))
                                    .min_size(egui::Vec2::new(button_width, button_text_size * 2.0))
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
                            .clicked()
                        {
                            let _ = xrp_modal_tx.send(XRPModalState {
                                import_wallet: None,
                                create_wallet: None,
                                view_type: ActiveView::Enable,
                            });
                            ui.ctx().request_repaint();
                        }
                    });

                ui.visuals_mut().widgets = original_visuals.widgets;
            });
        }
    });
}