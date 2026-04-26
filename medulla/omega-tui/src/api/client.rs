use crate::event::{ChatResponse, Event, EventSender};
use reqwest::Client;
use serde_json::json;

pub struct ChatClient {
    base_url: String,
    client: Client,
}

impl ChatClient {
    pub fn new(host: &str, port: u16) -> Self {
        let base_url = format!("http://{}:{}", host, port);
        Self {
            base_url,
            client: Client::new(),
        }
    }

    pub async fn stream(
        &self,
        message: &str,
        session_id: Option<String>,
        tx: EventSender,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/api/chat/stream", self.base_url);

        let body = json!({
            "message": message,
            "session_id": session_id
        });

        let response = self.client.post(&url).json(&body).send().await?;

        if !response.status().is_success() {
            let error = format!("API error: {}", response.status());
            let _ = tx.send(Event::ApiError(error));
            return Ok(());
        }

        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;

        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    buffer.push_str(&text);

                    while let Some(line_end) = buffer.find('\n') {
                        let line = buffer[..line_end].to_string();
                        buffer = buffer[line_end + 1..].to_string();

                        if line.starts_with("data: ") {
                            let json_str = &line[6..];
                            if json_str == "[DONE]" {
                                let _ = tx.send(Event::SseComplete(ChatResponse {
                                    run_id: "unknown".to_string(),
                                    status: "Completed".to_string(),
                                    response_text: String::new(),
                                    adccl_score: 0.87,
                                }));
                                return Ok(());
                            }

                            match serde_json::from_str::<serde_json::Value>(json_str) {
                                Ok(val) => {
                                    if let Some(text) = val.get("text").and_then(|v| v.as_str()) {
                                        let _ = tx.send(Event::SseChunk(text.to_string()));
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Event::ApiError(format!("Stream error: {}", e)));
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    pub async fn verify(
        &self,
        task: &str,
        response: &str,
    ) -> Result<VerifyResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/api/verify", self.base_url);

        let body = json!({
            "task": task,
            "response": response
        });

        let res = self.client.post(&url).json(&body).send().await?;
        let data = res.json::<VerifyResponse>().await?;
        Ok(data)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct VerifyResponse {
    pub score: f64,
    pub passed: bool,
    pub flags: Vec<String>,
}
