// Utility helpers for the application.
#![allow(dead_code)]

pub fn format_duration(secs: u64) -> String {
    let mins = secs / 60;
    let secs = secs % 60;
    format!("{:02}:{:02}", mins, secs)
}
