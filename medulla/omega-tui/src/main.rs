use omega_tui::app::{AppMode, AppState, MessageRole, Tab};
use omega_tui::api;
use omega_tui::event::Event;
use omega_tui::proc::ProcessManager;
use omega_tui::router::{RouteOutcome, Router};
use omega_tui::ui;
use crossterm::event::{self as ct_event, DisableMouseCapture, EnableMouseCapture, Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_host = std::env::var("CHYREN_API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let api_port: u16 = std::env::var("CHYREN_API_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()?;
    let telemetry_host = std::env::var("CHYREN_TELEMETRY_HOST").unwrap_or_else(|_| api_host.clone());
    let telemetry_port: u16 = std::env::var("CHYREN_TELEMETRY_PORT")
        .unwrap_or_else(|_| "9090".to_string())
        .parse()?;
    let repo_dir = std::env::var("CHYREN_REPO_DIR").unwrap_or_else(|_| "/home/mega/Chyren".to_string());
    let chyren_bin = std::env::var("CHYREN_BIN").unwrap_or_else(|_| format!("{}/chyren", repo_dir));

    setup_terminal()?;
    let res = run(&api_host, api_port, &telemetry_host, telemetry_port, &repo_dir, &chyren_bin).await;
    restore_terminal()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }
    Ok(())
}

async fn run(
    api_host: &str,
    api_port: u16,
    telemetry_host: &str,
    telemetry_port: u16,
    repo_dir: &str,
    chyren_bin: &str,
) -> Result<(), Box<dyn Error>> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let (tx, mut rx) = mpsc::unbounded_channel::<Event>();
    let mut state = AppState::new();
    let pm = ProcessManager::new();

    spawn_terminal_reader(tx.clone());
    spawn_tick_timer(tx.clone());

    let telemetry_url = format!("ws://{}:{}/ws", telemetry_host, telemetry_port);
    spawn_telemetry_listener(telemetry_url, tx.clone());

    api::spawn_status_poller(api_host.to_string(), api_port, tx.clone());
    api::spawn_mesh_poller(api_host.to_string(), api_port, tx.clone());

    loop {
        terminal.draw(|f| ui::draw(f, &state))?;

        let Some(evt) = rx.recv().await else { break; };

        match evt {
            Event::Key(key) => {
                handle_key(&mut state, key, &pm, tx.clone(), api_host, api_port, repo_dir, chyren_bin);
                if state.should_quit {
                    break;
                }
            }
            Event::Resize(_, _) => {}
            Event::SseChunk(chunk) => state.chat.add_stream_chunk(chunk),
            Event::SseComplete(resp) => {
                state.chat.adccl_score = resp.adccl_score;
                state.chat.finish_streaming();
            }
            Event::TelemetryWs(e) => state.telemetry.add_event(e),
            Event::ApiError(err) => {
                state.chat.add_message(MessageRole::Chyren, format!("⚠ {}", err));
            }
            Event::Connected => state.status.connected = true,
            Event::Disconnected => state.status.connected = false,
            Event::Tick => {}
            Event::ProcStarted { id, label } => {
                state.proc.upsert_started(&id, &label);
                if state.proc.order.len() == 1 {
                    state.proc.selected = 0;
                }
            }
            Event::ProcLine { id, line, is_err } => {
                state.proc.append_line(&id, line, is_err);
            }
            Event::ProcExited { id, code } => {
                state.proc.mark_exited(&id, code);
                let label = state
                    .proc
                    .entries
                    .get(&id)
                    .map(|e| e.label.clone())
                    .unwrap_or_else(|| id.clone());
                state.chat.add_message(
                    MessageRole::Chyren,
                    format!("◈ Process '{}' exited (code: {:?})", label, code),
                );
            }
            Event::StatusRefresh(snap) => state.apply_status(snap),
            Event::MeshRefresh(agents) => {
                if !agents.is_empty() {
                    state.mesh.replace_from_api(agents);
                }
            }
        }
    }

    pm.kill_all().await;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_key(
    state: &mut AppState,
    key: KeyEvent,
    pm: &ProcessManager,
    tx: mpsc::UnboundedSender<Event>,
    api_host: &str,
    api_port: u16,
    repo_dir: &str,
    chyren_bin: &str,
) {
    if state.show_command_palette {
        handle_palette_key(state, key, pm, tx, api_host, api_port, repo_dir, chyren_bin);
        return;
    }
    if state.active_tab == Tab::Telemetry && state.telemetry.filter_mode {
        handle_filter_key(state, key);
        return;
    }
    if state.show_help {
        match key.code {
            KeyCode::F(1) | KeyCode::Char('?') | KeyCode::Esc => state.show_help = false,
            _ => {}
        }
        return;
    }

    match state.mode {
        AppMode::Normal => handle_normal_mode(state, key),
        AppMode::Insert => handle_insert_mode(state, key, pm, tx, api_host, api_port, repo_dir, chyren_bin),
    }
}

