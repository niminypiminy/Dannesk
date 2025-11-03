use egui::{Ui, RichText, Frame, Margin, Color32, Stroke};
use crate::ui::settings::components::{SettingsMessage, SharedSettingsState, SettingComponent};
use crate::channel::CHANNEL;
use std::time::{Instant, Duration};

#[derive(Debug)]
pub struct NameComponent {
    name_input: String,
    feedback_message: Option<(String, Instant, FeedbackType)>,
}

#[derive(Debug)]
pub enum FeedbackType {
    Success,
    Error,
}

impl NameComponent {
    pub fn new() -> Self {
        Self {
            name_input: String::new(),
            feedback_message: None,
        }
    }
}

impl SettingComponent for NameComponent {
    fn render(&mut self, ui: &mut Ui, is_dark_mode: bool, _current_name: String) -> Option<SettingsMessage> {
        let text_color = if is_dark_mode {
            Color32::from_rgb(255, 254, 250) // #fffefa for dark mode
        } else {
            Color32::from_rgb(30, 29, 27) // #1e1d1b for light mode
        };

        let available_width = ui.available_width();
        // Dynamic sizing
        let label_size = (available_width * 0.035).clamp(12.0, 14.0);
        let button_text_size = (available_width * 0.04).clamp(14.0, 16.0);
        let feedback_text_size = (available_width * 0.035).clamp(12.0, 14.0);
        let spacing = (available_width * 0.03).clamp(8.0, 12.0);
        let input_width = (available_width * 0.8).clamp(150.0, 300.0);
        let button_width = (available_width * 0.2).clamp(100.0, 200.0);

        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.add_space(spacing);

                // Input field
                ui.vertical_centered(|ui| {
                    ui.add_space(spacing / 2.0);
                    let original_visuals = ui.visuals().clone();
                    ui.visuals_mut().widgets.inactive.bg_fill = if is_dark_mode {
                        Color32::from_rgba_premultiplied(50, 50, 50, 200)
                    } else {
                        Color32::from_rgb(220, 220, 220) // Match white_theme widgets.inactive.bg_fill
                    };
                    ui.visuals_mut().widgets.active.bg_stroke = Stroke::new(1.0, text_color);
                    ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::new(0.5, text_color);
                    ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::new(1.5, text_color);
                    Frame::NONE
                        .inner_margin(Margin {
                            left: 8,
                            right: 8,
                            top: 4,
                            bottom: 4,
                        })
                        .corner_radius(4.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.name_input)
                                    .desired_width(input_width)
                                    .text_color(text_color)
                                    .hint_text(RichText::new("Enter name").size(label_size).color(Color32::from_gray(100)))
                                    .margin(Margin {
                                        left: 4,
                                        right: 4,
                                        top: 2,
                                        bottom: 2,
                                    }),
                            );
                        });
                    ui.visuals_mut().widgets = original_visuals.widgets;
                });
                ui.add_space(spacing);

                // Submit button
                ui.vertical_centered(|ui| {
                    let original_visuals = ui.visuals().clone();
                    if !is_dark_mode {
                        ui.visuals_mut().widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
                        ui.visuals_mut().widgets.active.fg_stroke = egui::Stroke::new(2.0, text_color);
                        ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(210, 210, 210); // Match white_theme widgets.hovered.bg_fill
                    }
                    Frame::NONE
                        .inner_margin(Margin {
                            left: 8,
                            right: 8,
                            top: 4,
                            bottom: 4,
                        })
                        .corner_radius(4.0)
                        .show(ui, |ui| {
                            if ui
                                .add(
                                    egui::Button::new(RichText::new("Change Name").size(button_text_size).color(text_color))
                                        .min_size(egui::Vec2::new(button_width, button_text_size * 2.0))
                                        .fill(if is_dark_mode {
                                            Color32::from_rgb(50, 50, 50) // Match dark theme
                                        } else {
                                            Color32::from_rgb(200, 200, 200) // Match white_theme widgets.active.bg_fill
                                        })
                                        .stroke(if is_dark_mode {
                                            Stroke::new(1.0, Color32::from_rgb(180, 180, 180)) // Match render_asset_selector
                                        } else {
                                            Stroke::new(1.0, Color32::from_rgb(130, 130, 130)) // Match white_theme widgets.active.bg_stroke
                                        }),
                                )
                                .clicked()
                            {
                                let new_name = self.name_input.clone();
                                if new_name.is_empty() {
                                    self.feedback_message = Some((
                                        "Name cannot be empty".to_string(),
                                        Instant::now(),
                                        FeedbackType::Error,
                                    ));
                                } else if SharedSettingsState::save_to_file(&new_name) {
                                    if let Err(e) = CHANNEL.theme_user_tx.send((is_dark_mode, new_name.clone(), false)) {
                                        self.feedback_message = Some((
                                            format!("Failed to update name: {}", e),
                                            Instant::now(),
                                            FeedbackType::Error,
                                        ));
                                    } else {
                                        self.feedback_message = Some((
                                            "Name changed successfully".to_string(),
                                            Instant::now(),
                                            FeedbackType::Success,
                                        ));
                                        self.name_input.clear();
                                        // Signal to close the modal
                                        let mut new_state = CHANNEL.modal_rx.borrow().clone();
                                        new_state.name = false;
                                        if let Err(e) = CHANNEL.modal_tx.send(new_state) {
                                            self.feedback_message = Some((
                                                format!("Failed to close modal: {}", e),
                                                Instant::now(),
                                                FeedbackType::Error,
                                            ));
                                        }
                                    }
                                } else {
                                    self.feedback_message = Some((
                                        "Failed to save name".to_string(),
                                        Instant::now(),
                                        FeedbackType::Error,
                                    ));
                                }
                            }
                        });
                    ui.visuals_mut().widgets = original_visuals.widgets;
                });
                ui.add_space(spacing);

                // Feedback
                if let Some((msg, timestamp, feedback_type)) = &self.feedback_message {
                    let duration = match feedback_type {
                        FeedbackType::Success => Duration::from_secs(2),
                        FeedbackType::Error => Duration::from_secs(3),
                    };
                    if timestamp.elapsed() < duration {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(msg)
                                    .color(Color32::from_rgb(180, 180, 180))
                                    .size(feedback_text_size),
                            );
                        });
                    } else {
                        self.feedback_message = None;
                    }
                }
                ui.add_space(spacing);
            },
        );

        None // No message needed
    }
}