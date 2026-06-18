use std::io::{self, IsTerminal};

use clap::ValueEnum;

/// When to colorize encoding labels in batch output.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum ColorWhen {
    /// Colorize when stdout is a terminal and `NO_COLOR` is unset
    Auto,
    /// Never colorize (default)
    #[default]
    Never,
    /// Always colorize (ANSI escapes; useful for piping to `less -R`)
    Always,
}

impl ColorWhen {
    pub fn enabled(self) -> bool {
        match self {
            Self::Never => false,
            Self::Always => true,
            Self::Auto => io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none(),
        }
    }
}

const RESET: &str = "\x1b[0m";

/// Standard 16-color ANSI codes (bright foreground).
const ANSI_PALETTE: &[u8] = &[91, 92, 93, 94, 95, 96, 31, 32, 33, 34, 35, 36];

/// Wrap `label` (e.g. `[UTF-8]` or `[SKIP]`) with ANSI color; path stays unstyled.
pub fn paint_label(label: &str, encoding_key: &str, when: ColorWhen) -> String {
    if !when.enabled() {
        return label.to_string();
    }

    let (r, g, b) = if encoding_key == "skip" {
        (120, 120, 120)
    } else {
        encoding_rgb(encoding_key)
    };

    if use_truecolor() {
        format!("\x1b[38;2;{r};{g};{b}m{label}{RESET}")
    } else {
        let code = ANSI_PALETTE[stable_index(encoding_key) % ANSI_PALETTE.len()];
        format!("\x1b[{code}m{label}{RESET}")
    }
}

fn use_truecolor() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if let Ok(v) = std::env::var("COLORTERM") {
        let v = v.to_ascii_lowercase();
        if v.contains("truecolor") || v.contains("24bit") {
            return true;
        }
    }
    std::env::var("TERM")
        .is_ok_and(|t| t.contains("256color") || t.contains("xterm") || t.contains("kitty"))
}

fn stable_index(key: &str) -> usize {
    key.bytes().fold(0usize, |acc, b| {
        acc.wrapping_mul(31).wrapping_add(b as usize)
    })
}

/// Stable hue gradient: one encoding → one color.
fn encoding_rgb(encoding: &str) -> (u8, u8, u8) {
    let norm = encoding.to_ascii_lowercase();
    let idx = stable_index(&norm);
    let hue = (idx % 360) as f32;
    hsl_to_rgb(hue, 0.72, 0.58)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (rp, gp, bp) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (
        ((rp + m) * 255.0).round() as u8,
        ((gp + m) * 255.0).round() as u8,
        ((bp + m) * 255.0).round() as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_disables_color() {
        assert!(!ColorWhen::Never.enabled());
    }

    #[test]
    fn always_enables_color() {
        assert!(ColorWhen::Always.enabled());
    }

    #[test]
    fn plain_when_never() {
        assert_eq!(paint_label("[UTF-8]", "utf-8", ColorWhen::Never), "[UTF-8]");
    }

    #[test]
    fn stable_encoding_color() {
        assert_eq!(encoding_rgb("utf-8"), encoding_rgb("utf-8"));
        assert_ne!(encoding_rgb("utf-8"), encoding_rgb("gbk"));
    }
}
