use crate::app::AppState;
use crate::theme::Theme;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use ratatui::text::{Line, Span};
use std::io::Stdout;

pub fn draw(frame: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Live Events")
        .style(Theme::border());

    let events = state.telemetry.filtered_events();
    let mut lines: Vec<Line> = vec![];

    let max_lines = area.height.saturating_sub(2) as usize;
    let skip = events.len().saturating_sub(max_lines);

    for event in events.iter().skip(skip) {
        let style = match event.level.as_str() {
            "INFO" => Theme::telemetry_info(),
            "WARN" => Theme::telemetry_warn(),
            "CRITICAL" => Theme::telemetry_critical(),
            _ => Theme::text_default(),
        };

        let time_str = format!("{:.0}", event.timestamp % 100.0);
        let component = event.component.clone();
        let event_type = event.event_type.clone();

        let line = Line::from(vec![
            Span::raw("  "),
            Span::styled(time_str, Theme::text_dim()),
            Span::raw(" "),
            Span::styled(component, style),
            Span::raw(" "),
            Span::styled(event_type, Theme::text_dim()),
        ]);

        lines.push(line);
    }

    let para = Paragraph::new(lines)
        .block(block)
        .style(Theme::text_default());

    frame.render_widget(para, area);
}
