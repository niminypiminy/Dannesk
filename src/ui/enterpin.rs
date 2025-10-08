use egui::{Color32, Pos2, RichText, Rect, Ui, Vec2, UiBuilder, Layout, Align};
use crate::utils::json_storage;

pub struct EnterPinState {
    pin_digits: [String; 6],
    confirm_pin_digits: [String; 6],
    pin_error: Option<String>,
    attempts_left: u32,
    is_pin_set: bool,
    is_unlocked: bool,
    error_shake: f32,
    focused_index: usize,
    is_confirming: bool,
    submit_trigger_time: Option<f32>,
}

impl EnterPinState {
    pub fn new() -> Self {
let is_pin_set = json_storage::read_json::<crate::pin::PinData>("pin.json").is_ok();

        Self {
            pin_digits: Default::default(),
            confirm_pin_digits: Default::default(),
            pin_error: None,
            attempts_left: 5,
            is_pin_set,
            is_unlocked: false,
            error_shake: 0.0,
            focused_index: 0,
            is_confirming: false,
            submit_trigger_time: None,
        }
    }

    pub fn render_pin_screen(&mut self, ui: &mut Ui) -> bool {
    if self.is_unlocked {
        return true;
    }

    // Dynamic sizing based on available space
    let available_width = ui.available_width();
    let available_height = ui.available_height();

    // Scale sizes relative to a reference resolution (e.g., 800x600)
    let reference_width = 800.0;
    let scale_factor = (available_width / reference_width).clamp(0.5, 2.0); // Clamp to avoid extreme scaling
    let box_width = 40.0 * scale_factor; // Scales from 20 to 80
    let spacing = 8.0 * scale_factor; // Scales from 4 to 16
    let total_boxes = 6;
    let content_width = (box_width * total_boxes as f32) + (spacing * (total_boxes - 1) as f32) + 10.0 * scale_factor;
    let content_height = 200.0 * scale_factor; // Scales from 100 to 400
    let center = Pos2::new(available_width / 2.0, available_height / 2.0); // Center dynamically
    let content_rect = Rect::from_center_size(center, Vec2::new(content_width, content_height));

    // Check for auto-submit
    let current_time = ui.ctx().input(|i| i.time) as f32;
    let submit_delay = 0.1;
    if let Some(trigger_time) = self.submit_trigger_time {
        if current_time >= trigger_time + submit_delay {
            self.submit_trigger_time = None;
            self.submit_pin();
        }
    }

    ui.scope_builder(
        UiBuilder::new().max_rect(content_rect),
        |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                // Dynamic font size
                let font_size = (available_width * 0.015).clamp(12.0, 18.0); // Scales from 12 to 18

                // Heading and label
                if !self.is_pin_set {
                    if self.is_confirming {
                        ui.heading(RichText::new("Confirm Your PIN").size(font_size * 1.5));
                        ui.add_space(10.0 * scale_factor);
                        ui.label(RichText::new("Re-enter your six-digit PIN to confirm:").size(font_size));
                    } else {
                        ui.heading(RichText::new("Set a Six-Digit PIN").size(font_size * 1.5));
                        ui.add_space(10.0 * scale_factor);
                        ui.label(RichText::new("Enter a six-digit PIN to secure the app:").size(font_size));
                    }
                } else {
                    ui.heading(RichText::new("Enter PIN").size(font_size * 1.5));
                    ui.add_space(10.0 * scale_factor);
                    ui.label(
                        RichText::new(format!(
                            "Enter your six-digit PIN ({} attempts left):",
                            self.attempts_left
                        ))
                        .size(font_size),
                    );
                }
                ui.add_space(20.0 * scale_factor);

                // Six input boxes
                let is_confirming = self.is_confirming && !self.is_pin_set;
                let mut temp_digits = if is_confirming {
                    self.confirm_pin_digits.clone()
                } else {
                    self.pin_digits.clone()
                };
                let mut new_focused_index = self.focused_index;
                let mut digit_entered = false;

                ui.style_mut().spacing.item_spacing = Vec2::new(spacing, 0.0);
                ui.horizontal(|ui| {
                    let shake_offset = if self.error_shake > 0.0 {
                        (self.error_shake * 10.0).sin() * 5.0 * scale_factor
                    } else {
                        0.0
                    };
                    ui.add_space(shake_offset.abs());

                    ui.input(|input| {
                        if input.key_pressed(egui::Key::Backspace) {
                            temp_digits[self.focused_index] = String::new();
                            if self.focused_index > 0 {
                                new_focused_index = self.focused_index - 1;
                            }
                        } else {
                            let digit = if input.key_pressed(egui::Key::Num0) { Some(0) }
                                else if input.key_pressed(egui::Key::Num1) { Some(1) }
                                else if input.key_pressed(egui::Key::Num2) { Some(2) }
                                else if input.key_pressed(egui::Key::Num3) { Some(3) }
                                else if input.key_pressed(egui::Key::Num4) { Some(4) }
                                else if input.key_pressed(egui::Key::Num5) { Some(5) }
                                else if input.key_pressed(egui::Key::Num6) { Some(6) }
                                else if input.key_pressed(egui::Key::Num7) { Some(7) }
                                else if input.key_pressed(egui::Key::Num8) { Some(8) }
                                else if input.key_pressed(egui::Key::Num9) { Some(9) }
                                else { None };
                            if let Some(digit) = digit {
                                if self.focused_index < 6 {
                                    temp_digits[self.focused_index] = digit.to_string();
                                    if self.focused_index < 5 {
                                        new_focused_index = self.focused_index + 1;
                                    }
                                    digit_entered = true;
                                }
                            }
                        }
                    });

                    if digit_entered && temp_digits.iter().all(|d| d.len() == 1) {
                        self.submit_trigger_time = Some(current_time);
                    }

                    for i in 0..6 {
                        let mut text = temp_digits[i].clone();
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut text)
                                .desired_width(box_width)
                                .min_size(Vec2::new(box_width, 0.0))
                                .char_limit(1)
                                .password(true) // Enable password mode (shows ●)
                                .id_source(format!(
                                    "pin_{}_{}",
                                    if is_confirming { "confirm" } else { "pin" },
                                    i
                                )),
                        );

                        if i == new_focused_index {
                            ui.memory_mut(|mem| mem.request_focus(response.id));
                        }

                        if text.chars().all(|c| c.is_digit(10)) || text.is_empty() {
                            temp_digits[i] = text;
                        }

                        if response.gained_focus() {
                            new_focused_index = i;
                        }
                    }
                });

