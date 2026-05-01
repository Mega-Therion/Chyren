// Catppuccin Mocha palette — sovereign identity mapped to Teal
use ratatui::style::{Color, Modifier, Style};

// ── Base surface ─────────────────────────────────────────────────────────────
pub const BASE:     Color = Color::Rgb(30,  30,  46);
pub const MANTLE:   Color = Color::Rgb(24,  24,  37);
pub const CRUST:    Color = Color::Rgb(17,  17,  27);
pub const SURFACE0: Color = Color::Rgb(49,  50,  68);
pub const SURFACE1: Color = Color::Rgb(69,  71,  90);
pub const SURFACE2: Color = Color::Rgb(88,  91, 112);
pub const OVERLAY0: Color = Color::Rgb(108,112,134);
pub const OVERLAY1: Color = Color::Rgb(127,132,156);
pub const SUBTEXT0: Color = Color::Rgb(166,173,200);
pub const TEXT:     Color = Color::Rgb(205,214,244);

// ── Accent palette ───────────────────────────────────────────────────────────
pub const TEAL:     Color = Color::Rgb(148,226,213);
pub const SAPPHIRE: Color = Color::Rgb(116,199,236);
pub const SKY:      Color = Color::Rgb(137,220,235);
pub const LAVENDER: Color = Color::Rgb(180,190,254);
pub const MAUVE:    Color = Color::Rgb(203,166,247);
pub const PEACH:    Color = Color::Rgb(250,179,135);
pub const YELLOW:   Color = Color::Rgb(249,226,175);
pub const GREEN:    Color = Color::Rgb(166,227,161);
pub const RED:      Color = Color::Rgb(243,139,168);

pub struct Theme;

impl Theme {
    pub fn base_bg()   -> Style { Style::default().bg(BASE) }
    pub fn mantle_bg() -> Style { Style::default().bg(MANTLE) }

    pub fn border()        -> Style { Style::default().fg(SURFACE1).bg(BASE) }
    pub fn border_active() -> Style { Style::default().fg(TEAL).bg(BASE).add_modifier(Modifier::BOLD) }
    pub fn border_dim()    -> Style { Style::default().fg(SURFACE0).bg(BASE) }

    pub fn header_title()     -> Style { Style::default().fg(TEAL).bg(MANTLE).add_modifier(Modifier::BOLD) }
    pub fn header_subtitle()  -> Style { Style::default().fg(OVERLAY1).bg(MANTLE) }
    pub fn header_separator() -> Style { Style::default().fg(SURFACE1).bg(MANTLE) }

    pub fn tab_active()   -> Style { Style::default().fg(TEAL).bg(BASE).add_modifier(Modifier::BOLD) }
    pub fn tab_inactive() -> Style { Style::default().fg(OVERLAY0).bg(MANTLE) }
    pub fn tab_sep()      -> Style { Style::default().fg(SURFACE0).bg(MANTLE) }

    pub fn text()        -> Style { Style::default().fg(TEXT).bg(BASE) }
    pub fn text_muted()  -> Style { Style::default().fg(SUBTEXT0).bg(BASE) }
    pub fn text_dim()    -> Style { Style::default().fg(OVERLAY0).bg(BASE) }
    pub fn text_accent() -> Style { Style::default().fg(TEAL).bg(BASE) }

    pub fn user_gutter()   -> Style { Style::default().fg(LAVENDER).bg(BASE) }
    pub fn user_text()     -> Style { Style::default().fg(TEXT).bg(BASE) }
    pub fn user_meta()     -> Style { Style::default().fg(MAUVE).bg(BASE) }

    pub fn chyren_gutter() -> Style { Style::default().fg(TEAL).bg(BASE).add_modifier(Modifier::BOLD) }
    pub fn chyren_text()   -> Style { Style::default().fg(TEXT).bg(BASE) }
    pub fn chyren_meta()   -> Style { Style::default().fg(SAPPHIRE).bg(BASE) }
    pub fn streaming_text()-> Style { Style::default().fg(SKY).bg(BASE) }

