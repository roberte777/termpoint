use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{parse::Presentation, section::Section};

pub fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn ui(frame: &mut Frame, presentation: &Presentation, current_slide: usize) {
    let current_slide = &presentation[current_slide];
    // create slide from current_slide
    // render slide
    let size = frame.size();
    for section in current_slide.iter() {
        match section {
            Section::Text(text) => {
                frame.render_widget(
                    Paragraph::new(text.clone())
                        .block(Block::default().title("Slide").borders(Borders::ALL)),
                    size,
                );
            }
        }
    }
}
