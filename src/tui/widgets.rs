use chrono::Utc;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::borrow::Cow;

pub struct HeaderWidget {
    username: String,
}

pub struct SidebarWidget {
    content: String,
}

pub struct ContentWidget {
    content: String,
}

impl HeaderWidget {
    pub fn new(username: String) -> Self {
        Self { username }
    }

    pub fn widget(&self) -> Paragraph<'static> {
        let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        Paragraph::new(vec![Line::from(vec![
            Span::raw(Cow::from("Time: ")),
            Span::styled(Cow::from(current_time), Style::default().fg(Color::Yellow)),
            Span::raw(Cow::from(" | User: ")),
            Span::styled(
                Cow::from(self.username.clone()),
                Style::default().fg(Color::Yellow),
            ),
        ])])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(Cow::from("Header")),
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
