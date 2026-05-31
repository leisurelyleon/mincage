//! Command-line argument definitions.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A minimal container runtime demonstrating Linux process isolation.
#[derive(Debug, Parser)]
#[command(name = "mincage", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Launch a sandboxed process from a spec file.
    Run {
        /// Path to the container spec (TOML).
        #[arg(long, default_value = "spec.toml")]
        spec: PathBuf,
    },
    /// Print a sample container spec to stdout.
    Spec,
}
