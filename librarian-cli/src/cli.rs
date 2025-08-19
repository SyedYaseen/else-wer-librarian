use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn render(frame: &mut Frame) {
    Paragraph::new("Hello from application").render(frame.area(), frame.buffer_mut());
}
