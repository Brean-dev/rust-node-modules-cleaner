use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Holds the layout areas for the application
#[derive(Debug)]
#[allow(dead_code)]
pub struct AppLayout {
    pub header: Rect,
    pub content: Rect,
    pub status: Rect,
    // Add more areas as needed
}

impl AppLayout {
    /// Creates a new layout using the given terminal frame area
    pub fn new(area: Rect) -> Self {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header height
                Constraint::Min(0),    // Content area
                Constraint::Length(1), // Status bar height
            ])
            .split(area);

        Self {
            header: vertical[0],
            content: vertical[1],
            status: vertical[2],
        }
    }

    /// Gets the content areas split into sidebar and main
    pub fn content_areas(&self) -> (Rect, Rect) {
        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(self.content);

        (horizontal[0], horizontal[1])
    }
}
