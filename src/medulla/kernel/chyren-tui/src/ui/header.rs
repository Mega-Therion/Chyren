use crate::app::{AppState, Tab};
use crate::theme::{Theme, MANTLE, SURFACE0, TEAL, OVERLAY1, BASE};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::Frame;

const SIGIL: &str = "◈";
const BRAND: &str = " CHYREN";

pub fn draw_header(frame: &mut Frame, area: Rect, state: &AppState) {
    // Split header into top bar + tab row
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(2)])
        .split(area);

    draw_top_bar(frame, rows[0], state);
    draw_tab_bar(frame, rows[1], state);
}

fn draw_top_bar(frame: &mut Frame, area: Rect, state: &AppState) {
    // ── left: sigil + brand ───────────────────────────────────────────────
    let (conn_label, conn_style) = if state.status.connected {
        (" ONLINE ", Theme::pill_online())
    } else {
        (" OFFLINE", Theme::pill_offline())
    };

    let provider_text = format!(" {} ", state.status.provider);
    let chi_text    = format!(" χ {:.2} ", state.dream.chi);
    let omega_text  = format!(" Ω {:.2} ", state.dream.chyren);
    let latency_ms  = state.status.latency_ms;
    let latency_text = if latency_ms > 0.0 {
        format!(" {:.0}ms ", latency_ms)
    } else {
        String::new()
    };

    let left = vec![
        Span::styled(SIGIL, Theme::header_title()),
        Span::styled(BRAND, Theme::header_title()),
        Span::styled("  ", Style::default().bg(MANTLE)),
        Span::styled("Sovereign Intelligence Orchestrator", Theme::header_subtitle()),
    ];

    let right: Vec<Span> = vec![
        Span::styled(latency_text, Theme::pill_label()),
        Span::styled(" ", Style::default().bg(MANTLE)),
        Span::styled(chi_text,    Theme::metric_chi()),
        Span::styled(" ", Style::default().bg(MANTLE)),
        Span::styled(omega_text,  Theme::metric_chyren()),
        Span::styled("  ", Style::default().bg(MANTLE)),
        Span::styled(provider_text, Theme::pill_provider()),
        Span::styled(" ", Style::default().bg(MANTLE)),
        Span::styled(conn_label,  conn_style),
        Span::styled(" ", Style::default().bg(MANTLE)),
    ];

    // Render left-aligned then right-aligned by splitting area
    let left_width: u16 = left.iter().map(|s| s.content.len() as u16).sum();
    let right_width: u16 = right.iter().map(|s| s.content.len() as u16).sum();

    let split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(left_width),
            Constraint::Length(right_width + 1),
        ])
        .split(area);

    let left_para = Paragraph::new(Line::from(left)).style(Theme::mantle_bg());
    let right_para = Paragraph::new(Line::from(right)).style(Theme::mantle_bg());

    frame.render_widget(left_para, split[0]);
    frame.render_widget(right_para, split[1]);
}

fn draw_tab_bar(frame: &mut Frame, area: Rect, state: &AppState) {
    let tab_names: Vec<String> = (0..5)
        .filter_map(Tab::from_index)
        .map(|t| {
            let icon = match t {
                Tab::Chat      => "󰭻",
                Tab::Mesh      => "󰡉",
                Tab::Telemetry => "󱐋",
                Tab::Dream     => "󱩡",
                Tab::System    => "󰣖",
            };
            format!(" {} {} ", icon, t.label())
        })
        .collect();

    let active_idx = state.active_tab.index();

    let tabs = Tabs::new(tab_names)
        .select(active_idx)
        .style(Theme::tab_inactive())
        .highlight_style(Theme::tab_active())
        .divider(Span::styled("│", Style::default().fg(SURFACE0).bg(MANTLE)))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(SURFACE0).bg(MANTLE))
                .style(Style::default().bg(MANTLE)),
        );

    frame.render_widget(tabs, area);
}
