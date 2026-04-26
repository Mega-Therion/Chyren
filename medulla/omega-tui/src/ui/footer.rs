use crate::app::{AppMode, AppState};
use crate::theme::Theme;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::Span;
use ratatui::Frame;
use std::io::Stdout;

pub fn draw_footer(frame: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(30)])
        .split(area);

    draw_input(frame, chunks[0], state);
    draw_hints(frame, chunks[1], state);
}

fn draw_input(frame: &mut Frame, area: Rect, state: &AppState) {
    let prompt = match state.mode {
        AppMode::Normal => "  ",
        AppMode::Insert => " ▶",
    };

    let style = match state.mode {
        AppMode::Normal => Theme::input_normal(),
        AppMode::Insert => Theme::input_insert(),
    };

    let input_text = format!("{}  {}", prompt, state.input.buffer);

    let block = Block::default()
        .borders(Borders::TOP)
        .style(Theme::border());

    let para = Paragraph::new(input_text)
        .block(block)
        .style(style);

    frame.render_widget(para, area);

    if state.mode == AppMode::Insert && area.width > state.input.cursor as u16 + 5 {
        let cursor_x = area.x + state.input.cursor as u16 + 4;
        let cursor_y = area.y + 1;
        let size = frame.area();
        if cursor_x < size.width && cursor_y < size.height {
            // Visual cursor indication handled by terminal
        }
    }
}

fn draw_hints(frame: &mut Frame, area: Rect, _state: &AppState) {
    use ratatui::text::Line;

    let hints = Line::from(vec![
        Span::styled("[Ctrl+P]", Theme::border()),
        Span::raw(" "),
        Span::raw("cmd"),
        Span::raw("  "),
        Span::styled("[F1]", Theme::border()),
        Span::raw(" "),
        Span::raw("help"),
        Span::raw("  "),
        Span::styled("[Q]", Theme::border()),
        Span::raw(" "),
        Span::raw("quit"),
    ]);

    let para = Paragraph::new(hints)
        .style(Theme::text_dim());

    frame.render_widget(para, area);
}
