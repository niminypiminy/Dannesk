use egui::{Ui, RichText, Area, Pos2, Vec2, Color32, Frame, Align2, Grid, ScrollArea};
use crate::channel::{CHANNEL, BTCModalState, BTCActiveView, BitcoinTransactionStatus};
use chrono::{DateTime, Utc, TimeZone};

pub fn render(ui: &mut Ui) -> bool {
    let mut should_close = false;

    // Clone channels
    let btc_transactions_rx = CHANNEL.btc_transactions_rx.clone();
    let theme_user_rx = CHANNEL.theme_user_rx.clone();

    // Borrow data
    let transactions = btc_transactions_rx.borrow();
    let (is_dark_mode, _, _) = theme_user_rx.borrow().clone();

    // Define text color based on theme
    let text_color = if is_dark_mode {
        Color32::from_rgb(255, 254, 250) // #fffefa for dark theme
    } else {
        Color32::from_rgb(34, 34, 34) // #1e1d1b for light theme
    };

    // Calculate overlay position (centered)
    let screen_size = ui.ctx().input(|i| i.screen_rect.size());
    let modal_size = Vec2::new(
        (screen_size.x * 0.8).clamp(700.0, 1200.0),
        (screen_size.y * 0.6).clamp(300.0, 600.0),
    );
    let pos = Pos2::new(
        (screen_size.x - modal_size.x) / 2.0,
        (screen_size.y - modal_size.y) / 2.0,
    );

    // Render overlay using Area
    Area::new(egui::Id::new("btc_transaction_overlay"))
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
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_min_size(modal_size);
                    ui.set_max_size(modal_size);

                    // Dynamic sizing
                    let available_width = ui.available_width();
                    let header_size = (available_width * 0.035).clamp(14.0, 16.0);
                    let text_size = (available_width * 0.03).clamp(12.0, 14.0);
                    let spacing = (available_width * 0.03).clamp(8.0, 12.0);

                    // Close button
                    Area::new(egui::Id::new("btc_transaction_close_button"))
                        .anchor(Align2::RIGHT_TOP, Vec2::new(-5.0, 5.0))
                        .show(ui.ctx(), |ui| {
                            if ui
                                .button(RichText::new("X").size((modal_size.x * 0.04).clamp(12.0, 14.0)).color(text_color))
                                .clicked()
                            {
                                should_close = true;
                            }
                        });

                    // Center the grid
                    let total_grid_width = (available_width * 0.9).clamp(600.0, 1000.0);
                    if available_width > total_grid_width {
                        ui.add_space((available_width - total_grid_width) / 2.0);
                    }

                    // Scrollable grid for transactions
                    if transactions.transactions.is_empty() {
                        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                            ui.label(RichText::new("No Bitcoin transactions yet").size(header_size).color(text_color));
                        });
                    } else {
                        ScrollArea::both()
                            .auto_shrink([false, false])
                            .max_height(modal_size.y - spacing * 3.0)
                            .show(ui, |ui| {
                                Grid::new("btc_transactions_grid")
                                    .striped(true)
                                    .spacing([spacing / 2.0, spacing / 3.0])
                                    .min_col_width((total_grid_width - spacing * 6.0) / 7.0)
                                    .show(ui, |ui| {
                                        // Header row
                                        ui.label(RichText::new("Tx ID").size(header_size).strong().color(text_color));
                                        ui.label(RichText::new("Status").size(header_size).strong().color(text_color));
                                        ui.label(RichText::new("Amount").size(header_size).strong().color(text_color));
                                        ui.label(RichText::new("Fee").size(header_size).strong().color(text_color));
                                        ui.label(RichText::new("Receiver").size(header_size).strong().color(text_color));
                                        ui.label(RichText::new("Sender").size(header_size).strong().color(text_color));
                                        ui.label(RichText::new("Timestamp").size(header_size).strong().color(text_color));
                                        ui.end_row();

                                        // Collect and sort transactions by timestamp (newest first), limit to 20
                                        let mut sorted_transactions: Vec<_> = transactions.transactions.iter().collect();
                                        sorted_transactions.sort_by(|a, b| {
                                            let a_time = a.1.timestamp.parse::<i64>()
                                                .ok()
                                                .and_then(|secs| Utc.timestamp_opt(secs, 0).single())
                                                .unwrap_or(DateTime::<Utc>::MIN_UTC);
                                            let b_time = b.1.timestamp.parse::<i64>()
                                                .ok()
                                                .and_then(|secs| Utc.timestamp_opt(secs, 0).single())
                                                .unwrap_or(DateTime::<Utc>::MIN_UTC);
                                            b_time.cmp(&a_time)
                                        });
                                        sorted_transactions.truncate(20);

                                        // Transaction rows
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

                                            ui.label(
                                                RichText::new(format!("{:?}", tx.status))
                                                    .size(text_size)
                                                    .color(match tx.status {
                                                        BitcoinTransactionStatus::Success => Color32::from_rgb(0, 128, 0),
                                                        BitcoinTransactionStatus::Failed => Color32::from_rgb(200, 0, 0),
                                                        BitcoinTransactionStatus::Pending => Color32::from_rgb(255, 165, 0),
                                                        BitcoinTransactionStatus::Cancelled => Color32::from_rgb(128, 128, 128),
                                                    })
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
                                            ui.label(
                                                RichText::new(
                                                    if tx.fees.is_empty() {
                                                        "—".to_string()
                                                    } else {
                                                        tx.fees.clone()
                                                    }
                                                )
                                                .size(text_size)
                                                .color(text_color)
                                            );

                                           // Receiver: Truncate to 15 chars with copy button
let display_receiver = if tx.receiver_addresses.is_empty() {
    "—".to_string()
} else {
    let full_receiver = tx.receiver_addresses.join(", ");
    if full_receiver.len() > 15 {
        format!("{}...", &full_receiver[..15])
    } else {
        full_receiver
    }
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
        let full_receiver = tx.receiver_addresses.join(", ");
        ui.label(RichText::new(&format!("Full Receiver: {}", full_receiver)).color(text_color));
        if ui.button(RichText::new("Copy").size(text_size).color(text_color)).clicked() {
            ui.ctx().copy_text(full_receiver);
        }
    });
});

// Sender: Truncate to 15 chars with copy button
let display_sender = if tx.sender_addresses.is_empty() {
    "—".to_string()
} else {
    let full_sender = tx.sender_addresses.join(", ");
    if full_sender.len() > 15 {
        format!("{}...", &full_sender[..15])
    } else {
        full_sender
    }
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
        let full_sender = tx.sender_addresses.join(", ");
        ui.label(RichText::new(&format!("Full Sender: {}", full_sender)).color(text_color));
        if ui.button(RichText::new("Copy").size(text_size).color(text_color)).clicked() {
            ui.ctx().copy_text(full_sender);
        }
    });
});

                                            let display_timestamp = if tx.timestamp.is_empty() || tx.timestamp == "0" {
                                                "—".to_string()
                                            } else {
                                                match tx.timestamp.parse::<i64>() {
                                                    Ok(secs) => {
                                                        Utc.timestamp_opt(secs, 0)
                                                            .single()
                                                            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                                                            .unwrap_or_else(|| "Invalid timestamp".to_string())
                                                    }
                                                    Err(_) => "Invalid timestamp".to_string(),
                                                }
                                            };
                                            ui.add(
                                                egui::Label::new(
                                                    RichText::new(&display_timestamp)
                                                        .size(text_size)
                                                        .color(text_color)
                                                )
                                                .sense(egui::Sense::click())
                                            )
                                            .on_hover_text(&format!("Full Timestamp: {}", if tx.timestamp.is_empty() || tx.timestamp == "0" { "—" } else { &tx.timestamp }));

                                            ui.end_row();
                                        }
                                    });
                            });
                    }
                });
        });

    // Update state if closing
    if should_close {
        let _ = CHANNEL.btc_modal_tx.send(BTCModalState {
            import_wallet: None,
            create_wallet: None,
            view_type: BTCActiveView::BTC,
        });
    }

    should_close
}