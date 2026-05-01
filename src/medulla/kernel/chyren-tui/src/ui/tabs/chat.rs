use crate::app::{AppState, MessageRole};
use crate::theme::{self, Theme, BASE, MANTLE, SURFACE0, SURFACE1, TEAL, LAVENDER, OVERLAY0, TEXT};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap};
use ratatui::Frame;
use unicode_width::UnicodeWidthStr;

pub fn draw(frame: &mut Frame, area: Rect, state: &AppState) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(50), Constraint::Length(26)])
        .split(area);

    draw_messages(frame, cols[0], state);
    draw_sidebar(frame, cols[1], state);
}

fn draw_messages(frame: &mut Frame, area: Rect, state: &AppState) {
    let inner_width = area.width.saturating_sub(6) as usize;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Theme::border())
        .title(Span::styled(" ◈ Chat ", Theme::border_active()))
        .style(Theme::base_bg());

    let mut lines: Vec<Line> = vec![];

    for msg in &state.chat.messages {
        match msg.role {
            MessageRole::User => {
                // User: lavender gutter ▏on left, right-indented
                let ts = format_ts(msg.timestamp);
                lines.push(Line::from(vec![
                    Span::styled(" ▏", Theme::user_gutter()),
                    Span::styled(" you ", Theme::user_meta()),
                    Span::styled(ts, Theme::text_dim()),
                ]));
                for wrapped in wrap_text(&msg.content, inner_width) {
                    lines.push(Line::from(vec![
                        Span::styled(" ▏", Theme::user_gutter()),
                        Span::styled(format!("  {}", wrapped), Theme::user_text()),
                    ]));
                }
                lines.push(Line::from(Span::styled(" ▏", Theme::user_gutter())));
            }
            MessageRole::Chyren => {
                // Chyren: teal gutter ┃, sigil, ADCCL badge inline
                let ts = format_ts(msg.timestamp);
                let score_badge = msg.adccl_score.map(|s| {
                    let style = Theme::adccl_for_score(s);
                    Span::styled(format!(" {:.2} ", s), style)
                });
                let provider_badge = msg.provider.as_deref().map(|p| {
                    Span::styled(format!(" {} ", p), Theme::chyren_meta())
                });

                let mut header_spans = vec![
                    Span::styled(" ┃", Theme::chyren_gutter()),
                    Span::styled(" ◈ chyren ", Theme::chyren_gutter()),
                    Span::styled(ts, Theme::text_dim()),
                ];
                if let Some(b) = score_badge { header_spans.push(b); }
                if let Some(b) = provider_badge { header_spans.push(b); }
                lines.push(Line::from(header_spans));

                for wrapped in wrap_text(&msg.content, inner_width) {
                    lines.push(Line::from(vec![
                        Span::styled(" ┃", Theme::chyren_gutter()),
                        Span::styled(format!("  {}", wrapped), Theme::chyren_text()),
                    ]));
                }
                lines.push(Line::from(Span::styled(" ┃", Theme::chyren_gutter())));
            }
        }
    }

    // Streaming in-progress
    if state.chat.streaming {
        lines.push(Line::from(vec![
            Span::styled(" ┃", Theme::chyren_gutter()),
            Span::styled(" ◈ chyren ", Theme::chyren_gutter()),
            Span::styled(" generating… ", Theme::text_dim()),
        ]));
        for wrapped in wrap_text(&state.chat.streaming_buffer, inner_width) {
            lines.push(Line::from(vec![
                Span::styled(" ┃", Theme::chyren_gutter()),
                Span::styled(format!("  {}", wrapped), Theme::streaming_text()),
            ]));
        }
        // Blinking cursor on last line
        lines.push(Line::from(vec![
            Span::styled(" ┃", Theme::chyren_gutter()),
            Span::styled("  ▋", Theme::text_accent()),
        ]));
    }

    let total_lines = lines.len() as u16;
    let visible     = area.height.saturating_sub(2);
    let scroll      = total_lines.saturating_sub(visible + state.chat.scroll);

    let para = Paragraph::new(lines)
        .block(block)
        .style(Theme::base_bg())
        .scroll((scroll, 0))
        .wrap(Wrap { trim: false });

    frame.render_widget(para, area);
}