    pub fn pill_online()   -> Style { Style::default().fg(BASE).bg(GREEN).add_modifier(Modifier::BOLD) }
    pub fn pill_offline()  -> Style { Style::default().fg(BASE).bg(RED).add_modifier(Modifier::BOLD) }
    pub fn pill_normal()   -> Style { Style::default().fg(BASE).bg(SAPPHIRE).add_modifier(Modifier::BOLD) }
    pub fn pill_insert()   -> Style { Style::default().fg(BASE).bg(TEAL).add_modifier(Modifier::BOLD) }
    pub fn pill_warn()     -> Style { Style::default().fg(BASE).bg(PEACH).add_modifier(Modifier::BOLD) }
    pub fn pill_provider() -> Style { Style::default().fg(SAPPHIRE).bg(SURFACE0) }
    pub fn pill_label()    -> Style { Style::default().fg(OVERLAY1).bg(MANTLE) }

    pub fn adccl_high()  -> Style { Style::default().fg(GREEN) }
    pub fn adccl_mid()   -> Style { Style::default().fg(YELLOW) }
    pub fn adccl_low()   -> Style { Style::default().fg(RED) }
    pub fn adccl_track() -> Style { Style::default().fg(SURFACE1) }

    pub fn adccl_for_score(score: f64) -> Style {
        if score >= 0.7 { Self::adccl_high() }
        else if score >= 0.5 { Self::adccl_mid() }
        else { Self::adccl_low() }
    }

    pub fn input_normal()        -> Style { Style::default().fg(TEXT).bg(MANTLE) }
    pub fn input_insert()        -> Style { Style::default().fg(TEXT).bg(SURFACE0).add_modifier(Modifier::BOLD) }
    pub fn input_prompt_normal() -> Style { Style::default().fg(OVERLAY1).bg(MANTLE) }
    pub fn input_prompt_insert() -> Style { Style::default().fg(TEAL).bg(SURFACE0).add_modifier(Modifier::BOLD) }

    pub fn telemetry_info()     -> Style { Style::default().fg(SAPPHIRE) }
    pub fn telemetry_warn()     -> Style { Style::default().fg(PEACH) }
    pub fn telemetry_critical() -> Style { Style::default().fg(RED).add_modifier(Modifier::BOLD) }
    pub fn telemetry_debug()    -> Style { Style::default().fg(OVERLAY1) }

    pub fn agent_idle()    -> Style { Style::default().fg(GREEN) }
    pub fn agent_busy()    -> Style { Style::default().fg(PEACH) }
    pub fn agent_offline() -> Style { Style::default().fg(OVERLAY0) }

    pub fn sidebar_title()   -> Style { Style::default().fg(LAVENDER).bg(BASE).add_modifier(Modifier::BOLD) }
    pub fn sidebar_key()     -> Style { Style::default().fg(SUBTEXT0).bg(BASE) }
    pub fn sidebar_value()   -> Style { Style::default().fg(TEXT).bg(BASE) }
    pub fn sidebar_section() -> Style { Style::default().fg(OVERLAY0).bg(BASE) }

    pub fn metric_chi()    -> Style { Style::default().fg(MAUVE).add_modifier(Modifier::BOLD) }
    pub fn metric_chyren() -> Style { Style::default().fg(TEAL).add_modifier(Modifier::BOLD) }

    pub fn highlight()   -> Style { Style::default().fg(BASE).bg(TEAL).add_modifier(Modifier::BOLD) }
    pub fn selected()    -> Style { Style::default().fg(TEXT).bg(SURFACE0).add_modifier(Modifier::BOLD) }

    // Legacy compat aliases used by unmodified tabs
    pub fn header()          -> Style { Self::header_title() }
    pub fn header_inactive() -> Style { Self::header_subtitle() }
    pub fn active_tab()      -> Style { Self::tab_active() }
    pub fn inactive_tab()    -> Style { Self::tab_inactive() }
    pub fn text_default()    -> Style { Self::text() }
    pub fn adccl_pass()      -> Style { Self::adccl_high() }
    pub fn adccl_fail()      -> Style { Self::adccl_low() }
    pub fn user_bubble()     -> Style { Self::user_gutter() }
    pub fn chyren_bubble()   -> Style { Style::default().fg(TEAL) }
}

pub fn adccl_gauge(score: f64, width: usize) -> (String, String, Style) {
    let filled = ((score * width as f64).round() as usize).min(width);
    let track  = width - filled;
    (
        "█".repeat(filled),
        "░".repeat(track),
        Theme::adccl_for_score(score),
    )
}
