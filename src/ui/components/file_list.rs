#![allow(dead_code)]

use crate::git::RepoStatus;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};  // GEÄNDERT: Spans -> Line
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

/// Render the list of changed files.
pub fn draw_file_list(
    f: &mut Frame,  // GEÄNDERT: Kein <B: Backend> mehr
    area: Rect, 
    status: &RepoStatus
) {
    draw_file_list_with_selection(f, area, status, None);
}

/// Render the list with optional selection highlighting.
pub fn draw_file_list_with_selection(
    f: &mut Frame,
    area: Rect,
    status: &RepoStatus,
    selected_index: Option<usize>,
) {
    let mut items: Vec<ListItem> = Vec::new();

    // Modified files (yellow)
    for file in &status.modified {
        let line = Line::from(vec![
            Span::styled("● ", Style::default().fg(Color::Yellow)),
            Span::raw(file),
        ]);
        items.push(ListItem::new(line));
    }

    // Added files (green)
    for file in &status.added {
        let line = Line::from(vec![
            Span::styled("✚ ", Style::default().fg(Color::Green)),
            Span::raw(file),
        ]);
        items.push(ListItem::new(line));
    }

    // Deleted files (red)
    for file in &status.deleted {
        let line = Line::from(vec![
            Span::styled("✖ ", Style::default().fg(Color::Red)),
            Span::raw(file),
        ]);
        items.push(ListItem::new(line));
    }

    // If no changes, show a message
    if items.is_empty() {
        let line = Line::from(Span::styled(
            "No changes",
            Style::default().fg(Color::DarkGray),
        ));
        items.push(ListItem::new(line));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .title("Files")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("» ");

    // If we have a selected index, use stateful rendering
    if let Some(index) = selected_index {
        let mut state = ListState::default();
        state.select(Some(index));
        f.render_stateful_widget(list, area, &mut state);
    } else {
        f.render_widget(list, area);
    }
}

/// Helper to get the file at a given index across all categories
pub fn get_file_at_index(status: &RepoStatus, index: usize) -> Option<String> {
    let all_files = status.all_files();
    all_files.get(index).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_at_index() {
        let status = RepoStatus {
            modified: vec!["mod.rs".to_string()],
            added: vec!["new.rs".to_string()],
            deleted: vec!["old.rs".to_string()],
        };

        assert_eq!(get_file_at_index(&status, 0), Some("mod.rs".to_string()));
        assert_eq!(get_file_at_index(&status, 1), Some("new.rs".to_string()));
        assert_eq!(get_file_at_index(&status, 2), Some("old.rs".to_string()));
        assert_eq!(get_file_at_index(&status, 3), None);
    }
}