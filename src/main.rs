
const VERSION: &str = "0.5.0";

mod ui;
mod channel;
mod font;
mod theme;
mod ws;
mod encrypt;
mod wallet;
mod pin;
mod decrypt;
mod icon;
mod utils;
mod startup;


use eframe::{egui, epaint::Vec2};
use egui_extras;
use crate::channel::{CHANNEL, WSCommand};
use crate::ws::{run_exchange_websocket, run_crypto_websocket};
use crate::ui::enterpin::EnterPinState;
use crate::ui::update::UpdatePrompt;
use crate::icon::load_icon;
use crate::startup::init_startup;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::sync::mpsc;

enum AppState {
    UpdatePrompt(UpdatePrompt),
    PinEntry(EnterPinState),
    Transition,
    Dashboard,
}

struct CryptoApp {
    state: AppState,
    commands_tx: mpsc::Sender<WSCommand>,
    exchange_shutdown_tx: mpsc::Sender<()>,
    crypto_shutdown_tx: mpsc::Sender<()>,
}

impl CryptoApp {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        commands_tx: mpsc::Sender<WSCommand>,
        exchange_shutdown_tx: mpsc::Sender<()>,
        crypto_shutdown_tx: mpsc::Sender<()>,
    ) -> Self {
        let remote_version = CHANNEL.version_rx.borrow().clone();
        let initial_state = match remote_version {
            None => AppState::PinEntry(EnterPinState::new()),
            Some(ref version) if version == VERSION => AppState::PinEntry(EnterPinState::new()),
            Some(version) => AppState::UpdatePrompt(UpdatePrompt::new(version)),
        };

        Self {
            state: initial_state,
            commands_tx,
            exchange_shutdown_tx,
            crypto_shutdown_tx,
        }
    }
}

impl eframe::App for CryptoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let is_minimized = ctx.input(|i| i.viewport().minimized.unwrap_or(false));

        let is_dark_mode = CHANNEL.theme_user_rx.borrow().0;
        ctx.set_visuals(if is_dark_mode {
            theme::Theme::dark_theme()
        } else {
            theme::Theme::white_theme()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.state {
                AppState::UpdatePrompt(update_prompt) => {
                    if update_prompt.render_update_screen(ui) {
                        // Trigger restart
                        if let Ok(current_exe) = std::env::current_exe() {
                            println!("Restarting application: {}", current_exe.display());
                            if let Err(e) = std::process::Command::new(current_exe).spawn() {
                                println!("Failed to restart application: {}", e);
                            }
                        } else {
                            println!("Failed to get current executable path for restart");
                        }
                        std::process::exit(0);
                    }
                }
                AppState::PinEntry(pin_state) => {
                    if pin_state.render_pin_screen(ui) {
                        self.state = AppState::Transition;
                    }
                }
                AppState::Transition => {
                    self.state = AppState::Dashboard;
                }
                AppState::Dashboard => {
                    if let Some(new_theme_user) = ui::dashboard::render_dashboard(ui, self.commands_tx.clone()) {
                        let _ = CHANNEL.theme_user_tx.send(new_theme_user);
                    }
                }
            }
        });

        ui::modals::render_modals(ctx);
        if !is_minimized {
            ctx.request_repaint_after(Duration::from_millis(75));
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let exchange_shutdown_tx = self.exchange_shutdown_tx.clone();
        let crypto_shutdown_tx = self.crypto_shutdown_tx.clone();
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            runtime.spawn(async move {
                let _ = exchange_shutdown_tx.send(()).await;
                let _ = crypto_shutdown_tx.send(()).await;
            });
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_startup();

    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;

    let (commands_tx, commands_rx) = mpsc::channel(100);
    let (exchange_shutdown_tx, exchange_shutdown_rx) = mpsc::channel::<()>(1);
    let (crypto_shutdown_tx, crypto_shutdown_rx) = mpsc::channel::<()>(1);

    wallet::load_wallets(commands_tx.clone());

    let icon_data = load_icon()?;

    runtime.block_on(async {
        // Check version before starting WebSocket tasks
        let remote_version = CHANNEL.version_rx.borrow().clone();
        let start_websockets = match remote_version {
            None => true, // Start WebSockets if no remote version
            Some(ref version) if version == VERSION => true, // Start WebSockets if version matches
            Some(_) => false, // Skip WebSockets if version mismatch
        };

        if start_websockets {
            tokio::spawn(async {
                if let Err(_e) = run_exchange_websocket(exchange_shutdown_rx).await {
                    let _ = CHANNEL.exchange_ws_status_tx.send(false);
                }
            });
            tokio::spawn(async {
                if let Err(_e) = run_crypto_websocket(commands_rx, crypto_shutdown_rx).await {
                    let _ = CHANNEL.crypto_ws_status_tx.send(false);
                }
            });
        }

        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size(Vec2::new(800.0, 600.0))
                .with_min_inner_size(Vec2::new(800.0, 600.0))
                .with_icon(icon_data),
            ..Default::default()
        };

        eframe::run_native(
            "Dannesk",
            options,
            Box::new(|cc| {
                font::setup_custom_font(&cc.egui_ctx);
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Ok(Box::new(CryptoApp::new(
                    cc,
                    commands_tx,
                    exchange_shutdown_tx,
                    crypto_shutdown_tx,
                )))
            }),
        )
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    })
}