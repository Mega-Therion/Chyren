use std::process;

#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| {
        eprintln!("[TELEGRAM] TELEGRAM_BOT_TOKEN is not set. Exiting.");
        process::exit(1);
    });

    println!("[TELEGRAM] Starting Chyren Telegram Gateway...");

    if let Err(e) = omega_telegram_gateway::run_bridge(token).await {
        eprintln!("[TELEGRAM] Fatal error: {e}");
        process::exit(1);
    }
}
