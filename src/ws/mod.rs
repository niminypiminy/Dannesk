// src/ws/mod.rs
pub mod exchangesocket;
pub mod websocket;
pub mod connection;
pub mod config;
pub mod commands;

pub use exchangesocket::run_exchange_websocket;
pub use websocket::run_crypto_websocket;