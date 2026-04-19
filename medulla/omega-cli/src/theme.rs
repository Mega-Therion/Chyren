//! Neon backlit terminal theme.
//!
//! Mirrors the RWFY banner SVG: `#000103` deep-space black background,
//! full-spectrum HSL color-shift (0°→360° cycling) for headers and
//! gradient elements, complementary neon accents for status/scores,
//! ghost-white dim text for secondary labels.
//!
//! All output is gated on `is_color_enabled()` — piped or `--json`
//! invocations receive clean plain text with no escape sequences.

// ── Color-enable gate ────────────────────────────────────────────────────────

pub fn is_color_enabled() -> bool {
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }
    let term = std::env::var("TERM").unwrap_or_default();
    if term == "dumb" || term.is_empty() {
        return false;
    }
    true
}

// ── ANSI primitives ──────────────────────────────────────────────────────────

const R: &str = "\x1b[0m";      // reset
const B: &str = "\x1b[1m";      // bold
const DM: &str = "\x1b[2m";     // dim

fn fg(n: u8) -> String {
    format!("\x1b[38;5;{n}m")
}

fn paint(text: &str, n: u8) -> String {
    if is_color_enabled() { format!("{}{}{}", fg(n), text, R) } else { text.to_string() }
}

fn paint_bold(text: &str, n: u8) -> String {
    if is_color_enabled() { format!("{}{}{}{}", B, fg(n), text, R) } else { text.to_string() }
}

// ── Spectrum (HSL 180°→0°→120° neon palette) ────────────────────────────────
//
// Derived from the SVG banner's `color-cycle` keyframes:
//   0%  → hsl(0,   100%, 50%) red
//   50% → hsl(180, 100%, 50%) cyan
//  100% → hsl(360, 100%, 50%) red (full cycle)
//
// We represent this as a static sequence of 256-color indices so the
// terminal gradient reads left-to-right as if the SVG animation were
// frozen at a pleasing point in the cycle.

const SPECTRUM: &[u8] = &[
    51,  // neon cyan      HSL ~180° — banner primary
    45,  // cyan-sky
    39,  // sky blue
    33,  // royal blue
    27,  // electric blue
    93,  // blue-violet
    129, // violet
    165, // violet-magenta
    201, // hot magenta     HSL ~300°
    207, // pink-magenta
    213, // soft pink
    219, // pale rose
    225, // blush
    226, // neon yellow     HSL ~60°
    220, // amber
    214, // orange
    208, // deep orange
    202, // red-orange
    196, // neon red        HSL ~0°
    154, // yellow-green
    118, // lime
    82,  // neon lime
    46,  // neon green      HSL ~120°
    47,  // teal-green
    48,  // green-cyan
];

/// Apply a left-to-right spectrum gradient, one color per character.
/// `offset` shifts the start position so adjacent gradient calls can
/// flow continuously.
pub fn gradient(text: &str, offset: usize) -> String {
    if !is_color_enabled() {
        return text.to_string();
    }
    let mut out = String::new();
    for (i, ch) in text.chars().enumerate() {
        let idx = (offset + i) % SPECTRUM.len();
        out.push_str(&format!("\x1b[38;5;{}m{}", SPECTRUM[idx], ch));
    }
    out.push_str(R);
    out
}

// ── Semantic color helpers ────────────────────────────────────────────────────

/// Secondary label text — dim gray, recedes into the background.
pub fn label(s: &str) -> String {
    if is_color_enabled() { format!("{}{}{}", DM, s, R) } else { s.to_string() }
}

/// Primary value text — bright white.
pub fn value(s: &str) -> String {
    paint(s, 255)
}

/// ADCCL score: neon green ≥0.7, amber 0.4–0.7, red <0.4.
pub fn score(v: f64) -> String {
    let text = format!("{v:.3}");
    if v >= 0.7       { paint_bold(&text, 46)  }
    else if v >= 0.4  { paint_bold(&text, 226) }
    else              { paint_bold(&text, 196) }
}

/// Provider/model name — electric blue.
pub fn provider(s: &str) -> String {
    paint_bold(s, 39)
}

/// Verified / success status — neon green.
pub fn ok(s: &str) -> String {
    paint_bold(s, 46)
}

/// Rejected / failure status — neon red.
pub fn fail(s: &str) -> String {
    paint_bold(s, 196)
}

/// Warning / in-progress status — amber.
pub fn warn(s: &str) -> String {
    paint_bold(s, 226)
}

/// Tier badge — each tier gets its own spectrum position.
pub fn tier(label_str: &str) -> String {
    if label_str.contains("TIER-0") || label_str.contains("tier_0") {
        paint_bold(label_str, 51)   // cyan   — initial local
    } else if label_str.contains("TIER-1") || label_str.contains("tier_1") {
        paint_bold(label_str, 201)  // magenta — upshift
    } else if label_str.contains("TIER-2") || label_str.contains("tier_2") {
        paint_bold(label_str, 226)  // yellow  — council
    } else if label_str.contains("TERMINAL") || label_str.contains("FAIL") {
        paint_bold(label_str, 196)  // red     — terminal failure
    } else {
        paint(label_str, 245)
    }
}



/// Informational text — neon cyan, matching banner primary.
pub fn info(s: &str) -> String {
    paint(s, 51)
}

/// Ghost-signature text — very dim, near-invisible like the SVG signature.
pub fn ghost(s: &str) -> String {
    if is_color_enabled() { format!("{}\x1b[38;5;237m{}{}", DM, s, R) } else { s.to_string() }
}

/// Run-ID / hash — dim purple.
pub fn run_id(s: &str) -> String {
    paint(s, 93)
}

