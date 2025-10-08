use egui::{Color32, Visuals, CornerRadius, style::{Widgets, TextCursorStyle, Selection}};

pub struct Theme;

impl Theme {
    pub fn dark_theme() -> Visuals {
        Visuals {
            dark_mode: true,

            // Backgrounds
// Backgrounds with opacity
panel_fill: Color32::from_rgba_premultiplied(30, 30, 30, 230),  // 95% opacity (alpha = 245)
window_fill: Color32::from_rgba_premultiplied(30, 30, 30, 230),  // 95% opacity (alpha = 245)

            extreme_bg_color: Color32::from_rgb(20, 20, 20),
            code_bg_color: Color32::from_rgb(40, 40, 40),
            faint_bg_color: Color32::from_rgb(35, 35, 35), // Slightly adjusted for subtle separation

            // Selection
            selection: Selection {
                bg_fill: Color32::from_rgb(50, 50, 50),
                stroke: egui::Stroke::new(1.0, Color32::from_rgb(37, 37, 37)),
            },

            // Widget visuals
            widgets: Widgets {
                noninteractive: egui::style::WidgetVisuals {
bg_fill: Color32::from_rgb(60, 60, 58),
                    weak_bg_fill: Color32::from_rgb(45, 45, 43),
                    bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(30, 29, 27)),
                    corner_radius: CornerRadius::same(4),
                    fg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(255, 254, 250)),
                    expansion: 0.0,
                },
                inactive: egui::style::WidgetVisuals {
bg_fill: Color32::from_rgb(75, 75, 72),
                    weak_bg_fill: Color32::from_rgb(55, 55, 53),
                    bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(30, 29, 27)),
                    corner_radius: CornerRadius::same(4),
                    fg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(255, 254, 250)),
                    expansion: 0.0,
                },
                hovered: egui::style::WidgetVisuals {
                    bg_fill: Color32::from_rgb(80, 80, 78),
                    weak_bg_fill: Color32::from_rgb(70, 70, 68),
                    bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(100, 100, 100)),
                    corner_radius: CornerRadius::same(4),
                    fg_stroke: egui::Stroke::new(1.5, Color32::from_rgb(255, 254, 250)),
                    expansion: 1.0,
                },
                active: egui::style::WidgetVisuals {
                    bg_fill: Color32::from_rgb(64, 70, 76), // Slight blue tint
                    weak_bg_fill: Color32::from_rgb(65, 65, 63),
                    bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(120, 120, 120)),
                    corner_radius: CornerRadius::same(4),
                    fg_stroke: egui::Stroke::new(2.0, Color32::from_rgb(255, 254, 250)),
                    expansion: 1.0,
                },
                open: egui::style::WidgetVisuals {
                    bg_fill: Color32::from_rgb(65, 65, 63),
                    weak_bg_fill: Color32::from_rgb(55, 55, 53),
                    bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(30, 29, 27)),
                    corner_radius: CornerRadius::same(4),
                    fg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(255, 254, 250)),
                    expansion: 0.0,
                },
            },

            // Cursor
            text_cursor: TextCursorStyle {
                stroke: egui::Stroke::new(2.0, Color32::from_rgb(255, 254, 250)),
                ..Default::default()
            },

            ..Visuals::default()
        
        }
    }

  pub fn white_theme() -> Visuals {
    Visuals {
        dark_mode: false,

        // Backgrounds (light grey for warmth and softness)
        panel_fill: Color32::from_rgb(255, 255, 255), // Light grey instead of off-white
        window_fill: Color32::from_rgb(255, 255, 255), // Consistent with panel_fill
        extreme_bg_color: Color32::from_rgb(200, 200, 200), // Darker grey for depth
        code_bg_color: Color32::from_rgb(220, 220, 220), // Subtle grey for code blocks
        faint_bg_color: Color32::from_rgb(225, 225, 225), // Neutral separation

        // Selection (grey instead of soft blue)
        selection: Selection {
            bg_fill: Color32::from_rgb(200, 200, 200), // Matches render_asset_selector light mode
            stroke: egui::Stroke::new(1.0, Color32::from_rgb(150, 150, 150)), // Darker grey stroke
        },

        // Widgets (higher contrast, grey-based)
        widgets: Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(225, 225, 225), // Light grey
                weak_bg_fill: Color32::from_rgb(220, 220, 220), // Slightly darker
                bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(150, 150, 150)), // Grey border
                corner_radius: CornerRadius::same(4),
                fg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(30, 30, 30)), // Dark text
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(220, 220, 220), // Light grey
                weak_bg_fill: Color32::from_rgb(215, 215, 215), // Slightly darker
                bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(150, 150, 150)), // Grey border
                corner_radius: CornerRadius::same(4),
                fg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(30, 30, 30)), // Dark text
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(210, 210, 210), // Slightly darker grey for hover
                weak_bg_fill: Color32::from_rgb(205, 205, 205), // Subtle contrast
                bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(130, 130, 130)), // Stronger grey border
                corner_radius: CornerRadius::same(4),
                fg_stroke: egui::Stroke::new(1.5, Color32::from_rgb(20, 20, 20)), // Bolder text
                expansion: 1.0,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(200, 200, 200), // Grey for active, matches render_asset_selector
                weak_bg_fill: Color32::from_rgb(195, 195, 195), // Slightly darker
                bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(130, 130, 130)), // Grey border
                corner_radius: CornerRadius::same(4),
                fg_stroke: egui::Stroke::new(2.0, Color32::from_rgb(20, 20, 20)), // Bolder text
                expansion: 1.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(220, 220, 220), // Matches inactive
                weak_bg_fill: Color32::from_rgb(215, 215, 215),
                bg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(150, 150, 150)),
                corner_radius: CornerRadius::same(4),
                fg_stroke: egui::Stroke::new(1.0, Color32::from_rgb(30, 30, 30)),
                expansion: 0.0,
            },
        },

        // Cursor
        text_cursor: TextCursorStyle {
            stroke: egui::Stroke::new(2.0, Color32::from_rgb(30, 30, 30)), // Dark cursor for visibility
            ..Default::default()
        },

        ..Visuals::default()
    }
}

}