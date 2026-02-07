#![allow(dead_code)]


use crate::config::Config;
use crate::errors::GitzError;
use crate::git::Repository;
use crate::ui::views::repo_view::RepoView;
use crate::ui::views::worktrees_view::WorktreesView;
use crate::ui::views::workflow_view::WorkflowView;
use crate::event::AppEvent;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Modifier};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::text::Span;
use crossterm::event::{self, Event as CEvent, KeyCode};
use std::time::Duration;
use tokio::sync::mpsc::{self, Sender, Receiver};

/// Available views in the application.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    Files,
    Branches,
    Commits,
    Stashes,
    Remotes,
    Worktrees,
    Workflows,
}

impl View {
    /// Get the display name for the tab.
    pub fn name(&self) -> &'static str {
        match self {
            View::Files => "Files",
            View::Branches => "Branches",
            View::Commits => "Commits",
            View::Stashes => "Stashes",
            View::Remotes => "Remotes",
            View::Worktrees => "Worktrees",
            View::Workflows => "Workflows",
        }
    }

    /// Get all views for tab iteration.
    pub fn all() -> &'static [View] {
        &[View::Files, View::Branches, View::Commits, View::Stashes, View::Remotes, View::Worktrees, View::Workflows]
    }

    /// Switch to next view.
    pub fn next(&self) -> View {
        let all = View::all();
        let current_idx = all.iter().position(|&v| v == *self).unwrap_or(0);
        all[(current_idx + 1) % all.len()]
    }

    /// Switch to previous view.
    pub fn prev(&self) -> View {
        let all = View::all();
        let current_idx = all.iter().position(|&v| v == *self).unwrap_or(0);
        all[(current_idx + all.len() - 1) % all.len()]
    }
}

/// Global application state.
pub struct App {
    repo: Repository,
    config: Config,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    event_tx: Sender<AppEvent>,
    event_rx: Receiver<AppEvent>,
    current_view: View,
    repo_view: RepoView,
    worktrees_view: WorktreesView,
    workflow_view: WorkflowView,
}

impl App {
    /// Initialise the application.
    pub async fn new<P: AsRef<std::path::Path>>(repo_path: P, config: Config) -> Result<Self, GitzError> {
        // Open or initialise repository.
        let repo = if repo_path.as_ref().join(".git").exists() {
            Repository::open(repo_path)?
        } else {
            Repository::init(repo_path)?
        };

        // Terminal setup.
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        // Event channel.
        let (tx, rx) = mpsc::channel(100);

        // Initialise UI views.
        let repo_view = RepoView::new();
        let worktrees_view = WorktreesView::new();
        let workflow_view = WorkflowView::new();

        Ok(Self {
            repo,
            config,
            terminal,
            event_tx: tx,
            event_rx: rx,
            current_view: View::Files,
            repo_view,
            worktrees_view,
            workflow_view,
        })
    }

