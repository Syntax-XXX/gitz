#![allow(dead_code)]

use crate::errors::GitzError;
use crate::git::{RepoStatus, CommitInfo};  // HINZUFÜGEN
use git2::{Repository as Git2Repo, StatusOptions, Oid};
use std::path::PathBuf;

/// Wrapper around `git2::Repository` providing high‑level helpers.
pub struct Repository {
    inner: Git2Repo,
    path: PathBuf,  // HINZUFÜGEN: Für Clone
}

impl Repository {
    /// Open an existing repository.
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self, GitzError> {
        let repo = Git2Repo::open(path.as_ref())?;
        let path = repo.path().to_path_buf();
        Ok(Self { inner: repo, path })
    }

    /// Initialise a new repository.
    pub fn init<P: AsRef<std::path::Path>>(path: P) -> Result<Self, GitzError> {
        let repo = Git2Repo::init(path.as_ref())?;
        let path = repo.path().to_path_buf();
        Ok(Self { inner: repo, path })
    }

    /// Absolute path to the repository root.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Current branch name (or detached HEAD).
    pub fn current_branch(&self) -> Result<String, GitzError> {
        let head = self.inner.head()?;
        if head.is_branch() {
            Ok(head.shorthand().unwrap_or("HEAD").to_string())
        } else {
            Ok("HEAD (detached)".into())
        }
    }

    /// Stage all changes (equivalent to `git add .`).
    pub fn add_all(&self) -> Result<(), GitzError> {
        let mut index = self.inner.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        Ok(())
    }

    /// Create a commit with the given message.
    pub fn commit(&self, message: &str) -> Result<Oid, GitzError> {
        let sig = self.inner.signature()?;
        let mut index = self.inner.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.inner.find_tree(tree_id)?;
        let parent = self.inner.head()?.peel_to_commit()?;
        let oid = self.inner.commit(
            Some("HEAD"),
            &sig,
            &sig,
            message,
            &tree,
            &[&parent],
        )?;
        Ok(oid)
    }

    /// Get a short status (modified, added, deleted files).
    pub fn status(&self) -> Result<RepoStatus, GitzError> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true).recurse_untracked_dirs(true);
        let statuses = self.inner.statuses(Some(&mut opts))?;
        let mut modified = Vec::new();
        let mut added = Vec::new();
        let mut deleted = Vec::new();
        for entry in statuses.iter() {
            let path = entry.path().unwrap_or("<unknown>").to_string();
            let s = entry.status();
            if s.is_index_new() || s.is_wt_new() {
                added.push(path);
            } else if s.is_index_modified() || s.is_wt_modified() {
                modified.push(path);
            } else if s.is_index_deleted() || s.is_wt_deleted() {
                deleted.push(path);
            }
        }
        Ok(RepoStatus { modified, added, deleted })
    }

    /// Retrieve the last N commits (default 20).
    pub fn recent_commits(&self, n: usize) -> Result<Vec<CommitInfo>, GitzError> {
        let mut revwalk = self.inner.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;
        let mut commits = Vec::new();
        for oid_result in revwalk.take(n) {
            let oid = oid_result?;
            let commit = self.inner.find_commit(oid)?;
            commits.push(CommitInfo {
                oid,
                message: commit.message().unwrap_or("<no message>").to_string(),
                author: commit.author().name().unwrap_or("<unknown>").to_string(),
                time: commit.time().seconds(),
            });
        }
        Ok(commits)
    }
}

// Manuelles Clone implementieren
impl Clone for Repository {
    fn clone(&self) -> Self {
        Self::open(&self.path).expect("Failed to clone repository")
    }
}

// Manuelles Debug implementieren
impl std::fmt::Debug for Repository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Repository")
            .field("path", &self.path)
            .finish()
    }
}