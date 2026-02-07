use crate::errors::GitzError;
use crate::git::Repository;
use crate::config::Config;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Modifier};
use ratatui::widgets::{Block, Borders, Paragraph, List, ListItem};
use ratatui::text::Span;

/// Workflow view for managing Git workflows.
pub struct WorkflowView {
    workflows: Vec<String>,
    selected: usize,
}

impl WorkflowView {
    /// Create a new workflow view.
    pub fn new() -> Self {
        Self {
            workflows: vec![
                "Feature Branch Workflow".to_string(),
                "Hotfix Workflow".to_string(),
                "Release Workflow".to_string(),
                "Bugfix Workflow".to_string(),
            ],
            selected: 0,
        }
    }

    /// Handle key events for the workflow view.
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent, _repo: &Repository, _config: &Config) -> Result<(), GitzError> {
        match key.code {
            crossterm::event::KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            crossterm::event::KeyCode::Down => {
                if self.selected < self.workflows.len() - 1 {
                    self.selected += 1;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Draw the workflow view.
    pub fn draw(&mut self, f: &mut Frame, _repo: &Repository) -> Result<(), GitzError> {
        let size = f.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // title
                Constraint::Min(0),    // list
                Constraint::Length(3), // status
            ])
            .split(size);

        // Title
        let title = Paragraph::new("Git Workflows")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Workflow list
        let items: Vec<ListItem> = self.workflows
            .iter()
            .enumerate()
            .map(|(i, workflow)| {
                let style = if i == self.selected {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Span::styled(workflow.clone(), style))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Available Workflows"));
        f.render_widget(list, chunks[1]);

        // Status
        let status = Paragraph::new("Use ↑/↓ to navigate | Enter to select workflow")
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(status, chunks[2]);

        Ok(())
    }
}
