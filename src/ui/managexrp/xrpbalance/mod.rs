use egui::{Ui, RichText, Color32, Frame, CursorIcon, Margin, Grid};
use crate::channel::{CHANNEL, SignTransactionState, SignTransaction, XRPModalState, ActiveView, WSCommand};
use crate::ui::managexrp::xrpbalance::wallet_operations::WalletOperations;
use crate::ui::progressbar::ProgressBarState;
use uuid::Uuid;
use crate::utils::svg_render::SvgCanvas; // Import SvgCanvas
use tokio::sync::mpsc;

pub mod receive;
pub mod wallet_operations;

pub fn render_xrp_balance(ui: &mut Ui, commands_tx: mpsc::Sender<WSCommand>) {
    // Clone channels
    let wallet_balance_rx = CHANNEL.wallet_balance_rx.clone();
    let rates_rx = CHANNEL.rates_rx.clone();
    let theme_user_rx = CHANNEL.theme_user_rx.clone();
    let sign_transaction_tx = CHANNEL.sign_transaction_tx.clone();
    let xrp_modal_tx = CHANNEL.xrp_modal_tx.clone();

    // Clone inner data
    let (balance, wallet_address, xrp_active, private_key_deleted) = wallet_balance_rx.borrow().clone();
    let rates = rates_rx.borrow().clone();
    let (is_dark_mode, _, hide_balance) = theme_user_rx.borrow().clone();
    let exchange_rate = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;
    let wallet_value = balance * exchange_rate;
    let is_new_wallet = !xrp_active;

    // Define text color based on theme
    let text_color = if is_dark_mode {
        Color32::from_rgb(255, 254, 250) // Off-white for dark mode
    } else {
        Color32::from_rgb(34, 34, 34) // Dark grey for light mode
    };

    // Initialize progress bar state
    let mut progress_bar = ProgressBarState::new(
        if ui.data(|data| data.get_temp::<bool>("remove_wallet_triggered".into()).unwrap_or(false)) {
            "Removing Wallet".to_string()
        } else {
            "Deleting Key".to_string()
        }
    );

    // Render progress bar overlay; skip main UI if active
    if progress_bar.render_progress_bar(ui, 400.0) {
        return;
    }

    ui.set_min_height(ui.available_height());
    ui.vertical_centered(|ui| {
        let available_width = ui.available_width();
        let available_height = ui.available_height();

        // Dynamic vertical spacing
        let container_height = 220.0 + if is_new_wallet || xrp_active { 10.0 } else { 0.0 };
        ui.add_space((available_height * 0.5 - container_height / 2.0).max(20.0));

        // Dynamic font size for balance text
        let font_size = (available_width * 0.1).clamp(40.0, 100.0);
        let balance_text = if hide_balance {
            "****".to_string()
        } else {
            format!("${:.2}", wallet_value)
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

        ui.add_space(20.0 * (available_width / 800.0).clamp(0.5, 1.0));

        // Balance and Exchange Rate in Grid
ui.horizontal(|ui| {
    let total_grid_width = 210.0; // Fixed width to match original
    if available_width > total_grid_width {
        ui.add_space((available_width - total_grid_width) / 2.0);
    }
    Frame::new()
        .fill(ui.style().visuals.panel_fill) // Match theme
        .outer_margin(Margin {
            left: 15,
            right: 0,
            top: -20,
            bottom: 0,
        })
        .show(ui, |ui| {
            let col_width = (total_grid_width - 10.0) / 2.0;
            let text_size = (available_width * 0.015).clamp(12.0, 14.0);
            Grid::new("xrp_details_grid")
                .striped(true)
                .num_columns(2)
                .spacing([10.0 * (available_width / 800.0).clamp(0.5, 1.0), 5.0])
                .min_col_width(col_width)
                .show(ui, |ui| {
                    // Header row
                    ui.label(RichText::new("Balance").size(text_size).strong().color(text_color));
                    ui.label(RichText::new("Rate").size(text_size).strong().color(text_color));
                    ui.end_row();

                    // XRP row
                    ui.horizontal(|ui| {
    ui.add(
        SvgCanvas::paint_svg(if is_dark_mode { "xrp_white.svg" } else { "xrp_dark.svg" })
            .fit_to_exact_size(egui::vec2(16.0 * (available_width / 800.0).clamp(0.5, 1.0), 16.0 * (available_width / 1000.0).clamp(0.5, 1.0)))
    );
    ui.add_space(4.0 * (available_width / 800.0).clamp(0.5, 1.0));
    ui.label(
        RichText::new(if hide_balance {
            "**** XRP".to_string()
        } else {
            format!("{:.6}", balance)
        })
        .size(text_size)
        .color(text_color)
    )
    .on_hover_text(if hide_balance { "Balance hidden for privacy" } else { "XRP balance" });
});
                    ui.label(RichText::new(format!("${:.4}", exchange_rate)).size(text_size).color(text_color));
                    ui.end_row();
                });
        });
});

        ui.add_space(20.0 * (available_width / 800.0).clamp(0.5, 1.0));

        // Send/Receive Buttons
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 20.0 * (available_width / 800.0).clamp(0.5, 1.0);
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
            if send_response.clicked() {
                let _ = sign_transaction_tx.send(SignTransactionState {
                    send_transaction: Some(SignTransaction {
                        step: 1,
                        loading: false,
                        error: None,
                        done: false,
                        buffer_id: Some(Uuid::new_v4().to_string()),
                    }),
                });
                ui.ctx().request_repaint();
            }
            if receive_response.clicked() {
                let _ = xrp_modal_tx.send(XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: ActiveView::Receive,
                });
                ui.ctx().request_repaint();
            }
        });

        ui.add_space(20.0 * (available_width / 800.0).clamp(0.5, 1.0));

        // Remove/Delete Links
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 30.0 * (available_width / 800.0).clamp(0.5, 1.0);
            let text_size = (available_width * 0.02).clamp(14.0, 16.0);
            let remove_text = RichText::new("Remove Wallet")
                .size(text_size)
                .underline()
                .color(text_color);
            let mut delete_text = RichText::new(if private_key_deleted { "Key Deleted" } else { "Delete Key" })
                .size(text_size)
                .color(text_color);
            if private_key_deleted {
                delete_text = delete_text.strikethrough();
            } else {
                delete_text = delete_text.underline();
            }
            let remove_width = ui.fonts(|f| f.layout_no_wrap("Remove Wallet".into(), egui::FontId::proportional(text_size), text_color)).rect.width();
            let delete_width = ui.fonts(|f| f.layout_no_wrap(if private_key_deleted { "Key Deleted" } else { "Delete Key" }.into(), egui::FontId::proportional(text_size), text_color)).rect.width();
            let total_width = remove_width + ui.spacing().item_spacing.x + delete_width;
            if available_width > total_width {
                ui.add_space((available_width - total_width) / 2.0);
            }
            let remove_link = ui.add(
                egui::Label::new(remove_text)
                    .sense(egui::Sense::click())
            );
            let delete_link = ui.add(
                egui::Label::new(delete_text)
                    .sense(if private_key_deleted { egui::Sense::hover() } else { egui::Sense::click() })
            );
            if remove_link.hovered() || (delete_link.hovered() && !private_key_deleted) {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }
            if remove_link.clicked() {
                ui.data_mut(|data| {
                    data.insert_temp("remove_wallet_triggered".into(), true);
                });
                WalletOperations::remove_wallet(wallet_address.clone(), commands_tx.clone());
            }
            if delete_link.clicked() && !private_key_deleted {
                ui.data_mut(|data| {
                    data.insert_temp("remove_wallet_triggered".into(), false);
                });
                WalletOperations::delete_key(wallet_address.clone());
            }
        });

        ui.add_space(10.0 * (available_width / 800.0).clamp(0.5, 1.0));
    });
}