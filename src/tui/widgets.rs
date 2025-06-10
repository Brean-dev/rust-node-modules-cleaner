use chrono::Utc;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::borrow::Cow;
use std::path::PathBuf;

pub struct HeaderWidget {
    username: String,
}

pub struct SidebarWidget {
    content: String,
}

pub struct ContentWidget {
    content: String,
}

pub struct PathTreeWidget {
    paths: Vec<PathBuf>,
}

impl HeaderWidget {
    pub fn new(username: String) -> Self {
        Self { username }
    }

    pub fn widget(&self) -> Paragraph<'static> {
        let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        Paragraph::new(vec![Line::from(vec![
            Span::raw(Cow::from(
                "Total time ran: (current_time temp for debugging) ",
            )),
            Span::styled(Cow::from(current_time), Style::default().fg(Color::Yellow)),
            //Span::raw(Cow::from(" | User: ")),
            //Span::styled(
            //    Cow::from(self.username.clone()),
            //    Style::default().fg(Color::Yellow),
            //),
        ])])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(Cow::from("Node cleaner")),
        )
    }
}

impl SidebarWidget {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn widget(&self) -> Paragraph<'static> {
        Paragraph::new(vec![Line::from(vec![Span::styled(
            Cow::from(self.content.clone()),
            Style::default().fg(Color::LightRed),
        )])])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightRed))
                .title(Cow::from("Sidebar")),
        )
    }
}

impl ContentWidget {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn widget(&self) -> Paragraph<'static> {
        Paragraph::new(vec![Line::from(vec![Span::styled(
            Cow::from(self.content.clone()),
            Style::default().fg(Color::Magenta),
        )])])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta))
                .title(Cow::from("Content")),
        )
    }
}

impl PathTreeWidget {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self { paths }
    }

    pub fn widget(&self) -> Paragraph<'static> {
        // Convert each path to a String (owned, not a Cow)
        let path_strings: Vec<String> = self
            .paths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect();

        // Create a Vec<Line> with 'static spans
        let lines: Vec<Line<'static>> = path_strings
            .into_iter()
            .map(|s| {
                Line::from(vec![Span::styled(
                    s, // String is now owned and becomes 'static
                    Style::default().fg(Color::LightRed),
                )])
            })
            .collect();

        Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightRed))
                .title(Cow::from("Paths found")),
        )
    }
}
