// btcimport.rs (updated to defer JSON creation and channel update to response)

use egui::{Ui, RichText, Color32, Frame, Margin};
use bitcoin::bip32::{Xpriv, DerivationPath};
use bitcoin::{Network, CompressedPublicKey};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::address::Address;
use bip39::Mnemonic;
use keyring::Entry;
use serde_json;
use std::str::FromStr;
use crate::channel::{BTCModalState, ProgressState, WSCommand, BTCImport, BTCActiveView};
use crate::encrypt::encrypt_data;
use super::buffers;
use zeroize::Zeroize;
use crate::utils::json_storage;
use tokio::sync::mpsc;

pub fn render(ui: &mut Ui, import_state: &mut BTCImport, buffer_id: &str, commands_tx: mpsc::Sender<WSCommand>) {
    let btc_modal_tx = crate::channel::CHANNEL.btc_modal_tx.clone();
    let is_dark_mode = crate::channel::CHANNEL.theme_user_rx.borrow().0;
    let (seed_words, mut passphrase_buffer) = buffers::get_buffer(buffer_id);

    ui.label(RichText::new("Enter your passphrase to encrypt the mnemonic.").size(16.0));
    ui.add_space(5.0);
    ui.label("You'll need your passphrase to decrypt later");
    ui.add_space(5.0);

    let pass_edit = super::styles::styled_text_edit(ui, &mut passphrase_buffer, is_dark_mode, true);
    if pass_edit.changed() {
        buffers::update_buffer(buffer_id, seed_words.clone(), passphrase_buffer.clone());
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
                    let mut mnemonic_phrase = seed_words.iter().filter(|w| !w.is_empty()).map(|s| s.as_str()).collect::<Vec<_>>().join(" ").trim().to_string();
                    let mut passphrase = passphrase_buffer.trim().to_string();
                    if mnemonic_phrase.is_empty() {
                        import_state.error = Some("Mnemonic phrase cannot be empty.".to_string());
                        let _ = btc_modal_tx.send(BTCModalState {
                            import_wallet: Some(import_state.clone()),
                            create_wallet: None,
                            view_type: BTCActiveView::BTC,
                        });
                        // Zeroize and clear buffers on error
                        mnemonic_phrase.zeroize();
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

                    let modal_tx = btc_modal_tx.clone();
                    // REMOVED: bitcoin_wallet_tx clone (defer to response)
                    let progress_tx = crate::channel::CHANNEL.progress_tx.clone();
                    let buffer_id_clone = buffer_id.to_string();
                    let import_state_clone = import_state.clone();
                    let commands_tx_clone = commands_tx.clone();

                    std::thread::spawn(move || {
                        let mut new_state = import_state_clone;
                        match Mnemonic::from_str(&mnemonic_phrase) {
                            Ok(mnemonic) => {
                                match encrypt_data(passphrase.clone(), mnemonic_phrase.clone()) {
                                    Ok((encrypted_mnemonic, salt, iv)) => {
                                        let _ = progress_tx.send(Some(ProgressState {
                                            progress: 0.5,
                                            message: "Encrypting wallet data".to_string(),
                                        }));

                                        // Derive Bitcoin private key and address for testnet
                                        let seed = mnemonic.to_seed(""); // Changed to empty passphrase
                                        let network = Network::Bitcoin;
                                        let secp = Secp256k1::new();
                                        let xpriv = Xpriv::new_master(network, &seed).expect("Failed to create master key");
                                        let derivation_path = DerivationPath::from_str("m/84'/0'/0'/0/0").expect("Invalid derivation path");
                                        let child_xpriv = xpriv.derive_priv(&secp, &derivation_path)
                                            .expect("Failed to derive private key");
                                        let public_key = child_xpriv.to_priv().public_key(&secp);
                                        let compressed_pubkey = CompressedPublicKey(public_key.inner);
                                        let address = Address::p2wpkh(&compressed_pubkey, network);

                                        // Store encrypted mnemonic in keyring
                                        let sensitive_data = serde_json::to_string(&(encrypted_mnemonic, salt, iv))
                                            .expect("Failed to serialize sensitive data");
                                        let entry = Entry::new("bitcoin_wallet", &address.to_string())
                                            .expect("Failed to access keyring");
                                        entry.set_password(&sensitive_data)
                                            .expect("Failed to store in keyring");

                                        // REMOVED: json_storage::write_json (defer to response)

                                        // REMOVED: bitcoin_wallet_tx.send (defer to response)

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
                                                import_wallet: Some(new_state),
                                                create_wallet: None,
                                                view_type: BTCActiveView::BTC,
                                            });
                                            return;
                                        }

                                        println!("Sent import_bitcoin_wallet command for address: {}", address);
                                        // Modal stays open; UI reacts to progress 1.0 success for close
                                        new_state.done = true;
                                        let _ = modal_tx.send(BTCModalState {
                                            import_wallet: Some(new_state),
                                            create_wallet: None,
                                            view_type: BTCActiveView::BTC,
                                        });

                                        // REMOVED: progress_tx.send progress 1.0 (defer to response)
                                    }
                                    Err(e) => {
                                        new_state.error = Some(format!("Encryption failed: {}", e));
                                        let _ = modal_tx.send(BTCModalState {
                                            import_wallet: Some(new_state),
                                            create_wallet: None,
                                            view_type: BTCActiveView::BTC,
                                        });
                                        let _ = progress_tx.send(Some(ProgressState {
                                            progress: 1.0,
                                            message: format!("Error: Encryption failed: {}", e),
                                        }));
                                    }
                                }
                            }
                            Err(e) => {
                                new_state.error = Some(format!("Invalid mnemonic: {}", e));
                                let _ = modal_tx.send(BTCModalState {
                                    import_wallet: Some(new_state),
                                    create_wallet: None,
                                    view_type: BTCActiveView::BTC,
                                });
                                let _ = progress_tx.send(Some(ProgressState {
                                    progress: 1.0,
                                    message: format!("Error: Invalid mnemonic: {}", e),
                                }));
                            }
                        }
                        // Zeroize sensitive data
                        mnemonic_phrase.zeroize();
                        passphrase.zeroize();
                        buffers::clear_buffer(&buffer_id_clone);
                    });
                }
            });
        ui.visuals_mut().widgets = original_visuals.widgets;
    });
}