pub mod header;
pub mod footer;
pub mod tabs;
pub mod widgets;

use crate::app::{AppState, Tab};
use crate::router::COMMANDS;
use crate::theme::Theme;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, state: &AppState) {
    let size = frame.area();

    if size.height < 5 {
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(size);

    header::draw_header(frame, chunks[0], state);

    match state.active_tab {
        Tab::Chat => tabs::chat::draw(frame, chunks[1], state),
        Tab::Mesh => tabs::mesh::draw(frame, chunks[1], state),
        Tab::Telemetry => tabs::telemetry::draw(frame, chunks[1], state),
        Tab::Dream => tabs::dream::draw(frame, chunks[1], state),
        Tab::System => tabs::system::draw(frame, chunks[1], state),
    }

    footer::draw_footer(frame, chunks[2], state);

    if state.show_help {
        draw_help_overlay(frame, state);
    }

    if state.show_command_palette {
        draw_command_palette(frame, state);
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let w = width.min(area.width.saturating_sub(4));
    let h = height.min(area.height.saturating_sub(4));
    let x = area.x + (area.width.saturating_sub(w)) / 2;
    let y = area.y + (area.height.saturating_sub(h)) / 2;
    Rect { x, y, width: w, height: h }
}

fn draw_help_overlay(frame: &mut Frame, _state: &AppState) {
    let size = frame.area();
    let rect = centered_rect(72, 26, size);

    let lines: Vec<Line> = vec![
        Line::from(Span::styled("  KEYBINDINGS", Theme::header())),
        Line::from(""),
        Line::from(Span::styled("  Normal Mode", Theme::border_active())),
        Line::from("    i           Enter insert mode"),
        Line::from("    1–5         Switch tabs (Chat, Mesh, Telemetry, Dream, System)"),
        Line::from("    Tab         Cycle next tab"),
        Line::from("    Ctrl+P      Command palette"),
        Line::from("    F1 / ?      Toggle this help"),
        Line::from("    Ctrl+L      Clear chat history"),
        Line::from("    j / k       Select next/prev process (System tab)"),
        Line::from("    /           Filter mode (Telemetry tab)"),
        Line::from("    q           Quit"),
        Line::from(""),
        Line::from(Span::styled("  Insert Mode", Theme::border_active())),
        Line::from("    Esc         Return to normal"),
        Line::from("    Enter       Submit"),
        Line::from("    Up / Down   Input history"),
        Line::from("    Ctrl+A / E  Home / End"),
        Line::from(""),
        Line::from(Span::styled("  Slash Commands", Theme::border_active())),
        Line::from("    /run dream | live | server | sovereign | recon | reset"),
        Line::from("    /chat /mesh /telemetry /dream /system   navigate"),
        Line::from("    /clear /status /provider <name> /quit   utilities"),
        Line::from(""),
        Line::from(Span::styled("    Press F1 or ? to close.", Theme::text_dim())),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Help ")
        .style(Theme::border_active());

    let para = Paragraph::new(lines)
        .block(block)
        .style(Theme::text_default())
        .wrap(Wrap { trim: false });

    frame.render_widget(Clear, rect);
    frame.render_widget(para, rect);
}

fn draw_command_palette(frame: &mut Frame, state: &AppState) {
    let size = frame.area();
    let rect = centered_rect(70, 22, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .split(rect);

    let query_block = Block::default()
        .borders(Borders::ALL)
        .title(" Command Palette ")
        .style(Theme::border_active());
    let query_text = format!(" › {}", state.palette.query);
    let query_para = Paragraph::new(query_text).block(query_block).style(Theme::text_default());

    let matches = filter_commands(&state.palette.query);
    let mut items: Vec<ListItem> = Vec::new();
    for (i, (cmd, _score)) in matches.iter().enumerate() {
        let mut style = Theme::text_default();
        if i == state.palette.selected.min(matches.len().saturating_sub(1)) {
            style = style.add_modifier(Modifier::REVERSED);
        }
        let line = Line::from(vec![
            Span::styled(format!(" {:<22} ", cmd.name), style),
            Span::styled(format!("{:<10} ", cmd.category), Theme::text_dim()),
            Span::styled(cmd.description, Theme::text_dim()),
        ]);
        items.push(ListItem::new(line));
    }

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} matches  (Enter to run, Esc to cancel) ", matches.len()))
        .style(Theme::border());
    let list = List::new(items).block(list_block);

    frame.render_widget(Clear, rect);
    frame.render_widget(query_para, chunks[0]);
    frame.render_widget(list, chunks[1]);
}

pub fn filter_commands(query: &str) -> Vec<(&'static crate::router::CommandSpec, i64)> {
    let matcher = SkimMatcherV2::default();
    let mut scored: Vec<(&crate::router::CommandSpec, i64)> = Vec::new();
    if query.is_empty() {
        for c in COMMANDS {
            scored.push((c, 0));
        }
        return scored;
    }
    for c in COMMANDS {
        let hay = format!("{} {} {}", c.name, c.category, c.description);
        if let Some(score) = matcher.fuzzy_match(&hay, query) {
            scored.push((c, score));
        }
    }
    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored
}
