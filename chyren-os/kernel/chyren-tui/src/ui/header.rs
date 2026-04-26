use crate::app::{AppState, Tab};
use crate::theme::Theme;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn draw_header(frame: &mut Frame, area: Rect, state: &AppState) {
    let status_dot = if state.status.connected { "●" } else { "○" };
    let status_word = if state.status.connected { "ONLINE" } else { "OFFLINE" };
    let status_style = if state.status.connected {
        Theme::adccl_pass()
    } else {
        Theme::adccl_fail()
    };

    let header_line = Line::from(vec![
        Span::styled(" ◈ CHYREN", Theme::header()),
        Span::styled("  ·  Sovereign Intelligence Orchestrator  ", Theme::header_inactive()),
        Span::styled("│  ", Theme::header_inactive()),
        Span::styled(format!("{} {}", status_dot, status_word), status_style),
        Span::styled(
            format!("  │  prov: {}  │  χ {:.2}  Ω {:.2}  │  procs: {} ",
                state.status.provider,
                state.dream.chi,
                state.dream.chyren,
                state.proc.active_count(),
            ),
            Theme::header_inactive(),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .style(Theme::header());

    let para = Paragraph::new(header_line).block(block);
    frame.render_widget(para, area);

    let mut tab_spans: Vec<Span> = Vec::new();
    for i in 0..5 {
        let tab = Tab::from_index(i).unwrap();
        let label = format!(" [{}:{}] ", i + 1, tab.label());
        if tab == state.active_tab {
            tab_spans.push(Span::styled(label, Theme::active_tab()));
        } else {
            tab_spans.push(Span::styled(label, Theme::inactive_tab()));
        }
    }

    let tab_area = Rect {
        x: area.x,
        y: area.y + 1,
        width: area.width,
        height: 1,
    };

    let tab_para = Paragraph::new(Line::from(tab_spans)).style(Theme::text_default());
    frame.render_widget(tab_para, tab_area);
}
