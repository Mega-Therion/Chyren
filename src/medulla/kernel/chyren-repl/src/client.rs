//! HTTP client for the Chyren API — wraps /api/chat and /api/chat/stream.

use anyhow::{bail, Result};
use bytes::Bytes;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ChatRequest<'a> {
    message: &'a str,
    session_id: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub run_id: String,
    pub status: String,
    pub response_text: String,
    pub adccl_score: f64,
}

/// A single SSE data payload arriving from /api/chat/stream.
#[derive(Deserialize, Debug)]
pub struct StreamChunk {
    /// Partial or complete text fragment from the provider.
    pub text: Option<String>,
    /// Set when the stream is finished.
    pub done: Option<bool>,
    pub adccl_score: Option<f64>,
    pub run_id: Option<String>,
    pub status: Option<String>,
}

pub struct ChyrenClient {
    base_url: String,
    client: Client,
    timeout_secs: u64,
}

impl ChyrenClient {
    pub fn new(base_url: impl Into<String>, timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .expect("reqwest client init");
        Self {
            base_url: base_url.into(),
            client,
            timeout_secs,
        }
    }

    /// Synchronous chat — waits for full response.
    pub async fn chat(&self, message: &str, session_id: Option<&str>) -> Result<ChatResponse> {
        let url = format!("{}/api/chat", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&ChatRequest { message, session_id })
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("API error {status}: {body}");
        }

        Ok(resp.json::<ChatResponse>().await?)
    }

    /// Streaming chat via SSE — calls `on_chunk` for every text fragment received.
    /// Returns the final ADCCL score.
    pub async fn chat_stream<F>(
        &self,
        message: &str,
        session_id: Option<&str>,
        mut on_chunk: F,
    ) -> Result<f64>
    where
        F: FnMut(&str),
    {
        let url = format!("{}/api/chat/stream", self.base_url);
        let resp = self
            .client
            .post(&url)
            .timeout(std::time::Duration::from_secs(self.timeout_secs))
            .json(&ChatRequest { message, session_id })
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("API stream error {status}: {body}");
        }

        let mut stream = resp.bytes_stream();
        let mut buf = String::new();
        let mut final_score = 0.0_f64;

        while let Some(chunk) = stream.next().await {
            let bytes: Bytes = chunk?;
            buf.push_str(&String::from_utf8_lossy(&bytes));

            // SSE frames are delimited by \n\n
            while let Some(pos) = buf.find("\n\n") {
                let frame = buf[..pos].to_string();
                buf = buf[pos + 2..].to_string();

                for line in frame.lines() {
                    if let Some(data) = line.strip_prefix("data: ") {
                        if let Ok(chunk) = serde_json::from_str::<StreamChunk>(data) {
                            if let Some(text) = &chunk.text {
                                on_chunk(text);
                            }
                            if let Some(score) = chunk.adccl_score {
                                final_score = score;
                            }
                        }
                    }
                }
            }
        }

        Ok(final_score)
    }

    /// Check if the API server is reachable.
    pub async fn health_check(&self) -> bool {
        let url = format!("{}/api/chat", self.base_url);
        self.client
            .post(&url)
            .json(&ChatRequest {
                message: "ping",
                session_id: None,
            })
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .is_ok()
    }
}
