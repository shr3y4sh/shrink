use std::io::{Error, Write, stdout};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::size;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub fn initialize() -> Result<(), Error> {
    enable_raw_mode()?;
    clear_screen()?;
    move_cursor_to(Position { x: 0, y: 0 })?;
    execute()
}

pub fn execute() -> Result<(), Error> {
    stdout().flush()
}

pub fn clear_line() -> Result<(), Error> {
    queue!(stdout(), Clear(ClearType::CurrentLine))
}

pub fn show_cursor() -> Result<(), Error> {
    queue!(stdout(), Show)
}

pub fn hide_cursor() -> Result<(), Error> {
    queue!(stdout(), Hide)
}
pub fn clear_screen() -> Result<(), Error> {
    queue!(stdout(), Clear(ClearType::All))
}

pub fn terminate() -> Result<(), Error> {
    execute()?;
    disable_raw_mode()
}

pub fn get_size() -> Result<Size, Error> {
    let res = size()?;
    Ok(Size {
        width: res.0,
        height: res.1,
    })
}

pub fn print(c: &str) -> Result<(), Error> {
    queue!(stdout(), Print(c))
}

pub fn move_cursor_to(Position { x, y }: Position) -> Result<(), Error> {
    queue!(stdout(), MoveTo(x, y))
}