// ── Dividers ─────────────────────────────────────────────────────────────────

/// Full-width gradient divider — heavy line using spectrum colors.
pub fn divider(width: usize) -> String {
    if !is_color_enabled() {
        return "─".repeat(width);
    }
    let mut out = String::new();
    for i in 0..width {
        let idx = (i * SPECTRUM.len() / width.max(1)) % SPECTRUM.len();
        out.push_str(&format!("\x1b[38;5;{}m─", SPECTRUM[idx]));
    }
    out.push_str(R);
    out
}

/// Subtle dotted separator — stays in the cyan-blue end of the spectrum.
pub fn thin_div(width: usize) -> String {
    if !is_color_enabled() {
        return "·".repeat(width);
    }
    let mut out = String::new();
    for i in 0..width {
        let idx = i % 8; // stay in 51→27 (cyan→blue range)
        out.push_str(&format!("\x1b[38;5;{}m·", SPECTRUM[idx]));
    }
    out.push_str(R);
    out
}

// ── Banner ────────────────────────────────────────────────────────────────────
//
// Block-letter CHYREN in the Orbitron-style full-width caps from the SVG.
// Six rows each assigned a distinct spectrum hue so the logo reads as a
// top-to-bottom gradient from neon cyan → electric blue → violet → magenta.

const BANNER: &[&str] = &[
    " ██████╗ ██╗  ██╗██╗   ██╗██████╗ ███████╗███╗   ██╗",
    "██╔════╝ ██║  ██║╚██╗ ██╔╝██╔══██╗██╔════╝████╗  ██║",
    "██║      ███████║ ╚████╔╝ ██████╔╝█████╗  ██╔██╗ ██║",
    "██║      ██╔══██║  ╚██╔╝  ██╔══██╗██╔══╝  ██║╚██╗██║",
    "╚██████╗ ██║  ██║   ██║   ██║  ██║███████╗██║ ╚████║",
    " ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝",
];

// Per-row hues: cyan → sky-blue → electric-blue → violet → magenta → hot-magenta
const ROW_COLORS: &[u8] = &[51, 45, 39, 93, 165, 201];

pub fn print_banner() {
    let width = BANNER[0].chars().count() + 4;

    eprintln!();
    for (i, row) in BANNER.iter().enumerate() {
        let color = ROW_COLORS[i % ROW_COLORS.len()];
        eprintln!("  {}", paint_bold(row, color));
    }

    // Tagline — full gradient sweep across the full banner width
    let tagline = "  ·  S O V E R E I G N   I N T E L L I G E N C E   O R C H E S T R A T O R  ·";
    eprintln!("{}", gradient(tagline, 4));

    // Yettragrammaton ghost signature — right-aligned, barely visible
    let sig = "R.W.Ϝ.Y.";
    let pad_width = width.saturating_sub(sig.chars().count());
    eprintln!("{}{}", " ".repeat(pad_width), ghost(sig));

    eprintln!("{}", thin_div(width));
    eprintln!();
}

// ── Response block ────────────────────────────────────────────────────────────

/// Print a full task response with a neon left-gutter border and subtle formatting.
pub fn print_response(response_text: &str) {
    if !is_color_enabled() {
        println!("{}", response_text);
        return;
    }
    // Left gutter — spectrum gradient stripe, one char per line
    let lines: Vec<&str> = response_text.lines().collect();
    let n = lines.len();
    for (i, line) in lines.iter().enumerate() {
        let idx = (i * SPECTRUM.len() / n.max(1)) % SPECTRUM.len();
        println!(
            "\x1b[38;5;{}m▌\x1b[0m {}",
            SPECTRUM[idx], line
        );
    }
}

// ── Structured result block ───────────────────────────────────────────────────

pub fn print_result_header(run_id_str: &str, status_str: &str, adccl: f64, provider_str: &str) {
    let width = 60;
    println!("{}", divider(width));
    println!(
        "  {}  {}    {}  {}    {}  {}    {}  {}",
        label("RUN"),    run_id(run_id_str),
        label("STATUS"), if status_str.contains("Completed") || status_str.contains("verified") {
            ok(status_str)
        } else {
            fail(status_str)
        },
        label("ADCCL"),  score(adccl),
        label("VIA"),    provider(provider_str),
    );
    println!("{}", divider(width));
    println!();
}

pub fn print_status_block(
    system_status: &str,
    dream_episodes: usize,
    top_pattern: Option<&str>,
) {
    let width = 60;
    println!("{}", divider(width));
    println!("  {}  {}", label("SYSTEM"), ok(system_status));
    println!("  {}  {}", label("SEAL  "), gradient("R.W.Ϝ.Y.", 0));
    println!("  {}  {}", label("DREAM "), value(&format!("{dream_episodes} failure episodes")));
    if let Some(pat) = top_pattern {
        println!("  {}  {}", label("TOP Δ "), warn(pat));
    }
    println!("{}", thin_div(width));
    println!();
}

pub fn print_insights(insights: &[String]) {
    let width = 60;
    println!("{}", divider(width));
    println!("  {}", paint_bold("METACOGNITIVE REFLECTION / BOOT EPIPHANIES", 51));
    println!("{}", divider(width));
    
    for (i, insight) in insights.iter().enumerate() {
        let idx = (i * SPECTRUM.len() / insights.len().max(1)) % SPECTRUM.len();
        println!("  {} {}", paint("◈", SPECTRUM[idx]), insight);
    }
    
    if insights.is_empty() {
        println!("  {}", label("No significant cognitive drifts detected in this boot cycle."));
    }
    
    println!("{}", thin_div(width));
    println!();
}
