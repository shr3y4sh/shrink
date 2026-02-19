use std::fmt::Display;
use std::io::{Error, Write, stdout};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::size;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use crossterm::{Command, queue};

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
    queue_command(Clear(ClearType::CurrentLine))
}

pub fn show_cursor() -> Result<(), Error> {
    queue_command(Show)
}

pub fn hide_cursor() -> Result<(), Error> {
    queue_command(Hide)
}
pub fn clear_screen() -> Result<(), Error> {
    queue_command(Clear(ClearType::All))
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

pub fn queue_command<C: Command>(command: C) -> Result<(), Error> {
    queue!(stdout(), command)
}

pub fn print<T: Display>(string: T) -> Result<(), Error> {
    queue_command(Print(string))
}

pub fn move_cursor_to(Position { x, y }: Position) -> Result<(), Error> {
    queue_command(MoveTo(x, y))
}
