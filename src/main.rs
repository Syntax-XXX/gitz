#![allow(dead_code)]


use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};

mod app;
mod config;
mod errors;
mod event;
mod git;
mod commands;
mod ui;

use crate::app::App;
use crate::config::Config;

/// CLI arguments for gitz.
#[derive(Parser, Debug)]
#[command(name = "gitz", version, about = "⚡ Git, but zippier")]
struct Cli {
    /// Path to the repository (defaults to current directory).
    #[arg(default_value = ".")]
    repo_path: String,

    /// Use a custom configuration file.
    #[arg(long)]
    config: Option<String>,

    /// Set log level (debug, info, warn, error).
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialise logger based on RUST_LOG or the supplied level.
    let filter = EnvFilter::try_new(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))?;
    fmt::Subscriber::builder().with_env_filter(filter).init();

    let cli = Cli::parse();
    let cfg = Config::load(cli.config.as_deref())?;

    // Check if repo_path is current directory and if we have write permissions
    if cli.repo_path == "." {
        let current_dir = std::env::current_dir()?;
        if !current_dir.join(".git").exists() {
            // Try to create a test file to check write permissions
            let test_file = current_dir.join(".gitz_test");
            if std::fs::File::create(&test_file).is_err() {
                eprintln!("Error: Cannot initialize git repository in current directory due to insufficient permissions.");
                eprintln!("Please run gitz in a directory where you have write permissions, or specify a repository path:");
                eprintln!("  gitz /path/to/repo");
                std::process::exit(1);
            } else {
                // Clean up test file
                let _ = std::fs::remove_file(test_file);
            }
        }
    }

    let mut app = App::new(cli.repo_path, cfg).await?;
    app.run().await?;
    Ok(())
}
