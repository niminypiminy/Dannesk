// src/linux_icon_fix.rs
#[cfg(target_os = "linux")]
pub fn apply_linux_icon_and_desktop_entry() {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    // 1. Embed the 512 PNG directly in the binary
const ICON_512_PNG: &[u8] = include_bytes!("icons/icon-512.png");


    // 2. Write it to a reliable cache location
    let cache_dir = dirs::cache_dir()
        .map(|p| p.join("dannesk"))
        .unwrap_or_else(|| PathBuf::from(".cache/dannesk"));

    let _ = fs::create_dir_all(&cache_dir);
    let icon_path = cache_dir.join("icon.png");

    if !icon_path.exists() {
        let _ = File::create(&icon_path).and_then(|mut f| f.write_all(ICON_512_PNG));
    }

    // 3. Create or update the .desktop file
    let desktop_dir = dirs::data_local_dir()
        .map(|p| p.join("applications"))
        .unwrap_or_else(|| PathBuf::from(".local/share/applications"));

    let _ = fs::create_dir_all(&desktop_dir);
    let desktop_file = desktop_dir.join("dannesk.desktop");

    let exe = std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "dannesk".to_string());

    let desktop_content = format!(
        r#"[Desktop Entry]
Name=Dannesk
Comment=Crypto Wallet
Exec="{exe}"
Icon={icon}
Terminal=false
Type=Application
Categories=Utility;Finance;
StartupWMClass=Dannesk
X-GNOME-Autostart-enabled=false
"#,
        exe = exe.replace('"', "\\\""),
        icon = icon_path.to_str().unwrap_or("")
    );

    let _ = File::create(desktop_file).and_then(|mut f| f.write_all(desktop_content.as_bytes()));

    // 4. Tell GNOME/KDE to refresh (optional but helps instantly)
    let _ = std::process::Command::new("update-desktop-database")
        .arg(dirs::data_local_dir().unwrap_or_default().join("applications"))
        .output();

    // 5. Small delay so Wayland picks up the new .desktop file
    std::thread::sleep(std::time::Duration::from_millis(300));
}