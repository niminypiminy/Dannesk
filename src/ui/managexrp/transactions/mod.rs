use egui::{Ui, RichText, Area, Pos2, Vec2, Color32, Frame, Align2, Grid, ScrollArea};
use crate::channel::{CHANNEL, XRPModalState, ActiveView, TransactionStatus};
use chrono::{DateTime, Utc};
use crate::utils::svg_render::SvgCanvas;


pub fn render(ui: &mut Ui) -> bool {
    let mut should_close = false;

    // Clone channels
    let transactions_rx = CHANNEL.transactions_rx.clone();
    let theme_user_rx = CHANNEL.theme_user_rx.clone();

    // Borrow data
    let transactions = transactions_rx.borrow();
    let (is_dark_mode, _, _) = theme_user_rx.borrow().clone();

    // Define text color based on theme
    let text_color = if is_dark_mode {
        Color32::from_rgb(255, 254, 250) // #fffefa for dark theme
    } else {
        Color32::from_rgb(34, 34, 34) // #222222 for light theme
    };

    // Define modal fill color based on theme
    let modal_fill = if is_dark_mode {
        ui.style().visuals.panel_fill // Use default panel_fill for dark mode
    } else {
        Color32::from_rgb(240, 240, 240) // Light gray for light mode to ensure visibility
    };

    // Calculate overlay position (centered)
    let screen_size = ui.ctx().input(|i| i.screen_rect.size());
    // Dynamic modal size: 80% of screen width, 60% of screen height, clamped
    let modal_size = Vec2::new(
        (screen_size.x * 0.8).clamp(700.0, 1200.0),
        (screen_size.y * 0.6).clamp(300.0, 600.0),
    );
    let pos = Pos2::new(
        (screen_size.x - modal_size.x) / 2.0,
        (screen_size.y - modal_size.y) / 2.0,
    );

    // Render overlay using Area
    Area::new(egui::Id::new("transaction_overlay"))
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
                .fill(modal_fill)
                .stroke(egui::Stroke::new(1.0, if is_dark_mode { Color32::from_rgb(200, 200, 200) } else { Color32::from_rgb(180, 180, 180) }))
                .inner_margin(10)
                .show(ui, |ui| {
                    ui.set_min_size(modal_size);
                    ui.set_max_size(modal_size);

                    // Close button
                    Area::new(egui::Id::new("transaction_close_button"))
                        .anchor(Align2::RIGHT_TOP, Vec2::new(-5.0, 5.0))
                        .show(ui.ctx(), |ui| {
                            let button_size = (modal_size.x * 0.04).clamp(12.0, 14.0);
                            if ui.button(RichText::new("X").size(button_size).color(text_color)).clicked() {
                                should_close = true;
                            }
                        });

                    // Dynamic sizing
                    let available_width = ui.available_width();
                    let header_size = (available_width * 0.035).clamp(14.0, 16.0);
                    let text_size = (available_width * 0.03).clamp(12.0, 14.0);
                    let icon_size = 16.0 * (available_width / 1000.0).clamp(0.75, 1.0);
                    let spacing = (available_width * 0.03).clamp(8.0, 12.0);

                    ui.allocate_ui_with_layout(
                        modal_size,
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            ui.add_space(spacing);

                            // Center the grid
                            let total_grid_width = (available_width * 0.9).clamp(600.0, 1000.0);
                            if available_width > total_grid_width {
                                ui.add_space((available_width - total_grid_width) / 2.0);
                            }

                            // Scrollable grid for transactions
                            if transactions.transactions.is_empty() {
                                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                    ui.label(RichText::new("No transactions yet").size(text_size).color(text_color));
                                });
                            } else {
                                ScrollArea::both()
                                    .auto_shrink([false, false])
                                    .max_height(modal_size.y - spacing * 3.0)
                                    .show(ui, |ui| {
                                        Grid::new("transactions_grid")
                                            .striped(true)
                                            .spacing([spacing / 2.0, spacing / 3.0])
                                            .min_col_width((total_grid_width - spacing * 9.0) / 10.0) // Adjusted for 11 columns, more flexible min width
                                            .show(ui, |ui| {
                                                // Header row
                                                ui.label(RichText::new("Tx ID").size(header_size).strong().color(text_color));
                                                ui.label(RichText::new("Type").size(header_size).strong().color(text_color));
                                                ui.label(RichText::new("Status").size(header_size).strong().color(text_color));
                                                ui.label(RichText::new("Price").size(header_size).strong().color(text_color));
                                                ui.label(RichText::new("Amount").size(header_size).strong().color(text_color));
                                                ui.label(RichText::new("Currency").size(header_size).strong().color(text_color));
                                                ui.label(RichText::new("Fee").size(header_size).strong().color(text_color));
                                                ui.label(RichText::new("Flags").size(header_size).strong().color(text_color));
                                                ui.label(RichText::new("Receiver").size(header_size).strong().color(text_color));
                                                ui.label(RichText::new("Sender").size(header_size).strong().color(text_color));
                                                ui.label(RichText::new("Timestamp").size(header_size).strong().color(text_color));
                                                ui.end_row();

                                                // Transaction rows
                                                let mut sorted_transactions: Vec<_> = transactions.transactions.iter().collect();
                                                sorted_transactions.sort_by(|a, b| {
                                                    let a_time = DateTime::parse_from_rfc3339(a.1.timestamp.as_str())
                                                        .map(|dt| dt.with_timezone(&Utc))
                                                        .unwrap_or(DateTime::<Utc>::MIN_UTC);
                                                    let b_time = DateTime::parse_from_rfc3339(b.1.timestamp.as_str())
                                                        .map(|dt| dt.with_timezone(&Utc))
                                                        .unwrap_or(DateTime::<Utc>::MIN_UTC);
                                                    b_time.cmp(&a_time)
                                                });
                                                sorted_transactions.truncate(20);

for (tx_id, tx) in sorted_transactions {
    // Tx ID: Display first 15 characters with ellipsis if longer
    let display_tx_id = if tx_id.is_empty() {
        "—".to_string()
    } else if tx_id.len() > 15 {
        format!("{}...", &tx_id[..15])
    } else {
        tx_id.clone()
    };
    ui.add(
        egui::Label::new(
            RichText::new(&display_tx_id)
                .size(text_size)
                .color(text_color)
        )
        .sense(egui::Sense::click())
    )
    .on_hover_text(&format!("Full Tx ID: {}", tx_id));

                                                    ui.label(RichText::new(&tx.order_type).size(text_size).color(text_color));
                                                    ui.label(
                                                        RichText::new(format!("{:?}", tx.status))
                                                            .size(text_size)
                                                            .color(match tx.status {
                                                                TransactionStatus::Success => Color32::from_rgb(0, 128, 0),
                                                                TransactionStatus::Failed => Color32::from_rgb(200, 0, 0),
                                                                TransactionStatus::Pending => Color32::from_rgb(255, 165, 0),
                                                                TransactionStatus::Cancelled => Color32::from_rgb(128, 128, 128),
                                                            })
                                                    );
                                                    ui.label(
                                                        RichText::new(
                                                            if tx.execution_price.is_empty() || tx.execution_price == "0" {
                                                                "—".to_string()
                                                            } else {
                                                                tx.execution_price.clone()
                                                            }
                                                        )
                                                        .size(text_size)
                                                        .color(text_color)
                                                    );
                                                    ui.label(
                                                        RichText::new(
                                                            if tx.amount.is_empty() {
                                                                "—".to_string()
                                                            } else {
                                                                tx.amount.clone()
                                                            }
                                                        )
                                                        .size(text_size)
                                                        .color(text_color)
                                                    );

                                                    // Currency column with icon
ui.horizontal(|ui| {
    if tx.currency.is_empty() {
        ui.label(RichText::new("—").size(text_size).color(text_color));
    } else {
        let icon_path = match tx.currency.as_str() {
            "XRP" => if is_dark_mode {
                "xrp_white.svg"
            } else {
                "xrp_dark.svg"
            },
            "RLUSD" => "rlusd.svg",
            "EUROP" => "europ.svg",
            _ => {
                ui.label(RichText::new(&tx.currency).size(text_size).color(text_color));
                return;
            }
        };
        ui.add(
            SvgCanvas::paint_svg(icon_path)
                .fit_to_exact_size(egui::vec2(icon_size, icon_size))
                .tint(text_color),
        );
        ui.add_space(spacing / 3.0);
    }
});

                                                    ui.label(
                                                        RichText::new(
                                                            if tx.fee.is_empty() {
                                                                "—".to_string()
                                                            } else {
                                                                tx.fee.clone()
                                                            }
                                                        )
                                                        .size(text_size)
                                                        .color(text_color)
                                                    );
                                                    ui.label(
                                                        RichText::new(
                                                            tx.flags.as_ref()
                                                                .map(|f| if f.is_empty() { "—".to_string() } else { f.to_string() })
                                                                .unwrap_or("—".to_string())
                                                        )
                                                        .size(text_size)
                                                        .color(text_color)
                                                    );

                                                    // Receiver: Full display, no truncation
let display_receiver = if tx.receiver.is_empty() {
    "—".to_string()
} else if tx.receiver.len() > 15 {
    format!("{}...", &tx.receiver[..15])
} else {
    tx.receiver.clone()
};
ui.horizontal(|ui| {
    ui.add(
        egui::Label::new(
            RichText::new(&display_receiver)
                .size(text_size)
                .color(text_color)
        )
        .sense(egui::Sense::click())
    )
    .on_hover_ui(|ui| {
        ui.label(RichText::new(&format!("Full Receiver: {}", tx.receiver)).color(text_color));
        if ui.button(RichText::new("Copy").size(text_size).color(text_color)).clicked() {
            ui.ctx().copy_text(tx.receiver.clone());
        }
    });
});

// Sender: Truncate to 15 chars with copy button
let display_sender = if tx.sender.is_empty() {
    "—".to_string()
} else if tx.sender.len() > 15 {
    format!("{}...", &tx.sender[..15])
} else {
    tx.sender.clone()
};
ui.horizontal(|ui| {
    ui.add(
        egui::Label::new(
            RichText::new(&display_sender)
                .size(text_size)
                .color(text_color)
        )
        .sense(egui::Sense::click())
    )
    .on_hover_ui(|ui| {
        ui.label(RichText::new(&format!("Full Sender: {}", tx.sender)).color(text_color));
        if ui.button(RichText::new("Copy").size(text_size).color(text_color)).clicked() {
            ui.ctx().copy_text(tx.sender.clone());
        }
    });
});

                                                    // Timestamp: Full display, no truncation (removed len check)
                                                    let display_timestamp = if tx.timestamp.is_empty() {
                                                        "—".to_string()
                                                    } else {
                                                        tx.timestamp.clone()
                                                    };
                                                    ui.add(
                                                        egui::Label::new(
                                                            RichText::new(&display_timestamp)
                                                                .size(text_size)
                                                                .color(text_color)
                                                        )
                                                        .sense(egui::Sense::click())
                                                    )
                                                    .on_hover_text(&format!("Full Timestamp: {}", display_timestamp));

                                                    ui.end_row();
                                                }
                                            });
                                    });
                            }
                            ui.add_space(spacing);
                        },
                    );
                });
        });

    // Update state if closing
    if should_close {
        let _ = CHANNEL.xrp_modal_tx.send(XRPModalState {
            import_wallet: None,
            create_wallet: None,
            view_type: ActiveView::XRP,
        });
    }

    should_close
}