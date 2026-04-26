use ratatui::style::{Color, Modifier, Style};

pub const CORE_CYAN: Color = Color::Rgb(0, 245, 255);
pub const NEURAL_BLUE: Color = Color::Rgb(51, 102, 255);
pub const VOID_BLACK: Color = Color::Rgb(0, 1, 3);
pub const GHOST_WHITE: Color = Color::Rgb(224, 224, 224);
pub const AMETHYST: Color = Color::Rgb(170, 100, 255);
pub const EMBER: Color = Color::Rgb(255, 160, 0);
pub const VERMILLION: Color = Color::Rgb(255, 60, 60);
pub const SAGE: Color = Color::Rgb(80, 230, 150);
pub const OBSIDIAN: Color = Color::Rgb(40, 40, 50);

pub struct Theme;

impl Theme {
    pub fn header() -> Style {
        Style::default()
            .fg(CORE_CYAN)
            .bg(OBSIDIAN)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header_inactive() -> Style {
        Style::default()
            .fg(Color::DarkGray)
            .bg(OBSIDIAN)
    }

    pub fn active_tab() -> Style {
        Style::default()
            .fg(CORE_CYAN)
            .bg(OBSIDIAN)
            .add_modifier(Modifier::BOLD)
    }

    pub fn inactive_tab() -> Style {
        Style::default()
            .fg(Color::DarkGray)
            .bg(OBSIDIAN)
    }

    pub fn text_default() -> Style {
        Style::default().fg(GHOST_WHITE).bg(VOID_BLACK)
    }

    pub fn text_dim() -> Style {
        Style::default()
            .fg(Color::DarkGray)
            .bg(VOID_BLACK)
    }

    pub fn input_normal() -> Style {
        Style::default()
            .fg(GHOST_WHITE)
            .bg(Color::Rgb(20, 20, 30))
    }

    pub fn input_insert() -> Style {
        Style::default()
            .fg(GHOST_WHITE)
            .bg(Color::Rgb(30, 40, 50))
            .add_modifier(Modifier::BOLD)
    }

    pub fn user_bubble() -> Style {
        Style::default().fg(AMETHYST)
    }

    pub fn chyren_bubble() -> Style {
        Style::default().fg(CORE_CYAN)
    }

    pub fn adccl_pass() -> Style {
        Style::default().fg(SAGE)
    }

    pub fn adccl_fail() -> Style {
        Style::default().fg(VERMILLION)
    }

    pub fn agent_idle() -> Style {
        Style::default().fg(SAGE)
    }

    pub fn agent_busy() -> Style {
        Style::default().fg(EMBER)
    }

    pub fn agent_offline() -> Style {
        Style::default().fg(VERMILLION)
    }

    pub fn telemetry_info() -> Style {
        Style::default().fg(CORE_CYAN)
    }

    pub fn telemetry_warn() -> Style {
        Style::default().fg(EMBER)
    }

    pub fn telemetry_critical() -> Style {
        Style::default()
            .fg(VERMILLION)
            .add_modifier(Modifier::BOLD)
    }

    pub fn border() -> Style {
        Style::default().fg(NEURAL_BLUE)
    }

    pub fn border_active() -> Style {
        Style::default().fg(CORE_CYAN).add_modifier(Modifier::BOLD)
    }
}
