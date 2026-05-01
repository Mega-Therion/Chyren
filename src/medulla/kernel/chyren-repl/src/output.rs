//! Streaming renderer: syntax highlighting for code blocks, markdown formatting.

use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::{execute, queue};
use std::io::{stdout, Write};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;

pub struct OutputRenderer {
    ps: SyntaxSet,
    ts: ThemeSet,
    in_code_block: bool,
    code_lang: String,
    code_buf: String,
}

impl OutputRenderer {
    pub fn new() -> Self {
        Self {
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
            in_code_block: false,
            code_lang: String::new(),
            code_buf: String::new(),
        }
    }

    /// Feed a raw text fragment (as streamed from SSE).
    pub fn feed(&mut self, fragment: &str) {
        // Collect into a line buffer so we can detect fences
        let combined = format!("{}{}", self.code_buf_pending(), fragment);
        self.render_text(&combined);
    }

    fn code_buf_pending(&self) -> &str {
        ""
    }

    fn render_text(&mut self, text: &str) {
        for line in text.lines() {
            self.render_line(line);
            // lines() strips the newline; re-emit it
            let _ = execute!(stdout(), Print("\n"));
        }
        // If the fragment didn't end with \n, print without newline
        if !text.ends_with('\n') && !text.is_empty() {
            // already rendered inline — nothing extra needed
        }
        let _ = stdout().flush();
    }

    fn render_line(&mut self, line: &str) {
        if line.starts_with("```") {
            if self.in_code_block {
                // Flush + close block
                self.flush_code_block();
                self.in_code_block = false;
                self.code_lang.clear();
            } else {
                self.in_code_block = true;
                self.code_lang = line.trim_start_matches('`').to_string();
            }
            return;
        }

        if self.in_code_block {
            self.code_buf.push_str(line);
            self.code_buf.push('\n');
            // Render line immediately with syntax highlighting
            self.render_code_line(line);
            return;
        }

        // Markdown-lite rendering
        self.render_prose_line(line);
    }

    fn render_code_line(&self, line: &str) {
        let lang = if self.code_lang.is_empty() {
            "plain"
        } else {
            &self.code_lang
        };
        let syntax = self
            .ps
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| self.ps.find_syntax_plain_text());
        let theme = &self.ts.themes["base16-ocean.dark"];
        let mut h = HighlightLines::new(syntax, theme);

        let highlighted: Vec<(Style, &str)> = h
            .highlight_line(line, &self.ps)
            .unwrap_or_else(|_| vec![(Style::default(), line)]);

        let _ = queue!(stdout(), SetForegroundColor(Color::DarkGrey), Print("  "));
        for (style, text) in &highlighted {
            let r = style.foreground.r;
            let g = style.foreground.g;
            let b = style.foreground.b;
            let _ = queue!(
                stdout(),
                SetForegroundColor(Color::Rgb { r, g, b }),
                Print(text)
            );
        }
        let _ = execute!(stdout(), ResetColor);
    }

    fn flush_code_block(&mut self) {
        self.code_buf.clear();
        let _ = execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("  ─────────────────────────────"),
            ResetColor,
            Print("\n")
        );
    }

    fn render_prose_line(&self, line: &str) {
        if line.starts_with("# ") {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Cyan),
                Print(line.trim_start_matches("# ")),
                ResetColor
            );
        } else if line.starts_with("## ") {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Blue),
                Print(line.trim_start_matches("## ")),
                ResetColor
            );
        } else if line.starts_with("### ") {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Magenta),
                Print(line.trim_start_matches("### ")),
                ResetColor
            );
        } else if line.starts_with("- ") || line.starts_with("* ") {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::DarkGrey),
                Print("  • "),
                ResetColor,
                Print(&line[2..])
            );
        } else if line.starts_with("> ") {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::DarkYellow),
                Print("  ▌ "),
                Print(&line[2..]),
                ResetColor
            );
        } else {
            // Inline code: simple pass-through with colour for `backtick` spans
            print_with_inline_code(line);
        }
    }

    /// Flush any open code block at end of response.
    pub fn finish(&mut self) {
        if self.in_code_block {
            self.flush_code_block();
            self.in_code_block = false;
        }
        let _ = stdout().flush();
    }
}

fn print_with_inline_code(line: &str) {
    let mut rest = line;
    while let Some(start) = rest.find('`') {
        let before = &rest[..start];
        let _ = execute!(stdout(), Print(before));
        rest = &rest[start + 1..];
        if let Some(end) = rest.find('`') {
            let code = &rest[..end];
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(code),
                ResetColor
            );
            rest = &rest[end + 1..];
        } else {
            let _ = execute!(stdout(), Print("`"), Print(rest));
            return;
        }
    }
    let _ = execute!(stdout(), Print(rest));
}

/// Print a dimmed status bar line (run_id, score, provider).
pub fn print_meta(run_id: &str, score: f64, stream: bool) {
    let mode = if stream { "stream" } else { "sync" };
    let _ = execute!(
        stdout(),
        Print("\n"),
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "  ─ run:{run_id}  adccl:{score:.3}  mode:{mode}\n"
        )),
        ResetColor
    );
}

pub fn print_error(msg: &str) {
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        Print(format!("  [ERROR] {msg}\n")),
        ResetColor
    );
}

pub fn print_info(msg: &str) {
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  {msg}\n")),
        ResetColor
    );
}