                if is_confirming {
                    self.confirm_pin_digits = temp_digits;
                } else {
                    self.pin_digits = temp_digits;
                }
                self.focused_index = new_focused_index;

                ui.add_space(10.0 * scale_factor);

                if let Some(error) = &self.pin_error {
                    ui.colored_label(Color32::RED, error);
                    self.error_shake = 1.0;
                } else {
                    self.error_shake *= 0.9;
                    if self.error_shake < 0.01 {
                        self.error_shake = 0.0;
                    }
                }

                ui.add_space(10.0 * scale_factor);
                ui.label(
                    egui::RichText::new("You can reset your PIN later in Settings if needed.")
                        .weak()
                        .size(font_size * 0.8), // Slightly smaller for secondary text
                );
                ui.add_space(10.0 * scale_factor);

                if ui.input(|i| i.key_pressed(egui::Key::Enter))
                    && (if is_confirming {
                        &self.confirm_pin_digits
                    } else {
                        &self.pin_digits
                    })
                    .iter()
                    .all(|d| d.len() == 1)
                {
                    self.submit_pin();
                }
            });
        },
    );

    self.is_unlocked
}

    fn submit_pin(&mut self) {
        let pin: String = self.pin_digits.join("");
        let confirm_pin: String = self.confirm_pin_digits.join("");

        if !self.is_pin_set {
            if !self.is_confirming {
                if pin.chars().all(|c| c.is_digit(10)) && pin.len() == 6 {
                    self.is_confirming = true;
                    self.pin_error = None;
                    self.confirm_pin_digits = Default::default();
                    self.focused_index = 0;
                    self.pin_digits = self.pin_digits.clone();
                } else {
                    self.pin_error = Some("PIN must be a six-digit number".to_string());
                }
            } else {
                if confirm_pin.chars().all(|c| c.is_digit(10)) && confirm_pin.len() == 6 {
                    if pin == confirm_pin {
                        match crate::pin::set_pin(&pin) {
                            Ok(()) => {
                                self.is_pin_set = true;
                                self.is_unlocked = true;
                                self.pin_error = None;
                                self.pin_digits = Default::default();
                                self.confirm_pin_digits = Default::default();
                                self.focused_index = 0;
                                self.is_confirming = false;
                            }
                            Err(crate::pin::PinError::InvalidPin) => {
                                self.pin_error = Some("PIN must be a six-digit number".to_string());
                            }
                            Err(e) => {
                                self.pin_error = Some(e.to_string());
                            }
                        }
                    } else {
                        self.pin_error = Some("PINs do not match".to_string());
                        self.confirm_pin_digits = Default::default();
                        self.focused_index = 0;
                    }
                } else {
                    self.pin_error = Some("PIN must be a six-digit number".to_string());
                }
            }
        } else {
            if pin.chars().all(|c| c.is_digit(10)) && pin.len() == 6 {
                match crate::pin::verify_pin(&pin) {
                    Ok(()) => {
                        self.is_unlocked = true;
                        self.pin_error = None;
                        self.pin_digits = Default::default();
                        self.focused_index = 0;
                    }
                    Err(crate::pin::PinError::IncorrectPin) => {
                        self.attempts_left -= 1;
                        self.pin_error = Some("Incorrect PIN".to_string());
                        if self.attempts_left == 0 {
                            self.pin_error = Some("Too many attempts. App locked.".to_string());
                        }
                    }
                    Err(e) => {
                        self.pin_error = Some(e.to_string());
                    }
                }
            } else {
                self.pin_error = Some("PIN must be a six-digit number".to_string());
            }
        }
    }
}