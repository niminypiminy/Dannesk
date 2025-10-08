use egui::{Ui, RichText, CursorIcon, Color32, Stroke, Frame, Margin};
use bip39::{Mnemonic, Language};
use bitcoin::bip32::{Xpriv, DerivationPath};
use bitcoin::{Network, CompressedPublicKey};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::address::Address;
use rand::thread_rng;
use rand::RngCore;
use std::str::FromStr;
use super::{styles};
use crate::channel::{CHANNEL, BTCModalState, BTCImport, BTCActiveView};

pub fn render(ui: &mut Ui, create_state: &mut BTCImport, _buffer_id: &str) {
    let btc_modal_tx = CHANNEL.btc_modal_tx.clone();
    let is_dark_mode = CHANNEL.theme_user_rx.borrow().0;

    if create_state.seed.is_none() {
        ui.label(RichText::new("Create a new Bitcoin Wallet").size(16.0).color(styles::text_color(is_dark_mode)));
        ui.add_space(10.0);

        // Modernized Generate Button
        ui.vertical_centered(|ui| {
            let original_visuals = ui.visuals().clone();
            let text_color = ui.style().visuals.text_color();
            if !is_dark_mode {
                ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
                ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
            }
            Frame::new()
                .inner_margin(Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    let generate_button = ui.add(
                        egui::Button::new(RichText::new("Continue").size(14.0).color(text_color))
                            .min_size(egui::Vec2::new(100.0, 28.0)),
                    );
                    if generate_button.clicked() {
                        create_state.loading = true;
                        create_state.error = None;
                        let modal_tx = btc_modal_tx.clone();
                        let progress_tx = CHANNEL.progress_tx.clone();
                        let create_state_clone = create_state.clone();

                        std::thread::spawn(move || {
                            let mut new_state = create_state_clone;
                            let mut entropy = [0u8; 32];
                            thread_rng().fill_bytes(&mut entropy);
                            let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)
                                .expect("Failed to generate mnemonic");
                            let mnemonic_phrase = mnemonic.to_string();
                            new_state.seed = Some(mnemonic_phrase.clone());

                            let seed = mnemonic.to_seed("");
                            let network = Network::Bitcoin;
                            let secp = Secp256k1::new();
                            let xpriv = Xpriv::new_master(network, &seed).expect("Failed to create master key");
                            let derivation_path = DerivationPath::from_str("m/84'/0'/0'/0/0").expect("Invalid derivation path");
                            let child_xpriv = xpriv.derive_priv(&secp, &derivation_path).expect("Failed to derive private key");
                            let public_key = child_xpriv.to_priv().public_key(&secp);
                            let compressed_pubkey = CompressedPublicKey(public_key.inner);
                            let _address = Address::p2wpkh(&compressed_pubkey, network);

                            let _ = modal_tx.send(BTCModalState {
                                create_wallet: Some(new_state),
                                import_wallet: None,
                                view_type: BTCActiveView::BTC,
                            });
                            let _ = progress_tx.send(None);
                        });
                        let _ = btc_modal_tx.send(BTCModalState {
                            create_wallet: Some(create_state.clone()),
                            import_wallet: None,
                            view_type: BTCActiveView::BTC,
                        });
                    }
                });
            ui.visuals_mut().widgets = original_visuals.widgets;
        });
    } else if let Some(seed) = &create_state.seed {
        ui.add_space(5.0);
        ui.add_space(10.0);

        // Display mnemonic in a 4x6 grid without border
        Frame::new()
            .fill(if is_dark_mode { Color32::from_rgb(50, 50, 50) } else { Color32::from_rgb(200, 200, 200) })
            .stroke(Stroke::NONE)
            .inner_margin(Margin::same(8))
            .show(ui, |ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(8.0, 8.0);
                egui::Grid::new("mnemonic_grid")
                    .num_columns(6)
                    .spacing([10.0, 10.0])
                    .show(ui, |ui| {
                        let words: Vec<&str> = seed.split_whitespace().collect();
                        for (i, word) in words.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format!("{:2}. {}", i + 1, word))
                                        .monospace()
                                        .size(14.0)
                                        .color(styles::text_color(is_dark_mode)),
                                );
                            });
                            if (i + 1) % 6 == 0 {
                                ui.end_row();
                            }
                        }
                    });
            });

        ui.add_space(10.0);
        // Modernized Copy and Continue Buttons
        let seed_clone = seed.clone(); // Clone seed outside the closure
        ui.vertical_centered(|ui| {
            let original_visuals = ui.visuals().clone();
            let text_color = styles::text_color(is_dark_mode);
            if !is_dark_mode {
                ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
                ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
            }
            Frame::new()
                .inner_margin(Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Copy Button
                        let copy_button = ui.add(
                            egui::Button::new(RichText::new("Copy").size(14.0).color(text_color))
                                .min_size(egui::Vec2::new(100.0, 28.0)),
                        );
                        if copy_button.clicked() {
                            ui.ctx().copy_text(seed_clone.clone());
                        }
                        if copy_button.hovered() {
                            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                        }

                        // Continue Button
                        let continue_button = ui.add(
                            egui::Button::new(RichText::new("Continue").size(14.0).color(text_color))
                                .min_size(egui::Vec2::new(100.0, 28.0)),
                        );
                        if continue_button.clicked() {
                            create_state.step = 2;
                            let _ = btc_modal_tx.send(BTCModalState {
                                create_wallet: Some(create_state.clone()),
                                import_wallet: None,
                                view_type: BTCActiveView::BTC,
                            });
                            ui.ctx().request_repaint();
                        }
                    });
                });
            ui.visuals_mut().widgets = original_visuals.widgets;
        });
    }
}