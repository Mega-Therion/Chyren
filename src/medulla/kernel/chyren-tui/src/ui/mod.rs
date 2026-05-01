pub mod header;
pub mod footer;
pub mod tabs;
pub mod widgets;

use crate::app::{AppState, Tab};
use crate::router::COMMANDS;
use crate::theme::{Theme, BASE, MANTLE, SURFACE0};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, state: &AppState) {
    let size = frame.area();
    if size.height < 5 { return; }

    // Fill entire background with base color to eliminate gray artifacts
    let bg = Paragraph::new("").style(Theme::base_bg());
    frame.render_widget(bg, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // header: topbar (1) + tabs (2)
            Constraint::Min(5),
            Constraint::Length(3),   // footer: input + hints
        ])
        .split(size);

    header::draw_header(frame, chunks[0], state);

    match state.active_tab {
        Tab::Chat      => tabs::chat::draw(frame, chunks[1], state),
        Tab::Mesh      => tabs::mesh::draw(frame, chunks[1], state),
        Tab::Telemetry => tabs::telemetry::draw(frame, chunks[1], state),
        Tab::Dream     => tabs::dream::draw(frame, chunks[1], state),
        Tab::System    => tabs::system::draw(frame, chunks[1], state),
    }

    footer::draw_footer(frame, chunks[2], state);

    if state.show_help            { draw_help_overlay(frame); }
    if state.show_command_palette { draw_command_palette(frame, state); }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let w = width.min(area.width.saturating_sub(4));
    let h = height.min(area.height.saturating_sub(4));
    let x = area.x + (area.width.saturating_sub(w)) / 2;
    let y = area.y + (area.height.saturating_sub(h)) / 2;
    Rect { x, y, width: w, height: h }
}

fn draw_help_overlay(frame: &mut Frame) {
    let size = frame.area();
    let rect = centered_rect(70, 28, size);

    let lines: Vec<Line> = vec![
        Line::from(vec![Span::styled("  Keybindings", Theme::border_active())]),
        Line::from(""),
        Line::from(vec![Span::styled("  Normal Mode", Theme::chyren_gutter())]),
        Line::from("    i  ·  enter insert mode"),
        Line::from("    1–5  ·  switch tabs (Chat Mesh Telemetry Dream System)"),
        Line::from("    Tab  ·  cycle next tab"),
        Line::from("    ^P  ·  command palette"),
        Line::from("    F1 / ?  ·  toggle help"),
        Line::from("    ^L  ·  clear chat"),
        Line::from("    j / k  ·  select process (System tab)"),
        Line::from("    /  ·  filter mode (Telemetry tab)"),
        Line::from("    q  ·  quit"),
        Line::from(""),
        Line::from(vec![Span::styled("  Insert Mode", Theme::user_gutter())]),
        Line::from("    Esc  ·  return to normal"),
        Line::from("    Enter  ·  submit message"),
        Line::from("    ↑ / ↓  ·  input history"),
        Line::from("    ^A / ^E  ·  home / end"),
        Line::from(""),
        Line::from(vec![Span::styled("  Slash Commands", Theme::sidebar_title())]),
        Line::from("    /run dream | live | server | sovereign | recon | reset"),
        Line::from("    /chat /mesh /telemetry /dream /system  — navigate"),
        Line::from("    /clear /status /provider <name> /quit  — utilities"),
        Line::from(""),
        Line::from(vec![Span::styled("    F1 or ? to close", Theme::text_dim())]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Theme::border_active())
        .title(Span::styled(" Help ", Theme::border_active()))
        .style(Style::default().bg(MANTLE));

    let para = Paragraph::new(lines)
        .block(block)
        .style(Theme::text())
        .wrap(Wrap { trim: false });

    frame.render_widget(Clear, rect);
    frame.render_widget(para, rect);
}

fn draw_command_palette(frame: &mut Frame, state: &AppState) {
    let size = frame.area();
    let rect = centered_rect(68, 22, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .split(rect);

    // Search input
    let query_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Theme::border_active())
        .title(Span::styled(" ⌘ Command Palette ", Theme::border_active()))
        .style(Style::default().bg(MANTLE));
    let query_para = Paragraph::new(format!("  › {}", state.palette.query))
        .block(query_block)
        .style(Theme::input_insert());

    // Matches list
    let matches = filter_commands(&state.palette.query);
    let total   = matches.len();
    let selected = state.palette.selected.min(total.saturating_sub(1));

    let items: Vec<ListItem> = matches
        .iter()
        .enumerate()
        .map(|(i, (cmd, _score))| {
            let icon = category_icon(cmd.category);
            let is_sel = i == selected;

            let name_style = if is_sel { Theme::highlight() } else { Theme::text() };
            let cat_style  = if is_sel {
                Style::default().fg(crate::theme::BASE).bg(crate::theme::TEAL)
            } else {
                Theme::text_dim()
            };
            let desc_style = if is_sel {
                Style::default().fg(crate::theme::BASE).bg(crate::theme::TEAL)
            } else {
                Theme::text_dim()
            };

            let line = Line::from(vec![
                Span::styled(format!(" {} {:<22}", icon, cmd.name), name_style),
                Span::styled(format!("{:<10}", cmd.category), cat_style),
                Span::styled(cmd.description, desc_style),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Theme::border())
        .title(Span::styled(
            format!(" {} results  ↑↓ select  Enter run  Esc cancel ", total),
            Theme::text_dim(),
        ))
        .style(Style::default().bg(MANTLE));
    let list = List::new(items).block(list_block);

    frame.render_widget(Clear, rect);
    frame.render_widget(query_para, chunks[0]);
    frame.render_widget(list, chunks[1]);
}

fn category_icon(cat: &str) -> &'static str {
    match cat {
        "navigate"  => "󰝰",
        "run"       => "󰐊",
        "utility"   => "󰒓",
        "debug"     => "󰃤",
        _           => "󰘥",
    }
}

pub fn filter_commands(query: &str) -> Vec<(&'static crate::router::CommandSpec, i64)> {
    let matcher = SkimMatcherV2::default();
    let mut scored: Vec<(&crate::router::CommandSpec, i64)> = Vec::new();
    if query.is_empty() {
        for c in COMMANDS { scored.push((c, 0)); }
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

