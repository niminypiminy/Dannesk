use std::error::Error;

pub struct IconData {
    pub rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub fn load_icon() -> Result<IconData, Box<dyn Error>> {
    
     let icon_bytes = include_bytes!("icons/icon64.png");

    // Load PNG (or any format image crate supports) from memory
    let img = image::load_from_memory(icon_bytes)?.to_rgba8();

    let (width, height) = img.dimensions();
    let rgba = img.into_raw();

    Ok(IconData { rgba, width, height })
}