use super::app;
use super::layout;
use super::widgets;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::*;
use std::io;
use std::path::PathBuf;

#[allow(clippy::collapsible_if)]
fn run(terminal: &mut Terminal<impl Backend>, app: &mut app::App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Esc {
                    return Ok(());
                }
            }
        }
    }
}

fn ui(frame: &mut Frame, app: &app::App) {
    // Temp fake data for testing
    let paths = vec![
        PathBuf::from("/home/user/Documents/file1.txt"),
        PathBuf::from("/home/user/Downloads/file2.png"),
        PathBuf::from("/etc/config.yaml"),
        PathBuf::from("relative/path/to/file3.rs"),
    ];
    // Create the layout
    let layout = layout::AppLayout::new(frame.area());

    // Create and render the header widget
    let header = widgets::HeaderWidget::new(app.username.clone());

    // Get content areas
    let main_layout = layout.content_areas();
    let main_content = widgets::ContentWidget::new(app.content_title.clone());
    let tree_widget = widgets::PathTreeWidget::new(paths);

    frame.render_widget(widgets::HeaderWidget::widget(&header), layout.header);

    frame.render_widget(widgets::ContentWidget::widget(&main_content), main_layout.1);

    frame.render_widget(widgets::PathTreeWidget::widget(&tree_widget), main_layout.0);
}

pub fn run_tui() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = app::App::new();

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
