#![allow(dead_code)]

#[derive(Debug, Clone, Default)]
pub struct RepoStatus {
    pub modified: Vec<String>,
    pub added: Vec<String>,
    pub deleted: Vec<String>,
}

impl RepoStatus {
    /// Create a new empty status
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if the repository is clean (no changes)
    pub fn is_clean(&self) -> bool {
        self.modified.is_empty() && self.added.is_empty() && self.deleted.is_empty()
    }

    /// Total number of changed files
    pub fn total_changes(&self) -> usize {
        self.modified.len() + self.added.len() + self.deleted.len()
    }

    /// Human-readable summary
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        if !self.modified.is_empty() {
            parts.push(format!("{} modified", self.modified.len()));
        }
        if !self.added.is_empty() {
            parts.push(format!("{} added", self.added.len()));
        }
        if !self.deleted.is_empty() {
            parts.push(format!("{} deleted", self.deleted.len()));
        }
        if parts.is_empty() {
            "clean".into()
        } else {
            parts.join(", ")
        }
    }

    /// Get all changed files as a single list
    pub fn all_files(&self) -> Vec<String> {
        let mut all = Vec::new();
        all.extend(self.modified.clone());
        all.extend(self.added.clone());
        all.extend(self.deleted.clone());
        all
    }

    /// Check if a specific file has changes
    pub fn has_file(&self, path: &str) -> bool {
        self.modified.contains(&path.to_string())
            || self.added.contains(&path.to_string())
            || self.deleted.contains(&path.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_status_is_clean() {
        let status = RepoStatus::new();
        assert!(status.is_clean());
        assert_eq!(status.summary(), "clean");
    }

    #[test]
    fn test_status_with_changes() {
        let status = RepoStatus {
            modified: vec!["file1.rs".to_string()],
            added: vec!["file2.rs".to_string(), "file3.rs".to_string()],
            deleted: vec![],
        };
        
        assert!(!status.is_clean());
        assert_eq!(status.total_changes(), 3);
        assert_eq!(status.summary(), "1 modified, 2 added");
    }

    #[test]
    fn test_has_file() {
        let status = RepoStatus {
            modified: vec!["file1.rs".to_string()],
            added: vec![],
            deleted: vec![],
        };
        
        assert!(status.has_file("file1.rs"));
        assert!(!status.has_file("file2.rs"));
    }

    #[test]
    fn test_all_files() {
        let status = RepoStatus {
            modified: vec!["mod.rs".to_string()],
            added: vec!["new.rs".to_string()],
            deleted: vec!["old.rs".to_string()],
        };
        
        let all = status.all_files();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&"mod.rs".to_string()));
        assert!(all.contains(&"new.rs".to_string()));
        assert!(all.contains(&"old.rs".to_string()));
    }
}
