#[allow(dead_code)]
use crate::errors::GitzError;
use crate::git::Repository;

/// Stage all changes in the repository.
pub fn stage_all(repo: &Repository) -> Result<(), GitzError> {
    repo.add_all()
}

/// Stage a specific file.
pub fn stage_file(repo: &Repository, _path: &str) -> Result<(), GitzError> {
    // TODO: Implementiere einzelne Datei staging
    // Für jetzt: stage alles
    repo.add_all()
}