    /// Main event loop.
    pub async fn run(&mut self) -> Result<(), GitzError> {
        // Spawn a task that polls crossterm events.
        let tx = self.event_tx.clone();
        tokio::spawn(async move {
            loop {
                if event::poll(Duration::from_millis(250)).unwrap() {
                    if let CEvent::Key(key) = event::read().unwrap() {
                        let _ = tx.send(AppEvent::Key(key)).await;
                    }
                }
            }
        });

        // Initial draw.
        self.terminal.clear()?;
        self.draw()?;

        // Event handling loop.
        while let Some(event) = self.event_rx.recv().await {
            match event {
                AppEvent::Key(key) => {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }

                    // Handle global key bindings first
                    if !self.handle_global_key(key)? {
                        // If not a global key, handle in current view
                        self.handle_view_key(key)?;
                    }

                    // Redraw after handling key
                    self.draw()?;
                }
                AppEvent::Refresh => {
                    self.draw()?;
                }
                AppEvent::Quit => {
                    break;
                }
            }
        }
        self.terminal.clear()?;
        Ok(())
    }

    /// Handle global key bindings that work across all views.
    fn handle_global_key(&mut self, key: crossterm::event::KeyEvent) -> Result<bool, GitzError> {
        match key.code {
            KeyCode::Tab => {
                self.current_view = self.current_view.next();
                Ok(true)
            }
            KeyCode::BackTab => {
                self.current_view = self.current_view.prev();
                Ok(true)
            }
            KeyCode::Char('1') => {
                self.current_view = View::Files;
                Ok(true)
            }
            KeyCode::Char('2') => {
                self.current_view = View::Branches;
                Ok(true)
            }
            KeyCode::Char('3') => {
                self.current_view = View::Commits;
                Ok(true)
            }
            KeyCode::Char('4') => {
                self.current_view = View::Stashes;
                Ok(true)
            }
            KeyCode::Char('5') => {
                self.current_view = View::Remotes;
                Ok(true)
            }
            KeyCode::Char('6') => {
                self.current_view = View::Worktrees;
                Ok(true)
            }
            KeyCode::Char('7') => {
                self.current_view = View::Workflows;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// Handle key events for the current view.
    fn handle_view_key(&mut self, key: crossterm::event::KeyEvent) -> Result<(), GitzError> {
        match self.current_view {
            View::Files => {
                self.repo_view.handle_key(key, &self.repo, &self.config)?;
            }
            View::Branches => {
            }
            View::Commits => {
            }
            View::Stashes => {
            }
            View::Remotes => {
            }
            View::Worktrees => {
                self.worktrees_view.handle_key(key, &self.repo, &self.config)?;
            }
            View::Workflows => {
                self.workflow_view.handle_key(key, &self.repo, &self.config)?;
            }
        }
        Ok(())
    }

    /// Draw the current view.
    fn draw(&mut self) -> Result<(), GitzError> {
        let current_view = self.current_view;
        let repo = &self.repo;
        let repo_view = &self.repo_view;
        let worktrees_view = &self.worktrees_view;
        let workflow_view = &mut self.workflow_view;
        self.terminal.draw(move |f| {
            let _ = Self::draw_ui_static(f, current_view, repo, repo_view, worktrees_view, workflow_view);
        })?;
        Ok(())
    }

    /// Draw the UI for the current view.
    fn draw_ui_static(f: &mut ratatui::Frame, current_view: View, repo: &Repository, repo_view: &RepoView, worktrees_view: &WorktreesView, workflow_view: &mut WorkflowView) -> Result<(), GitzError> {
        match current_view {
            View::Files => {
                repo_view.draw(f, repo)?;
            }
            View::Branches => {
                Self::draw_placeholder_view_static(f, "Branches", current_view);
            }
            View::Commits => {
                Self::draw_placeholder_view_static(f, "Commits", current_view);
            }
            View::Stashes => {
                Self::draw_placeholder_view_static(f, "Stashes", current_view);
            }
            View::Remotes => {
                Self::draw_placeholder_view_static(f, "Remotes", current_view);
            }
            View::Worktrees => {
                worktrees_view.draw(f, repo)?;
            }
            View::Workflows => {
                workflow_view.draw(f, repo)?;
            }
        }
        Ok(())
    }

    /// Draw a placeholder view for unimplemented tabs.
    fn draw_placeholder_view_static(f: &mut ratatui::Frame, title: &str, current_view: View) {

        let size = f.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // top bar
                Constraint::Min(0),    // main area
                Constraint::Length(3), // status bar
            ])
            .split(size);

        // Top bar with tabs
        Self::draw_tab_bar_static(f, chunks[0], current_view);

        // Placeholder content
        let placeholder = Paragraph::new(format!("{} view - Coming soon!", title))
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(placeholder, chunks[1]);

        // Status bar
        let status = Paragraph::new("Press Tab to switch views | q to quit")
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(status, chunks[2]);
    }

    /// Draw the tab bar at the top.
    fn draw_tab_bar_static(f: &mut ratatui::Frame, area: ratatui::layout::Rect, current_view: View) {
        let titles = View::all()
            .iter()
            .map(|v| Span::raw(v.name()))
            .collect::<Vec<_>>();

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("gitpulse"))
            .select(View::all().iter().position(|&v| v == current_view).unwrap_or(0))
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_widget(tabs, area);
    }
}
