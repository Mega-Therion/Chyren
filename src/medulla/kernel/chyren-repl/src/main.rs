mod client;
mod config;
mod output;
mod session;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use indicatif::{ProgressBar, ProgressStyle};
use rustyline::{error::ReadlineError, DefaultEditor};
use std::{
    io::stdout,
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::Mutex;

use client::ChyrenClient;
use config::Config;
use output::{print_error, print_info, print_meta, OutputRenderer};
use session::Session;

/// Sovereign REPL — interactive terminal client for the Chyren API server.
#[derive(Parser)]
#[command(name = "chyren-repl", version, about)]
struct Cli {
    /// Chyren API base URL (overrides config).
    #[arg(long, env = "CHYREN_API_URL")]
    api_url: Option<String>,

    /// Disable SSE streaming; use synchronous /api/chat instead.
    #[arg(long)]
    no_stream: bool,

    /// Path to config file (default: ~/.chyren/config.toml).
    #[arg(long)]
    config: Option<String>,
}

const BANNER: &str = r#"
  ██████╗██╗  ██╗██╗   ██╗██████╗ ███████╗███╗   ██╗
 ██╔════╝██║  ██║╚██╗ ██╔╝██╔══██╗██╔════╝████╗  ██║
 ██║     ███████║ ╚████╔╝ ██████╔╝█████╗  ██╔██╗ ██║
 ██║     ██╔══██║  ╚██╔╝  ██╔══██╗██╔══╝  ██║╚██╗██║
 ╚██████╗██║  ██║   ██║   ██║  ██║███████╗██║ ╚████║
  ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝
                     Sovereign REPL
"#;

fn print_banner() {
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print(BANNER),
        ResetColor,
        Print("  Type /help for commands. Ctrl+D to exit.\n\n"),
    );
}

fn prompt_str() -> String {
    "\x1b[36m❯\x1b[0m ".to_string()
}

fn print_help() {
    print_info("Available slash commands:");
    print_info("  /help           — this menu");
    print_info("  /clear          — clear the screen");
    print_info("  /history        — show conversation history");
    print_info("  /config         — open ~/.chyren/config.toml in $EDITOR");
    print_info("  /reset          — clear session history");
    print_info("  /load [file]    — load a saved session JSON");
    print_info("  /tools          — list tools Chyren can call");
    print_info("  /paste          — open $EDITOR for multi-line input");
    print_info("  /exit /quit     — exit the REPL");
    print_info("");
    print_info("Special input syntax:");
    print_info("  @path/to/file   — inject file contents into message");
    print_info("  !!cmd           — run shell command, send output as message");
}

fn open_editor(path: &str) {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let _ = Command::new(&editor).arg(path).status();
}

fn clear_screen() {
    let _ = execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All));
    let _ = execute!(stdout(), crossterm::cursor::MoveTo(0, 0));
}

/// Expand @file syntax — returns the processed input string.
fn expand_at_files(input: &str) -> Result<String> {
    let bytes = input.as_bytes();
    let mut segments: Vec<String> = Vec::new();
    let mut last = 0usize;
    let mut i = 0usize;

    while i < bytes.len() {
        if bytes[i] == b'@' {
            let start = i + 1;
            let end = bytes[start..]
                .iter()
                .position(|&b| b == b' ' || b == b'\n' || b == b'\t')
                .map(|p| start + p)
                .unwrap_or(bytes.len());
            let path = &input[start..end];
            if !path.is_empty() {
                segments.push(input[last..i].to_string());
                match std::fs::read_to_string(path) {
                    Ok(contents) => segments.push(format!("\n--- {path} ---\n{contents}\n---\n")),
                    Err(e) => segments.push(format!("[ERROR reading {path}: {e}]")),
                }
                last = end;
                i = end;
                continue;
            }
        }
        i += 1;
    }
    segments.push(input[last..].to_string());
    Ok(segments.join(""))
}

/// Handle !!cmd syntax — run and return output.
fn run_shell_cmd(cmd: &str) -> String {
    let output = Command::new("sh").arg("-c").arg(cmd).output();
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout).to_string();
            let stderr = String::from_utf8_lossy(&o.stderr).to_string();
            format!("$ {cmd}\n{stdout}{stderr}")
        }
        Err(e) => format!("[shell error: {e}]"),
    }
}

