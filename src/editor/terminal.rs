use std::io::stdout;

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::size;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};

pub fn initialize() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    clear_screen()?;
    move_cursor(0, 0)
}

pub fn clear_screen() -> Result<(), std::io::Error> {
    execute!(stdout(), Clear(ClearType::All))
}

pub fn terminate() -> Result<(), std::io::Error> {
    disable_raw_mode()
}

pub fn terminal_size() -> (u16, u16) {
    size().unwrap()
}

pub fn move_cursor(posx: u16, posy: u16) -> Result<(), std::io::Error> {
    execute!(stdout(), MoveTo(posx, posy))
}
