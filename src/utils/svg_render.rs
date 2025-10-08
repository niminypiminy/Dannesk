// src/utils/svgrender.rs
use egui::Image;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;
use once_cell::sync::Lazy;

pub struct SvgCanvas;

// Static TempDir initialized once
static SVG_TEMP_DIR: Lazy<TempDir> = Lazy::new(|| {
    tempfile::tempdir().expect("Failed to create temporary directory for SVGs")
});

impl SvgCanvas {
    pub fn paint_svg(name: &str) -> Image<'static> {
        let svg_data: &[u8] = match name {
            "pin.svg" => &*include_bytes!("../svgs/pin.svg"),
            "exchange.svg" => &*include_bytes!("../svgs/exchange.svg"),
            "hidden.svg" => &*include_bytes!("../svgs/hidden.svg"),
            "nothidden.svg" => &*include_bytes!("../svgs/nothidden.svg"),
            "xrp_white.svg" => &*include_bytes!("../svgs/xrp_white.svg"),
            "xrp_dark.svg" => &*include_bytes!("../svgs/xrp_dark.svg"),
            "btc.svg" => &*include_bytes!("../svgs/btc.svg"),
            "rlusd.svg" => &*include_bytes!("../svgs/rlusd.svg"),
            "europ.svg" => &*include_bytes!("../svgs/europ.svg"),
            "transfer.svg" => &*include_bytes!("../svgs/transfer.svg"),
            _ => panic!("Unknown SVG canvas requested: {}", name),
        };

        let temp_file_path = SVG_TEMP_DIR.path().join(name);
        if !temp_file_path.exists() {
            let mut temp_file = File::create(&temp_file_path)
                .expect("Failed to create temporary SVG file");
            temp_file.write_all(svg_data)
                .expect("Failed to write SVG to temporary file");
        }

        Image::new(format!("file://{}", temp_file_path.display()))
    }
}