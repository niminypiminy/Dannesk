use egui::{Ui, RichText, Color32, Frame, Margin};
use bip39::Mnemonic;
use bitcoin::bip32::{Xpriv, DerivationPath};
use bitcoin::{Network, CompressedPublicKey};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::address::Address;
use keyring::Entry;
use serde_json;
use std::str::FromStr;
use crate::channel::{BTCModalState, ProgressState, WSCommand, BTCImport, BTCActiveView};
use crate::encrypt::encrypt_data;
use super::{buffers, styles};
use zeroize::Zeroize;
use crate::utils::json_storage;
use tokio::sync::mpsc;

pub fn render(ui: &mut Ui, create_state: &mut BTCImport, buffer_id: &str, commands_tx: mpsc::Sender<WSCommand>) {
    let btc_modal_tx = crate::channel::CHANNEL.btc_modal_tx.clone();
    let is_dark_mode = crate::channel::CHANNEL.theme_user_rx.borrow().0;

    let mut passphrase_buffer = buffers::get_buffer(buffer_id);

    ui.label(RichText::new("Enter Passphrase").size(16.0).color(styles::text_color(is_dark_mode)));
    ui.add_space(5.0);
    ui.label("Enter a passphrase to encrypt your wallet.");
    ui.add_space(5.0);

    let pass_edit = super::styles::styled_text_edit(ui, &mut passphrase_buffer, is_dark_mode);
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
                    let trimmed_passphrase = passphrase_buffer.trim().to_string();
                    if trimmed_passphrase.len() < 6 {
                        create_state.error = Some("Passphrase must be at least 6 characters.".to_string());
                        let _ = btc_modal_tx.send(BTCModalState {
                            create_wallet: Some(create_state.clone()),
                            import_wallet: None,
                            view_type: BTCActiveView::BTC,
                        });
                        let _ = crate::channel::CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 0.0,
                            message: "Error: Passphrase too short".to_string(),
                        }));
                    } else if let Some(seed) = create_state.seed.clone() {
                        let _ = crate::channel::CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 0.0,
                            message: "Starting wallet creation".to_string(),
                        }));
                        ui.ctx().request_repaint();

                        let modal_tx = btc_modal_tx.clone();
                        let bitcoin_wallet_tx = crate::channel::CHANNEL.bitcoin_wallet_tx.clone();
                        let progress_tx = crate::channel::CHANNEL.progress_tx.clone();
                        let buffer_id_clone = buffer_id.to_string();
                        let create_state_clone = create_state.clone();
                        let seed_clone = seed.clone();
                        let commands_tx_clone = commands_tx.clone();

                        std::thread::spawn(move || {
                            let mut new_state = create_state_clone;

                            // Validate mnemonic and derive address
                            let mnemonic = match Mnemonic::from_str(&seed_clone) {
                                Ok(m) => m,
                                Err(e) => {
                                    let _ = progress_tx.send(Some(ProgressState {
                                        progress: 0.0,
                                        message: format!("Error: Invalid mnemonic: {}", e),
                                    }));
                                    new_state.error = Some(format!("Invalid mnemonic: {}", e));
                                    let _ = modal_tx.send(BTCModalState {
                                        create_wallet: Some(new_state),
                                        import_wallet: None,
                                        view_type: BTCActiveView::BTC,
                                    });
                                    return;
                                }
                            };

                            // Derive address
                            let seed = mnemonic.to_seed("");
                            let network = Network::Bitcoin;
                            let secp = Secp256k1::new();
                            let xpriv = Xpriv::new_master(network, &seed).expect("Failed to create master key");
                            let derivation_path = DerivationPath::from_str("m/84'/0'/0'/0/0").expect("Invalid derivation path");
                            let child_xpriv = xpriv.derive_priv(&secp, &derivation_path).expect("Failed to derive private key");
                            let public_key = child_xpriv.to_priv().public_key(&secp);
                            let compressed_pubkey = CompressedPublicKey(public_key.inner);
                            let address = Address::p2wpkh(&compressed_pubkey, network);

                            // Encrypt mnemonic
                            let (encrypted_mnemonic, salt, iv) = match encrypt_data(trimmed_passphrase, seed_clone.clone()) {
                                Ok(data) => data,
                                Err(e) => {
                                    let _ = progress_tx.send(Some(ProgressState {
                                        progress: 0.0,
                                        message: format!("Error: Encryption failed: {}", e),
                                    }));
                                    new_state.error = Some(format!("Encryption failed: {}", e));
                                    let _ = modal_tx.send(BTCModalState {
                                        create_wallet: Some(new_state),
                                        import_wallet: None,
                                        view_type: BTCActiveView::BTC,
                                    });
                                    return;
                                }
                            };

                            let _ = progress_tx.send(Some(ProgressState {
                                progress: 0.5,
                                message: "Encrypting wallet data".to_string(),
                            }));

                            // Store encrypted data in keyring
                            let sensitive_data = serde_json::to_string(&(encrypted_mnemonic, salt, iv))
                                .expect("Failed to serialize sensitive data");
                            let entry = Entry::new("bitcoin_wallet", &address.to_string())
                                .expect("Failed to access keyring");
                            entry.set_password(&sensitive_data)
                                .expect("Failed to store in keyring");

                            // Save wallet data to file
                           let wallet_data = serde_json::json!({
    "address": address.to_string(),
    "private_key_deleted": false
});
json_storage::write_json("btc.json", &wallet_data)
    .expect("Failed to write btc.json");

                            // Send address to bitcoin_wallet_tx
                            let _ = bitcoin_wallet_tx.send((0.0, Some(address.to_string()), false));

                            // Send WSCommand
                            let command = WSCommand {
                                command: "import_bitcoin_wallet".to_string(),
                                wallet: Some(address.to_string()),
                                recipient: None,
                                amount: None,
                                passphrase: None,
                                trustline_limit: None,
                                tx_type: None,
                                taker_pays: None,
                                taker_gets: None,
                                seed: None,
                                flags: None,
                                wallet_type: Some("bitcoin_testnet".to_string()),
                            };
                            if let Err(e) = commands_tx_clone.try_send(command) {
                                let _ = progress_tx.send(Some(ProgressState {
                                    progress: 1.0,
                                    message: format!("Error: Failed to send command: {}", e),
                                }));
                                new_state.error = Some(format!("Failed to send command: {}", e));
                                let _ = modal_tx.send(BTCModalState {
                                    create_wallet: Some(new_state),
                                    import_wallet: None,
                                    view_type: BTCActiveView::BTC,
                                });
                                return;
                            }

                            new_state.done = true;
                            if let Some(mut seed) = new_state.seed.take() {
                                seed.zeroize();
                            }
                            let _ = modal_tx.send(BTCModalState {
                                create_wallet: Some(new_state),
                                import_wallet: None,
                                view_type: BTCActiveView::BTC,
                            });
                            buffers::clear_buffer(&buffer_id_clone);
                        });
                    } else {
                        create_state.error = Some("No seed available to encrypt.".to_string());
                        let _ = btc_modal_tx.send(BTCModalState {
                            create_wallet: Some(create_state.clone()),
                            import_wallet: None,
                            view_type: BTCActiveView::BTC,
                        });
                        let _ = crate::channel::CHANNEL.progress_tx.send(Some(ProgressState {
                            progress: 0.0,
                            message: "Error: No seed available to encrypt".to_string(),
                        }));
                    }
                    ui.ctx().request_repaint();
                }
            });
        ui.visuals_mut().widgets = original_visuals.widgets;
    });
}