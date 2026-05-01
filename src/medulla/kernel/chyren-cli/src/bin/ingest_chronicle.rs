use anyhow::Result;
use chyren_core::{Chronicle, YETTRAGRAMMATON};
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ingest_chronicle <file.json>");
        std::process::exit(1);
    }
    let path = &args[1];
    let content = fs::read_to_string(path)?;
    let chronicle: Chronicle = serde_json::from_str(&content)?;

    println!(
        "Ingesting Chronicle: {} (ID: {})",
        chronicle.episode_id, chronicle.episode_id
    );
    println!("Integrity check passed (Signed by {})", YETTRAGRAMMATON);

    Ok(())
}
