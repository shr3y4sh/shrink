use std::io::{Error, Write, stdout};

use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};

#[derive(Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub fn terminate() -> Result<(), Error> {
    execute()?;
    disable_raw_mode()
}

pub fn initialize() -> Result<(), Error> {
    enable_raw_mode()?;
    clear_screen()?;
    execute()
}

pub fn execute() -> Result<(), Error> {
    stdout().flush()
}

pub fn clear_line() -> Result<(), Error> {
    queue_command(Clear(ClearType::CurrentLine))
}

pub fn clear_screen() -> Result<(), Error> {
    queue_command(Clear(ClearType::All))
}

pub fn get_size() -> Result<Size, Error> {
    let res = size()?;
    Ok(Size {
        width: res.0 as usize,
        height: res.1 as usize,
    })
}

pub fn queue_command<C: Command>(command: C) -> Result<(), Error> {
    queue!(stdout(), command)
}

pub fn print(string: &str) -> Result<(), Error> {
    queue_command(Print(string))
}
