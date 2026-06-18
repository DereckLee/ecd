use std::io::{self, Write};
use std::path::Path;

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
}

pub struct Stats {
    pub checked: usize,
    pub printed: usize,
    pub skipped: usize,
    pub ignored: usize,
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

    for result in results {
        match &result.detection {
            Detection::Encoded(enc) => {
                if should_ignore(config.ignore_encoding, enc) {
                    stats.ignored += 1;
                    continue;
                }
                if !config.quiet {
                    println!("[{}] {}", display_encoding(enc), display_path(&result.path));
                    stats.printed += 1;
                }
            }
            Detection::Skip(_) => {
                stats.skipped += 1;
                if !config.quiet {
                    println!("[SKIP] {}", display_path(&result.path));
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
