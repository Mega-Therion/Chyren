//! Telegram gateway bridge: routes bot messages through the OmegA conductor pipeline.

pub mod aegis;

use aegis::check_message;
use omega_cli::conductor::Conductor;
use omega_core::{gen_id, now, EvidencePacket, RunEnvelope, RunStatus};
use std::sync::Arc;
use teloxide::prelude::*;

/// Start the Telegram gateway bridge using the provided bot token.
///
/// Initialises a `Conductor`, bootstraps identity, then enters the teloxide
/// event loop. Every incoming message is:
///   1. Screened by the AEGIS gate
///   2. Planned by the conductor (alignment check)
///   3. Executed through the full pipeline (provider → ADCCL → ledger)
///   4. Replied to with the verified response
pub async fn run_bridge(token: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conductor = Arc::new(Conductor::new());
    conductor.bootstrap_identity().await.ok();

    let bot = Bot::new(token);
    println!("[TELEGRAM] Bridge operational.");

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let conductor = conductor.clone();
        async move {
            let Some(text) = msg.text() else {
                return respond(());
            };

            // 1. AEGIS gate.
            let aegis = check_message(text);
            if !aegis.passed {
                bot.send_message(
                    msg.chat.id,
                    format!("⛔ [{:?}] {}", aegis.threat_level, aegis.note),
                )
                .await
                .ok();
                return respond(());
            }

            // 2. Alignment + planning.
            let plan = match conductor.plan_task(text).await {
                Ok(p) => p,
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("⚠️ Rejected: {e}"))
                        .await
                        .ok();
                    return respond(());
                }
            };

            // 3. Execute through full pipeline.
            let mut envelope = RunEnvelope {
                task_id: gen_id("task"),
                run_id: gen_id("run"),
                task: text.to_string(),
                task_text: text.to_string(),
                created_at: now(),
                status: RunStatus::Admitted,
                risk_score: 0.0,
                verified_payload: None,
                evidence_packet: EvidencePacket::default(),
            };

            match conductor.execute_plan(plan, &mut envelope).await {
                Ok(result) => {
                    let suffix = result
                        .verification
                        .map(|v| format!("\n\n_[ADCCL: {:.2}]_", v.score))
                        .unwrap_or_default();
                    bot.send_message(msg.chat.id, format!("{}{}", result.response_text, suffix))
                        .await
                        .ok();
                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("⚠️ {e}")).await.ok();
                }
            }

            respond(())
        }
    })
    .await;

    Ok(())
}
