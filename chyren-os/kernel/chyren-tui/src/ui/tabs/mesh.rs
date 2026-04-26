use crate::app::{AgentStatus, AppState};
use crate::theme::Theme;
use ratatui::layout::{Constraint, Rect};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Cell, Row, Table};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Agent Mesh  ({} agents) ", state.mesh.agents.len()))
        .style(Theme::border());

    let now = chrono::Local::now().timestamp() as f64;

    let rows: Vec<Row> = state
        .mesh
        .agents
        .iter()
        .map(|agent| {
            let (status_str, status_style) = match agent.status {
                AgentStatus::Idle => ("● IDLE   ", Theme::agent_idle()),
                AgentStatus::Busy => ("◑ BUSY   ", Theme::agent_busy()),
                AgentStatus::Offline => ("○ OFFLINE", Theme::agent_offline()),
            };

            let elapsed = (now - agent.last_active).max(0.0);
            let last_active = if elapsed < 60.0 {
                format!("{:.0}s ago", elapsed)
            } else if elapsed < 3600.0 {
                format!("{:.0}m ago", elapsed / 60.0)
            } else {
                format!("{:.1}h ago", elapsed / 3600.0)
            };

            let caps = agent.capabilities.join(", ");

            Row::new(vec![
                Cell::from(Span::styled(agent.id.clone(), Theme::text_default())),
                Cell::from(Span::styled(status_str, status_style)),
                Cell::from(Span::styled(last_active, Theme::text_dim())),
                Cell::from(Span::styled(caps, Theme::text_default())),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Min(20),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Min(20),
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
