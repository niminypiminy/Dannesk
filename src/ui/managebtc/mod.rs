use egui::{Ui, RichText};
use crate::channel::{CHANNEL, BTCModalState, BTCImport, BTCActiveView, WSCommand, SignTransactionState};
use tokio::sync::mpsc;
use crate::utils::svg_render::SvgCanvas; // Import SvgCanvas
use uuid::Uuid;

pub mod btcimport;
pub mod btccreate;
pub mod btcsend;
pub mod btcbalance;
pub mod btctransactions;

pub fn render_manage_btc(ui: &mut Ui, commands_tx: mpsc::Sender<WSCommand>) {
    let bitcoin_wallet_rx = crate::channel::CHANNEL.bitcoin_wallet_rx.clone();
    let btc_modal_rx = crate::channel::CHANNEL.btc_modal_rx.clone();
    let btc_modal_tx = crate::channel::CHANNEL.btc_modal_tx.clone();
    let sign_transaction_rx = crate::channel::CHANNEL.sign_transaction_rx.clone();
    let sign_transaction_tx = crate::channel::CHANNEL.sign_transaction_tx.clone();
    let theme_user_rx = CHANNEL.theme_user_rx.clone();

    let (_btc_balance, btc_wallet_address, _private_key_deleted) = bitcoin_wallet_rx.borrow().clone();
    let btc_modal_state = btc_modal_rx.borrow().clone();
    let sign_transaction_state = sign_transaction_rx.borrow().clone();
    let (is_dark_mode, _, _hide_balance) = theme_user_rx.borrow().clone();

    // Define text color based on theme for non-button text
    let text_color = if is_dark_mode {
        egui::Color32::from_rgb(255, 254, 250) // #fffefa for dark theme
    } else {
        egui::Color32::from_rgb(34, 34, 34) // #2d3a4b for light theme
    };

    ui.vertical(|ui| {
        // Handle import modal
        if let Some(mut import_state) = btc_modal_state.import_wallet {
            let should_close = btcimport::view(ui, &mut import_state, commands_tx.clone());
            let new_state = BTCModalState {
                import_wallet: if should_close || import_state.done { None } else { Some(import_state.clone()) },
                create_wallet: None,
                view_type: if should_close || import_state.done {
                    BTCActiveView::BTC
                } else {
                    btc_modal_state.view_type
                },
            };
            let _ = btc_modal_tx.send(new_state);
            ui.ctx().request_repaint();
            return;
        }

        // Handle create modal
        if let Some(mut create_state) = btc_modal_state.create_wallet {
            let should_close = btccreate::view(ui, &mut create_state, commands_tx.clone());
            let new_state = BTCModalState {
                import_wallet: None,
                create_wallet: if should_close || create_state.done { None } else { Some(create_state.clone()) },
                view_type: if should_close || create_state.done {
                    BTCActiveView::BTC
                } else {
                    btc_modal_state.view_type
                },
            };
            let _ = btc_modal_tx.send(new_state);
            ui.ctx().request_repaint();
            return;
        }

        // Handle other views
        match btc_modal_state.view_type {
            BTCActiveView::BTC => {
                if btc_wallet_address.is_none() {
                    let title_height = 30.0;
                    let button_height = 36.0;
                    let total_content_height = title_height + (20.0 * 2.0) + (button_height * 2.0);

                    ui.add_space((ui.available_height() - total_content_height) / 2.0);

                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("Manage Bitcoin Wallet").size(30.0).color(text_color));
                        ui.add_space(20.0);

                        ui.style_mut().spacing.button_padding = egui::vec2(20.0, 10.0);
                        ui.style_mut().visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(12);
                        ui.style_mut().visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(12);
                        ui.style_mut().visuals.widgets.active.corner_radius = egui::CornerRadius::same(12);

                        if ui.button(RichText::new("Create Wallet").size(16.0).extra_letter_spacing(1.2).color(text_color)).clicked() {
                            let _ = btc_modal_tx.send(BTCModalState {
                                import_wallet: None,
                                create_wallet: Some(BTCImport {
                                    step: 1,
                                    loading: false,
                                    seed: None,
                                    error: None,
                                    done: false,
                                    buffer_id: Some(Uuid::new_v4().to_string()),
                                }),
                                view_type: BTCActiveView::BTC,
                            });
                            ui.ctx().request_repaint();
                        }

                        ui.add_space(20.0);

                        if ui.button(RichText::new("Import Wallet").size(16.0).extra_letter_spacing(1.2).color(text_color)).clicked() {
                            let _ = btc_modal_tx.send(BTCModalState {
                                import_wallet: Some(BTCImport {
                                    step: 1,
                                    loading: false,
                                    seed: None,
                                    error: None,
                                    done: false,
                                    buffer_id: Some(Uuid::new_v4().to_string()),
                                }),
                                create_wallet: None,
                                view_type: BTCActiveView::BTC,
                            });
                            ui.ctx().request_repaint();
                        }
                    });
                } else {
                    // Render balance view
                    ui.allocate_ui(egui::vec2(ui.available_width(), ui.available_height() * 0.9), |ui| {
                        btcbalance::render_btc_balance(ui, commands_tx.clone());
                    });
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0; // Tighter spacing
    let button_width = 40.0; // Consistent button width

    // Place transactions button on the left
  ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
    if ui.add_sized(
        [button_width, 28.0],
        egui::Button::image(
            SvgCanvas::paint_svg("transfer.svg")
                .fit_to_exact_size(egui::vec2(16.0, 16.0))
                .tint(text_color), // Apply theme-aware tint
        ),
    ).clicked() {
        let _ = btc_modal_tx.send(BTCModalState {
            import_wallet: None,
            create_wallet: None,
            view_type: BTCActiveView::Transactions,
        });
        ui.ctx().request_repaint();
    }
});
                    });
                }
            }
            BTCActiveView::Receive => {
                let should_close = btcbalance::receivebtc::render(ui, &btc_wallet_address);
                let new_state = BTCModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: if should_close { BTCActiveView::BTC } else { BTCActiveView::Receive },
                };
                let _ = btc_modal_tx.send(new_state);
                ui.ctx().request_repaint();
            }
            BTCActiveView::Transactions => {
                let should_close = btctransactions::render(ui);
                let new_state = BTCModalState {
                    import_wallet: None,
                    create_wallet: None,
                    view_type: if should_close { BTCActiveView::BTC } else { BTCActiveView::Transactions },
                };
                let _ = btc_modal_tx.send(new_state);
                ui.ctx().request_repaint();
            }
        }

        // Handle sign transaction
        if let Some(mut send_state) = sign_transaction_state.send_transaction {
            let should_close = btcsend::view(ui, &mut send_state, commands_tx.clone());
            let new_state = if should_close || send_state.done {
                SignTransactionState { send_transaction: None }
            } else {
                SignTransactionState { send_transaction: Some(send_state) }
            };
            let _ = sign_transaction_tx.send(new_state);
            ui.ctx().request_repaint();
        }
    });
}