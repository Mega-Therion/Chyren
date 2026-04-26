use crate::event::{Event, EventSender, MeshAgent, StatusSnapshot};
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

pub fn spawn_status_poller(host: String, port: u16, tx: EventSender) {
    tokio::spawn(async move {
        let client = Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_else(|_| Client::new());
        let url = format!("http://{}:{}/api/status", host, port);

        loop {
            let snap = fetch_status(&client, &url).await;
            let _ = tx.send(Event::StatusRefresh(snap.clone()));
            if snap.api_reachable {
                let _ = tx.send(Event::Connected);
            } else {
                let _ = tx.send(Event::Disconnected);
            }
            sleep(Duration::from_secs(5)).await;
        }
    });
}

pub fn spawn_mesh_poller(host: String, port: u16, tx: EventSender) {
    tokio::spawn(async move {
        let client = Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_else(|_| Client::new());
        let url = format!("http://{}:{}/api/mesh", host, port);

        loop {
            if let Some(agents) = fetch_mesh(&client, &url).await {
                let _ = tx.send(Event::MeshRefresh(agents));
            }
            sleep(Duration::from_secs(8)).await;
        }
    });
}

async fn fetch_status(client: &Client, url: &str) -> StatusSnapshot {
    let mut snap = StatusSnapshot::default();
    match client.get(url).send().await {
        Ok(resp) if resp.status().is_success() => {
            snap.api_reachable = true;
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                snap.provider = json
                    .get("provider")
                    .and_then(|v| v.as_str())
                    .unwrap_or("openrouter")
                    .to_string();
                snap.adccl_score = json.get("adccl_score").and_then(|v| v.as_f64()).unwrap_or(0.0);
                snap.active_runs = json.get("active_runs").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                snap.total_runs = json.get("total_runs").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                snap.dream_episodes = json.get("dream_episodes").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                snap.latency_ms = json.get("latency_ms").and_then(|v| v.as_f64()).unwrap_or(0.0);
                snap.chi = json.get("chi").and_then(|v| v.as_f64()).unwrap_or(0.0);
                snap.omega = json.get("omega").and_then(|v| v.as_f64()).unwrap_or(0.0);
            }
        }
        _ => {
            snap.api_reachable = false;
        }
    }
    snap
}

async fn fetch_mesh(client: &Client, url: &str) -> Option<Vec<MeshAgent>> {
    let resp = client.get(url).send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.json::<Vec<MeshAgent>>().await.ok()
}
