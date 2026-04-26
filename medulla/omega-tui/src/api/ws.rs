use crate::event::{Event, EventSender, SystemEvent};
use futures::StreamExt;
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::connect_async;

pub struct TelemetrySocket;

impl TelemetrySocket {
    pub async fn connect_and_listen(url: &str, tx: EventSender) {
        let mut backoff = Duration::from_secs(2);
        let max_backoff = Duration::from_secs(30);

        loop {
            match Self::try_connect(&url, tx.clone()).await {
                Ok(_) => {
                    let _ = tx.send(Event::Connected);
                    backoff = Duration::from_secs(2);
                }
                Err(e) => {
                    let error_msg = format!("WS error: {}", e);
                    let _ = tx.send(Event::ApiError(error_msg));
                    sleep(backoff).await;
                    backoff = backoff.mul_f32(1.5).min(max_backoff);
                }
            }
        }
    }

    async fn try_connect(url: &str, tx: EventSender) -> Result<(), String> {
        let (ws_stream, _) = connect_async(url).await.map_err(|e| e.to_string())?;
        let (_, mut read) = ws_stream.split();

        while let Some(msg) = read.next().await {
            match msg {
                Ok(tungstenite::Message::Text(text)) => {
                    match serde_json::from_str::<SystemEvent>(&text) {
                        Ok(event) => {
                            let _ = tx.send(Event::TelemetryWs(event));
                        }
                        Err(_) => {}
                    }
                }
                Ok(tungstenite::Message::Close(_)) => {
                    return Ok(());
                }
                Err(e) => {
                    return Err(e.to_string());
                }
                _ => {}
            }
        }

        Ok(())
    }
}
