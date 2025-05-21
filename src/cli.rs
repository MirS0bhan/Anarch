use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Anarch | arch config manager
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// custom anarch config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Apply configuration to the system
    Apply,
    /// Check the validity of the configuration file
    Check,
    /// Generate a new configuration file
    Generate,
}
