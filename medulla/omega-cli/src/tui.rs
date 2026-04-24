use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use omega_cli::conductor::Conductor;
use omega_core::mesh::{AgentRegistry, AgentStatus};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
    Terminal,
};
use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[allow(dead_code)]
pub struct TuiApp {
    pub conductor: Arc<Conductor>,
    pub registry: Arc<Mutex<AgentRegistry>>,
    pub logs: Vec<String>,
    pub last_tick: Instant,
}

impl TuiApp {
    pub fn new(conductor: Arc<Conductor>, registry: Arc<Mutex<AgentRegistry>>) -> Self {
        Self {
            conductor,
            registry,
            logs: Vec::new(),
            last_tick: Instant::now(),
        }
    }

    pub fn push_log(&mut self, log: String) {
        self.logs.push(log);
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }
}

pub async fn run_tui(app: Arc<Mutex<TuiApp>>) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let _tick_rate = Duration::from_millis(250);

    loop {
        {
            let mut app = app.lock().await;
            terminal.draw(|f| ui(f, &mut app))?;
        }

        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &mut TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Main area (Mesh + Memory)
            Constraint::Length(10), // Logs
        ])
        .split(f.size());

    // --- Header ---
    let header = Paragraph::new(" CHYREN — Sovereign Intelligence Hub | [Q] to Quit")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    f.render_widget(header, chunks[0]);

    // --- Main Area (Split Horizontal) ---
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Agent Mesh
            Constraint::Percentage(40), // Memory/State
        ])
        .split(chunks[1]);

    // 1. Agent Mesh Table
    // We block_on or use some sync way to get registry data?
    // Actually, TuiApp should have a snapshot or we use a sync bridge.
    // For now, we'll just show placeholders or try to lock.
    // NOTE: UI is sync, so we rely on TuiApp being updated by a background task.

    let registry = futures::executor::block_on(app.registry.lock());
    let rows: Vec<Row> = registry
        .agents
        .values()
        .map(|a| {
            let status_style = match a.status {
                AgentStatus::Idle => Style::default().fg(Color::Green),
                AgentStatus::Busy => Style::default().fg(Color::Yellow),
                AgentStatus::Offline => Style::default().fg(Color::Red),
            };
            Row::new(vec![
                Cell::from(a.id.clone()),
                Cell::from(format!("{:?}", a.status)).style(status_style),
                Cell::from(format!(
                    "{:?}",
                    a.capabilities
                        .iter()
                        .map(|c| &c.category)
                        .collect::<Vec<_>>()
                )),
            ])
        })
        .collect();

    let mesh_table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(50),
        ],
    )
    .header(
        Row::new(vec!["Agent ID", "Status", "Capabilities"])
            .style(Style::default().fg(Color::Yellow)),
    )
    .block(
        Block::default()
            .title(" Sovereign Agent Mesh ")
            .borders(Borders::ALL),
    );
    f.render_widget(mesh_table, main_chunks[0]);

    // 2. Memory / Epistemic Shards
    let memory_info = vec![
        ListItem::new(" • ColdStore: IPFS Bridge Active"),
        ListItem::new(" • Vector Sharding: Enabled (Domain-Aware)"),
        ListItem::new(format!(" • Neocortex Entropy: {:.2}", 0.42)), // Placeholder
    ];
    let memory_list = List::new(memory_info).block(
        Block::default()
            .title(" Epistemic Memory ")
            .borders(Borders::ALL),
    );
    f.render_widget(memory_list, main_chunks[1]);

    // --- Logs ---
    let log_items: Vec<ListItem> = app
        .logs
        .iter()
        .rev()
        .take(8)
        .map(|l| ListItem::new(l.as_str()))
        .collect();
    let logs = List::new(log_items).block(
        Block::default()
            .title(" Telemetry Stream ")
            .borders(Borders::ALL),
    );
    f.render_widget(logs, chunks[2]);
}
