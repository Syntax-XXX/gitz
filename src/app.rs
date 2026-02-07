#![allow(dead_code)]


use crate::config::Config;
use crate::errors::GitzError;
use crate::git::Repository;
use crate::ui::views::repo_view::RepoView;
use crate::event::AppEvent;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crossterm::event::{self, Event as CEvent, KeyCode};
use std::time::Duration;
use tokio::sync::mpsc::{self, Sender, Receiver};

/// Global application state.
pub struct App {
    repo: Repository,
    config: Config,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    event_tx: Sender<AppEvent>,
    event_rx: Receiver<AppEvent>,
    view: RepoView,
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

        // Initialise UI view.
        let view = RepoView::new();

        Ok(Self { repo, config, terminal, event_tx: tx, event_rx: rx, view })
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
        self.terminal.draw(|f| {
            if let Err(e) = self.view.draw(f, &self.repo) {
                eprintln!("Draw error: {}", e);
            }
        })?;

        // Event handling loop.
        while let Some(event) = self.event_rx.recv().await {
            match event {
                AppEvent::Key(key) => {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                    // Forward key to view for handling (navigation, actions, etc.)
                    self.view.handle_key(key, &self.repo, &self.config)?;
                    
                    // Redraw after handling key
                    self.terminal.draw(|f| {
                        if let Err(e) = self.view.draw(f, &self.repo) {
                            eprintln!("Draw error: {}", e);
                        }
                    })?;
                }
                AppEvent::Refresh => {
                    self.terminal.draw(|f| {
                        if let Err(e) = self.view.draw(f, &self.repo) {
                            eprintln!("Draw error: {}", e);
                        }
                    })?;
                }
                AppEvent::Quit => {
                    break;
                }
            }
        }
        self.terminal.clear()?;
        Ok(())
    }
}
