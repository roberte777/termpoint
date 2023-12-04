use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io::{self, stdout};
use termpoint::parse::parse;
use termpoint::ui::{handle_events, ui};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Parse markdown
    parse();

    // Render
    let mut should_quit = false;
    let current_slide = 0;
    let presentation = parse();
    while !should_quit {
        terminal.draw(|f| ui(f, &presentation, current_slide))?;
        should_quit = handle_events()?;
    }

    // Teardown terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
