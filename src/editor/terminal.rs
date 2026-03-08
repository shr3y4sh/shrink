use std::io::{Error, Write, stdout};

use crossterm::cursor::{Hide, MoveTo, Show};

use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};

#[derive(Copy, Clone, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub column: usize,
    pub row: usize,
}

/// Moves the cursor position
/// * `position`: `Position` struct where the cursor should go
pub fn move_caret(position: Position) -> Result<(), Error> {
    #[allow(clippy::cast_possible_truncation)]
    queue_command(MoveTo(position.column as u16, position.row as u16))
}

pub fn show_caret() -> Result<(), Error> {
    queue_command(Show)
}

pub fn hide_caret() -> Result<(), Error> {
    queue_command(Hide)
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
