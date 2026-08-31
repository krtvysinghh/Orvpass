pub mod app;
pub mod events;
pub mod ui;

use app::TuiApp;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use orvpass_core::models::VaultItem;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

pub fn run_tui(items: Vec<VaultItem>) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = TuiApp::new(items);

    while !app.should_quit {
        terminal.draw(|f| ui::render(f, &app))?;
        events::handle_events(&mut app)?;
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
