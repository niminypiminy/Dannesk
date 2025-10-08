use egui::{Pos2, Rect, RichText, Ui, Frame, Margin, Vec2, UiBuilder, Layout, Align};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use zip::read::ZipArchive;
use flate2::read::GzDecoder;
use tar::Archive;
#[cfg(windows)]
use self_replace::self_replace;
use crate::channel::CHANNEL;

pub struct UpdatePrompt {
    remote_version: String,
    update_url: String,
    is_updating: bool,
    update_status: String,
    update_task: Option<mpsc::Receiver<Result<(), String>>>,
    success_display_start: Option<Instant>,
}

impl UpdatePrompt {
    pub fn new(remote_version: String) -> Self {
        let update_url = CHANNEL.update_url_rx.borrow().clone().unwrap_or_default();
        Self {
            remote_version,
            update_url: update_url.clone(),
            is_updating: false,
            update_status: if update_url.is_empty() {
                String::from("Update URL not available")
            } else {
                String::new()
            },
            update_task: None,
            success_display_start: None,
        }
    }

    pub fn render_update_screen(&mut self, ui: &mut Ui) -> bool {
        let available_width = ui.available_width();
        let available_height = ui.available_height();
        let reference_width = 800.0;
        let scale_factor = (available_width / reference_width).clamp(0.5, 2.0);
        let content_width = 300.0 * scale_factor;
        let content_height = 200.0 * scale_factor;
        let center = Pos2::new(available_width / 2.0, available_height / 2.0);
        let content_rect = Rect::from_center_size(center, Vec2::new(content_width, content_height));

        let mut request_repaint = false;
        let mut should_restart = false;

        ui.scope_builder(UiBuilder::new().max_rect(content_rect), |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let font_size = (available_width * 0.015).clamp(12.0, 18.0);
                ui.add_space(10.0 * scale_factor);
                ui.label(
                    RichText::new(format!(
                        "A new version ({}) is available. Please update to continue.",
                        self.remote_version
                    ))
                    .size(font_size * 1.3),
                );
                ui.add_space(20.0 * scale_factor);

                ui.vertical_centered(|ui| {
                    ui.style_mut().spacing.item_spacing = Vec2::new(10.0 * scale_factor, 0.0);
                    if self.is_updating {
                        ui.label(RichText::new("Updating...").size(font_size));
                        if let Some(receiver) = self.update_task.as_mut() {
                            match receiver.try_recv() {
                                Ok(result) => {
                                    match result {
                                        Ok(()) => {
                                            self.update_status = String::from("Update successful! Restarting in 3 seconds...");
                                            self.is_updating = false;
                                            self.success_display_start = Some(Instant::now());
                                        }
                                        Err(e) => {
                                            self.update_status = e;
                                            self.is_updating = false;
                                        }
                                    }
                                    request_repaint = true;
                                }
                                Err(mpsc::error::TryRecvError::Empty) => {
                                    request_repaint = true;
                                }
                                Err(mpsc::error::TryRecvError::Disconnected) => {
                                    self.update_status = String::from("Update task failed unexpectedly");
                                    self.is_updating = false;
                                    request_repaint = true;
                                }
                            }
                        }
                    } else {
                        let text_color = ui.style().visuals.text_color();
                        Frame::new()
                            .inner_margin(Margin::symmetric(8, 4))
                            .show(ui, |ui| {
                                let button = egui::Button::new(
                                    RichText::new("Continue")
                                        .size(14.0 * scale_factor)
                                        .color(text_color),
                                )
                                .min_size(Vec2::new(150.0 * scale_factor, 36.0 * scale_factor));
                                if self.update_url.is_empty() {
                                    ui.add_enabled(false, button);
                                } else if ui.add(button).clicked() {
                                    if self.update_task.is_none() {
                                        self.is_updating = true;
                                        self.update_status = String::from("Starting update...");
                                        let update_url = self.update_url.clone();
                                        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
                                            let (tx, rx) = mpsc::channel(1);
                                            runtime.spawn(async move {
                                                let result = perform_update(&update_url).await;
                                                let _ = tx.send(result).await;
                                            });
                                            self.update_task = Some(rx);
                                            request_repaint = true;
                                        } else {
                                            self.update_status = String::from("Failed to access async runtime");
                                            self.is_updating = false;
                                            request_repaint = true;
                                        }
                                    }
                                }
                            });
                    }
                });
                ui.add_space(10.0 * scale_factor);
                ui.label(
                    RichText::new(if self.update_status.is_empty() {
                        "Updating ensures you have the latest features and security."
                    } else {
                        &self.update_status
                    })
                    .weak()
                    .size(font_size * 1.2),
                );
            });
        });

        if let Some(start) = self.success_display_start {
            if start.elapsed() >= Duration::from_secs(3) {
                should_restart = true;
            } else {
                request_repaint = true;
            }
        }

        if request_repaint {
            ui.ctx().request_repaint();
        }

        should_restart
    }
}

