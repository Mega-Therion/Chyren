pub mod aegis;
use teloxide::prelude::*;
use crate::aegis::check_message;

#[tokio::main]
pub async fn run_bridge(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let bot = Bot::new(token);

    println!("[TELEGRAM] Bridge operational.");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            // Apply AEGIS gate
            let aegis_result = check_message(text);
            if !aegis_result.passed {
                bot.send_message(msg.chat.id, format!("⛔ System Blocked: {}", aegis_result.note)).await.ok();
                return respond(());
            }

            // Route to OmegA Hub logic here
            bot.send_message(msg.chat.id, format!("Received: {}", text)).await.ok();
        }
        respond(())
    }).await;

    Ok(())
}
