mod app;
mod api;
mod event;
mod input;
mod theme;
mod ui;

use app::{AppMode, AppState, Tab};
use api::{ChatClient, TelemetrySocket};
use crossterm::event::{self as ct_event, DisableMouseCapture, EnableMouseCapture, Event as CrosstermEvent, KeyCode, KeyModifiers};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_host = std::env::var("CHYREN_API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let api_port: u16 = std::env::var("CHYREN_API_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()?;
    let telemetry_host = std::env::var("CHYREN_TELEMETRY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let telemetry_port: u16 = std::env::var("CHYREN_TELEMETRY_PORT")
        .unwrap_or_else(|_| "9090".to_string())
        .parse()?;

    setup_terminal()?;
    let res = run(&api_host, api_port, &telemetry_host, telemetry_port).await;
    restore_terminal()?;

    if let Err(err) = res {
        println!("Error: {}", err);
    }

    Ok(())
}

async fn run(
    api_host: &str,
    api_port: u16,
    telemetry_host: &str,
    telemetry_port: u16,
) -> Result<(), Box<dyn Error>> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut state = AppState::new();

    spawn_terminal_reader(tx.clone());
    spawn_tick_timer(tx.clone());

    let chat_client = ChatClient::new(api_host, api_port);
    let telemetry_url = format!("ws://{}:{}/ws", telemetry_host, telemetry_port);
    spawn_telemetry_listener(telemetry_url, tx.clone());

    loop {
        terminal.draw(|f| ui::draw(f, &state))?;

        if let Some(app_evt) = rx.recv().await {
            match app_evt {
                event::Event::Key(key) => {
                    let chat_client_clone = ChatClient::new(api_host, api_port);
                    handle_key(&mut state, key, &chat_client_clone, tx.clone());
                }
                event::Event::Resize(_, _) => {
                    // Terminal handles this automatically
                }
                event::Event::SseChunk(chunk) => {
                    state.chat.add_stream_chunk(chunk);
                }
                event::Event::SseComplete(resp) => {
                    state.chat.adccl_score = resp.adccl_score;
                    state.chat.finish_streaming();
                }
                event::Event::TelemetryWs(evt) => {
                    state.telemetry.add_event(evt);
                }
                event::Event::ApiError(err) => {
                    state.chat.add_message(app::MessageRole::Chyren, format!("⚠ Error: {}", err));
                }
                event::Event::Connected => {
                    state.status.connected = true;
                }
                event::Event::Disconnected => {
                    state.status.connected = false;
                }
                event::Event::Tick => {
                    // Render happens in main loop
                }
            }
        }
    }
}

fn handle_key(
    state: &mut AppState,
    key: crossterm::event::KeyEvent,
    chat_client: &ChatClient,
    tx: mpsc::UnboundedSender<event::Event>,
) {
    match state.mode {
        AppMode::Normal => handle_normal_mode(state, key, tx),
        AppMode::Insert => handle_insert_mode(state, key, chat_client, tx),
    }
}

fn handle_normal_mode(
    state: &mut AppState,
    key: crossterm::event::KeyEvent,
    _tx: mpsc::UnboundedSender<event::Event>,
) {
    match key.code {
        KeyCode::Char('q') => std::process::exit(0),
        KeyCode::Char('i') => state.set_mode(AppMode::Insert),
        KeyCode::Char('1') => state.switch_tab(Tab::Chat),
        KeyCode::Char('2') => state.switch_tab(Tab::Mesh),
        KeyCode::Char('3') => state.switch_tab(Tab::Telemetry),
        KeyCode::Char('4') => state.switch_tab(Tab::Dream),
        KeyCode::Tab => state.next_tab(),
        KeyCode::Char('p') if key.modifiers == KeyModifiers::CONTROL => {
            state.show_command_palette = !state.show_command_palette;
        }
        KeyCode::F(1) => state.show_help = !state.show_help,
        KeyCode::Char('?') => state.show_help = !state.show_help,
        KeyCode::Char('l') if key.modifiers == KeyModifiers::CONTROL => {
            state.chat.clear();
        }
        _ => {}
    }
}

fn handle_insert_mode(
    state: &mut AppState,
    key: crossterm::event::KeyEvent,
    chat_client: &ChatClient,
    tx: mpsc::UnboundedSender<event::Event>,
) {
    match key.code {
        KeyCode::Esc => state.set_mode(AppMode::Normal),
        KeyCode::Char(c) => {
            state.input.insert_char(c);
        }
        KeyCode::Backspace => state.input.backspace(),
        KeyCode::Delete => state.input.delete(),
        KeyCode::Left => state.input.move_left(),
        KeyCode::Right => state.input.move_right(),
        KeyCode::Home => state.input.move_home(),
        KeyCode::End => state.input.move_end(),
        KeyCode::Up => state.input.prev_history(),
        KeyCode::Down => state.input.next_history(),
        KeyCode::Enter => {
            if key.modifiers == KeyModifiers::SHIFT {
                state.input.insert_char('\n');
            } else {
                let msg = state.input.submit();
                if !msg.is_empty() {
                    state.chat.add_message(app::MessageRole::User, msg.clone());
                    state.chat.start_streaming();

                    let client_host = "127.0.0.1".to_string();
                    let client_port = 8080u16;
                    let tx_clone = tx.clone();
                    let msg_clone = msg.clone();
                    tokio::spawn(async move {
                        let chat_client = ChatClient::new(&client_host, client_port);
                        let _ = chat_client.stream(&msg_clone, None, tx_clone).await;
                    });
                }
                state.set_mode(AppMode::Normal);
            }
        }
        KeyCode::Char('a') if key.modifiers == KeyModifiers::CONTROL => state.input.move_home(),
        KeyCode::Char('e') if key.modifiers == KeyModifiers::CONTROL => state.input.move_end(),
        _ => {}
    }
}

fn spawn_terminal_reader(tx: mpsc::UnboundedSender<event::Event>) {
    tokio::spawn(async move {
        use futures::StreamExt;
        let mut reader = ct_event::EventStream::new();

        while let Some(evt) = reader.next().await {
            if let Ok(CrosstermEvent::Key(key)) = evt {
                let _ = tx.send(event::Event::Key(key));
            } else if let Ok(CrosstermEvent::Resize(w, h)) = evt {
                let _ = tx.send(event::Event::Resize(w, h));
            }
        }
    });
}

fn spawn_tick_timer(tx: mpsc::UnboundedSender<event::Event>) {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_millis(16)); // ~60 fps

        loop {
            interval.tick().await;
            let _ = tx.send(event::Event::Tick);
        }
    });
}

fn spawn_telemetry_listener(url: String, tx: mpsc::UnboundedSender<event::Event>) {
    tokio::spawn(async move {
        let _ = TelemetrySocket::connect_and_listen(&url, tx).await;
    });
}

fn setup_terminal() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(())
}

fn restore_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
