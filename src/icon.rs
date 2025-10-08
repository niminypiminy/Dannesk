use eframe::egui;
use egui_extras::image::load_image_bytes;

pub fn load_icon() -> Result<egui::IconData, Box<dyn std::error::Error>> {
    let image_data = include_bytes!("icons/icon64.png");
    let color_image = load_image_bytes(image_data)?;
    if color_image.size[0] != 64 || color_image.size[1] != 64 {
        return Err(format!("Icon must be 64x64, got {}x{}", color_image.size[0], color_image.size[1]).into());
    }
    Ok(egui::IconData {
        rgba: color_image.pixels.into_iter().flat_map(|c| c.to_array()).collect(),
        width: color_image.size[0] as u32,
        height: color_image.size[1] as u32,
    })
}