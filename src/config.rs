#![allow(dead_code)]

use config::{Config as Cfg, File, FileFormat};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct UiConfig {
    pub theme: String,
    pub diff_context_lines: usize,
    pub show_line_numbers: bool,
    pub tab_size: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GitConfig {
    pub default_branch: String,
    pub auto_fetch_interval: u64,
    pub sign_commits: bool,
    pub gpg_key: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Performance {
    pub max_commits_to_load: usize,
    pub cache_enabled: bool,
    pub parallel_operations: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub ui: UiConfig,
    pub git: GitConfig,
    pub performance: Performance,
}

impl Config {
    /// Load configuration from the default location or a custom file.
    pub fn load(custom_path: Option<&str>) -> Result<Self, anyhow::Error> {
        let mut builder = Cfg::builder();
        // Defaults
        builder = builder.set_default("ui.theme", "dark")?;
        builder = builder.set_default("ui.diff_context_lines", 3)?;
        builder = builder.set_default("ui.show_line_numbers", true)?;
        builder = builder.set_default("ui.tab_size", 4)?;
        builder = builder.set_default("git.default_branch", "main")?;
        builder = builder.set_default("git.auto_fetch_interval", 300)?;
        builder = builder.set_default("git.sign_commits", false)?;
        builder = builder.set_default("performance.max_commits_to_load", 1000)?;
        builder = builder.set_default("performance.cache_enabled", true)?;
        builder = builder.set_default("performance.parallel_operations", true)?;

        // Determine config file path.
        let path = if let Some(p) = custom_path {
            PathBuf::from(p)
        } else {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("gitz")
                .join("config.toml")
        };
        if path.exists() {
            builder = builder.add_source(File::from(path).format(FileFormat::Toml));
        }
        let cfg = builder.build()?.try_deserialize()?;
        Ok(cfg)
    }
}
