use crate::app::AppState;
use crate::theme::Theme;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, area: Rect, state: &AppState) {
    let title = if state.telemetry.filter_mode {
        format!(" Live Events · filter: {}_ ", state.telemetry.filter)
    } else if !state.telemetry.filter.is_empty() {
        format!(" Live Events · filter: {} ", state.telemetry.filter)
    } else {
        " Live Events  (press / to filter) ".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Theme::border());

    let events = state.telemetry.filtered_events();
    let max_lines = area.height.saturating_sub(2) as usize;
    let skip = events.len().saturating_sub(max_lines);

    let mut lines: Vec<Line> = Vec::new();
    for event in events.iter().skip(skip) {
        let level_style = match event.level.as_str() {
            "INFO" => Theme::telemetry_info(),
            "WARN" | "WARNING" => Theme::telemetry_warn(),
            "CRITICAL" | "ERROR" => Theme::telemetry_critical(),
            _ => Theme::text_default(),
        };

        let time_str = format_clock(event.timestamp);
        let component = event.component.clone();
        let event_type = event.event_type.clone();
        let level_label = format!("{:>5}", event.level);

        let line = Line::from(vec![
            Span::raw("  "),
            Span::styled(time_str, Theme::text_dim()),
            Span::raw("  "),
            Span::styled(level_label, level_style),
            Span::raw("  "),
            Span::styled(component, Theme::chyren_bubble()),
            Span::raw("  "),
            Span::styled(event_type, Theme::text_default()),
        ]);

        lines.push(line);
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "  (no events match filter)",
            Theme::text_dim(),
        )));
    }

    let para = Paragraph::new(lines).block(block).style(Theme::text_default());
    frame.render_widget(para, area);
}

fn format_clock(ts: f64) -> String {
    use chrono::{Local, TimeZone};
    let secs = ts as i64;
    let dt = Local.timestamp_opt(secs, 0).single();
    if let Some(dt) = dt {
        dt.format("%H:%M:%S").to_string()
    } else {
        format!("{:.0}", ts)
    }
}
