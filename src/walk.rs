use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use globset::{Glob, GlobSetBuilder};

use crate::cli::CheckArgs;

pub fn collect_paths(args: &CheckArgs) -> Result<Vec<PathBuf>> {
    let dirs = args.effective_dirs();
    if args.files.is_empty() && dirs.is_empty() {
        anyhow::bail!("no input: provide at least one --file (-f) or --directory (-d)");
    }

    let pattern = build_globset(&effective_pattern(args.effective_pattern(), args.recursive))?;
    let excludes = args.effective_excludes();
    let mut paths = Vec::new();

    for file in &args.files {
        let path = file.clone();
        if !path.exists() {
            anyhow::bail!("path not found: {}", path.display());
        }
        if path.is_file() && matches_pattern(&pattern, &path) {
            paths.push(path);
        }
    }

    for dir in &dirs {
        let path = dir.clone();
        if !path.exists() {
            anyhow::bail!("path not found: {}", path.display());
        }
        if path.is_file() {
            if matches_pattern(&pattern, &path) {
                paths.push(path);
            }
            continue;
        }
        if args.recursive {
            walk_dir_recursive(&path, &pattern, &excludes, &mut paths)?;
        } else {
            walk_dir_shallow(&path, &pattern, &mut paths)?;
        }
    }

    paths.sort();
    paths.dedup();
    Ok(paths)
}

/// Expand user patterns for recursive vs shallow directory scans.
pub fn effective_pattern(pattern: &str, recursive: bool) -> String {
    if recursive {
        if pattern.contains('/') || pattern.starts_with("**") {
            pattern.to_string()
        } else {
            format!("**/{pattern}")
        }
    } else {
        pattern.trim_start_matches("**/").to_string()
    }
}

fn build_globset(pattern: &str) -> Result<globset::GlobSet> {
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new(pattern).context("invalid glob pattern")?);
    builder.build().context("failed to compile glob pattern")
}

fn matches_pattern(set: &globset::GlobSet, path: &Path) -> bool {
    set.is_match(path) || set.is_match(path.file_name().unwrap_or_default())
}

fn walk_dir_shallow(root: &Path, pattern: &globset::GlobSet, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(root).with_context(|| format!("read dir {}", root.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && matches_pattern(pattern, &path) {
            out.push(path);
        }
    }
    Ok(())
}

fn walk_dir_recursive(
    root: &Path,
    pattern: &globset::GlobSet,
    excludes: &[String],
    out: &mut Vec<PathBuf>,
) -> Result<()> {
    let mut builder = ignore::WalkBuilder::new(root);
    builder.hidden(false);
    builder.git_ignore(true);
    builder.git_global(true);
    builder.git_exclude(true);
    builder.ignore(true);

    for name in excludes {
        let name = name.clone();
        builder.filter_entry(move |entry| {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                entry.file_name().to_string_lossy() != name
            } else {
                true
            }
        });
    }

    for result in builder.build() {
        let entry = result.context("directory walk failed")?;
        if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
            continue;
        }
        let path = entry.into_path();
        if matches_pattern(pattern, &path) {
            out.push(path);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_expands_simple_glob() {
        assert_eq!(effective_pattern("*.java", true), "**/*.java");
    }

    #[test]
    fn recursive_keeps_explicit_glob() {
        assert_eq!(effective_pattern("**/*.rs", true), "**/*.rs");
    }

    #[test]
    fn shallow_strips_recursive_prefix() {
        assert_eq!(effective_pattern("**/*.java", false), "*.java");
    }
}
