mod app;
mod layout;
mod widgets;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use layout::AppLayout;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::*;
use std::io;
use widgets::{ContentWidget, HeaderWidget};

#[allow(clippy::collapsible_if)]
fn run(terminal: &mut Terminal<impl Backend>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;
        // Handle events here
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Esc {
                    return Ok(());
                }
            }
        }
    }
}

fn ui(frame: &mut Frame, app: &App) {
    // Create the layout
    let layout = AppLayout::new(frame.size());

    // Create and render the header widget
    let header = HeaderWidget::new(app.username.clone());
    frame.render_widget(HeaderWidget::widget(&header), layout.header);
    let sidebar = ContentWidget::new(app.content.clone());
    // Get content areas
    let main_content = layout.content_areas();
    frame.render_widget(ContentWidget::widget(&sidebar), main_content.0);
    // Render other widgets...
    // You can create more custom widgets in widgets.rs and render them here
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Run the app
    let res = run(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
