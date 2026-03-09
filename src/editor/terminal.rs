use std::io::{Error, Write, stdout};

use crossterm::style::Print;
use crossterm::terminal::{
    Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
    enable_raw_mode, size,
};
use crossterm::{Command, queue};

use super::cursor;

#[derive(Copy, Clone, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub fn terminate() -> Result<(), Error> {
    exit_alt_screen()?;
    cursor::show_caret()?;
    execute()?;
    disable_raw_mode()
}

pub fn initialize() -> Result<(), Error> {
    enable_raw_mode()?;
    enter_alt_screen()?;
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

pub fn print_row(row: usize, text: &str) -> Result<(), Error> {
    cursor::move_caret(cursor::Position { column: 0, row })?;
    clear_line()?;
    print(text)
}

fn exit_alt_screen() -> Result<(), Error> {
    queue_command(LeaveAlternateScreen)
}

fn enter_alt_screen() -> Result<(), Error> {
    queue_command(EnterAlternateScreen)
}

pub fn queue_command<C: Command>(command: C) -> Result<(), Error> {
    queue!(stdout(), command)
}

pub fn print(string: &str) -> Result<(), Error> {
    queue_command(Print(string))
}
