use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::color::ColorWhen;

pub const DEFAULT_EXCLUDES: &[&str] = &[".git", "node_modules", "target"];

pub const AFTER_HELP: &str = "\
EXAMPLES:
  ecd check -f man.txt              Detect encoding of one file
  ecd check -d ./src                Scan directory (top level only)
  ecd check -r -p \"*.java\"          Find Java files under current directory
  ecd check -r -d . -p \"*.java\"     Recursively find Java files in a path
  ecd check -d . -p \"*.rs\"          Filter by glob pattern
  ecd check -f a.txt -f b.txt       Multiple files
  ecd check -d . -i ascii -v        Skip ASCII, show stats
";

#[derive(Parser, Debug)]
#[command(
    name = "ecd",
    version,
    about = "Detect text file character encodings",
    long_about = "Detect text file character encodings.\n\n\
                  Supports 38 encodings including UTF-8, GBK, Big5, Shift_JIS, and Windows code pages.\n\
                  Scan files or directories in parallel; single-file mode prints the encoding name only.",
    after_help = AFTER_HELP
)]
pub struct Cli {
    /// When to colorize encoding labels (`[UTF-8]`, `[SKIP]`, …) in batch output
    #[arg(
        long = "color",
        value_name = "WHEN",
        default_value = "never",
        global = true
    )]
    pub color: ColorWhen,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Detect file encoding(s)
    Check(CheckArgs),
}

#[derive(Args, Debug, Clone)]
pub struct CheckArgs {
    /// File(s) to check (repeatable)
    #[arg(short = 'f', long = "file", value_name = "PATH")]
    pub files: Vec<PathBuf>,

    /// Directory/directories to scan (repeatable); defaults to `.` when `-r` or `-p` is given alone
    #[arg(short = 'd', long = "dir", value_name = "PATH")]
    pub dirs: Vec<PathBuf>,

    /// Recursively scan into subdirectories (used with `-d`, or current directory when `-d` is omitted)
    #[arg(short = 'r', long = "recursive")]
    pub recursive: bool,

    /// Glob pattern to filter files (`*.java`; with `-r` matches at any depth)
    #[arg(short = 'p', long = "pattern", value_name = "GLOB")]
    pub pattern: Option<String>,

    /// Ignore files with this encoding (case-insensitive)
    #[arg(short = 'i', long = "ignore", value_name = "ENC")]
    pub ignore_encoding: Option<String>,

    /// Additional directory names to exclude
    #[arg(short = 'e', long = "exclude", value_name = "NAME")]
    pub excludes: Vec<String>,

    /// Disable default directory excludes (.git, node_modules, target)
    #[arg(long = "no-default-excludes")]
    pub no_default_excludes: bool,

    /// Number of parallel jobs
    #[arg(short = 'j', long = "jobs", value_name = "N")]
    pub jobs: Option<usize>,

    /// Print statistics to stderr
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Suppress normal output (errors only)
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,
}

impl CheckArgs {
    pub fn effective_excludes(&self) -> Vec<String> {
        let mut excludes = Vec::new();
        if !self.no_default_excludes {
            excludes.extend(DEFAULT_EXCLUDES.iter().map(|s| (*s).to_string()));
        }
        excludes.extend(self.excludes.clone());
        excludes
    }

    /// Glob pattern for file filtering; `*` when `-p` is omitted.
    pub fn effective_pattern(&self) -> &str {
        self.pattern.as_deref().unwrap_or("*")
    }

    /// Scan roots: explicit `-d` paths, or `.` when `-r`/`-p` is used without `-d`.
    pub fn effective_dirs(&self) -> Vec<PathBuf> {
        if !self.dirs.is_empty() {
            return self.dirs.clone();
        }
        if self.files.is_empty() && (self.recursive || self.pattern.is_some()) {
            vec![PathBuf::from(".")]
        } else {
            vec![]
        }
    }

    pub fn is_single_file_mode(&self) -> bool {
        self.files.len() == 1 && self.effective_dirs().is_empty()
    }
}