async fn perform_update(update_url: &str) -> Result<(), String> {
    if update_url.is_empty() {
        return Err(String::from("Update URL is not available"));
    }

    let temp_dir = tempfile::TempDir::new().map_err(|e| format!("Failed to create temp dir: {}", e))?;
    let temp_archive_path = match std::env::consts::OS {
        "windows" => temp_dir.path().join("Dannesk.zip"),
        "macos" => temp_dir.path().join("Dannesk.dmg"),
        "linux" => temp_dir.path().join("Dannesk.tar.gz"),
        _ => return Err(format!("Unsupported operating system: {}", std::env::consts::OS)),
    };

    let response = reqwest::get(update_url)
        .await
        .map_err(|e| format!("Failed to download update: {}", e))?;
    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}", response.status()));
    }
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let mut file = File::create(&temp_archive_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e))?;
    let _exe_dir = current_exe
        .parent()
        .ok_or_else(|| format!("Failed to get parent directory of {}", current_exe.display()))?;

    match std::env::consts::OS {
        "windows" => {
            let file = File::open(&temp_archive_path)
                .map_err(|e| format!("Failed to open ZIP file: {}", e))?;
            let mut archive = ZipArchive::new(file)
                .map_err(|e| format!("Failed to read ZIP archive: {}", e))?;

            let mut new_exe_path = None;
            for i in 0..archive.len() {
                let file = archive
                    .by_index(i)
                    .map_err(|e| format!("Failed to read ZIP entry {}: {}", i, e))?;
                if file.name().ends_with("Dannesk.exe") {
                    new_exe_path = Some(file.name().to_string());
                    break;
                }
            }
            let new_exe_name = new_exe_path.ok_or("Dannesk.exe not found in ZIP archive")?;
            let temp_exe_path = temp_dir.path().join("Dannesk.exe");
            let mut zipped_file = archive
                .by_name(&new_exe_name)
                .map_err(|e| format!("Failed to find {} in ZIP: {}", new_exe_name, e))?;
            let mut temp_file = File::create(&temp_exe_path)
                .map_err(|e| format!("Failed to create temp executable: {}", e))?;
            io::copy(&mut zipped_file, &mut temp_file)
                .map_err(|e| format!("Failed to extract Dannesk.exe: {}", e))?;

            if !temp_exe_path.exists() || temp_file.metadata().map(|m| m.len()).unwrap_or(0) == 0 {
                return Err(format!("New executable at {} is invalid or empty", temp_exe_path.display()));
            }

            #[cfg(windows)]
            replace_executable(&temp_exe_path).map_err(|e| {
                format!("Failed to replace executable: {}. Ensure the application has permission to modify its directory.", e)
            })?;
        }
        "linux" => {
            let file = File::open(&temp_archive_path)
                .map_err(|e| format!("Failed to open tar.gz file: {}", e))?;
            let gz = GzDecoder::new(file);
            let mut archive = Archive::new(gz);

            let mut new_exe_path = None;
            for entry in archive.entries().map_err(|e| format!("Failed to read tar.gz entries: {}", e))? {
                let file = entry.map_err(|e| format!("Failed to read tar.gz entry: {}", e))?;
                if file.path().map(|p| p.file_name().map(|n| n == "Dannesk").unwrap_or(false)).unwrap_or(false) {
                    new_exe_path = Some(file.path().map(|p| p.to_path_buf()).unwrap_or_default());
                    break;
                }
            }
            let new_exe_name = new_exe_path.ok_or("Dannesk binary not found in tar.gz archive")?;
            let temp_exe_path = temp_dir.path().join("Dannesk");
            let mut archive = Archive::new(GzDecoder::new(File::open(&temp_archive_path).map_err(|e| format!("Failed to reopen tar.gz: {}", e))?));
            archive.unpack(temp_dir.path()).map_err(|e| format!("Failed to extract tar.gz: {}", e))?;
            let extracted_exe = temp_dir.path().join(new_exe_name);
            fs::rename(&extracted_exe, &temp_exe_path)
                .map_err(|e| format!("Failed to rename extracted binary: {}", e))?;

            if !temp_exe_path.exists() || File::open(&temp_exe_path).map(|f| f.metadata().map(|m| m.len()).unwrap_or(0)).unwrap_or(0) == 0 {
                return Err(format!("New executable at {} is invalid or empty", temp_exe_path.display()));
            }

            #[cfg(unix)]
            fs::set_permissions(&temp_exe_path, fs::Permissions::from_mode(0o755))
                .map_err(|e| format!("Failed to set executable permissions: {}", e))?;

            #[cfg(unix)]
            {
                let target_exe = _exe_dir.join("Dannesk");
                replace_executable(&current_exe, &temp_exe_path, &target_exe).map_err(|e| {
                    format!(
                        "Failed to replace executable: {}. Ensure the application has permission to modify its directory.",
                        e
                    )
                })?;
            }
        }
        "macos" => {
            return Err(String::from("macOS update not yet implemented (DMG handling required)"));
        }
        _ => return Err(format!("Unsupported operating system: {}", std::env::consts::OS)),
    }

    Ok(())
}

