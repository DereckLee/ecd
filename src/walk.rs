use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use globset::{Glob, GlobSetBuilder};

use crate::cli::CheckArgs;

pub fn collect_paths(args: &CheckArgs) -> Result<Vec<PathBuf>> {
    if args.files.is_empty() && args.dirs.is_empty() {
        anyhow::bail!("no input: provide at least one --file (-f) or --directory (-d)");
    }

    let pattern = build_globset(&args.pattern)?;
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

    for dir in &args.dirs {
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
        walk_dir(&path, &pattern, &excludes, &mut paths)?;
    }

    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn build_globset(pattern: &str) -> Result<globset::GlobSet> {
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new(pattern).context("invalid glob pattern")?);
    builder.build().context("failed to compile glob pattern")
}

fn matches_pattern(set: &globset::GlobSet, path: &Path) -> bool {
    set.is_match(path) || set.is_match(path.file_name().unwrap_or_default())
}

fn walk_dir(
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
