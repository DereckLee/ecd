use std::io::{self, Write};
use std::path::Path;

use crate::color::{ColorWhen, paint_label};
use crate::detect::{Detection, SkipReason};

#[derive(Debug, Clone)]
pub struct FileResult {
    pub path: std::path::PathBuf,
    pub detection: Detection,
}

pub struct OutputConfig<'a> {
    pub single_file_mode: bool,
    pub ignore_encoding: Option<&'a str>,
    pub quiet: bool,
    pub verbose: bool,
    pub color: ColorWhen,
}

pub struct Stats {
    pub checked: usize,
    pub printed: usize,
    pub skipped: usize,
    pub ignored: usize,
}

enum BatchLine<'a> {
    Encoded { encoding: &'a str, path: &'a Path },
    Skip { path: &'a Path },
}

pub fn print_results(
    results: &[FileResult],
    config: &OutputConfig<'_>,
    stderr: &mut dyn Write,
) -> io::Result<Stats> {
    let mut stats = Stats {
        checked: results.len(),
        printed: 0,
        skipped: 0,
        ignored: 0,
    };

    if config.single_file_mode {
        if let Some(result) = results.first() {
            match &result.detection {
                Detection::Encoded(enc) => {
                    if should_ignore(config.ignore_encoding, enc) {
                        stats.ignored += 1;
                    } else if !config.quiet {
                        println!("{enc}");
                        stats.printed += 1;
                    }
                }
                Detection::Skip(reason) => {
                    stats.skipped += 1;
                    if config.verbose {
                        writeln!(
                            stderr,
                            "skip {}: {}",
                            result.path.display(),
                            skip_reason_message(reason)
                        )?;
                    }
                }
            }
        }
        return Ok(stats);
    }

    let mut lines: Vec<BatchLine<'_>> = Vec::new();
    for result in results {
        match &result.detection {
            Detection::Encoded(enc) => {
                if should_ignore(config.ignore_encoding, enc) {
                    stats.ignored += 1;
                    continue;
                }
                if !config.quiet {
                    lines.push(BatchLine::Encoded {
                        encoding: enc,
                        path: &result.path,
                    });
                    stats.printed += 1;
                }
            }
            Detection::Skip(_) => {
                stats.skipped += 1;
                if !config.quiet {
                    lines.push(BatchLine::Skip { path: &result.path });
                }
                if config.verbose
                    && let Detection::Skip(reason) = &result.detection
                {
                    writeln!(
                        stderr,
                        "skip {}: {}",
                        result.path.display(),
                        skip_reason_message(reason)
                    )?;
                }
            }
        }
    }

    let label_width = lines
        .iter()
        .map(|line| match line {
            BatchLine::Encoded { encoding, .. } => display_encoding(encoding).len(),
            BatchLine::Skip { .. } => 4, // SKIP
        })
        .max()
        .unwrap_or(0);

    for line in lines {
        match line {
            BatchLine::Encoded { encoding, path } => {
                let inner = format!("{:>label_width$}", display_encoding(encoding));
                let label = format!("[{inner}]");
                let painted = paint_label(&label, encoding, config.color);
                println!("{painted} {}", display_path(path));
            }
            BatchLine::Skip { path } => {
                let inner = format!("{:>label_width$}", "SKIP");
                let label = format!("[{inner}]");
                let painted = paint_label(&label, "skip", config.color);
                println!("{painted} {}", display_path(path));
            }
        }
    }

    Ok(stats)
}

pub fn print_stats(stats: &Stats, stderr: &mut dyn Write) -> io::Result<()> {
    writeln!(
        stderr,
        "checked: {}, printed: {}, skipped: {}, ignored: {}",
        stats.checked, stats.printed, stats.skipped, stats.ignored
    )
}

fn should_ignore(ignore: Option<&str>, encoding: &str) -> bool {
    ignore.is_some_and(|i| i.eq_ignore_ascii_case(encoding))
}

fn display_encoding(enc: &str) -> String {
    enc.to_ascii_uppercase()
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn skip_reason_message(reason: &SkipReason) -> &'static str {
    match reason {
        SkipReason::Empty => "empty file",
        SkipReason::Binary => "binary content",
        SkipReason::Unknown => "encoding unknown",
        SkipReason::ReadError(_) => "read error",
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn label_width_aligns_gbk_after_utf8() {
        let utf8 = format!("[{:>5}]", "UTF-8");
        let gbk = format!("[{:>5}]", "GBK");
        assert_eq!(utf8, "[UTF-8]");
        assert_eq!(gbk, "[  GBK]");
    }
}
