use egui::{Ui, RichText, Color32, Vec2, Layout, Align, Frame, Margin};
use crate::channel::{CHANNEL, BTCImport};
use super::{buffers, styles};

pub struct ImportSeedState {
    seed_words: [String; 24], // Array to hold up to 24 mnemonic words
    error: Option<String>,
}

impl ImportSeedState {
    pub fn from_existing(import_state: &BTCImport, buffer_id: &str) -> Self {
        let (seed_words, _passphrase_buffer) = buffers::get_buffer(buffer_id);
        Self {
            seed_words,
            error: import_state.error.clone(),
        }
    }

    pub fn render(&mut self, ui: &mut Ui, import_state: &mut BTCImport, buffer_id: &str) -> bool {
        let is_dark_mode = CHANNEL.theme_user_rx.borrow().0;

        // Layout constants
        let box_width = 80.0; // Compact for modal
        let spacing = 6.0; // Reduced spacing
        let columns = 4; // 4 columns for 24 boxes (6 rows)
        let total_boxes = 24;
        let content_width = (box_width * columns as f32) + (spacing * (columns - 1) as f32) + 20.0;
        let content_height = (32.0 * (total_boxes / columns) as f32) + (spacing * (total_boxes / columns - 1) as f32) + 80.0;

        let mut submitted = false;
        ui.allocate_ui_with_layout(
            Vec2::new(content_width, content_height),
            Layout::top_down(Align::Center),
            |ui| {
                // Heading and label
                ui.heading(RichText::new("Enter Bitcoin Mnemonic").size(18.0).color(styles::text_color(is_dark_mode)));
                ui.add_space(8.0);
                ui.label(RichText::new("Enter your 12, 18, or 24-word mnemonic phrase:").size(12.0).color(styles::text_color(is_dark_mode)));
                ui.add_space(12.0);

                // Grid of 24 text boxes
                ui.style_mut().spacing.item_spacing = Vec2::new(spacing, spacing);
                let mut pasted = None; // Store (index, pasted_text) for paste events
                egui::Grid::new("mnemonic_grid")
                    .num_columns(columns)
                    .spacing([spacing, spacing])
                    .show(ui, |ui| {
                        for i in 0..total_boxes {
                            let mut word = self.seed_words[i].clone();
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut word)
                                    .desired_width(box_width)
                                    .min_size(Vec2::new(box_width, 24.0))
                                    .hint_text(format!("{}", i + 1))
                                    .id_source(format!("seed_word_{}", i)),
                            );

                            // Detect paste or edit
                            if response.changed() {
                                if word.contains(' ') {
                                    pasted = Some((i, word.clone()));
                                } else {
                                    self.seed_words[i] = word.clone();
                                    // Update buffer for single-word edits
                                    buffers::update_buffer(buffer_id, self.seed_words.clone(), String::new());
                                }
                            }

                            if (i + 1) % columns == 0 {
                                ui.end_row();
                            }
                        }
                    });

                // Handle paste after grid to update all boxes
                if let Some((start_index, pasted_text)) = pasted {
                    let words: Vec<&str> = pasted_text.trim().split_whitespace().collect();
                    for (j, w) in words.iter().enumerate().take(24 - start_index) {
                        self.seed_words[start_index + j] = w.to_string();
                    }
                    // Update buffer after paste
                    buffers::update_buffer(buffer_id, self.seed_words.clone(), String::new());
                }

                ui.add_space(10.0);

                // Error display
                if let Some(error) = &self.error {
                    ui.colored_label(Color32::RED, error);
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
                            let continue_button = ui.add(
                                egui::Button::new(RichText::new("Continue").size(14.0).color(text_color))
                                    .min_size(egui::Vec2::new(100.0, 28.0)),
                            );
                            if continue_button.clicked() {
                                if self.submit(import_state, buffer_id) {
                                    submitted = true;
                                }
                            }
                        });
                    ui.visuals_mut().widgets = original_visuals.widgets;
                });

                ui.add_space(10.0);
            },
        );

        submitted
    }

    fn submit(&mut self, import_state: &mut BTCImport, buffer_id: &str) -> bool {
        let seed_phrase = self.seed_words.iter().filter(|w| !w.is_empty()).map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
        let word_count = self.seed_words.iter().filter(|w| !w.is_empty()).count();
        // Enforce 24-word mnemonic
        if word_count != 24 {
            self.error = Some("Mnemonic must be 24 words.".to_string());
            import_state.error = self.error.clone();
            return false;
        }

        import_state.error = None;
        import_state.step = 2;
        import_state.seed = Some(seed_phrase);
        buffers::update_buffer(buffer_id, self.seed_words.clone(), String::new());
        true
    }
}

pub fn render(ui: &mut Ui, import_state: &mut BTCImport, buffer_id: &str) {
    let mut state = ImportSeedState::from_existing(import_state, buffer_id);
    if state.render(ui, import_state, buffer_id) {
        ui.ctx().request_repaint(); // Request repaint to reflect step change
    }
}