//! Chyren "Glassmorphism" TrueColor Theme.
//!
//! Replicating the high-end aesthetic of Claude Code and Codex:
//! - 24-bit TrueColor (RGB) for subtle glows and transparency simulations.
//! - Rounded double-line borders.
//! - Braille-simulated "glass" transparency and parallax.
//! - Streaming "typewriter" response rendering.

use std::io::{stdout, Write};
use termimad::MadSkin;

// ── Glass Colors (Hex) ───────────────────────────────────────────────────────

pub const CORE_CYAN: &str = "#00f5ff";
pub const NEURAL_BLUE: &str = "#3366ff";
#[allow(dead_code)]
pub const VOID_BLACK: &str = "#000103"; // Deep space black
pub const GHOST_WHITE: &str = "#e0e0e0";
#[allow(dead_code)]
pub const GLASS_GLOW: &str = "#1a1a2e"; // Subtle blue-gray background tint

// ── ANSI TrueColor Helpers ───────────────────────────────────────────────────

fn hex_fg(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

#[allow(dead_code)]
fn hex_bg(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}

const R: &str = "\x1b[0m";
const B: &str = "\x1b[1m";
const D: &str = "\x1b[2m";

pub fn is_color_enabled() -> bool {
    std::env::var("NO_COLOR").is_err() && std::env::var("TERM").unwrap_or_default() != "dumb"
}

// ── Spectrum ────────────────────────────────────────────────────────────────

const PALETTE: &[&str] = &[
    "#00f5ff", "#00d1ff", "#00adff", "#3366ff", "#5e33ff", "#8a33ff", "#b533ff", "#e033ff",
    "#ff33e0", "#ff33b5", "#ff338a", "#ff335e", "#ff3333", "#ff5e33", "#ff8a33", "#ffb533",
    "#ffe033", "#e0ff33", "#b5ff33", "#8aff33", "#5eff33", "#33ff33", "#33ff5e", "#33ff8a",
    "#33ffb5",
];

pub fn gradient(text: &str, offset: usize) -> String {
    if !is_color_enabled() {
        return text.to_string();
    }
    let mut out = String::new();
    let n = PALETTE.len();

    for (i, ch) in text.chars().enumerate() {
        // Use a slower shift (offset / 4) and linear interpolation-like effect
        let idx = ((offset / 4) + i) % n;
        let hex = PALETTE[idx];
        out.push_str(&hex_fg(hex));
        out.push(ch);
    }
    out.push_str(R);
    out
}

// ── Glass Container (Glassmorphism Effect) ──────────────────────────────────

pub fn glass_border(color: &str) -> String {
    hex_fg(color).to_string()
}

/*
pub fn glass_container(width: usize, color: &str) -> String {
    // 3px border, inside container with VOID_BLACK
    format!("{}{}{} {} ", hex_fg(color), "█".repeat(width), R, hex_bg(VOID_BLACK))
}

pub fn render_glass(width: usize, color: &str, content: &str) -> String {
    // Outer Border (Neon color)
    let border = format!("{}{}", hex_fg(color), "█".repeat(width));
    // Inner Container (Void Black + Transparency Simulation)
    let inner_width = width.saturating_sub(4);
    let inner = format!("{}{}{}", hex_bg(VOID_BLACK), " ".repeat(inner_width), R);

    format!("{}\n{}{}{}\n{}", border, hex_fg(color), "█", inner, hex_fg(color))
}
*/

pub fn parallax_dot(i: usize) -> char {
    let frames = ['⠂', '⠶', '⠴', '⠾', '⠶', '⠂'];
    frames[i % frames.len()]
}

/*
pub fn glass_divider(width: usize) -> String {
    if !is_color_enabled() { return "─".repeat(width); }
    let mut out = String::new();
    out.push_str(&hex_fg("#2a2a2a"));
    for i in 0..width {
        out.push(parallax_dot(i));
    }
    out.push_str(R);
    out
}
*/

pub fn box_top(width: usize, title: &str) -> String {
    let t_len = title.chars().count();
    let side = width.saturating_sub(t_len + 4) / 2;
    format!(
        "{}╭{} {} {}╮{}",
        glass_border(CORE_CYAN),
        "─".repeat(side),
        gradient(title, 0),
        "─".repeat(side),
        R
    )
}

pub fn box_bottom(width: usize) -> String {
    format!(
        "{}╰{}╯{}",
        glass_border(CORE_CYAN),
        "─".repeat(width.saturating_sub(2)),
        R
    )
}

// ── Semantic styles ─────────────────────────────────────────────────────────

pub fn label(s: &str) -> String {
    format!("{D}{s}{R}")
}
pub fn val(s: &str) -> String {
    format!("{B}{}{s}{R}", hex_fg(GHOST_WHITE))
}
pub fn cyan(s: &str) -> String {
    format!("{}{s}{R}", hex_fg(CORE_CYAN))
}
pub fn ok(s: &str) -> String {
    format!("{B}{}{s}{R}", hex_fg("#00ff66"))
}
pub fn fail(s: &str) -> String {
    format!("{B}{}{s}{R}", hex_fg("#ff3333"))
}
pub fn warn(s: &str) -> String {
    format!("{B}{}{s}{R}", hex_fg("#ffcc00"))
}
pub fn info(s: &str) -> String {
    format!("{}{s}{R}", hex_fg("#00ccff"))
}

pub fn score(v: f64) -> String {
    let text = format!("{v:.3}");
    if v >= 0.7 {
        ok(&text)
    } else if v >= 0.4 {
        warn(&text)
    } else {
        fail(&text)
    }
}

// ── Banner ────────────────────────────────────────────────────────────────────

const BANNER: &[&str] = &[
    " ██████╗ ██╗  ██╗██╗   ██╗██████╗ ███████╗███╗   ██╗",
    "██╔════╝ ██║  ██║╚██╗ ██╔╝██╔══██╗██╔════╝████╗  ██║",
    "██║      ███████║ ╚████╔╝ ██████╔╝█████╗  ██╔██╗ ██║",
    "██║      ██╔══██║  ╚██╔╝  ██╔══██╗██╔══╝  ██║╚██╗██║",
    "╚██████╗ ██║  ██║   ██║   ██║  ██║███████╗██║ ╚████║",
    " ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝",
];

pub fn print_banner() {
    println!();
    for (i, line) in BANNER.iter().enumerate() {
        println!("  {}", gradient(line, i * 2));
    }
    println!(
        "  {}",
        gradient(
            "      S O V E R E I G N   I N T E L L I G E N C E   O R C H E S T R A T O R",
            5
        )
    );
    println!("  {:>50}", label("R.W.Ϝ.Y. / v1.0.0"));
    println!();
}

// ── Interactive UI ───────────────────────────────────────────────────────────

pub fn prompt() -> String {
    format!(
        "{} chyren {}❯{} ",
        hex_fg(CORE_CYAN),
        hex_fg(NEURAL_BLUE),
        R
    )
}

#[allow(dead_code)]
pub fn print_thinking(msg: &str, frame: usize) {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let spinner = gradient(frames[frame % frames.len()], frame);
    print!("\r  {} {} {}  ", spinner, hex_fg("#444444"), info(msg));
    stdout().flush().ok();
}

pub fn print_markdown(text: &str) {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(termimad::rgb(0, 245, 255)); // Core Cyan
    skin.bold.set_fg(termimad::rgb(224, 224, 224)); // Ghost White
    skin.italic.set_fg(termimad::rgb(51, 102, 255)); // Neural Blue
    skin.inline_code.set_bg(termimad::rgb(26, 26, 46)); // Glass Glow
    skin.code_block.set_bg(termimad::rgb(26, 26, 46)); // Glass Glow
    skin.code_block.set_fg(termimad::rgb(224, 224, 224));

    // Custom list bullet
    skin.bullet.set_fg(termimad::rgb(0, 245, 255));

    skin.print_text(text);
}

pub fn print_response(text: &str) {
    println!("  {}", box_top(70, "CHYREN RESPONSE"));
    // We use a custom indentation approach for markdown
    let mut skin = MadSkin::default();
    skin.set_headers_fg(termimad::rgb(0, 245, 255));
    skin.bold.set_fg(termimad::rgb(224, 224, 224));
    skin.italic.set_fg(termimad::rgb(51, 102, 255));
    skin.inline_code.set_bg(termimad::rgb(26, 26, 46));
    skin.code_block.set_bg(termimad::rgb(26, 26, 46));

    // Print lines with the border
    for line in text.lines() {
        println!("  {}│ {}", hex_fg("#333333"), line);
    }
    println!("  {}", box_bottom(70));
}

// ── Compatibility Layer ──────────────────────────────────────────────────────

pub fn value(s: &str) -> String {
    val(s)
}
pub fn run_id(s: &str) -> String {
    format!("{}{s}{R}", hex_fg("#8a33ff"))
}
pub fn tier(label_str: &str) -> String {
    let hex = if label_str.contains("0") {
        "#00f5ff"
    } else if label_str.contains("1") {
        "#ff33e0"
    } else if label_str.contains("2") {
        "#ffe033"
    } else {
        "#ff3333"
    };
    format!("{B}{}{label_str}{R}", hex_fg(hex))
}

pub fn parallax_spacer(width: usize) -> String {
    let mut out = String::new();
    out.push_str(&hex_fg("#1a1a1a"));
    for i in 0..width {
        out.push(parallax_dot(i));
    }
    out.push_str(R);
    out
}

pub fn print_status_block(status: &str, dream_n: usize, top: Option<&str>) {
    let width = 64;
    println!("  {}", box_top(width, "SYSTEM STATUS"));
    println!(
        "  {}│  {}  {}",
        hex_fg("#333333"),
        label("SYSTEM"),
        ok(status)
    );
    println!(
        "  {}│  {}  {}",
        hex_fg("#333333"),
        label("SEAL  "),
        gradient("R.W.Ϝ.Y.", 0)
    );
    println!(
        "  {}│  {}  {}",
        hex_fg("#333333"),
        label("DREAM "),
        val(&format!("{dream_n} failure episodes"))
    );
    if let Some(p) = top {
        println!("  {}│  {}  {}", hex_fg("#333333"), label("TOP Δ "), warn(p));
    }
    println!("  {}", box_bottom(width));
}

pub fn print_insights(insights: &[String]) {
    let width = 64;
    println!("  {}", box_top(width, "METALOGICAL REFLECTION"));
    for (i, msg) in insights.iter().enumerate() {
        println!("  {}│  {} {}", hex_fg("#333333"), gradient("◈", i), msg);
    }
    println!("  {}", box_bottom(width));
}

pub fn print_result_header(run_id: &str, status: &str, score_v: f64, provider: &str) {
    print_execution_metrics(run_id, status, score_v, provider);
}

// DEPRECATED: use the updated print_response above

#[derive(Default)]
pub struct ThinkingAnimation {
    pub frame: usize,
}

impl ThinkingAnimation {
    pub fn next_frame(&mut self) -> String {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let f = self.frame;
        self.frame += 1;
        gradient(frames[f % frames.len()], f)
    }
}

pub fn print_execution_metrics(run_id: &str, status: &str, score_v: f64, provider: &str) {
    let width = 70;
    println!("  {}", box_top(width, "COGNITIVE TRACE"));
    println!(
        "  {}│ {} {}  {} {}  {} {}  {} {}",
        hex_fg("#333333"),
        label("RUN"),
        val(run_id),
        label("STATUS"),
        if status.contains("Completed") {
            ok(status)
        } else {
            fail(status)
        },
        label("ADCCL"),
        score(score_v),
        label("VIA"),
        cyan(provider)
    );
    println!("  {}", box_bottom(width));
}
