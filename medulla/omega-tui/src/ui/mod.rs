pub mod header;
pub mod footer;
pub mod tabs;
pub mod widgets;

use crate::app::{AppState, Tab};
use header::draw_header;
use footer::draw_footer;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

pub fn draw<B: Backend>(frame: &mut Frame<B>, state: &AppState) {
    let size = frame.size();

    if size.height < 3 {
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

    draw_header(frame, chunks[0], state);

    match state.active_tab {
        Tab::Chat => tabs::chat::draw(frame, chunks[1], state),
        Tab::Mesh => tabs::mesh::draw(frame, chunks[1], state),
        Tab::Telemetry => tabs::telemetry::draw(frame, chunks[1], state),
        Tab::Dream => tabs::dream::draw(frame, chunks[1], state),
    }

    draw_footer(frame, chunks[2], state);

    if state.show_help {
        draw_help_overlay(frame, state);
    }

    if state.show_command_palette {
        draw_command_palette(frame, state);
    }
}

fn draw_help_overlay<B: Backend>(frame: &mut Frame<B>, _state: &AppState) {
    use ratatui::widgets::{Block, Borders, Paragraph};
    use ratatui::style::Style;
    use crate::theme::Theme;

    let size = frame.size();
    let width = 60.min(size.width - 4);
    let height = 20.min(size.height - 4);
    let x = (size.width.saturating_sub(width)) / 2;
    let y = (size.height.saturating_sub(height)) / 2;

    let help_rect = Rect {
        x,
        y,
        width,
        height,
    };

    let help_text = r#"
    KEYBINDINGS

    Normal Mode:
      i        - Enter insert mode
      1-4      - Switch tabs (Chat, Mesh, Telemetry, Dream)
      Tab      - Cycle next tab
      Ctrl+P   - Command palette
      Ctrl+L   - Clear chat
      PgUp/Dn  - Scroll
      g/G      - Top/Bottom
      q        - Quit

    Insert Mode:
      Esc              - Return to normal
      Enter            - Submit message
      Shift+Enter      - New line
      Up/Down          - History
      Ctrl+A / Ctrl+E  - Home / End

    Telemetry Tab:
      /        - Enter filter mode
"#;

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Help")
        .style(Theme::border_active());

    let para = Paragraph::new(help_text)
        .block(block)
        .style(Theme::text_default());

    frame.render_widget(para, help_rect);
}

fn draw_command_palette<B: Backend>(frame: &mut Frame<B>, _state: &AppState) {
    use ratatui::widgets::{Block, Borders, Paragraph};
    use crate::theme::Theme;

    let size = frame.size();
    let width = 50.min(size.width - 4);
    let height = 15.min(size.height - 4);
    let x = (size.width.saturating_sub(width)) / 2;
    let y = (size.height.saturating_sub(height)) / 2;

    let cmd_rect = Rect {
        x,
        y,
        width,
        height,
    };

    let commands = r#"
    /status              - Show status
    /clear               - Clear chat
    /solve <problem>     - Solve problem
    /discipline <name>   - Learn discipline
    /help                - Show help"#;

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Command Palette (Ctrl+P)")
        .style(Theme::border_active());

    let para = Paragraph::new(commands)
        .block(block)
        .style(Theme::text_default());

    frame.render_widget(para, cmd_rect);
}
