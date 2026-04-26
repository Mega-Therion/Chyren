use crate::app::{AppMode, AppState};
use crate::theme::Theme;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::Span;
use ratatui::Frame;

pub fn draw_footer<B: Backend>(frame: &mut Frame<B>, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(30)])
        .split(area);

    draw_input(frame, chunks[0], state);
    draw_hints(frame, chunks[1], state);
}

fn draw_input<B: Backend>(frame: &mut Frame<B>, area: Rect, state: &AppState) {
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
        if cursor_x < frame.size().width && cursor_y < frame.size().height {
            // Visual cursor indication handled by terminal
        }
    }
}

fn draw_hints<B: Backend>(frame: &mut Frame<B>, area: Rect, _state: &AppState) {
    let hints = vec![
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
    ];

    let para = Paragraph::new(hints)
        .style(Theme::text_dim());

    frame.render_widget(para, area);
}
