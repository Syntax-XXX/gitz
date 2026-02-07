#![allow(dead_code)]

use crate::git::Repository;
use crate::config::Config;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

/// The worktrees view – shows and manages worktrees.
pub struct WorktreesView {
    worktrees: Vec<String>,
    selected_index: usize,
    status_message: String,
}

impl WorktreesView {
    pub fn new() -> Self {
        Self {
            worktrees: Vec::new(),
            selected_index: 0,
            status_message: "Ready".to_string(),
        }
    }

    /// Refresh the view data from the repository.
    pub fn refresh(&mut self, repo: &Repository) -> Result<(), crate::errors::GitzError> {
        self.worktrees = repo.list_worktrees()?;
        self.status_message = format!("Refreshed: {} worktrees", self.worktrees.len());
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
            crossterm::event::KeyCode::Char('r') | crossterm::event::KeyCode::F(5) => {
                // Refresh manually
                self.refresh(repo)?;
            }
            crossterm::event::KeyCode::Char('q') => {
                return Ok(true); // Signal to quit
            }
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                // Navigate down in worktrees list
                if !self.worktrees.is_empty() && self.selected_index < self.worktrees.len() - 1 {
                    self.selected_index += 1;
                }
            }
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                // Navigate up in worktrees list
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            crossterm::event::KeyCode::Home | crossterm::event::KeyCode::Char('g') => {
                // Go to first worktree
                self.selected_index = 0;
            }
            crossterm::event::KeyCode::End | crossterm::event::KeyCode::Char('G') => {
                // Go to last worktree
                if !self.worktrees.is_empty() {
                    self.selected_index = self.worktrees.len() - 1;
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
        let size = f.area();

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
            "gitz - Repository: {}   Branch: {}   Worktrees: {}",
            repo.path().display(),
            branch_name,
            self.worktrees.len()
        );

        let top_bar = Paragraph::new(top_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default()
                .borders(Borders::ALL)
                .title("⚡ gitz - Worktrees"));

        f.render_widget(top_bar, chunks[0]);

        // Main area: worktrees list and details.
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ])
            .split(chunks[1]);

        // Worktrees list on the left with selection.
        let items: Vec<ListItem> = self.worktrees.iter()
            .enumerate()
            .map(|(i, name)| {
                let style = if i == self.selected_index {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(name.as_str()).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Worktrees"))
            .highlight_style(Style::default().add_modifier(ratatui::style::Modifier::BOLD));

        f.render_widget(list, main_chunks[0]);

        // Details on the right.
        let details = if let Some(selected) = self.worktrees.get(self.selected_index) {
            Paragraph::new(format!("Selected worktree: {}\n\nDetails coming soon...", selected))
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Details"))
        } else {
            Paragraph::new("No worktrees available")
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::default().borders(Borders::ALL).title("Details"))
        };

        f.render_widget(details, main_chunks[1]);

        // Bottom status bar with keybindings help.
        let help_text = format!(
            "{} | [r]efresh [q]uit [j/k]navigate",
            self.status_message
        );
        let status_bar = Paragraph::new(help_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(status_bar, chunks[2]);

        Ok(())
    }
}

impl Default for WorktreesView {
    fn default() -> Self {
        Self::new()
    }
}
