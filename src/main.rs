use clap::Parser;
use std::path::PathBuf;

mod anarch;
mod cli;

use crate::anarch::{AnarchConfig, PackageManager};

const DEFAULT_PATH: &str = "~/.config/anarch/.config";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let anarch_cli = cli::Cli::parse();

    let config_file: PathBuf = anarch_cli
        .config
        .unwrap_or_else(|| PathBuf::from(DEFAULT_PATH));

    match &anarch_cli.command {
        cli::Commands::Apply => {
            let config = AnarchConfig::load(&config_file)?;
            let package_manager = PackageManager::from(&config);
            package_manager.install_all();

            println!("Configuration applied successfully!");
        }
        cli::Commands::Generate => {
            let configs = PackageManager::extract_installed_packages().unwrap();
            configs.save(config_file).unwrap();
            println!("Generated");
        }
        cli::Commands::Check => match AnarchConfig::load(&config_file) {
            Ok(_) => println!("Configuration file is valid!"),
            Err(e) => println!("Configuration error: {}", e),
        },
        _ => {
            println!("Invalid command");
        }
    }

    Ok(())
}
