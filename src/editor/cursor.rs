use std::io::Error;

use super::terminal::queue_command;

use crossterm::cursor::{Hide, MoveTo, Show};

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
