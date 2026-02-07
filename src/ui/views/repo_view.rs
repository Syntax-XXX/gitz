#![allow(dead_code)]


use crate::git::{Repository, RepoStatus};
use crate::config::Config;
use crate::ui::components::{file_list, status_bar};
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

/// The main repository view – shows status and a placeholder for diff.
pub struct RepoView {
    status: RepoStatus,
    selected_file_index: usize,
    status_message: String,
    // In a full implementation we would keep selected file, diff view, etc.
}

impl RepoView {
    pub fn new() -> Self {
        Self { 
            status: RepoStatus::default(),
            selected_file_index: 0,
            status_message: "Ready".to_string(),
        }
    }

    /// Refresh the view data from the repository.
    pub fn refresh(&mut self, repo: &Repository) -> Result<(), crate::errors::GitzError> {
        self.status = repo.status()?;
        self.status_message = format!("Refreshed: {}", self.status.summary());
        Ok(())
    }

    /// Handle a key press.
    pub fn handle_key(
        &mut self, 
        key: KeyEvent, 
        repo: &Repository, 
        _cfg: &Config
    ) -> Result<bool, crate::errors::GitzError> {
        match key.code {
            crossterm::event::KeyCode::Char('s') => {
                // Stage all changes.
                crate::commands::add::stage_all(repo)?;
                self.refresh(repo)?;
                self.status_message = "Staged all changes".to_string();
            }
            crossterm::event::KeyCode::Char('c') => {
                // Simple commit – in a real app we would open an editor.
                if self.status.is_clean() {
                    self.status_message = "Nothing to commit".to_string();
                } else {
                    crate::commands::commit::commit(repo, "quick commit")?;
                    self.refresh(repo)?;
                    self.status_message = "Committed changes".to_string();
                }
            }
            crossterm::event::KeyCode::Char('r') | crossterm::event::KeyCode::F(5) => {
                // Refresh manually
                self.refresh(repo)?;
            }
            crossterm::event::KeyCode::Char('q') => {
                return Ok(true); // Signal to quit
            }
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                // Navigate down in file list
                let total_files = self.status.total_changes();
                if total_files > 0 && self.selected_file_index < total_files - 1 {
                    self.selected_file_index += 1;
                }
            }
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                // Navigate up in file list
                if self.selected_file_index > 0 {
                    self.selected_file_index -= 1;
                }
            }
            crossterm::event::KeyCode::Home | crossterm::event::KeyCode::Char('g') => {
                // Go to first file
                self.selected_file_index = 0;
            }
            crossterm::event::KeyCode::End | crossterm::event::KeyCode::Char('G') => {
                // Go to last file
                let total_files = self.status.total_changes();
                if total_files > 0 {
                    self.selected_file_index = total_files - 1;
                }
            }
            _ => {}
        }
        Ok(false) // Continue running
    }

    /// Draw the UI.
    pub fn draw(
        &self, 
        f: &mut ratatui::Frame, 
        repo: &Repository
    ) -> Result<(), crate::errors::GitzError> {
        let size = f.area(); // Use area() instead of size()
        
        // Layout: top bar, main area split, bottom status.
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // top bar (with border)
                Constraint::Min(0),    // main area
                Constraint::Length(3), // status bar (with border)
            ])
            .split(size);

        // Top bar with repo path and branch.
        let branch_name = repo.current_branch()
            .unwrap_or_else(|_| "unknown".to_string());
        
        let top_text = format!(
            "gitz - Repository: {}   Branch: {}   Status: {}", 
            repo.path().display(), 
            branch_name,
            self.status.summary()
        );
        
        let top_bar = Paragraph::new(top_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default()
                .borders(Borders::ALL)
                .title("⚡ gitz"));
        
        f.render_widget(top_bar, chunks[0]);

        // Main area split into file list and diff placeholder.
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40), 
                Constraint::Percentage(60)
            ])
            .split(chunks[1]);

        // File list on the left.
        file_list::draw_file_list(f, main_chunks[0], &self.status);

        // Diff placeholder on the right.
        let diff_block = Block::default()
            .title("Diff Preview")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));
        
        let diff_content = if self.status.is_clean() {
            Paragraph::new("No changes to display")
                .style(Style::default().fg(Color::DarkGray))
                .block(diff_block)
        } else {
            Paragraph::new("Select a file to view diff\n(Feature coming soon...)")
                .style(Style::default().fg(Color::Yellow))
                .block(diff_block)
        };
        
        f.render_widget(diff_content, main_chunks[1]);

        // Bottom status bar with keybindings help.
        let help_text = format!(
            "{} | [s]tage [c]ommit [r]efresh [q]uit [j/k]navigate",
            self.status_message
        );
        status_bar::draw_status_bar(f, chunks[2], &help_text);

        Ok(())
    }
}

impl Default for RepoView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_view_is_clean() {
        let view = RepoView::new();
        assert!(view.status.is_clean());
        assert_eq!(view.selected_file_index, 0);
    }

    #[test]
    fn test_navigation() {
        let mut view = RepoView::new();
        view.status = RepoStatus {
            modified: vec!["file1.rs".to_string(), "file2.rs".to_string()],
            added: vec![],
            deleted: vec![],
        };

        // Start at 0
        assert_eq!(view.selected_file_index, 0);

        // Can't go below 0
        view.selected_file_index = 0;
        assert_eq!(view.selected_file_index, 0);

        // Move down
        view.selected_file_index = 1;
        assert_eq!(view.selected_file_index, 1);
    }
}