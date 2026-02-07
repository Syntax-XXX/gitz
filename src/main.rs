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

    let mut app = App::new(cli.repo_path, cfg).await?;
    app.run().await?;
    Ok(())
}
