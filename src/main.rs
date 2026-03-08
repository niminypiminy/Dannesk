//#![cfg_attr(windows, windows_subsystem = "windows")]

const VERSION: &str = "0.3.0";


use dioxus_native::prelude::*;
use winit::dpi::LogicalSize;
use winit::window::WindowAttributes;
use std::any::Any;
use winit_core::icon::Icon;
use tokio::sync::mpsc;
use tokio::runtime::Builder;
use std::sync::OnceLock;
#[cfg(target_os = "windows")]
use winit::icon::{RgbaIcon};


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

use crate::theme::{DARK_CSS, LIGHT_CSS};
use crate::ui::enterpin::PinScreen;
use crate::channel::{WSCommand}; 
use crate::ws::{run_exchange_websocket, run_crypto_websocket}; 
use crate::context::GlobalContext;
use crate::ui::update::UpdatePrompt; 
use crate::startup::init_startup;
#[cfg(target_os = "windows")]
use crate::icon::load_icon;


static UI_COMMANDS_TX: OnceLock<mpsc::Sender<WSCommand>> = OnceLock::new();

#[derive(Clone, Copy, PartialEq, Eq)]
enum AppState {
    PinEntry,
    Dashboard,
    UpdatePrompt,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
//better performance with vulkan drivers. Consistency in rendering. 

    #[cfg(target_os = "windows")]
    unsafe { std::env::set_var("WGPU_BACKEND", "vulkan"); }

    #[cfg(target_os = "linux")]
    unsafe { std::env::set_var("WGPU_BACKEND", "vulkan"); }
    
    startup::init_globals();

    println!("Starting main - before init_startup");

    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;

    let handle = runtime.handle().clone();
    
    
    init_startup(&handle);

    let (commands_tx, commands_rx) = mpsc::channel::<WSCommand>(100);
    let (exchange_shutdown_tx, exchange_shutdown_rx) = mpsc::channel::<()>(1);
    let (crypto_shutdown_tx, crypto_shutdown_rx) = mpsc::channel::<()>(1);

    let _ = UI_COMMANDS_TX.set(commands_tx.clone());

    let mut join_handles: Vec<tokio::task::JoinHandle<()>> = vec![];

    let exchange_handle = handle.spawn(async move {
        if let Err(_e) = run_exchange_websocket(exchange_shutdown_rx).await {
            println!("Exchange websocket error: {:?}", _e);
        }
    });
    join_handles.push(exchange_handle);

    let crypto_handle = handle.spawn(async move {
        if let Err(_e) = run_crypto_websocket(commands_rx, crypto_shutdown_rx).await {
            
        }
    });
    join_handles.push(crypto_handle);
    
    let tx_clone = commands_tx.clone();
    let wallet_handle = handle.spawn_blocking(move || {
        wallet::load_wallets(tx_clone); 
    });
    join_handles.push(wallet_handle);

    
    #[cfg(target_os = "windows")]
let window_icon = {
    let icon_data = load_icon()?;
    let rgba_icon = RgbaIcon::new(icon_data.rgba, icon_data.width, icon_data.height)?;
    Some(Icon::from(rgba_icon))
};

    #[cfg(target_os = "linux")]
    let window_icon: Option<Icon> = None;

    #[cfg(target_os = "windows")]
    let default_size = LogicalSize::new(960.0, 720.0);
    #[cfg(target_os = "linux")]
    let default_size = LogicalSize::new(1100.0, 800.0);

       #[cfg(target_os = "linux")]  //linux needs mut
    let mut window_attr = WindowAttributes::default()
        .with_title("Dannesk")
        .with_surface_size(default_size)
        .with_resizable(true)
        .with_window_icon(window_icon);

    #[cfg(target_os = "windows")] //windows doesn't need mut
    let window_attr = WindowAttributes::default()
        .with_title("Dannesk")
        .with_surface_size(default_size)
        .with_resizable(true)
        .with_window_icon(window_icon);


   #[cfg(target_os = "linux")]
{
    use winit_core::window::PlatformWindowAttributes; 
    use winit_wayland::WindowAttributesWayland;
    use winit_x11::WindowAttributesX11;

    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();

    let platform_attr: Box<dyn PlatformWindowAttributes> = if session_type == "wayland" {
        // Wayland
        Box::new(WindowAttributesWayland::default().with_name("dannesk", "dannesk"))
    } else {
        // X11
        Box::new(WindowAttributesX11::default().with_name("Dannesk", "dannesk"))
    };

    window_attr = window_attr.with_platform_attributes(platform_attr);
}
    println!("Launching Dioxus app");
    dioxus_native::launch_cfg(App, vec![], vec![Box::new(window_attr) as Box<dyn Any>]);
    println!("Dioxus app exited");

    // shutdown logic
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
    
    // Track if the user has successfully unlocked pin
    let mut unlocked = use_signal(|| false);
    
    // Read the version signal directly.
    let remote_version = global.version.read();
    
  
    let current_view = match remote_version.as_ref() {
        // FORCE update if version exists and doesn't match
        Some(v) if v != VERSION => AppState::UpdatePrompt,
        
        _ => if *unlocked.read() { AppState::Dashboard } else { AppState::PinEntry },
    };

    let theme_css = if is_dark { DARK_CSS } else { LIGHT_CSS };

    rsx! {
        style { "body {{ margin: 0; padding: 0; }} {theme_css}" }
        
        div {
            class: "theme-root",
            class: if is_dark { "dark" }, 
            style: "display: flex; flex-direction: column; height: 100vh; width: 100%; overflow: hidden;",

            

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
