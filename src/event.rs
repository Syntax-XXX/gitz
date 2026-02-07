#![allow(dead_code)]

use crossterm::event::KeyEvent;

/// Application events that can be sent through the event channel.
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// A key was pressed.
    Key(KeyEvent),
    
    /// Request a UI refresh.
    Refresh,
    
    /// Application should quit.
    Quit,
}
