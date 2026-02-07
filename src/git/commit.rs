#[allow(dead_code)]
use git2::Oid;

/// Minimal commit information used by the UI.
#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub oid: Oid,
    pub message: String,
    pub author: String,
    pub time: i64, // seconds since epoch
}
