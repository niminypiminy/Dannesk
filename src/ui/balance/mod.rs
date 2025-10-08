// src/ui/balance/mod.rs
use egui::{Ui, RichText, Grid, Frame};
use crate::channel::CHANNEL;
use crate::utils::svg_render::SvgCanvas;

pub fn render_balance(ui: &mut Ui) {
    // Clone channels
    let wallet_balance_rx = CHANNEL.wallet_balance_rx.clone();
    let bitcoin_wallet_rx = CHANNEL.bitcoin_wallet_rx.clone();
    let rates_rx = CHANNEL.rates_rx.clone();
    let rlusd_rx = CHANNEL.rlusd_rx.clone();
    let euro_rx = CHANNEL.euro_rx.clone();
    let theme_user_rx = CHANNEL.theme_user_rx.clone();

    // Clone inner data to avoid borrowing
    let (xrp_balance, _wallet_address, _is_active, _privatekey) = wallet_balance_rx.borrow().clone();
    let (btc_balance, _btc_address, _private_key_deleted) = bitcoin_wallet_rx.borrow().clone();
    let rates = rates_rx.borrow().clone();
    let (rlusd_balance, _has_trustline, _trust_limit) = rlusd_rx.borrow().clone();
    let (euro_balance, _euro_has_trustline, _euro_trust_limit) = euro_rx.borrow().clone();
    let (is_dark_mode, _, hide_balance) = theme_user_rx.borrow().clone();

    // Define text color based on theme
    let text_color = if is_dark_mode {
        egui::Color32::from_rgb(255, 254, 250) // Off-white for dark mode
    } else {
        egui::Color32::from_rgb(34, 34, 34) // Dark grey for light mode
    };

    ui.vertical(|ui| {
        // Get rates and fix RLUSD rate at $1.00, EUROP rate at €1.00 for display
        let xrp_rate = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;
        let btc_rate = rates.get("BTC/USD").copied().unwrap_or(0.0) as f64;
        let rlusd_rate = 1.0;
        let euro_display_rate = 1.0;
        let mut total_usd = xrp_balance * xrp_rate + btc_balance * btc_rate + rlusd_balance * rlusd_rate;
        if let Some(euro_to_usd_rate) = rates.get("EUR/USD").copied() {
            total_usd += euro_balance * euro_to_usd_rate as f64;
        }

        ui.set_min_height(ui.available_height());
        ui.vertical_centered(|ui| {
            // Dynamic vertical spacing
            let available_height = ui.available_height();
            ui.add_space((available_height * 0.3).max(20.0));

            // Dynamic font size for balance text
            let available_width = ui.available_width();
            let font_size = (available_width * 0.1).clamp(40.0, 100.0);

            let balance_text = if hide_balance {
                "****".to_string()
            } else {
                format!("${:.2}", total_usd)
            };
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
            ui.add_space(20.0 * (available_width / 800.0).clamp(0.5, 1.0));
            ui.horizontal(|ui| {
                // Dynamic grid width
                let total_grid_width = (ui.available_width() * 0.8).min(400.0);
                let available_width = ui.available_width();
                if available_width > total_grid_width {
                    ui.add_space((available_width - total_grid_width) / 2.0);
                }
                Frame::new()
                    .outer_margin(egui::Margin {
                        left: 30,
                        right: 0,
                        top: 0,
                        bottom: 0,
                    })
                    .show(ui, |ui| {
                        // Dynamic column width
                        let col_width = (total_grid_width - 20.0) / 3.0;
                        Grid::new("balance_grid")
                            .striped(true)
                            .num_columns(3)
                            .spacing([10.0 * (available_width / 800.0).clamp(0.5, 1.0), 5.0])
                            .min_col_width(col_width)
                            .show(ui, |ui| {
                                let text_size = (available_width * 0.015).clamp(12.0, 14.0);
                                ui.label(RichText::new("Token").size(text_size).strong().color(text_color));
                                ui.label(RichText::new("Balance").size(text_size).strong().color(text_color));
                                ui.label(RichText::new("Rate").size(text_size).strong().color(text_color));
                                ui.end_row();

                                // XRP Row
                                ui.horizontal(|ui| {
                                    ui.add(
                                        SvgCanvas::paint_svg(if is_dark_mode {
                                            "xrp_white.svg"
                                        } else {
                                            "xrp_dark.svg"
                                        })
                                        .fit_to_exact_size(egui::vec2(16.0 * (available_width / 800.0).clamp(0.5, 1.0), 16.0 * (available_width / 1000.0).clamp(0.5, 1.0))),
                                    );
                                    ui.add_space(4.0 * (available_width / 800.0).clamp(0.5, 1.0)); // Space between icon and text
                                    ui.label(RichText::new("XRP").size(text_size).color(text_color));
                                });
                                ui.label(
                                    RichText::new(if hide_balance {
                                        "**** XRP".to_string()
                                    } else {
                                        format!("{:.6} XRP", xrp_balance)
                                    })
                                    .size(text_size)
                                    .color(text_color)
                                )
                                .on_hover_text(if hide_balance { "Balance hidden for privacy" } else { "XRP balance" });
                                ui.label(RichText::new(format!("${:.4}", xrp_rate)).size(text_size).color(text_color));
                                ui.end_row();

                                // BTC Row
                                ui.horizontal(|ui| {
                                    ui.add(
                                        SvgCanvas::paint_svg("btc.svg")
                                            .fit_to_exact_size(egui::vec2(16.0 * (available_width / 800.0).clamp(0.5, 1.0), 16.0 * (available_width / 1000.0).clamp(0.5, 1.0))),
                                    );
                                    ui.add_space(4.0 * (available_width / 800.0).clamp(0.5, 1.0));
                                    ui.label(RichText::new("BTC").size(text_size).color(text_color));
                                });
                                ui.label(
                                    RichText::new(if hide_balance {
                                        "**** BTC".to_string()
                                    } else {
                                        format!("{:.6} BTC", btc_balance)
                                    })
                                    .size(text_size)
                                    .color(text_color)
                                )
                                .on_hover_text(if hide_balance { "Balance hidden for privacy" } else { "BTC balance" });
                                ui.label(RichText::new(format!("${:.2}", btc_rate)).size(text_size).color(text_color));
                                ui.end_row();

                                // RLUSD Row
                                ui.horizontal(|ui| {
                                    ui.add(
                                        SvgCanvas::paint_svg("rlusd.svg")
                                            .fit_to_exact_size(egui::vec2(16.0 * (available_width / 800.0).clamp(0.5, 1.0), 16.0 * (available_width / 1000.0).clamp(0.5, 1.0))),
                                    );
                                    ui.add_space(4.0 * (available_width / 800.0).clamp(0.5, 1.0));
                                    ui.label(RichText::new("RLUSD").size(text_size).color(text_color));
                                });
                                ui.label(
                                    RichText::new(if hide_balance {
                                        "**** RLUSD".to_string()
                                    } else {
                                        format!("{:.2} RLUSD", rlusd_balance)
                                    })
                                    .size(text_size)
                                    .color(text_color)
                                )
                                .on_hover_text(if hide_balance { "Balance hidden for privacy" } else { "RLUSD balance" });
                                ui.label(RichText::new(format!("${:.2}", rlusd_rate)).size(text_size).color(text_color));
                                ui.end_row();

                                // EUROP Row
                                ui.horizontal(|ui| {
                                    ui.add(
                                        SvgCanvas::paint_svg("europ.svg")
                                            .fit_to_exact_size(egui::vec2(16.0 * (available_width / 800.0).clamp(0.5, 1.0), 16.0 * (available_width / 1000.0).clamp(0.5, 1.0))),
                                    );
                                    ui.add_space(4.0 * (available_width / 800.0).clamp(0.5, 1.0));
                                    ui.label(RichText::new("EUROP").size(text_size).color(text_color));
                                });
                                ui.label(
                                    RichText::new(if hide_balance {
                                        "**** EUROP".to_string()
                                    } else {
                                        format!("{:.2} EUROP", euro_balance)
                                    })
                                    .size(text_size)
                                    .color(text_color)
                                )
                                .on_hover_text(if hide_balance { "Balance hidden for privacy" } else { "EUROP balance" });
                                ui.label(RichText::new(format!("€{:.2}", euro_display_rate)).size(text_size).color(text_color));
                                ui.end_row();
                            });
                    });
            });
        });
    });
}