#[cfg(windows)]
fn replace_executable(new_exe: &Path) -> Result<(), String> {
    if !new_exe.exists() {
        return Err(format!("New executable does not exist: {}", new_exe.display()));
    }

    let mut perms = fs::metadata(new_exe)
        .map_err(|e| format!("Failed to get metadata for {}: {}", new_exe.display(), e))?
        .permissions();
    if perms.readonly() {
        perms.set_readonly(false);
        fs::set_permissions(new_exe, perms)
            .map_err(|e| format!("Failed to set permissions for {}: {}", new_exe.display(), e))?;
    }

    self_replace(new_exe).map_err(|e| {
        format!("Failed to replace executable with {}: {}", new_exe.display(), e)
    })?;

    Ok(())
}

#[cfg(unix)]
fn replace_executable(current_exe: &Path, new_exe: &Path, target_exe: &Path) -> Result<(), String> {
    if !new_exe.exists() {
        return Err(format!("New executable does not exist: {}", new_exe.display()));
    }

    let backup_exe = current_exe.with_extension("bak");
    if current_exe.exists() {
        fs::rename(current_exe, &backup_exe).map_err(|e| {
            format!(
                "Failed to rename current executable to backup: {}. Ensure no other instances are running.",
                e
            )
        })?;
    }

    fs::rename(new_exe, target_exe)
        .map_err(|e| format!("Failed to move new executable to {}: {}", target_exe.display(), e))?;

    if !target_exe.exists() {
        if backup_exe.exists() {
            let _ = fs::rename(&backup_exe, current_exe);
        }
        return Err(format!("New executable at {} is missing after move", target_exe.display()));
    }

    if backup_exe.exists() {
        fs::remove_file(&backup_exe)
            .map_err(|e| format!("Failed to remove backup file: {}", e))?;
    }

    Ok(())
}