use tokio_tungstenite::{connect_async, tungstenite::{Message, protocol::{CloseFrame, frame::coding::CloseCode}}, WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;
use futures_util::{StreamExt, SinkExt};
use crate::ws::config::{WEBSOCKET_URL, MAX_RECONNECT_ATTEMPTS, RECONNECT_BACKOFF_SECONDS};
use tokio::time::{timeout, Duration};
use serde_json::Value;
use crate::channel::{CHANNEL};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WsMessage {
    payload: Value,
    auth_token: String,  // Simple shared secret, e.g., "dannesk"
}

const AUTH_TOKEN: &str = "dannesk";  // Hardcode or load from env/config if needed

pub struct ConnectionManager {
    ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    // Removed startup_rx since no longer needed
}

impl ConnectionManager {
    pub fn new() -> Self {  // Simplified constructor
        Self {
            ws_stream: None,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.ws_stream.is_some()
    }

    pub async fn connect(&mut self) -> Result<(), String> {
        let ws_status_tx = CHANNEL.crypto_ws_status_tx.clone();
        let mut attempts = 0;

        while attempts < MAX_RECONNECT_ATTEMPTS {
            let _ = ws_status_tx.send(false); // Disconnected
            match connect_async(WEBSOCKET_URL).await {
                Ok((stream, _)) => {
                    self.ws_stream = Some(stream);
                    let _ = ws_status_tx.send(true); // Connected

                    // Optional: Send initial auth message on connect to prove legitimacy early
                    // (Backend can close immediately if invalid)
                    let initial_payload = serde_json::json!({ "type": "auth_init" });
                    if let Err(e) = self.send(Message::Text(initial_payload.to_string())).await {
                        return Err(format!("Failed to send initial auth: {}", e));
                    }

                    return Ok(());
                }
                Err(_) => {
                    attempts += 1;
                    let _ = ws_status_tx.send(false); // Disconnected, triggers banner
                    tokio::time::sleep(Duration::from_secs(attempts as u64)).await;
                }
            }
        }

        let _ = ws_status_tx.send(false); // Disconnected, triggers banner
        tokio::time::sleep(Duration::from_secs(RECONNECT_BACKOFF_SECONDS)).await;
        Err("Failed to connect after max attempts".to_string())
    }

    pub async fn send(&mut self, message: Message) -> Result<(), String> {
        if let Some(ref mut ws) = self.ws_stream {
            let message_to_send = match message {
                Message::Text(payload) => {
                    let payload_json: Value = serde_json::from_str(&payload)
                        .map_err(|e| format!("Invalid JSON payload: {}", e))?;

                    let ws_message = WsMessage {
                        payload: payload_json,
                        auth_token: AUTH_TOKEN.to_string(),
                    };

                    let serialized = serde_json::to_string(&ws_message)
                        .map_err(|e| format!("Failed to serialize WsMessage: {}", e))?;
                    Message::Text(serialized)
                }
                Message::Ping(data) => Message::Ping(data),
                Message::Pong(data) => Message::Pong(data),
                Message::Close(frame) => Message::Close(frame),
                _ => {
                    return Err("Unsupported message type".to_string());
                }
            };

            ws.send(message_to_send)
                .await
                .map_err(|e| format!("Failed to send message: {}", e))?;
            Ok(())
        } else {
            Err("WebSocket not connected".to_string())
        }
    }

    pub async fn next_message(&mut self) -> Option<Result<Message, String>> {
        let ws_status_tx = CHANNEL.crypto_ws_status_tx.clone();
        if let Some(ref mut ws) = self.ws_stream {
            match timeout(Duration::from_secs(90), ws.next()).await {
                Ok(Some(Ok(Message::Ping(data)))) => {
                    let data_clone = data.clone();
                    if let Err(_e) = ws.send(Message::Pong(data)).await {
                        return Some(Err(format!("Failed to send pong: {}", _e)));
                    }
                    Some(Ok(Message::Pong(data_clone)))
                }
                Ok(Some(Ok(Message::Pong(data)))) => {
                    Some(Ok(Message::Pong(data)))
                }
                Ok(Some(Ok(Message::Close(_frame)))) => {
                    if let Err(_e) = ws.close(None).await {
                        // Ignore close error
                    }
                    self.ws_stream = None;
                    let _ = ws_status_tx.send(false); // Disconnected, no banner unless reconnection fails
                    Some(Err("WebSocket closed by server".to_string()))
                }
                Ok(Some(Ok(Message::Text(text)))) => {
                    match serde_json::from_str::<WsMessage>(&text) {
                        Ok(ws_message) => {
                            // Optional: Verify auth_token here if you want client-side checks (but backend should handle)
                            if ws_message.auth_token != AUTH_TOKEN {
                                return Some(Err("Invalid auth_token received".to_string()));
                            }
                            let payload_text = match serde_json::to_string(&ws_message.payload) {
                                Ok(text) => text,
                                Err(e) => {
                                    return Some(Err(format!("Failed to serialize payload: {}", e)));
                                }
                            };
                            Some(Ok(Message::Text(payload_text)))
                        }
                        Err(_) => {
                            if serde_json::from_str::<Value>(&text).is_ok() {
                                Some(Ok(Message::Text(text)))
                            } else {
                                return Some(Err("Invalid JSON message".to_string()));
                            }
                        }
                    }
                }
                Ok(Some(Ok(message))) => {
                    Some(Ok(message))
                }
                Ok(Some(Err(e))) => {
                    if e.to_string().contains("connection reset") || e.to_string().contains("broken pipe") {
                        self.ws_stream = None;
                        let _ = ws_status_tx.send(false); // Disconnected, triggers banner
                        Some(Err(format!("Critical WebSocket error: {}", e)))
                    } else {
                        Some(Err(format!("WebSocket error: {}", e)))
                    }
                }
                Ok(None) => {
                    if let Err(_e) = ws.close(None).await {
                        // Ignore close error
                    }
                    self.ws_stream = None;
                    let _ = ws_status_tx.send(false); 
                    Some(Err("WebSocket stream ended".to_string()))
                }
                Err(_) => {
                    if let Err(_e) = ws.close(None).await {
                        // Ignore close error
                    }
                    self.ws_stream = None;
                    let _ = ws_status_tx.send(false); 
                    Some(Err("WebSocket read timed out".to_string()))
                }
            }
        } else {
            None
        }
    }

    pub async fn close(&mut self) -> Result<(), String> {
        let ws_status_tx = CHANNEL.crypto_ws_status_tx.clone();
        if let Some(ref mut ws) = self.ws_stream {
            let close_frame = CloseFrame {
                code: CloseCode::Normal,
                reason: "Client shutting down".into(),
            };
            if let Err(_e) = ws.send(Message::Close(Some(close_frame))).await {
                // Ignore send error
            }
            if let Err(_e) = ws.close(None).await {
                // Ignore close error
            }
            self.ws_stream = None;
            let _ = ws_status_tx.send(false); // Disconnected, no banner
        }
        Ok(())
    }
}