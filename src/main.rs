mod app;
mod tui;
mod ui;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    // Setup terminal
    let mut terminal = tui::init()?;
    
    // Create app and run it
    let mut app = App::new();

    // Main loop
    while !app.should_quit {
        // Draw UI
        terminal.draw(|f| ui::render(f, &app))?;
        
        // Handle events
        app.handle_events()?;
    }

    // Restore terminal
    tui::restore(terminal)?;

    Ok(())
}