fn draw_sidebar(frame: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default()
        .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
        .border_style(Theme::border_dim())
        .title(Span::styled(" Signals ", Theme::sidebar_title()))
        .style(Theme::base_bg());

    let gauge_width = 10usize;
    let (filled, track, gauge_style) = theme::adccl_gauge(state.chat.adccl_score, gauge_width);

    let adccl_pct = (state.chat.adccl_score * 100.0).round() as u8;
    let threshold_passed = state.chat.adccl_score >= 0.7;

    let mut lines: Vec<Line> = vec![
        Line::from(""),

        // ADCCL Section
        Line::from(Span::styled("  ADCCL GATE", Theme::sidebar_title())),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(&filled, gauge_style),
            Span::styled(&track, Theme::adccl_track()),
            Span::styled(format!(" {}%", adccl_pct), gauge_style),
        ]),
        Line::from(vec![
            Span::raw("  "),
            if threshold_passed {
                Span::styled("● PASS  ≥ 0.7", Theme::adccl_high())
            } else {
                Span::styled("○ FAIL  < 0.7", Theme::adccl_low())
            }
        ]),
        Line::from(""),

        // Provider
        Line::from(Span::styled("  PROVIDER", Theme::sidebar_section())),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(&state.status.provider, Theme::pill_provider()),
        ]),
        Line::from(""),

        // Metrics
        Line::from(Span::styled("  METRICS", Theme::sidebar_section())),
        Line::from(vec![
            Span::styled("  runs    ", Theme::sidebar_key()),
            Span::styled(format!("{}", state.status.total_runs), Theme::sidebar_value()),
        ]),
        Line::from(vec![
            Span::styled("  active  ", Theme::sidebar_key()),
            Span::styled(format!("{}", state.status.active_runs), Theme::sidebar_value()),
        ]),
        Line::from(vec![
            Span::styled("  latency ", Theme::sidebar_key()),
            Span::styled(
                if state.status.latency_ms > 0.0 {
                    format!("{:.0}ms", state.status.latency_ms)
                } else {
                    "—".into()
                },
                Theme::sidebar_value(),
            ),
        ]),
        Line::from(""),

        // Sovereign scores
        Line::from(Span::styled("  SOVEREIGN", Theme::sidebar_section())),
        Line::from(vec![
            Span::styled("  χ  ", Theme::metric_chi()),
            Span::styled(format!("{:.3}", state.dream.chi), Theme::metric_chi()),
        ]),
        Line::from(vec![
            Span::styled("  Ω  ", Theme::metric_chyren()),
            Span::styled(format!("{:.3}", state.dream.chyren), Theme::metric_chyren()),
        ]),
        Line::from(""),

        // Dream
        Line::from(Span::styled("  DREAM", Theme::sidebar_section())),
        Line::from(vec![
            Span::styled("  episodes ", Theme::sidebar_key()),
            Span::styled(format!("{}", state.status.dream_episodes), Theme::sidebar_value()),
        ]),
    ];

    let para = Paragraph::new(lines).block(block).style(Theme::base_bg());
    frame.render_widget(para, area);
}

fn format_ts(ts: f64) -> String {
    let secs = ts as u64;
    let h = (secs / 3600) % 24;
    let m = (secs / 60) % 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 { return vec![text.to_string()]; }
    let mut lines = vec![];
    for input_line in text.lines() {
        if input_line.is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut current = String::new();
        for word in input_line.split_whitespace() {
            let w = UnicodeWidthStr::width(word);
            let cur_w = UnicodeWidthStr::width(current.as_str());
            if cur_w + w + 1 > width && !current.is_empty() {
                lines.push(current.clone());
                current.clear();
            }
            if !current.is_empty() { current.push(' '); }
            current.push_str(word);
        }
        if !current.is_empty() { lines.push(current); }
    }
    lines
}
