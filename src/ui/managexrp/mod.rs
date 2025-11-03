use egui::{Ui, RichText, Frame, Margin};
use crate::channel::{CHANNEL, XRPModalState, XRPImport, SignTransactionState, SendEuroTransactionState, SendRLUSDTransactionState, ActiveView, WSCommand};
use uuid::Uuid;
use crate::utils::svg_render::SvgCanvas; // Import SvgCanvas
use tokio::sync::mpsc;

pub mod xrpimport;
pub mod xrpcreate;
pub mod xrpbalance;
pub mod trade;
pub mod xrpsend;
pub mod rlusd;
pub mod transactions;
pub mod euro;

pub fn render_manage_xrp(ui: &mut Ui, commands_tx: mpsc::Sender<WSCommand>) {
    // Clone channels
    let wallet_balance_rx = CHANNEL.wallet_balance_rx.clone();
    let xrp_modal_rx = CHANNEL.xrp_modal_rx.clone();
    let xrp_modal_tx = CHANNEL.xrp_modal_tx.clone();
    let sign_transaction_rx = CHANNEL.sign_transaction_rx.clone();
    let sign_transaction_tx = CHANNEL.sign_transaction_tx.clone();
    let send_rlusd_rx = CHANNEL.send_rlusd_rx.clone();
    let send_rlusd_tx = CHANNEL.send_rlusd_tx.clone();
    let theme_user_rx = CHANNEL.theme_user_rx.clone();
    let send_euro_rx = CHANNEL.send_euro_rx.clone();
    let send_euro_tx = CHANNEL.send_euro_tx.clone();

    // Clone inner data
    let (_balance, wallet_address, _inactive, _privatekey) = wallet_balance_rx.borrow().clone();
    let xrp_modal_state = xrp_modal_rx.borrow().clone();
    let sign_transaction_state = sign_transaction_rx.borrow().clone();
    let send_rlusd_state = send_rlusd_rx.borrow().clone();
    let send_euro_state = send_euro_rx.borrow().clone();
    let (is_dark_mode, _, _hide_balance) = theme_user_rx.borrow().clone();

    // Define text color based on theme for non-button text
    let text_color = if is_dark_mode {
        egui::Color32::from_rgb(255, 254, 250) 
    } else {
        egui::Color32::from_rgb(34, 34, 34) 
    };

    ui.vertical(|ui| {
        // Handle import modal
        if let Some(mut import_state) = xrp_modal_state.import_wallet {
            let should_close = xrpimport::view(ui, &mut import_state, commands_tx.clone());
            let new_state = XRPModalState {
                import_wallet: if should_close || import_state.done { None } else { Some(import_state.clone()) },
                create_wallet: None,
                view_type: if should_close || import_state.done {
                    ActiveView::XRP
                } else {
                    xrp_modal_state.view_type
                },
            };
            let _ = xrp_modal_tx.send(new_state);
            ui.ctx().request_repaint();
            return;
        }

        // Handle create modal
        if let Some(mut create_state) = xrp_modal_state.create_wallet {
            let should_close = xrpcreate::view(ui, &mut create_state, commands_tx.clone());
            let new_state = XRPModalState {
                import_wallet: None,
                create_wallet: if should_close || create_state.done { None } else { Some(create_state.clone()) },
                view_type: if should_close || create_state.done {
                    ActiveView::XRP
                } else {
                    xrp_modal_state.view_type
                },
            };
            let _ = xrp_modal_tx.send(new_state);
            ui.ctx().request_repaint();
            return;
        }

        // Handle other views
        match xrp_modal_state.view_type {
            ActiveView::XRP | ActiveView::RLUSD | ActiveView::EURO => {
                if wallet_address.is_none() {
    let title_height = 30.0;
    let button_height = 36.0;
    let total_content_height = title_height + (20.0 * 2.0) + (button_height * 2.0);

    ui.add_space((ui.available_height() - total_content_height) / 2.0);

    ui.vertical_centered(|ui| {
        ui.label(RichText::new("Manage XRP Wallet").size(30.0).color(text_color));
        ui.add_space(20.0);

        ui.style_mut().spacing.button_padding = egui::vec2(20.0, 10.0);
        ui.style_mut().visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(12);
        ui.style_mut().visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(12);
        ui.style_mut().visuals.widgets.active.corner_radius = egui::CornerRadius::same(12);

        if ui.button(RichText::new("Create Wallet").size(16.0).extra_letter_spacing(1.2).color(text_color)).clicked() {
            let _ = xrp_modal_tx.send(XRPModalState {
                import_wallet: None,
                create_wallet: Some(XRPImport {
                    step: 1,
                    loading: false,
                    seed: None,
                    error: None,
                    done: false,
                    buffer_id: Some(Uuid::new_v4().to_string()),
                }),
                view_type: ActiveView::XRP,
            });
            ui.ctx().request_repaint();
        }

        ui.add_space(20.0);

        if ui.button(RichText::new("Import Wallet").size(16.0).extra_letter_spacing(1.2).color(text_color)).clicked() {
            let _ = xrp_modal_tx.send(XRPModalState {
                import_wallet: Some(XRPImport {
                    step: 1,
                    loading: false,
                    seed: None,
                    error: None,
                    done: false,
                    buffer_id: Some(Uuid::new_v4().to_string()),
                }),
                create_wallet: None,
                view_type: ActiveView::XRP,
            });
            ui.ctx().request_repaint();
        }
    });
                } else {
                    // Render balance view based on view_type
                    ui.allocate_ui(egui::vec2(ui.available_width(), ui.available_height() * 0.9), |ui| {
                        match xrp_modal_state.view_type {
                            ActiveView::XRP => xrpbalance::render_xrp_balance(ui, commands_tx.clone()),
                            ActiveView::RLUSD => rlusd::render_rlusd_balance(ui, commands_tx.clone()),
                            ActiveView::EURO => euro::render_euro_balance(ui, commands_tx.clone()),
                            _ => unreachable!(), // Other variants are handled in separate match arms
                        }
                    });
                    ui.add_space(10.0);
            ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0; // Tighter spacing
   

    let button_text = RichText::new("\u{21C6}")
        .size(10.0)
        .color(text_color) // Use theme-aware text_color
        .font(egui::FontId::new(14.0, egui::FontFamily::Name("DejaVuSansMono".into())));
    let button_width = 40.0; // Consistent button width
    let total_button_width = button_width * 2.0; // Both buttons
    let available_width = ui.available_width();

    // Place transactions button on the left
ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
    ui.add_sized(
        [button_width, 28.0],
        egui::Button::image(
            SvgCanvas::paint_svg("transfer.svg")
                .fit_to_exact_size(egui::vec2(16.0, 16.0))
                .tint(text_color), // Apply theme-aware tint
        ),
    ).clicked().then(|| {
        let _ = xrp_modal_tx.send(XRPModalState {
            import_wallet: None,
            create_wallet: None,
            view_type: ActiveView::Transactions,
        });
        ui.ctx().request_repaint();
    });
});

// Center selectable labels
ui.horizontal_centered(|ui| {
    ui.spacing_mut().item_spacing.x = 4.0; // Reduced spacing from 8.0
    let total_label_width = 212.0; // 3 buttons × 60px + 2 gaps × 4px
    ui.add_space((available_width - total_label_width - total_button_width) / 2.0 - 10.0); // Original centering logic unchanged

    // XRP Selectable Button
    let xrp_selected = matches!(xrp_modal_state.view_type, ActiveView::XRP);
    Frame::new() // egui 0.31.1, no ID argument
        .inner_margin(Margin::symmetric(4, 2)) // Tighter margins from 8,4
        .show(ui, |ui| {
            ui.add_sized(
                egui::Vec2::new(60.0, 28.0), // Reduced width from 80.0
                egui::Button::selectable(
                    xrp_selected,
                    RichText::new("XRP").color(text_color).size(12.0), // Smaller text from 14.0
                ),
            )
            .clicked()
            .then(|| {
                let _ = xrp_modal_tx.send(XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: ActiveView::XRP,
                });
                ui.ctx().request_repaint();
            });
        });

    // RLUSD Selectable Button
    let rlusd_selected = matches!(xrp_modal_state.view_type, ActiveView::RLUSD);
    Frame::new()
        .inner_margin(Margin::symmetric(4, 2))
        .show(ui, |ui| {
            ui.add_sized(
                egui::Vec2::new(60.0, 28.0),
                egui::Button::selectable(
                    rlusd_selected,
                    RichText::new("RLUSD").color(text_color).size(12.0),
                ),
            )
            .clicked()
            .then(|| {
                let _ = xrp_modal_tx.send(XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: ActiveView::RLUSD,
                });
                ui.ctx().request_repaint();
            });
        });

    // EURO Selectable Button
    let euro_selected = matches!(xrp_modal_state.view_type, ActiveView::EURO);
    Frame::new()
        .inner_margin(Margin::symmetric(4, 2))
        .show(ui, |ui| {
            ui.add_sized(
                egui::Vec2::new(60.0, 28.0),
                egui::Button::selectable(
                    euro_selected,
                    RichText::new("EURO").color(text_color).size(12.0),
                ),
            )
            .clicked()
            .then(|| {
                let _ = xrp_modal_tx.send(XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: ActiveView::EURO,
                });
                ui.ctx().request_repaint();
            });
        });
});
    // Place swap button on the right
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.add_sized(
            [button_width, 28.0],
            egui::Button::new(button_text),
        ).clicked().then(|| {
            let _ = xrp_modal_tx.send(XRPModalState {
                import_wallet: None,
                create_wallet: None,
                view_type: ActiveView::Trade,
            });
            ui.ctx().request_repaint();
        });
    });
});
                }
            }
            ActiveView::Enable => {
                let should_close = rlusd::enable::view(ui, commands_tx.clone());
                let new_state = XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: if should_close { ActiveView::RLUSD } else { ActiveView::Enable },
                };
                let _ = xrp_modal_tx.send(new_state);
                ui.ctx().request_repaint();
            }
            ActiveView::EnableEURO => {
                let should_close = euro::enable::view(ui, commands_tx.clone());
                let new_state = XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: if should_close { ActiveView::EURO } else { ActiveView::EnableEURO },
                };
                let _ = xrp_modal_tx.send(new_state);
                ui.ctx().request_repaint();
            }
            ActiveView::TrustLine => {
                let should_close = rlusd::modify::render(ui, commands_tx.clone());
                let new_state = XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: if should_close { ActiveView::RLUSD } else { ActiveView::TrustLine },
                };
                let _ = xrp_modal_tx.send(new_state);
                ui.ctx().request_repaint();
            }
              ActiveView::TrustLineEURO => {
                let should_close = euro::modify::render(ui, commands_tx.clone());
                let new_state = XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: if should_close { ActiveView::EURO } else { ActiveView::TrustLineEURO},
                };
                let _ = xrp_modal_tx.send(new_state);
                ui.ctx().request_repaint();
            }
            ActiveView::Receive => {
                let should_close = xrpbalance::receive::render(ui, &wallet_address);
                let new_state = XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: if should_close { ActiveView::XRP } else { ActiveView::Receive },
                };
                let _ = xrp_modal_tx.send(new_state);
                ui.ctx().request_repaint();
            }
            ActiveView::Trade => {
                let should_close = trade::view(ui, commands_tx.clone());
                let new_state = XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: if should_close { ActiveView::XRP } else { ActiveView::Trade },
                };
                let _ = xrp_modal_tx.send(new_state);
                ui.ctx().request_repaint();
            }
            ActiveView::ReceiveRLUSD => {
                let should_close = rlusd::receiverlusd::render(ui, &wallet_address);
                let new_state = XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: if should_close { ActiveView::RLUSD } else { ActiveView::ReceiveRLUSD },
                };
                let _ = xrp_modal_tx.send(new_state);
                ui.ctx().request_repaint();
            }
                ActiveView::ReceiveEURO => {
                let should_close = euro::receiveeuro::render(ui, &wallet_address);
                let new_state = XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: if should_close { ActiveView::EURO } else { ActiveView::ReceiveEURO },
                };
                let _ = xrp_modal_tx.send(new_state);
                ui.ctx().request_repaint();
            }
            ActiveView::Transactions => {
                let should_close = transactions::render(ui);
                let new_state = XRPModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: if should_close { ActiveView::XRP } else { ActiveView::Transactions },
                };
                let _ = xrp_modal_tx.send(new_state);
                ui.ctx().request_repaint();
            }
        }

        // Handle sign transaction
        if let Some(mut send_state) = sign_transaction_state.send_transaction {
            let should_close = xrpsend::view(ui, &mut send_state, commands_tx.clone());
            let new_state = if should_close || send_state.done {
                SignTransactionState { send_transaction: None }
            } else {
                SignTransactionState { send_transaction: Some(send_state) }
            };
            let _ = sign_transaction_tx.send(new_state);
            ui.ctx().request_repaint();
        }

        // Handle rlusd transaction
        if let Some(mut send_state) = send_rlusd_state.send_rlusd {
            let should_close = rlusd::send::view(ui, &mut send_state, commands_tx.clone());
            let new_state = if should_close || send_state.done {
                SendRLUSDTransactionState { send_rlusd: None }
            } else {
                SendRLUSDTransactionState { send_rlusd: Some(send_state) }
            };
            let _ = send_rlusd_tx.send(new_state);
            ui.ctx().request_repaint();
        }

             // Handle Euro transaction
        if let Some(mut send_state) = send_euro_state.send_euro {
            let should_close = euro::send::view(ui, &mut send_state, commands_tx.clone());
            let new_state = if should_close || send_state.done {
                SendEuroTransactionState { send_euro: None }
            } else {
                SendEuroTransactionState { send_euro: Some(send_state) }
            };
            let _ = send_euro_tx.send(new_state);
            ui.ctx().request_repaint();
        }
    });
}