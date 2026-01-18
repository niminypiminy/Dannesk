#![cfg_attr(windows, windows_subsystem = "windows")]

const VERSION: &str = "0.5.0";


use dioxus::prelude::*;
use winit::dpi::LogicalSize;
use winit::window::{WindowAttributes, Icon};
use std::any::Any;
use tokio::sync::mpsc;
use tokio::runtime::Builder;
use std::sync::OnceLock;

mod ui;
mod channel;
mod theme;
mod utils;
mod pin;
mod encrypt;
mod decrypt;
mod ws;     
mod wallet; 
mod context;
mod startup; 
#[cfg(target_os = "windows")]
mod icon;
#[cfg(target_os = "linux")]
mod linux_icon_fix;

use crate::theme::{DARK_CSS, LIGHT_CSS};
use crate::ui::enterpin::PinScreen;
use crate::channel::{WSCommand}; 
use crate::ws::{run_exchange_websocket, run_crypto_websocket}; 
use crate::context::GlobalContext;
use crate::ui::update::UpdatePrompt; 
use crate::startup::init_startup;
#[cfg(target_os = "windows")]
use crate::icon::load_icon;
#[cfg(target_os = "linux")]
use crate::linux_icon_fix::apply_linux_icon_and_desktop_entry;

static UI_COMMANDS_TX: OnceLock<mpsc::Sender<WSCommand>> = OnceLock::new();

#[derive(Clone, Copy, PartialEq, Eq)]
enum AppState {
    PinEntry,
    Dashboard,
    UpdatePrompt,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    unsafe { std::env::set_var("WGPU_BACKEND", "metal"); }

    #[cfg(target_os = "windows")]
    unsafe { std::env::set_var("WGPU_BACKEND", "vulkan"); }

    #[cfg(target_os = "linux")]
    unsafe { std::env::set_var("WGPU_BACKEND", "vulkan"); }

    println!("Starting main - before init_startup");
    // We don't call init_startup() here anymore because it requires the runtime handle
    
    // LINUX ICON FIX — ONLY RUNS ON LINUX
    #[cfg(target_os = "linux")]
    apply_linux_icon_and_desktop_entry();

    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;

    let handle = runtime.handle().clone();
    
    // NOW we call init_startup with the handle so it runs in the background
    init_startup(&handle);
    println!("init_startup (async) triggered");

    let (commands_tx, commands_rx) = mpsc::channel::<WSCommand>(100);
    let (exchange_shutdown_tx, exchange_shutdown_rx) = mpsc::channel::<()>(1);
    let (crypto_shutdown_tx, crypto_shutdown_rx) = mpsc::channel::<()>(1);

    let _ = UI_COMMANDS_TX.set(commands_tx.clone());
    println!("UI_COMMANDS_TX set");

    let mut join_handles: Vec<tokio::task::JoinHandle<()>> = vec![];

    // Always spawn websockets so the app launches regardless of connection
    println!("Spawning exchange websocket");
    let exchange_handle = handle.spawn(async move {
        println!("Exchange websocket task started");
        if let Err(_e) = run_exchange_websocket(exchange_shutdown_rx).await {
            println!("Exchange websocket error: {:?}", _e);
        }
        println!("Exchange websocket task ended");
    });
    join_handles.push(exchange_handle);

    let commands_rx_clone = commands_rx;
    println!("Spawning crypto websocket");
    let crypto_handle = handle.spawn(async move {
        println!("Crypto websocket task started");
        if let Err(_e) = run_crypto_websocket(commands_rx_clone, crypto_shutdown_rx).await {
            println!("Crypto websocket error: {:?}", _e);
        }
        println!("Crypto websocket task ended");
    });
    join_handles.push(crypto_handle);
    
    let tx_clone = commands_tx.clone();
    println!("Spawning wallet load");
    let wallet_handle = handle.spawn_blocking(move || {
        println!("Wallet load task started");
        wallet::load_wallets(tx_clone); 
        println!("Wallet load task ended");
    });
    join_handles.push(wallet_handle);

    #[cfg(not(target_os = "linux"))]
    let icon_data = load_icon()?;
    #[cfg(not(target_os = "linux"))]
    let window_icon = Some(Icon::from_rgba(icon_data.rgba, icon_data.width, icon_data.height)?);
    #[cfg(target_os = "linux")]
    let window_icon: Option<Icon> = None;

    #[cfg(target_os = "windows")]
    let default_size = LogicalSize::new(960.0, 720.0);
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let default_size = LogicalSize::new(1200.0, 900.0);

    let window_attr = WindowAttributes::default()
        .with_title("Dannesk")
        .with_inner_size(default_size)
        .with_resizable(true)
        .with_window_icon(window_icon);

    #[cfg(target_os = "linux")]
    {
        let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
        if session_type == "wayland" {
            use winit::platform::wayland::WindowAttributesExtWayland;
            window_attr = window_attr.with_name("dannesk", "dannesk");
        } else {
            use winit::platform::x11::WindowAttributesExtX11;
            window_attr = window_attr.with_name("dannesk", "dannesk");
        }
    }

    println!("Launching Dioxus app");
    dioxus_native::launch_cfg(App, vec![], vec![Box::new(window_attr) as Box<dyn Any>]);
    println!("Dioxus app exited");

    handle.block_on(async {
        println!("Sending websocket shutdown signals.");
        let _ = exchange_shutdown_tx.send(()).await;
        let _ = crypto_shutdown_tx.send(()).await;
        for jh in join_handles {
            let _ = jh.await;
        }
        println!("All tasks completed.");
    });

    Ok(())
}

#[component]
fn App() -> Element {
    let tx = UI_COMMANDS_TX.get().expect("UI_COMMANDS_TX not set").clone();
    context::setup_contexts(tx);

    let global = use_context::<GlobalContext>();
    let is_dark = global.theme_user.read().0;
    
    // Track if the user has successfully unlocked the PIN
    let mut unlocked = use_signal(|| false);
    
    // Read the version signal directly. This ensures that if the background
    // fetch finishes 2 seconds after launch, this component re-renders.
    let remote_version = global.version.read();
    
    // Determine the current view
    let current_view = match remote_version.as_ref() {
        // FORCE update if version exists and doesn't match
        Some(v) if v != VERSION => AppState::UpdatePrompt,
        // Otherwise, check if we are in Dashboard or PinEntry
        _ => if *unlocked.read() { AppState::Dashboard } else { AppState::PinEntry },
    };

    let theme_css = if is_dark { DARK_CSS } else { LIGHT_CSS };

    let zoom = if cfg!(target_os = "windows") { "zoom: 0.8;" } else { "" };

    rsx! {
        style { "body {{ margin: 0; padding: 0; }} {theme_css}" }
        // 1. OUTER WRAPPER: Always 100vh, holds the background colors/classes.
        div {
            class: "theme-root",
            class: if is_dark { "dark" }, 
            style: "display: flex; flex-direction: column; height: 100vh; width: 100%; overflow: hidden;",

            // 2. INNER CONTENT WRAPPER: This is where the zoom lives.
            // margin: auto ensures the zoomed content fills the parent available space.
            div {
                style: "display: flex; flex-direction: column; flex: 1; width: 100%; margin: auto; {zoom}",


            match current_view {
                AppState::UpdatePrompt => rsx! { UpdatePrompt {} },
                AppState::PinEntry => rsx! {
                    PinScreen { on_unlock: move |_| unlocked.set(true) }
                },
                AppState::Dashboard => rsx! {
                    ui::dashboard::render_dashboard {}
                }
            }
        }
        }
    }
}