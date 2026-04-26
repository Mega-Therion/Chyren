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
        _session_id: Option<String>,
        tx: EventSender,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/api/chat", self.base_url);

        let body = json!({
            "message": message
        });

        match self.client.post(&url).json(&body).send().await {
            Ok(response) => {
                if !response.status().is_success() {
                    let error = format!("API error: {}", response.status());
                    let _ = tx.send(Event::ApiError(error));
                    return Ok(());
                }

                match response.json::<ChatResponse>().await {
                    Ok(resp) => {
                        let _ = tx.send(Event::SseChunk(resp.response_text.clone()));
                        let _ = tx.send(Event::SseComplete(resp));
                        Ok(())
                    }
                    Err(e) => {
                        let _ = tx.send(Event::ApiError(format!("Parse error: {}", e)));
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(Event::ApiError(format!("Request error: {}", e)));
                Err(Box::new(e))
            }
        }
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
