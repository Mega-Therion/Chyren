use crate::app::AppState;
use crate::theme::Theme;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph, Table, Row, Cell};
use ratatui::Frame;
use ratatui::text::{Line, Span};

pub fn draw(frame: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(6)])
        .split(area);

    draw_episodes(frame, chunks[0], state);
    draw_holonomy(frame, chunks[1], state);
}

fn draw_episodes(frame: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Dream Episodes")
        .style(Theme::border());

    if state.dream.episodes.is_empty() {
        let para = Paragraph::new("  No episodes recorded yet.")
            .block(block)
            .style(Theme::text_dim());
        frame.render_widget(para, area);
        return;
    }

    let rows: Vec<Row> = state
        .dream
        .episodes
        .iter()
        .take(area.height.saturating_sub(2) as usize)
        .map(|ep| {
            let score_style = if ep.score > 0.7 {
                Theme::adccl_pass()
            } else {
                Theme::adccl_fail()
            };

            Row::new(vec![
                Cell::from(Span::styled(&ep.id, Theme::text_default())),
                Cell::from(Span::styled(&ep.task_summary, Theme::text_default())),
                Cell::from(Span::styled(format!("{:.2}", ep.score), score_style)),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(12),
            Constraint::Min(20),
            Constraint::Length(8),
        ],
    )
    .header(Row::new(vec![
        Cell::from(Span::styled("Episode ID", Theme::header())),
        Cell::from(Span::styled("Task", Theme::header())),
        Cell::from(Span::styled("Score", Theme::header())),
    ]))
    .block(block)
    .style(Theme::text_default());

    frame.render_widget(table, area);
}

fn draw_holonomy(frame: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Holonomy Pulse")
        .style(Theme::border());

    let chi_bar = format_waveform(state.dream.chi);
    let chyren_bar = format_waveform(state.dream.chyren);
    let chi_val = format!("{:.2}", state.dream.chi);
    let chyren_val = format!("{:.2}", state.dream.chyren);

    let lines = vec![
        Line::from(vec![
            Span::raw("  χ (Chiral)  "),
            Span::styled(chi_bar, Theme::telemetry_info()),
            Span::raw("  "),
            Span::styled(chi_val, Theme::telemetry_info()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  Ω (Sovereign) "),
            Span::styled(chyren_bar, Theme::border()),
            Span::raw("  "),
            Span::styled(chyren_val, Theme::border()),
        ]),
    ];

    let para = Paragraph::new(lines)
        .block(block)
        .style(Theme::text_default());

    frame.render_widget(para, area);
}

fn format_waveform(value: f64) -> String {
    let clamped = (value * 10.0) as usize;
    let braille = ["⠀", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
    braille[clamped.min(8)].repeat(16)
}
