use crate::app::{AppMode, AppState, Tab};
use crate::theme::{Theme, MANTLE, SURFACE0};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn draw_footer(frame: &mut Frame, area: Rect, state: &AppState) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Length(52)])
        .split(area);

    draw_input(frame, cols[0], state);
    draw_hints(frame, cols[1], state);
}

fn draw_input(frame: &mut Frame, area: Rect, state: &AppState) {
    let insert = state.mode == AppMode::Insert;
    let (prompt_glyph, prompt_style, input_style, block_style) = if insert {
        ("▶ ", Theme::input_prompt_insert(), Theme::input_insert(),
         Style::default().fg(SURFACE0).bg(SURFACE0))
    } else {
        ("› ", Theme::input_prompt_normal(), Theme::input_normal(),
         Style::default().fg(SURFACE0).bg(MANTLE))
    };

    let display = if state.active_tab == Tab::Telemetry && state.telemetry.filter_mode {
        format!("  filter: {}_", state.telemetry.filter)
    } else {
        let buf = &state.input.buffer;
        if insert && !buf.is_empty() {
            format!("  {}{}", prompt_glyph, buf)
        } else if insert {
            format!("  {}▋", prompt_glyph)
        } else {
            format!("  {}{}",  prompt_glyph, buf)
        }
    };

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(block_style);

    let para = Paragraph::new(display).block(block).style(input_style);
    frame.render_widget(para, area);
}

fn draw_hints(frame: &mut Frame, area: Rect, state: &AppState) {
    let (mode_text, mode_style) = match state.mode {
        AppMode::Normal => (" NORMAL ", Theme::pill_normal()),
        AppMode::Insert => (" INSERT ", Theme::pill_insert()),
    };

    let hints = Line::from(vec![
        Span::styled(mode_text, mode_style),
        Span::raw(" "),
        Span::styled("^P", Theme::border_active()),
        Span::styled(" palette", Theme::text_dim()),
        Span::raw("  "),
        Span::styled("F1", Theme::border_active()),
        Span::styled(" help", Theme::text_dim()),
        Span::raw("  "),
        Span::styled("q", Theme::border_active()),
        Span::styled(" quit", Theme::text_dim()),
        Span::raw(" "),
    ]);

    let para = Paragraph::new(hints)
        .style(Style::default().bg(MANTLE))
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(SURFACE0).bg(MANTLE))
        );
    frame.render_widget(para, area);
}

