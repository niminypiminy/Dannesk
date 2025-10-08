use egui::{Ui, RichText, Color32, Frame, Margin};
use xrpl::wallet::Wallet;
use keyring::Entry;
use serde_json;
use crate::channel::{XRPModalState, ProgressState, WSCommand, XRPImport, ActiveView};
use crate::encrypt::encrypt_data;
use super::{buffers, styles};
use zeroize::Zeroize;
use crate::utils::json_storage;
use tokio::sync::mpsc;

pub fn render(ui: &mut Ui, create_state: &mut XRPImport, buffer_id: &str, commands_tx: mpsc::Sender<WSCommand>) {
    let xrp_modal_tx = crate::channel::CHANNEL.xrp_modal_tx.clone();
    let is_dark_mode = crate::channel::CHANNEL.theme_user_rx.borrow().0;

    let mut passphrase_buffer = buffers::get_buffer(buffer_id);

    ui.label(RichText::new("Enter Passphrase").size(16.0).color(styles::text_color(is_dark_mode)));
    ui.add_space(5.0);
    ui.label(RichText::new("Enter a passphrase to encrypt your wallet.").color(styles::text_color(is_dark_mode)));
    ui.add_space(5.0);

    let pass_edit = styles::styled_text_edit(ui, &mut passphrase_buffer, is_dark_mode);
    if pass_edit.changed() {
        buffers::update_buffer(buffer_id, passphrase_buffer.clone());
        create_state.error = None;
    }

    if let Some(error) = &create_state.error {
        ui.colored_label(Color32::RED, error);
        ui.add_space(5.0);
    }

    ui.add_space(10.0);
    // Modernized Continue Button
    ui.vertical_centered(|ui| {
        let original_visuals = ui.visuals().clone();
        let text_color = ui.style().visuals.text_color();
        if !is_dark_mode {
            ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
            ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
        }
        Frame::new() // egui 0.31.1, no ID argument
            .inner_margin(Margin::symmetric(8, 4))
            .show(ui, |ui| {
                let create_button = ui.add(
                    egui::Button::new(RichText::new("Continue").size(14.0).color(text_color))
                        .min_size(egui::Vec2::new(100.0, 28.0)),
                );
                if create_button.clicked() {
                    let mut trimmed_passphrase = passphrase_buffer.trim().to_string(); // Make mutable
                    if trimmed_passphrase.len() < 6 {
                        create_state.error = Some("Passphrase must be at least 6 characters.".to_string());
                        let _ = xrp_modal_tx.send(XRPModalState {
                            create_wallet: Some(create_state.clone()),
                            import_wallet: None,
                            view_type: ActiveView::XRP,
                        });
                        let _ = crate::channel::CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 0.0,
                            message: "Error: Passphrase too short".to_string(),
                        }));
                        // Zeroize and clear buffer on error
                        trimmed_passphrase.zeroize();
                        buffers::clear_buffer(buffer_id);
                        ui.ctx().request_repaint();
                    } else if let Some(seed) = create_state.seed.clone() {
                        let _ = crate::channel::CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 0.0,
                            message: "Starting wallet creation".to_string(),
                        }));
                        ui.ctx().request_repaint();

                        let modal_tx = xrp_modal_tx.clone();
                        let wallet_balance_tx = crate::channel::CHANNEL.wallet_balance_tx.clone();
                        let progress_tx = crate::channel::CHANNEL.progress_tx.clone();
                        let buffer_id_clone = buffer_id.to_string();
                        let create_state_clone = create_state.clone();
                        let mut seed_clone = seed.clone(); // Make mutable
                        let commands_tx_clone = commands_tx.clone();

                        std::thread::spawn(move || {
                            let mut new_state = create_state_clone;
                            let wallet = match Wallet::new(&seed_clone, 0) {
                                Ok(wallet) => wallet,
                                Err(e) => {
                                    new_state.error = Some(format!("Wallet creation failed: {}", e));
                                    let _ = modal_tx.send(XRPModalState {
                                        create_wallet: Some(new_state),
                                        import_wallet: None,
                                        view_type: ActiveView::XRP,
                                    });
                                    let _ = progress_tx.send(Some(ProgressState {
                                        progress: 1.0,
                                        message: format!("Error: Wallet creation failed: {}", e),
                                    }));
                                    // Zeroize sensitive data
                                    seed_clone.zeroize();
                                    trimmed_passphrase.zeroize();
                                    buffers::clear_buffer(&buffer_id_clone);
                                    return;
                                }
                            };

                            let (encrypted_seed, salt, iv) = match encrypt_data(trimmed_passphrase.clone(), seed_clone.clone()) {
                                Ok(data) => data,
                                Err(e) => {
                                    new_state.error = Some(format!("Encryption failed: {}", e));
                                    let _ = modal_tx.send(XRPModalState {
                                        create_wallet: Some(new_state),
                                        import_wallet: None,
                                        view_type: ActiveView::XRP,
                                    });
                                    let _ = progress_tx.send(Some(ProgressState {
                                        progress: 1.0,
                                        message: format!("Error: Encryption failed: {}", e),
                                    }));
                                    // Zeroize sensitive data
                                    seed_clone.zeroize();
                                    trimmed_passphrase.zeroize();
                                    buffers::clear_buffer(&buffer_id_clone);
                                    return;
                                }
                            };

                            let _ = progress_tx.send(Some(ProgressState {
                                progress: 0.5,
                                message: "Encrypting wallet data".to_string(),
                            }));
                            let sensitive_data = serde_json::to_string(&(encrypted_seed, salt, iv))
                                .expect("Failed to serialize sensitive data");
                            let entry = Entry::new("rust_wallet", &wallet.classic_address)
                                .expect("Failed to access keyring");
                            entry.set_password(&sensitive_data)
                                .expect("Failed to store in keyring");

                            let wallet_data = serde_json::json!({
    "address": wallet.classic_address,
    "private_key_deleted": false
});
json_storage::write_json("xrp.json", &wallet_data)
    .expect("Failed to write xrp.json");

                            let _ = wallet_balance_tx.send((
                                0.0,
                                Some(wallet.classic_address.clone()),
                                false,
                                false,
                            ));

                            let command = WSCommand {
                                command: "import_wallet".to_string(),
                                wallet: Some(wallet.classic_address.clone()),
                                recipient: None,
                                amount: None,
                                passphrase: None,
                                trustline_limit: None,
                                tx_type: None,
                                taker_pays: None,
                                taker_gets: None,
                                seed: None,
                                flags: None,
                                wallet_type: None,
                            };

                            if let Err(e) = commands_tx_clone.try_send(command) {
                                println!("Failed to send import_wallet command: {}", e);
                                new_state.error = Some(format!("Failed to send command: {}", e));
                                let _ = modal_tx.send(XRPModalState {
                                    create_wallet: Some(new_state),
                                    import_wallet: None,
                                    view_type: ActiveView::XRP,
                                });
                                let _ = progress_tx.send(Some(ProgressState {
                                    progress: 1.0,
                                    message: format!("Error: Failed to send command: {}", e),
                                }));
                            } else {
                                println!("Sent import_wallet command for address: {}", wallet.classic_address);
                                new_state.done = true;
                                let _ = modal_tx.send(XRPModalState {
                                    create_wallet: Some(new_state),
                                    import_wallet: None,
                                    view_type: ActiveView::XRP,
                                });
                                let _ = progress_tx.send(Some(ProgressState {
                                    progress: 1.0,
                                    message: "Wallet creation completed".to_string(),
                                }));
                            }

                            // Zeroize sensitive data
                            seed_clone.zeroize();
                            trimmed_passphrase.zeroize();
                            buffers::clear_buffer(&buffer_id_clone);
                        });
                    } else {
                        create_state.error = Some("No seed available to encrypt.".to_string());
                        let _ = xrp_modal_tx.send(XRPModalState {
                            create_wallet: Some(create_state.clone()),
                            import_wallet: None,
                            view_type: ActiveView::XRP,
                        });
                        let _ = crate::channel::CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 0.0,
                            message: "Error: No seed available to encrypt".to_string(),
                        }));
                        // Zeroize and clear buffer
                        trimmed_passphrase.zeroize();
                        buffers::clear_buffer(buffer_id);
                        ui.ctx().request_repaint();
                    }
                }
            });
        ui.visuals_mut().widgets = original_visuals.widgets;
    });
}