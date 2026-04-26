use crate::app::{AppMode, AppState, Tab};
use crate::theme::Theme;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn draw_footer(frame: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(40)])
        .split(area);

    draw_input(frame, chunks[0], state);
    draw_hints(frame, chunks[1], state);
}

fn draw_input(frame: &mut Frame, area: Rect, state: &AppState) {
    let (prompt, style) = match state.mode {
        AppMode::Normal => ("  ›", Theme::input_normal()),
        AppMode::Insert => ("  ▶", Theme::input_insert()),
    };

    let mut buf = state.input.buffer.clone();
    if state.active_tab == Tab::Telemetry && state.telemetry.filter_mode {
        buf = format!("filter: {}", state.telemetry.filter);
    } else if state.show_command_palette {
        buf = format!("palette: {}", state.palette.query);
    }

    let input_text = format!("{} {}", prompt, buf);

    let block = Block::default().borders(Borders::TOP).style(Theme::border());
    let para = Paragraph::new(input_text).block(block).style(style);

    frame.render_widget(para, area);
}

fn draw_hints(frame: &mut Frame, area: Rect, state: &AppState) {
    let mode_label = match state.mode {
        AppMode::Normal => Span::styled(" NORMAL ", Theme::header()),
        AppMode::Insert => Span::styled(" INSERT ", Theme::active_tab()),
    };

    let hints = Line::from(vec![
        mode_label,
        Span::raw(" "),
        Span::styled("[Ctrl+P]", Theme::border()),
        Span::raw(" cmd  "),
        Span::styled("[F1]", Theme::border()),
        Span::raw(" help  "),
        Span::styled("[q]", Theme::border()),
        Span::raw(" quit"),
    ]);

    let para = Paragraph::new(hints).style(Theme::text_dim());
    frame.render_widget(para, area);
}
