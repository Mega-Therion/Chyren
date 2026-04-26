use crate::app::{AppState, MessageRole};
use crate::theme::Theme;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;
use ratatui::text::{Line, Span};
use std::io::Stdout;

pub fn draw(frame: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(50), Constraint::Length(28)])
        .split(area);

    draw_messages(frame, chunks[0], state);
    draw_sidebar(frame, chunks[1], state);
}

fn draw_messages(frame: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Messages")
        .style(Theme::border());

    let mut lines: Vec<Line> = vec![];

    for msg in &state.chat.messages {
        let label = match msg.role {
            MessageRole::User => "[user]",
            MessageRole::Chyren => "[chyren]",
        };

        let style = match msg.role {
            MessageRole::User => Theme::user_bubble(),
            MessageRole::Chyren => Theme::chyren_bubble(),
        };

        let timestamp_str = format!("{:.0}", msg.timestamp % 100.0);
        let mut header_spans = vec![
            Span::styled(label, style),
            Span::raw(" "),
            Span::styled(timestamp_str, Theme::text_dim()),
        ];

        if let Some(score) = msg.adccl_score {
            header_spans.push(Span::styled(format!(" ● {:.2}", score), Theme::adccl_pass()));
        }

        let header = Line::from(header_spans);
        lines.push(header);

        let wrapped = wrap_text(&msg.content, area.width.saturating_sub(4) as usize);
        for line in wrapped {
            lines.push(Line::from(vec![Span::styled(
                format!("  {}", line),
                if msg.role == MessageRole::User {
                    Theme::user_bubble()
                } else {
                    Theme::chyren_bubble()
                },
            )]));
        }

        lines.push(Line::from(""));
    }

    if state.chat.streaming {
        let streaming_header = Line::from(vec![
            Span::styled("[chyren]", Theme::chyren_bubble()),
            Span::raw(" "),
            Span::styled("streaming...", Theme::telemetry_info()),
        ]);
        lines.push(streaming_header);

        let wrapped = wrap_text(&state.chat.streaming_buffer, area.width.saturating_sub(4) as usize);
        for line in wrapped {
            lines.push(Line::from(vec![Span::styled(
                format!("  {} ▌", line),
                Theme::chyren_bubble(),
            )]));
        }
    }

    let para = Paragraph::new(lines)
        .block(block)
        .style(Theme::text_default())
        .wrap(Wrap { trim: false });

    frame.render_widget(para, area);
}

fn draw_sidebar(frame: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default()
        .borders(Borders::LEFT)
        .style(Theme::border());

    let adccl_bar = format_adccl_bar(state.chat.adccl_score);
    let provider = state.status.provider.clone();
    let tier_str = format!("{}", state.chat.tier);
    let active_runs_str = format!("{}", state.status.active_runs);
    let latency_str = format!("{:.2}s", state.status.latency_ms / 1000.0);
    let dream_episodes_str = format!("episodes: {}", state.status.dream_episodes);
    let adccl_score_str = format!("{:.2}", state.chat.adccl_score);

    let mut lines: Vec<Line> = vec![];

    lines.push(Line::from(Span::styled("  TELEMETRY", Theme::header())));
    lines.push(Line::from(""));

    lines.push(Line::from(vec![
        Span::raw("  ADCCL  "),
        Span::styled(adccl_bar, Theme::adccl_pass()),
        Span::raw("  "),
        Span::styled(adccl_score_str, Theme::adccl_pass()),
    ]));

    lines.push(Line::from(vec![
        Span::raw("  Provider"),
        Span::raw("  "),
        Span::styled(provider, Theme::telemetry_info()),
    ]));

    lines.push(Line::from(vec![
        Span::raw("  Tier      "),
        Span::styled(tier_str, Theme::telemetry_info()),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("  Runs   "),
        Span::styled("active: ", Theme::text_dim()),
        Span::raw(active_runs_str),
    ]));

    lines.push(Line::from(vec![
        Span::raw("  Latency  "),
        Span::styled(latency_str, Theme::text_dim()),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("  Dream"),
        Span::raw("  "),
        Span::styled(dream_episodes_str, Theme::border()),
    ]));

    let para = Paragraph::new(lines)
        .block(block)
        .style(Theme::text_default());

    frame.render_widget(para, area);
}

fn format_adccl_bar(score: f64) -> String {
    let filled = ((score * 10.0) as usize).min(10);
    let empty = 10 - filled;
    format!(
        "{}{}",
        "▓".repeat(filled),
        "░".repeat(empty)
    )
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }

    let mut lines = vec![];
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 > width {
            if !current.is_empty() {
                lines.push(current.clone());
                current.clear();
            }
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}
