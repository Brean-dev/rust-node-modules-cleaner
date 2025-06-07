use color_eyre::eyre::{Ok, Result, eyre};
use color_eyre::owo_colors::OwoColorize;
use ratatui::style::Color;
use ratatui::text::Text;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        event::{self, Event},
        terminal,
    },
    style::Style,
    widgets::{Block, Borders, Paragraph, Widget},
};
fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();
    result
}
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break Ok(());
                }
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Node cleaner forever",
        Style::default().fg(Color::Magenta),
    ))
    .block(title_block)
    .render(frame.area(), frame.buffer_mut());
}
