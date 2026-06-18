use std::path::PathBuf;

use anyhow::Context;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "xtask",
    about = "ecd developer tasks",
    disable_version_flag = true
)]
struct Xtask {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate man pages into man/
    Man,
    /// Generate shell completion scripts into completions/
    Completions {
        /// Output directory
        #[arg(long, default_value = "completions")]
        out_dir: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    match Xtask::parse().command {
        Command::Man => generate_man()?,
        Command::Completions { out_dir } => generate_completions(&out_dir)?,
    }
    Ok(())
}

fn generate_man() -> anyhow::Result<()> {
    let out_dir = PathBuf::from("man");
    std::fs::create_dir_all(&out_dir).context("failed to create man directory")?;

    let cmd = ecd::Cli::command();
    clap_mangen::generate_to(cmd, &out_dir).context("failed to generate man page")?;

    println!("generated man pages in {}", out_dir.display());
    Ok(())
}

fn generate_completions(out_dir: &PathBuf) -> anyhow::Result<()> {
    ecd::completions::generate_to_dir(out_dir).context("failed to generate completions")?;
    println!("generated shell completions in {}", out_dir.display());
    Ok(())
}
