use crate::app::{AppState, AgentStatus};
use crate::theme::Theme;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Table, Row, Cell};
use ratatui::Frame;
use ratatui::text::Span;
use std::io::Stdout;

pub fn draw(frame: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Agent Mesh")
        .style(Theme::border());

    let rows: Vec<Row> = state
        .mesh
        .agents
        .iter()
        .map(|agent| {
            let status_str = match agent.status {
                AgentStatus::Idle => "● IDLE",
                AgentStatus::Busy => "◑ BUSY",
                AgentStatus::Offline => "○ OFFLINE",
            };

            let status_style = match agent.status {
                AgentStatus::Idle => Theme::agent_idle(),
                AgentStatus::Busy => Theme::agent_busy(),
                AgentStatus::Offline => Theme::agent_offline(),
            };

            let caps = agent.capabilities.join(", ");
            let last_active = format!("{:.0}s ago", state.status.latency_ms);

            Row::new(vec![
                Cell::from(Span::styled(&agent.id, Theme::text_default())),
                Cell::from(Span::styled(status_str, status_style)),
                Cell::from(Span::styled(last_active, Theme::text_dim())),
                Cell::from(Span::styled(caps, Theme::text_default())),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            ratatui::layout::Constraint::Min(18),
            ratatui::layout::Constraint::Length(12),
            ratatui::layout::Constraint::Length(10),
            ratatui::layout::Constraint::Min(20),
        ],
    )
    .header(Row::new(vec![
        Cell::from(Span::styled("Agent ID", Theme::header())),
        Cell::from(Span::styled("Status", Theme::header())),
        Cell::from(Span::styled("Last Active", Theme::header())),
        Cell::from(Span::styled("Capabilities", Theme::header())),
    ]))
    .block(block)
    .style(Theme::text_default());

    frame.render_widget(table, area);
}