fn handle_normal_mode(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => state.should_quit = true,
        KeyCode::Char('i') => state.set_mode(AppMode::Insert),
        KeyCode::Char('1') => state.switch_tab(Tab::Chat),
        KeyCode::Char('2') => state.switch_tab(Tab::Mesh),
        KeyCode::Char('3') => state.switch_tab(Tab::Telemetry),
        KeyCode::Char('4') => state.switch_tab(Tab::Dream),
        KeyCode::Char('5') => state.switch_tab(Tab::System),
        KeyCode::Tab => state.next_tab(),
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            open_palette(state);
        }
        KeyCode::F(1) | KeyCode::Char('?') => state.show_help = !state.show_help,
        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            state.chat.clear();
        }
        KeyCode::Char('/') if state.active_tab == Tab::Telemetry => {
            state.telemetry.filter_mode = true;
            state.telemetry.filter.clear();
        }
        KeyCode::Char('j') if state.active_tab == Tab::System => state.proc.select_next(),
        KeyCode::Char('k') if state.active_tab == Tab::System => state.proc.select_prev(),
        KeyCode::Down if state.active_tab == Tab::System => state.proc.select_next(),
        KeyCode::Up if state.active_tab == Tab::System => state.proc.select_prev(),
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_insert_mode(
    state: &mut AppState,
    key: KeyEvent,
    pm: &ProcessManager,
    tx: mpsc::UnboundedSender<Event>,
    api_host: &str,
    api_port: u16,
    repo_dir: &str,
    chyren_bin: &str,
) {
    match key.code {
        KeyCode::Esc => state.set_mode(AppMode::Normal),
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
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
        KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => state.input.move_home(),
        KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => state.input.move_end(),
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            state.set_mode(AppMode::Normal);
            open_palette(state);
        }
        KeyCode::Enter => {
            let raw = state.input.submit();
            if !raw.is_empty() {
                let outcome = {
                    let mut router = Router {
                        state,
                        tx: tx.clone(),
                        pm: pm.clone(),
                        repo_dir: repo_dir.to_string(),
                        chyren_bin: chyren_bin.to_string(),
                    };
                    router.dispatch(&raw)
                };
                match outcome {
                    RouteOutcome::Quit => state.should_quit = true,
                    RouteOutcome::SendToChat(msg) => {
                        send_chat(state, msg, tx.clone(), api_host, api_port);
                    }
                    RouteOutcome::Handled => {}
                }
            }
            state.set_mode(AppMode::Normal);
        }
        _ => {}
    }
}

fn handle_filter_key(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            state.telemetry.filter_mode = false;
            state.telemetry.filter.clear();
        }
        KeyCode::Enter => {
            state.telemetry.filter_mode = false;
        }
        KeyCode::Backspace => {
            state.telemetry.filter.pop();
        }
        KeyCode::Char(c) => {
            state.telemetry.filter.push(c);
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_palette_key(
    state: &mut AppState,
    key: KeyEvent,
    pm: &ProcessManager,
    tx: mpsc::UnboundedSender<Event>,
    api_host: &str,
    api_port: u16,
    repo_dir: &str,
    chyren_bin: &str,
) {
    match key.code {
        KeyCode::Esc => {
            state.show_command_palette = false;
            state.palette = Default::default();
        }
        KeyCode::Backspace => {
            state.palette.query.pop();
            state.palette.selected = 0;
        }
        KeyCode::Up => {
            if state.palette.selected > 0 {
                state.palette.selected -= 1;
            }
        }
        KeyCode::Down => {
            state.palette.selected += 1;
        }
        KeyCode::Enter => {
            let matches = ui::filter_commands(&state.palette.query);
            let total = matches.len();
            if total > 0 {
                let idx = state.palette.selected.min(total - 1);
                let cmd_name = matches[idx].0.name.to_string();
                state.show_command_palette = false;
                state.palette = Default::default();
                let outcome = {
                    let mut router = Router {
                        state,
                        tx: tx.clone(),
                        pm: pm.clone(),
                        repo_dir: repo_dir.to_string(),
                        chyren_bin: chyren_bin.to_string(),
                    };
                    router.dispatch(&cmd_name)
                };
                match outcome {
                    RouteOutcome::Quit => state.should_quit = true,
                    RouteOutcome::SendToChat(msg) => send_chat(state, msg, tx.clone(), api_host, api_port),
                    RouteOutcome::Handled => {}
                }
            }
        }
        KeyCode::Char(c) => {
            state.palette.query.push(c);
            state.palette.selected = 0;
        }
        _ => {}
    }
}

fn open_palette(state: &mut AppState) {
    state.show_command_palette = true;
    state.palette = Default::default();
}

fn send_chat(
    state: &mut AppState,
    msg: String,
    tx: mpsc::UnboundedSender<Event>,
    api_host: &str,
    api_port: u16,
) {
    state.chat.add_message(MessageRole::User, msg.clone());
    state.chat.start_streaming();

    let host = api_host.to_string();
    let port = api_port;
    tokio::spawn(async move {
        let client = api::ChatClient::new(&host, port);
        let _ = client.stream(&msg, None, tx).await;
    });
}

fn spawn_terminal_reader(tx: mpsc::UnboundedSender<Event>) {
    tokio::spawn(async move {
        use futures::StreamExt;
        let mut reader = ct_event::EventStream::new();
        while let Some(evt) = reader.next().await {
            match evt {
                Ok(CrosstermEvent::Key(key)) => {
                    let _ = tx.send(Event::Key(key));
                }
                Ok(CrosstermEvent::Resize(w, h)) => {
                    let _ = tx.send(Event::Resize(w, h));
                }
                _ => {}
            }
        }
    });
}

fn spawn_tick_timer(tx: mpsc::UnboundedSender<Event>) {
    tokio::spawn(async move {
        let mut iv = interval(Duration::from_millis(250));
        loop {
            iv.tick().await;
            let _ = tx.send(Event::Tick);
        }
    });
}

fn spawn_telemetry_listener(url: String, tx: mpsc::UnboundedSender<Event>) {
    tokio::spawn(async move {
        api::TelemetrySocket::connect_and_listen(&url, tx).await;
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
