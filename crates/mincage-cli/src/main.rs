//! `mincage` command-line entry point.

use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

use mincage_cli::cli::{Cli, Command};
use mincage_core::{ContainerConfig, SAMPLE_SPEC};
use mincage_runtime::launch;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Spec => {
            print!("{SAMPLE_SPEC}");
            ExitCode::SUCCESS
        }
        Command::Run { spec } => match run(&spec) {
            Ok(code) => ExitCode::from(code.clamp(0, 255) as u8),
            Err(err) => {
                eprintln!("mincage: {err}");
                ExitCode::FAILURE
            }
        },
    }
}

/// Loads a spec and launches the container, returning its exit code.
fn run(spec: &Path) -> anyhow::Result<i32> {
    let text = std::fs::read_to_string(spec)?;
    let config: ContainerConfig = toml::from_str(&text)?;
    config.validate()?;
    let code = launch(&config)?;
    Ok(code)
}