/// Build a temporary file, open in editor, return contents.
fn paste_editor() -> Result<String> {
    let tmp = std::env::temp_dir().join("chyren_paste.txt");
    std::fs::write(&tmp, "")?;
    open_editor(tmp.to_str().unwrap_or("/tmp/chyren_paste.txt"));
    Ok(std::fs::read_to_string(&tmp).unwrap_or_default())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load config
    let mut cfg = Config::load()?;
    if let Some(url) = cli.api_url {
        cfg.api.base_url = url;
    }
    let use_stream = cfg.ui.stream && !cli.no_stream;

    print_banner();

    let client = Arc::new(ChyrenClient::new(&cfg.api.base_url, cfg.api.timeout_secs));
    let history_path = cfg.history_path();
    let session = Arc::new(Mutex::new(Session::new(history_path.clone())));

    // rustyline editor with file-backed input history
    let rl_history = dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".chyren")
        .join("input_history");
    let mut rl = DefaultEditor::new()?;
    let _ = rl.load_history(&rl_history);

    // Cancellation flag for Ctrl+C during streaming
    let cancelled = Arc::new(AtomicBool::new(false));

    let ctrlc_flag = cancelled.clone();
    ctrlc::set_handler(move || {
        ctrlc_flag.store(true, Ordering::SeqCst);
    })
    .ok();

    loop {
        cancelled.store(false, Ordering::SeqCst);

        let readline = rl.readline(&prompt_str());
        match readline {
            Err(ReadlineError::Interrupted) => {
                print_info("(Ctrl+C — use /exit to quit)");
                continue;
            }
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                print_error(&format!("readline error: {e}"));
                break;
            }
            Ok(raw_line) => {
                let line = raw_line.trim().to_string();
                if line.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(&line);

                // --- Slash commands ---
                if line.starts_with('/') {
                    let parts: Vec<&str> = line.splitn(2, ' ').collect();
                    match parts[0] {
                        "/help" => print_help(),
                        "/clear" => clear_screen(),
                        "/history" => {
                            let s = session.lock().await;
                            s.print_history();
                        }
                        "/reset" => {
                            let mut s = session.lock().await;
                            s.clear();
                            print_info("Session history cleared.");
                        }
                        "/config" => {
                            open_editor(
                                config::config_path()
                                    .to_str()
                                    .unwrap_or("~/.chyren/config.toml"),
                            );
                        }
                        "/load" => {
                            let path = parts.get(1).copied().unwrap_or("").trim();
                            if path.is_empty() {
                                print_error("Usage: /load <path>");
                            } else {
                                match session::Session::load_from(&std::path::PathBuf::from(path)) {
                                    Ok(turns) => {
                                        let mut s = session.lock().await;
                                        s.turns = turns;
                                        print_info(&format!(
                                            "Loaded {} turns from {path}",
                                            s.turns.len()
                                        ));
                                    }
                                    Err(e) => print_error(&format!("Failed to load: {e}")),
                                }
                            }
                        }
                        "/tools" => {
                            print_info("Tools available via Chyren conductor:");
                            print_info("  plan_task, execute_plan, verify_text, record_dream");
                            print_info("  (provider-specific tools depend on active spokes)");
                        }
                        "/paste" => match paste_editor() {
                            Ok(text) if !text.trim().is_empty() => {
                                let msg = text.trim().to_string();
                                dispatch_message(
                                    &client,
                                    &session,
                                    &msg,
                                    use_stream,
                                    &cancelled,
                                    cfg.ui.status_bar,
                                )
                                .await;
                            }
                            _ => print_info("(empty paste, skipped)"),
                        },
                        "/exit" | "/quit" => break,
                        other => print_error(&format!("Unknown command: {other}. Try /help")),
                    }
                    continue;
                }

                // --- !! shell command ---
                if let Some(cmd) = line.strip_prefix("!!") {
                    let output = run_shell_cmd(cmd);
                    let _ = execute!(
                        stdout(),
                        SetForegroundColor(Color::DarkGrey),
                        Print(&output),
                        Print("\n"),
                        ResetColor
                    );
                    dispatch_message(
                        &client,
                        &session,
                        &output,
                        use_stream,
                        &cancelled,
                        cfg.ui.status_bar,
                    )
                    .await;
                    continue;
                }

                // --- @file injection ---
                let message = match expand_at_files(&line) {
                    Ok(m) => m,
                    Err(e) => {
                        print_error(&format!("File expand error: {e}"));
                        continue;
                    }
                };

                dispatch_message(
                    &client,
                    &session,
                    &message,
                    use_stream,
                    &cancelled,
                    cfg.ui.status_bar,
                )
                .await;
            }
        }
    }

    let _ = rl.save_history(&rl_history);
    let s = session.lock().await;
    let _ = s.save();
    print_info("Goodbye.");
    Ok(())
}

async fn dispatch_message(
    client: &Arc<ChyrenClient>,
    session: &Arc<Mutex<Session>>,
    message: &str,
    use_stream: bool,
    cancelled: &Arc<AtomicBool>,
    show_meta: bool,
) {
    let session_id = {
        let s = session.lock().await;
        s.id.clone()
    };

    {
        let mut s = session.lock().await;
        s.push_user(message);
    }

    if use_stream {
        // Spinner until first token
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("  {spinner:.cyan} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Synthesizing...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let mut renderer = OutputRenderer::new();
        let mut full_response = String::new();
        let mut got_first = false;

        let cancel_flag = cancelled.clone();
        let result = client
            .chat_stream(message, Some(&session_id), |chunk| {
                if cancel_flag.load(Ordering::SeqCst) {
                    return;
                }
                if !got_first {
                    spinner.finish_and_clear();
                    println!(); // newline after spinner
                    got_first = true;
                }
                full_response.push_str(chunk);
                renderer.feed(chunk);
            })
            .await;

        if !got_first {
            spinner.finish_and_clear();
        }
        renderer.finish();

        match result {
            Ok(score) => {
                if show_meta {
                    print_meta("(stream)", score, true);
                }
                let mut s = session.lock().await;
                s.push_assistant(&full_response, "(stream)", score);
            }
            Err(e) => print_error(&format!("Stream error: {e}")),
        }
    } else {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("  {spinner:.cyan} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.set_message("Synthesizing...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));

        let result = client.chat(message, Some(&session_id)).await;
        spinner.finish_and_clear();

        match result {
            Ok(resp) => {
                let mut renderer = OutputRenderer::new();
                println!();
                renderer.feed(&resp.response_text);
                renderer.finish();
                if show_meta {
                    print_meta(&resp.run_id, resp.adccl_score, false);
                }
                let mut s = session.lock().await;
                s.push_assistant(&resp.response_text, &resp.run_id, resp.adccl_score);
            }
            Err(e) => print_error(&format!("API error: {e}")),
        }
    }
}
