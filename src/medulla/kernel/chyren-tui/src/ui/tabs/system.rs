use crate::app::AppState;
use crate::theme::Theme;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(34), Constraint::Min(40)])
        .split(area);

    draw_proc_list(frame, chunks[0], state);
    draw_proc_log(frame, chunks[1], state);
}

fn draw_proc_list(frame: &mut Frame, area: Rect, state: &AppState) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(4)])
        .split(area);

    let actions = vec![
        Line::from(vec![
            Span::styled("  ACTIONS", Theme::header()),
        ]),
        Line::from(""),
        Line::from(vec![Span::raw("  /run dream         "), Span::styled("maintenance", Theme::text_dim())]),
        Line::from(vec![Span::raw("  /run live          "), Span::styled("api+web", Theme::text_dim())]),
        Line::from(vec![Span::raw("  /run server        "), Span::styled("api only", Theme::text_dim())]),
        Line::from(vec![Span::raw("  /run sovereign     "), Span::styled("docker", Theme::text_dim())]),
        Line::from(vec![Span::raw("  /run reset --yes   "), Span::styled("destructive", Theme::adccl_fail())]),
    ];

    let actions_block = Block::default()
        .borders(Borders::ALL)
        .title(" Actions ")
        .style(Theme::border());

    let actions_para = Paragraph::new(actions)
        .block(actions_block)
        .style(Theme::text_default());

    frame.render_widget(actions_para, outer[0]);

    let mut items: Vec<ListItem> = Vec::new();
    for (i, id) in state.proc.order.iter().enumerate() {
        if let Some(entry) = state.proc.entries.get(id) {
            let (dot, dot_style) = if entry.running {
                ("●", Theme::agent_busy())
            } else if entry.exit_code == Some(0) {
                ("✓", Theme::agent_idle())
            } else {
                ("✗", Theme::agent_offline())
            };

            let mut style = Theme::text_default();
            if i == state.proc.selected {
                style = style.add_modifier(Modifier::REVERSED);
            }

            let line = Line::from(vec![
                Span::styled(format!(" {} ", dot), dot_style),
                Span::styled(entry.label.clone(), style),
            ]);
            items.push(ListItem::new(line));
        }
    }

    if items.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "  (no processes — try /run dream)",
            Theme::text_dim(),
        ))));
    }

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Processes [{} active] ", state.proc.active_count()))
        .style(Theme::border());

    let list = List::new(items).block(list_block);

    frame.render_widget(list, outer[1]);
}

fn draw_proc_log(frame: &mut Frame, area: Rect, state: &AppState) {
    let mut lines: Vec<Line> = Vec::new();

    let title = if let Some(entry) = state.proc.selected_entry() {
        let status = if entry.running {
            "running".to_string()
        } else {
            format!("exited {:?}", entry.exit_code)
        };
        format!(" {} · {} ", entry.label, status)
    } else {
        " Output ".to_string()
    };

    if let Some(entry) = state.proc.selected_entry() {
        let inner_height = area.height.saturating_sub(2) as usize;
        let start = entry.log.len().saturating_sub(inner_height);
        for pl in &entry.log[start..] {
            let style = if pl.is_err {
                Style::default().fg(crate::theme::PEACH)
            } else {
                Theme::text_default()
            };
            lines.push(Line::from(Span::styled(pl.line.clone(), style)));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "Select a process from the left, or run one via the input bar.",
            Theme::text_dim(),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Press 'i' to open the input bar, then type a command like:",
            Theme::text_dim(),
        )));
        lines.push(Line::from(Span::styled(
            "  /run dream",
            Theme::telemetry_info(),
        )));
        lines.push(Line::from(Span::styled(
            "  /run live",
            Theme::telemetry_info(),
        )));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Theme::border());

    let para = Paragraph::new(lines)
        .block(block)
        .style(Theme::text_default())
        .wrap(Wrap { trim: false });

    frame.render_widget(para, area);
}
