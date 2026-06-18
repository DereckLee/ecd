use std::path::Path;

use anyhow::Context;
use clap::CommandFactory;
use ecd::Cli;

fn main() -> anyhow::Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("man") => generate_man()?,
        _ => {
            eprintln!("usage: cargo xtask man");
            std::process::exit(1);
        }
    }
    Ok(())
}

fn generate_man() -> anyhow::Result<()> {
    let out_dir = Path::new("man");
    std::fs::create_dir_all(out_dir).context("failed to create man directory")?;

    let cmd = Cli::command();
    clap_mangen::generate_to(cmd, out_dir).context("failed to generate man page")?;

    println!("generated man pages in {}", out_dir.display());
    Ok(())
}
