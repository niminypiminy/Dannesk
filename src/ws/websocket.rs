use serde_json::Value;
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::ws::commands::Command;
use crate::channel::{CHANNEL, WSCommand};

pub async fn run_crypto_websocket(
    mut commands_rx: Receiver<WSCommand>,
    mut shutdown_rx: Receiver<()>,
) -> Result<(), String> {
    let mut connection = ConnectionManager::new(CHANNEL.startup_rx.clone());
    let mut current_wallet = String::new();
    let mut bitcoin_current_wallet = String::new();

    loop {
        if !connection.is_connected() {
            connection.connect().await?;
        }

        tokio::select! {
            _ = shutdown_rx.recv() => {
                connection.close().await?;
                break;
            }
            cmd = commands_rx.recv() => {
                match cmd {
                    Some(cmd) => {
                        if let Some(command) = Command::from_str(&cmd.command) {
                            let _ = command
                                .execute(&mut connection, &mut current_wallet, &mut bitcoin_current_wallet, cmd)
                                .await;
                        }
                    }
                    None => {
                        connection.close().await?;
                        break;
                    }
                }
            }
            message = connection.next_message() => {
                match message {
                    Some(Ok(Message::Text(text))) => {
                        let command = if let Ok(data) = serde_json::from_str::<Value>(&text) {
                            data.get("command").and_then(|c| c.as_str()).and_then(Command::from_str)
                        } else {
                            None
                        };
                        if let Some(command) = command {
                            let _ = command
                                .process_response(Message::Text(text), &current_wallet, &bitcoin_current_wallet)
                                .await;
                        }
                    }
                    Some(Ok(_)) => {}
                    Some(Err(_)) => {}
                    None => {
                        connection.close().await?;
                    }
                }
            }
        }
    }

    Ok(())
}