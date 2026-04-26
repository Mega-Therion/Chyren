use crate::app::{AppState, Tab};
use crate::theme::Theme;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use std::io::Stdout;

pub fn draw_header(frame: &mut Frame, area: Rect, state: &AppState) {
    let status_dot = if state.status.connected { "●" } else { "○" };

    let header_text = format!(
        " ◈ CHYREN  ·  Sovereign Intelligence Orchestrator  │  v2.5  │  {}  {}",
        if state.status.connected { "ONLINE" } else { "OFFLINE" },
        status_dot
    );

    let mut tabs = String::new();
    for i in 0..4 {
        let tab = Tab::from_index(i).unwrap();
        let name = match tab {
            Tab::Chat => "[1:Chat]",
            Tab::Mesh => "[2:Mesh]",
            Tab::Telemetry => "[3:Telemetry]",
            Tab::Dream => "[4:Dream]",
        };

        if tab == state.active_tab {
            tabs.push_str(&format!(" {} ", name));
        } else {
            tabs.push_str(&format!("  {}  ", name));
        }
    }

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .style(Theme::header());

    let para = Paragraph::new(header_text)
        .block(block)
        .style(Theme::header());

    frame.render_widget(para, area);

    let tab_area = Rect {
        x: area.x,
        y: area.y + 1,
        width: area.width,
        height: 1,
    };

    let tab_para = Paragraph::new(tabs)
        .style(Theme::text_default());

    frame.render_widget(tab_para, tab_area);
}
