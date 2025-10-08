use egui::{Ui, RichText, CursorIcon, Sense, Frame, Margin};
use xrpl::wallet::Wallet;
use xrpl::constants::CryptoAlgorithm;
use crate::channel::{CHANNEL, XRPModalState, XRPImport, ActiveView};
use super::styles; // For text_color, button_text_color, seed_frame

pub fn render(ui: &mut Ui, create_state: &mut XRPImport, _buffer_id: &str) {
    let xrp_modal_tx = CHANNEL.xrp_modal_tx.clone();
    let is_dark_mode = CHANNEL.theme_user_rx.borrow().0;

    if create_state.seed.is_none() {
        ui.label(RichText::new("Generate New XRP Wallet").size(16.0).color(styles::text_color(is_dark_mode)));
        ui.add_space(10.0);
        
        // Modernized Generate Button
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
                    let generate_button = ui.add(
                        egui::Button::new(RichText::new("Continue").size(14.0).color(text_color))
                            .min_size(egui::Vec2::new(100.0, 28.0)),
                    );
                    if generate_button.clicked() {
                        create_state.loading = true;
                        create_state.error = None;
                        let modal_tx = xrp_modal_tx.clone();
                        let progress_tx = CHANNEL.progress_tx.clone();
                        let create_state_clone = create_state.clone();

                        std::thread::spawn(move || {
                            let mut new_state = create_state_clone;
                            match Wallet::create(Some(CryptoAlgorithm::ED25519)) {
                                Ok(wallet) => {
                                    new_state.seed = Some(wallet.seed.clone());
                                    let _ = modal_tx.send(XRPModalState {
                                        create_wallet: Some(new_state),
                                        import_wallet: None,
                                        view_type: ActiveView::XRP,
                                    });
                                    let _ = progress_tx.send(None);
                                }
                                Err(e) => {
                                    new_state.error = Some(format!("Wallet creation failed: {}", e));
                                    let _ = modal_tx.send(XRPModalState {
                                        create_wallet: Some(new_state),
                                        import_wallet: None,
                                        view_type: ActiveView::XRP,
                                    });
                                    let _ = progress_tx.send(None);
                                }
                            }
                        });
                        let _ = xrp_modal_tx.send(XRPModalState {
                            create_wallet: Some(create_state.clone()),
                            import_wallet: None,
                            view_type: ActiveView::XRP,
                        });
                    }
                });
            ui.visuals_mut().widgets = original_visuals.widgets;
        });
    } else if let Some(seed) = &create_state.seed {
        ui.label(RichText::new("Wallet Generated").size(16.0).color(styles::text_color(is_dark_mode)));
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.set_max_width(280.0);
            styles::seed_frame(ui, |ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(8.0, 8.0);
                ui.horizontal(|ui| {
                    // Seed phrase
                    ui.add(
                        egui::Label::new(
                            RichText::new(seed)
                                .monospace()
                                .size(14.0)
                                .color(styles::text_color(is_dark_mode)),
                        )
                    );
                    // Modernized Copy Button with Emoji
                    let original_visuals = ui.visuals().clone();
                    let text_color = ui.style().visuals.text_color();
                    if !is_dark_mode {
                        ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
                        ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
                    }
                    Frame::new() // egui 0.31.1, no ID argument
                        .inner_margin(Margin::symmetric(8, 4))
                        .show(ui, |ui| {
                            let copy_label = ui.add(
                                egui::Label::new(RichText::new("📋").size(12.0).color(text_color))
                                    .sense(Sense::click()),
                            )
                            .on_hover_text("Copy to clipboard");
                            if copy_label.clicked() {
                                ui.ctx().copy_text(seed.clone());
                            }
                            if copy_label.hovered() {
                                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                            }
                        });
                    ui.visuals_mut().widgets = original_visuals.widgets;
                });
            });
        });

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
                    if ui
                        .add(
                            egui::Button::new(RichText::new("Continue").size(14.0).color(text_color))
                                .min_size(egui::Vec2::new(100.0, 28.0)),
                        )
                        .clicked()
                    {
                        create_state.step = 2;
                        let _ = xrp_modal_tx.send(XRPModalState {
                            create_wallet: Some(create_state.clone()),
                            import_wallet: None,
                            view_type: ActiveView::XRP,
                        });
                        ui.ctx().request_repaint();
                    }
                });
            ui.visuals_mut().widgets = original_visuals.widgets;
        });
    }
}