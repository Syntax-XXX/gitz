#[allow(dead_code)]
use crate::errors::GitzError;
use crate::git::Repository;
use std::path::Path;

/// Initialize a new git repository.
pub fn init<P: AsRef<Path>>(path: P) -> Result<Repository, GitzError> {
    Repository::init(path)
}