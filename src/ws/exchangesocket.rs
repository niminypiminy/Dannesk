use serde_json::Value;
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};
use crate::channel::CHANNEL;

// Constants aligned with crypto WebSocket
pub const MAX_RECONNECT_ATTEMPTS: u32 = 10;
pub const RECONNECT_BACKOFF_SECONDS: u64 = 60;

pub async fn run_exchange_websocket(mut shutdown_rx: Receiver<()>) -> Result<(), String> {
    let url = "";
    let rates_tx = CHANNEL.rates_tx.clone();
    let ws_status_tx = CHANNEL.exchange_ws_status_tx.clone();

    let mut attempts = 0;

    loop {
        if attempts >= MAX_RECONNECT_ATTEMPTS {
            let _ = ws_status_tx.send(false); // Disconnected
            tokio::time::sleep(tokio::time::Duration::from_secs(RECONNECT_BACKOFF_SECONDS)).await;
            return Err("Failed to connect after max attempts".to_string());
        }

        let _ = ws_status_tx.send(false); // Disconnected

        let ws_stream = match connect_async(url).await {
            Ok((stream, _)) => {
                attempts = 0; // Reset attempts on successful connection
                let _ = ws_status_tx.send(true); // Connected
                stream
            }
            Err(_) => {
                attempts += 1;
                let delay_secs = attempts as u64; // Exponential backoff: 1s, 2s, 3s, ..., 10s
                let _ = ws_status_tx.send(false); // Disconnected
                tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
                continue;
            }
        };

        let (mut ws_sink, mut ws_stream) = ws_stream.split();
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    let _ = ws_sink.close().await;
                    let _ = ws_status_tx.send(false); // Disconnected
                    return Ok::<(), String>(());
                }
                result = ws_stream.next() => {
                    match result {
                        Some(Ok(Message::Text(text))) => {
                            match serde_json::from_str::<Value>(&text) {
                                Ok(data) => {
                                    if let (Some(symbol), Some(price)) = (
                                        data.get("symbol").and_then(|v| v.as_str()),
                                        data.get("price").and_then(|v| v.as_str()),
                                    ) {
                                        if let Some(pair) = binance_stream_to_pair(symbol) {
                                            if let Ok(rate) = price.parse::<f32>() {
                                                let mut new_rates = rates_tx.borrow().clone();
                                                new_rates.insert(pair, rate);
                                                let _ = rates_tx.send(new_rates);
                                            }
                                        }
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                        Some(Ok(Message::Ping(data))) => {
                            let _ = ws_sink.send(Message::Pong(data)).await;
                        }
                        Some(Ok(Message::Close(_))) => {
                            let _ = ws_sink.close().await;
                            let _ = ws_status_tx.send(false); // Disconnected
                            break;
                        }
                        Some(Err(_)) => {
                            let _ = ws_status_tx.send(false); // Disconnected
                            break;
                        }
                        None => {
                            let _ = ws_status_tx.send(false); // Disconnected
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        // After disconnection, increment attempts and apply backoff
        attempts += 1;
        let delay_secs = attempts as u64; // Exponential backoff: 1s, 2s, 3s, ..., 10s
        tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
    }
}

fn binance_stream_to_pair(stream: &str) -> Option<String> {
    match stream {
        "xrpusdt@ticker" => Some("XRP/USD".to_string()),
        "btcusdt@ticker" => Some("BTC/USD".to_string()),
        "eurusdt@ticker" => Some("EUR/USD".to_string()),
        "xrpeur@ticker" => Some("XRP/EUR".to_string()),
        "btceur@ticker" => Some("BTC/EUR".to_string()),
        _ => None,
    }
}