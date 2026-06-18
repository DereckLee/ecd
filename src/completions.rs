use std::io::Write;
use std::path::Path;

use clap::CommandFactory;
use clap_complete::{Shell, generate, generate_to};

use crate::Cli;

/// Shells for which static completion scripts are generated into `completions/`.
pub const STATIC_SHELLS: [Shell; 3] = [Shell::Bash, Shell::Zsh, Shell::Fish];

fn command() -> clap::Command {
    let mut cmd = Cli::command();
    cmd.set_bin_name("ecd");
    cmd
}

/// Write shell completion script for `shell` to `w`.
pub fn write_completions(shell: Shell, w: &mut dyn Write) {
    let mut cmd = command();
    generate(shell, &mut cmd, "ecd", w);
}

/// Generate bash/zsh/fish completion scripts into `out_dir`.
pub fn generate_to_dir(out_dir: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(out_dir)?;
    let mut cmd = command();
    for shell in STATIC_SHELLS {
        generate_to(shell, &mut cmd, "ecd", out_dir)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn bash_completion_contains_bin_name() {
        let mut buf = Cursor::new(Vec::new());
        write_completions(Shell::Bash, &mut buf);
        let text = String::from_utf8(buf.into_inner()).unwrap();
        assert!(text.contains("ecd"));
    }

    #[test]
    fn generate_to_dir_writes_all_shells() {
        let dir = tempfile::tempdir().unwrap();
        generate_to_dir(dir.path()).unwrap();
        assert!(dir.path().join("ecd.bash").is_file());
        assert!(dir.path().join("_ecd").is_file());
        assert!(dir.path().join("ecd.fish").is_file());
    }
}
