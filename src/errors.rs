#![allow(dead_code)]


use thiserror::Error;

/// Central error type for the application.
#[derive(Error, Debug)]
pub enum GitzError {
    #[error("Repository not found at `{0}`")]
    RepoNotFound(String),

    #[error("Git operation failed: {0}")]
    GitOperationFailed(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Merge conflict detected")]
    MergeConflict,

    #[error("Invalid branch name: {0}")]
    InvalidBranchName(String),

    #[error("Authentication failed")]
    AuthFailed,

    #[error("Operation cancelled by user")]
    Cancelled,

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
