use egui::{Ui, RichText, Grid, Frame, Margin, Color32, CursorIcon};
use crate::channel::{CHANNEL, XRPModalState, SendRLUSDTransactionState, SendRLUSDTransaction, ActiveView};
use crate::utils::svg_render::SvgCanvas; // Import SvgCanvas
use uuid::Uuid;

pub fn render_rlusd_balance(ui: &mut Ui) {
    // Clone channels
    let rlusd_rx = CHANNEL.rlusd_rx.clone();
    let theme_user_rx = CHANNEL.theme_user_rx.clone();
    let xrp_modal_tx = CHANNEL.xrp_modal_tx.clone();
    let send_rlusd_tx = CHANNEL.send_rlusd_tx.clone();

    // Clone inner data to avoid borrowing
    let (rlusd_balance, _has_rlusd, _trust_line_limit) = rlusd_rx.borrow().clone();
    let (is_dark_mode, _, hide_balance) = theme_user_rx.borrow().clone();

    // Define text color based on theme
    let text_color = if is_dark_mode {
        Color32::from_rgb(255, 254, 250) // Off-white for dark mode
    } else {
        Color32::from_rgb(34, 34, 34) // Dark grey for light mode
    };

    // Fixed RLUSD rate at $1.00
    let rlusd_rate = 1.0;
    let total_usd = rlusd_balance * rlusd_rate;

    ui.set_min_height(ui.available_height());
    ui.vertical_centered(|ui| {
        let available_width = ui.available_width();
        let available_height = ui.available_height();

        // Dynamic vertical spacing
        ui.add_space((available_height * 0.3).max(20.0));

        // Dynamic font size for balance text
        let font_size = (available_width * 0.1).clamp(40.0, 100.0);
        let balance_text = if hide_balance {
            "****".to_string()
        } else {
            format!("${:.2}", total_usd)
        };

        // Render balance text
        ui.label({
            let mut job = egui::text::LayoutJob::default();
            if !hide_balance {
                let decimal_pos = balance_text.find('.').unwrap_or(balance_text.len());
                job.append(
                    "$",
                    0.0,
                    egui::TextFormat {
                        font_id: egui::FontId::new(font_size, egui::FontFamily::Name("DejaVuSansMono".into())),
                        color: text_color,
                        ..Default::default()
                    },
                );
                job.append(
                    &balance_text[1..decimal_pos],
                    0.0,
                    egui::TextFormat {
                        font_id: egui::FontId::new(font_size, egui::FontFamily::Name("DejaVuSansMonoBold".into())),
                        color: text_color,
                        ..Default::default()
                    },
                );
                if decimal_pos < balance_text.len() {
                    job.append(
                        &balance_text[decimal_pos..],
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::new(font_size, egui::FontFamily::Name("DejaVuSansMono".into())),
                            color: text_color,
                            ..Default::default()
                        },
                    );
                }
            } else {
                job.append(
                    &balance_text,
                    0.0,
                    egui::TextFormat {
                        font_id: egui::FontId::new(font_size, egui::FontFamily::Name("DejaVuSansMono".into())),
                        color: text_color,
                        ..Default::default()
                    },
                );
            }
            ui.fonts(|f| f.layout_job(job))
        });

        ui.add_space(20.0 * (available_width / 900.0).clamp(0.5, 1.0));

    // Balance and Trustline Info in Grid
ui.horizontal(|ui| {
    let total_grid_width = 4.0 * 100.0 + 3.0 * 10.0; // Original width: 4 columns * 100.0 + 3 spacings * 10.0
    if available_width > total_grid_width {
        ui.add_space((available_width - total_grid_width) / 2.0);
    }
    Frame::new()
        .outer_margin(Margin {
            left: 25,
            right: 0,
            top: 0,
            bottom: 0,
        })
        .show(ui, |ui| {
            let col_width = 100.0; // Original column width
            let text_size = (available_width * 0.015).clamp(12.0, 14.0);
            Grid::new("rlusd_details_grid")
                .striped(true)
                .num_columns(4)
                .spacing([10.0 * (available_width / 900.0).clamp(0.5, 1.0), 5.0])
                .min_col_width(col_width)
                .show(ui, |ui| {
                    // Header row
                    ui.label(RichText::new("Active").size(text_size).strong().color(text_color));
                    ui.label(RichText::new("Limit").size(text_size).strong().color(text_color));
                    ui.label(RichText::new("Balance").size(text_size).strong().color(text_color));
                    ui.label(RichText::new("Rate").size(text_size).strong().color(text_color));
                    ui.end_row();

                    // RLUSD row
                    ui.label(RichText::new(if _has_rlusd { "Yes" } else { "No" }).size(text_size).color(text_color));
                    ui.label(
                        RichText::new(
                            _trust_line_limit
                                .map(|limit| format!("{:.0}", limit))
                                .unwrap_or("0".to_string())
                        )
                        .size(text_size)
                        .color(text_color)
                    );
                   ui.horizontal(|ui| {
    let _ = ui.add(
        SvgCanvas::paint_svg("rlusd.svg")
            .fit_to_exact_size(egui::vec2(16.0 * (available_width / 900.0).clamp(0.5, 1.0), 16.0 * (available_width / 900.0).clamp(0.5, 1.0)))
    );
    ui.add_space(4.0 * (available_width / 900.0).clamp(0.5, 1.0));
    ui.label(
        RichText::new(if hide_balance {
            "**** RLUSD".to_string()
        } else {
            format!("{:.2}", rlusd_balance)
        })
        .size(text_size)
        .color(text_color)
    )
    
                    });
                    ui.label(RichText::new(format!("${:.2}", rlusd_rate)).size(text_size).color(text_color));
                    ui.end_row();
                });
        });
});
        ui.add_space(20.0 * (available_width / 900.0).clamp(0.5, 1.0));

        // Send/Receive Buttons
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 20.0 * (available_width / 900.0).clamp(0.5, 1.0);
            let text_size = (available_width * 0.03).clamp(20.0, 24.0);
            let send_text = RichText::new("↑")
                .size(text_size)
                .color(text_color)
                .font(egui::FontId::new(text_size, egui::FontFamily::Name("DejaVuSansMono".into())));
            let receive_text = RichText::new("↓")
                .size(text_size)
                .color(text_color)
                .font(egui::FontId::new(text_size, egui::FontFamily::Name("DejaVuSansMono".into())));
            let button_width = (available_width * 0.1).clamp(50.0, 60.0);
            let button_height = (available_width * 0.06).clamp(35.0, 40.0);
            let total_width = button_width * 2.0 + ui.spacing().item_spacing.x;
            if available_width > total_width {
                ui.add_space((available_width - total_width) / 2.0);
            }
            let send_response = ui.add_sized(
                [button_width, button_height],
                egui::Button::new(send_text),
            );
            let receive_response = ui.add_sized(
                [button_width, button_height],
                egui::Button::new(receive_text),
            );
            if receive_response.clicked() {
                let _ = xrp_modal_tx.send(XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: ActiveView::ReceiveRLUSD,
                });
                ui.ctx().request_repaint();
            }
            if send_response.clicked() {
                let _ = send_rlusd_tx.send(SendRLUSDTransactionState {
                    send_rlusd: Some(SendRLUSDTransaction {
                        step: 1,
                        loading: false,
                        error: None,
                        done: false,
                        buffer_id: Some(Uuid::new_v4().to_string()),
                    }),
                });
                ui.ctx().request_repaint();
            }
        });

        ui.add_space(20.0 * (available_width / 900.0).clamp(0.5, 1.0));

        // Modify Trustline Link
        let text_size = (available_width * 0.02).clamp(14.0, 16.0);
        let modify_trust = ui.add(
            egui::Label::new(
                RichText::new("Modify Trustline")
                    .size(text_size)
                    .underline()
                    .color(text_color)
            )
            .sense(egui::Sense::click())
        );

        if modify_trust.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        if modify_trust.clicked() {
            let _ = xrp_modal_tx.send(XRPModalState {
                import_wallet: None,
                create_wallet: None,
                view_type: ActiveView::TrustLine,
            });
            ui.ctx().request_repaint();
        }

        ui.add_space(10.0 * (available_width / 900.0).clamp(0.5, 1.0));
    });
}
