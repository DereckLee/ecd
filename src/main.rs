use std::io::{self, Write};
use std::process::ExitCode;

use clap::{CommandFactory, Parser};
use rayon::prelude::*;

use ecd::cli::{CheckArgs, Cli, Commands};
use ecd::detect::detect_file;
use ecd::output::{FileResult, OutputConfig, print_results, print_stats};
use ecd::walk::collect_paths;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("ecd: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let args = match cli.command {
        Some(Commands::Check(args)) => args,
        None => {
            let mut stdout = io::stdout().lock();
            Cli::command().print_help()?;
            writeln!(stdout)?;
            return Ok(());
        }
    };

    execute_check(args)
}

fn execute_check(args: CheckArgs) -> anyhow::Result<()> {
    if let Some(jobs) = args.jobs {
        rayon::ThreadPoolBuilder::new()
            .num_threads(jobs)
            .build_global()
            .map_err(|e| anyhow::anyhow!("failed to configure thread pool: {e}"))?;
    }

    let single_file_mode = args.is_single_file_mode();
    let paths = collect_paths(&args)?;

    let results: Vec<FileResult> = paths
        .par_iter()
        .map(|path| FileResult {
            path: path.clone(),
            detection: detect_file(path),
        })
        .collect();

    let ignore_encoding = args.ignore_encoding.as_deref();
    let config = OutputConfig {
        single_file_mode,
        ignore_encoding,
        quiet: args.quiet,
        verbose: args.verbose,
    };

    let mut stderr = io::stderr().lock();
    let stats = print_results(&results, &config, &mut stderr)?;

    if args.verbose {
        print_stats(&stats, &mut stderr)?;
    }

    Ok(())
}
