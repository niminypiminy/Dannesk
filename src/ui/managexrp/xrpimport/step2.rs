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

pub fn render(ui: &mut Ui, import_state: &mut XRPImport, buffer_id: &str, commands_tx: mpsc::Sender<WSCommand>) {
    let xrp_modal_tx = crate::channel::CHANNEL.xrp_modal_tx.clone();
    let is_dark_mode = crate::channel::CHANNEL.theme_user_rx.borrow().0;
    let (seed_buffer, mut passphrase_buffer) = buffers::get_buffer(buffer_id);

    ui.label(RichText::new("Enter your passphrase to encrypt the wallet.").size(16.0).color(styles::text_color(is_dark_mode)));
    ui.add_space(5.0);

    let pass_edit = super::styles::styled_text_edit(ui, &mut passphrase_buffer, is_dark_mode, true);
    if pass_edit.changed() {
        buffers::update_buffer(buffer_id, seed_buffer.clone(), passphrase_buffer.clone());
        import_state.error = None;
    }
    ui.add_space(5.0);

    if let Some(error) = &import_state.error {
        ui.colored_label(Color32::RED, error);
        ui.add_space(5.0);
    }

    ui.add_space(5.0);
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
                let continue_button = ui.add(
                    egui::Button::new(RichText::new("Continue").size(14.0).color(text_color))
                        .min_size(egui::Vec2::new(100.0, 28.0)),
                );
                if continue_button.clicked() {
                    let mut seed = seed_buffer.trim().to_string();
                    let mut passphrase = passphrase_buffer.trim().to_string();
                    if seed.is_empty() {
                        import_state.error = Some("Seed cannot be empty.".to_string());
                        let _ = xrp_modal_tx.send(XRPModalState {
                            import_wallet: Some(import_state.clone()),
                            create_wallet: None,
                            view_type: ActiveView::XRP,
                        });
                        // Zeroize and clear buffers on error
                        seed.zeroize();
                        passphrase.zeroize();
                        buffers::clear_buffer(buffer_id);
                        ui.ctx().request_repaint();
                        return;
                    }

                    let _ = crate::channel::CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 0.0,
                        message: "Starting wallet import".to_string(),
                    }));
                    ui.ctx().request_repaint();

                    let modal_tx = xrp_modal_tx.clone();
                    // REMOVED: wallet_balance_tx clone (no longer used here)
                    let progress_tx = crate::channel::CHANNEL.progress_tx.clone();
                    let buffer_id_clone = buffer_id.to_string();
                    let import_state_clone = import_state.clone();
                    let commands_tx_clone = commands_tx.clone();

                    std::thread::spawn(move || {
                        let mut new_state = import_state_clone;
                        match Wallet::new(&seed, 0) {
                            Ok(wallet) => {
                                match encrypt_data(passphrase.clone(), seed.clone()) {
                                    Ok((encrypted_seed, salt, iv)) => {
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

                                        // REMOVED: json_storage::write_json (defer to response)

                                        // REMOVED: wallet_balance_tx.send (defer to response)

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
                                            new_state.error = Some(format!("Failed to send command: {}", e)); // Set error for modal
                                            let _ = modal_tx.send(XRPModalState {  // Send error modal
                                                import_wallet: Some(new_state),
                                                create_wallet: None,
                                                view_type: ActiveView::XRP,
                                            });
                                            let _ = progress_tx.send(Some(ProgressState {
                                                progress: 1.0,
                                                message: format!("Error: Failed to send command: {}", e),
                                            }));
                                        } else {
                                            println!("Sent import_wallet command for address: {}", wallet.classic_address);
                                            // Modal stays open; UI reacts to progress 1.0 success for close
                                            new_state.done = true;
                                            let _ = modal_tx.send(XRPModalState {
                                                import_wallet: Some(new_state),
                                                create_wallet: None,
                                                view_type: ActiveView::XRP,
                                            });
                                        }

                                        // REMOVED: new_state.done = true; and modal_tx.send (defer to response or UI)
                                    }
                                    Err(e) => {
                                        new_state.error = Some(format!("Encryption failed: {}", e));
                                        let _ = modal_tx.send(XRPModalState {
                                            import_wallet: Some(new_state),
                                            create_wallet: None,
                                            view_type: ActiveView::XRP,
                                        });
                                        let _ = progress_tx.send(Some(ProgressState {
                                            progress: 1.0,
                                            message: format!("Error: Encryption failed: {}", e),
                                        }));
                                    }
                                }
                            }
                            Err(e) => {
                                new_state.error = Some(format!("Wallet import failed: {}", e));
                                let _ = modal_tx.send(XRPModalState {
                                    import_wallet: Some(new_state),
                                    create_wallet: None,
                                    view_type: ActiveView::XRP,
                                });
                                let _ = progress_tx.send(Some(ProgressState {
                                    progress: 1.0,
                                    message: format!("Error: Wallet import failed: {}", e),
                                }));
                            }
                        }
                        // Zeroize sensitive data in the thread
                        seed.zeroize();
                        passphrase.zeroize();
                        buffers::clear_buffer(&buffer_id_clone);
                    });
                }
            });
        ui.visuals_mut().widgets = original_visuals.widgets;
    });
}