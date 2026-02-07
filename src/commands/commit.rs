#[allow(dead_code)]
use crate::errors::GitzError;
use crate::git::Repository;
use git2::Oid;

/// Create a commit with the given message.
pub fn commit(repo: &Repository, message: &str) -> Result<Oid, GitzError> {
    if message.trim().is_empty() {
        return Err(GitzError::InvalidInput("Commit message cannot be empty".into()));
    }
    repo.commit(message)
}