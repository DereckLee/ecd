use std::io::{self, Write};
use std::process::ExitCode;

use clap::{CommandFactory, Parser};
use rayon::prelude::*;

use ecd::cli::{CheckArgs, Cli, Commands, CompleteArgs, ConvertArgs, EncodingsArgs};
use ecd::color::ColorWhen;
use ecd::convert::{ConvertOptions, convert_file};
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

    match cli.command {
        Some(Commands::Check(args)) => execute_check(args, cli.color),
        Some(Commands::Encodings(args)) => execute_encodings(args),
        Some(Commands::Convert(args)) => execute_convert(args),
        Some(Commands::Complete(args)) => execute_complete(args),
        None => {
            let mut stdout = io::stdout().lock();
            Cli::command().print_help()?;
            writeln!(stdout)?;
            Ok(())
        }
    }
}

fn execute_encodings(_args: EncodingsArgs) -> anyhow::Result<()> {
    let mut stdout = io::stdout().lock();
    for name in ecd::encodings::all_names() {
        writeln!(stdout, "{name}")?;
    }
    Ok(())
}

fn execute_complete(args: CompleteArgs) -> anyhow::Result<()> {
    let mut stdout = io::stdout().lock();
    ecd::completions::write_completions(args.shell, &mut stdout);
    Ok(())
}

fn execute_convert(args: ConvertArgs) -> anyhow::Result<()> {
    let opts = ConvertOptions {
        strict: args.strict,
        write_bom: args.write_bom,
        force: args.force,
    };
    convert_file(
        &args.file,
        args.output.as_deref(),
        &args.from_encoding,
        &args.to_encoding,
        &opts,
    )
    .map_err(|e| anyhow::anyhow!("{e}"))
}

fn execute_check(args: CheckArgs, color: ColorWhen) -> anyhow::Result<()> {
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
        color,
    };

    let mut stderr = io::stderr().lock();
    let stats = print_results(&results, &config, &mut stderr)?;

    if args.verbose {
        print_stats(&stats, &mut stderr)?;
    }

    Ok(())
}
