use egui::{Ui, ProgressBar, Color32, Pos2, Rect, Vec2, RichText, Area, Order, Sense, UiBuilder, Layout, Align, CornerRadius};
use crate::channel::{CHANNEL};
use tokio::time::{sleep, Duration};

// Helper function to interpolate between two Color32 values
fn lerp_color32(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0); // Ensure t is in [0, 1]
    Color32::from_rgba_premultiplied(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
        (a.a() as f32 + (b.a() as f32 - a.a() as f32) * t) as u8,
    )
}

pub struct ProgressBarState {
    operation_name: String,
}

impl ProgressBarState {
    pub fn new(operation_name: String) -> Self {
        Self { operation_name }
    }

    pub fn render_progress_bar(&mut self, ui: &mut Ui, _width: f32) -> bool {
        let progress_rx = CHANNEL.progress_rx.clone();
        let progress_state = progress_rx.borrow().clone();

        if let Some(state) = &progress_state {
            Area::new("progress_bar_overlay".into())
                .order(Order::Foreground)
                .fixed_pos(Pos2::new(0.0, 0.0))
                .show(ui.ctx(), |ui| {
                    let screen_rect = ui.ctx().input(|i| i.screen_rect);

                    ui.visuals_mut().panel_fill = Color32::from_rgb(32, 32, 32);
                    ui.visuals_mut().window_fill = Color32::from_rgb(32, 32, 32);
                    ui.visuals_mut().extreme_bg_color = Color32::from_rgb(32, 32, 32);

                    ui.painter().rect_filled(screen_rect, 0.0, Color32::from_rgb(32, 32, 32)); // Fully opaque dark gray
                    let button_response = ui.allocate_rect(screen_rect, Sense::click());
                    button_response.on_hover_cursor(egui::CursorIcon::NotAllowed);

                    let center = Pos2::new(screen_rect.width() / 2.0, screen_rect.height() / 2.0);
                    let content_width = 300.0;
                    let content_height = 100.0; // Reduced height since no icon is displayed

                    ui.scope_builder(
                        UiBuilder::new().max_rect(Rect::from_center_size(
                            center,
                            Vec2::new(content_width, content_height),
                        )),
                        |ui| {
                            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                ui.visuals_mut().override_text_color = Some(Color32::from_rgb(255, 254, 250));

                                // Determine fill color and progress value based on state
                                let (fill_color, progress, text_color) = if state.message.to_lowercase().contains("error") || state.message.to_lowercase().contains("failed") {
                                    // Failure: Red bar, full progress
                                    (
                                        Color32::from_rgb(255, 99, 71), // Red for failure
                                        1.0,
        Color32::from_rgb(255, 255, 255), // White text for contrast
                                    )
                                } else if state.progress >= 1.0 {
                                    // Success: Green bar, full progress
                                    (
                                        Color32::from_rgb(50, 205, 50), // Green for success
                                        1.0,
        Color32::from_rgb(255, 255, 255), // White text for contrast
                                    )
                                } else {
                                    // In-progress: Blue bar with pulse animation
                                    let anim_id = ui.id().with("progress_pulse");
                                    let anim_value = ui.ctx().animate_value_with_time(anim_id, 1.0, 0.5); // 0.5s cycle
                                    let pulse_factor = 0.8 + 0.2 * anim_value.sin(); // Intensity: 0.8 to 1.0
                                    let base_color = Color32::from_rgb(100, 100, 255); // Blue base
                                    let pulse_color = Color32::from_rgb(150, 150, 255); // Lighter blue pulse
                                    (
                                        lerp_color32(base_color, pulse_color, pulse_factor),
                                        state.progress.clamp(0.0, 1.0),
                                        Color32::from_rgb(255, 254, 250), // White text for progress
                                    )
                                };

                                let status_text = format!("{}: {}", self.operation_name, state.message);

                                ui.add(
                                    ProgressBar::new(progress)
                                        .desired_width(content_width)
                                        .desired_height(20.0)
                                        .corner_radius(CornerRadius::same(4))
                                        .text(
                                            RichText::new(status_text)
                                                .color(text_color)
                                                .size(14.0)
                                                .strong(),
                                        )
                                        .fill(fill_color),
                                );

                                // Automatically clear the progress bar after 1.5s for success or failure
                                if state.progress >= 1.0 || state.message.to_lowercase().contains("error") || state.message.to_lowercase().contains("failed") {
                                    ui.ctx().request_repaint_after(Duration::from_millis(1000));
                                    tokio::spawn(async {
                                        sleep(Duration::from_millis(1000)).await;
                                        let _ = CHANNEL.progress_tx.send(None);
                                    });
                                }
                            });
                        },
                    );
                });

            true
        } else {
            false
        }
    }
}