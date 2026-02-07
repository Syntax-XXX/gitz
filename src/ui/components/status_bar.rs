#![allow(dead_code)]

use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};

/// Draw the status bar at the bottom of the screen.
pub fn draw_status_bar(f: &mut ratatui::Frame, area: Rect, msg: &str) {
    let status = Paragraph::new(Line::from(msg))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}
