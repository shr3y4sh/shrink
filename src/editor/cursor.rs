//! # Cursor Module
//!
//! This module provides the Cursor struct and its implementation functions
use std::io::Error;

use core::cmp::min;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::KeyCode,
};

use crate::editor::terminal::{Position, Size, get_size, queue_command};

/// ### Represents the location in the text of the file
/// Different from the Caret position (denoted by struct Position)
#[derive(Clone, Copy, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

/// * `visibility` - to store state of cursor visibility (will be used later meaningfully)
/// * `location` - keeps the location of cursor always in sync
#[derive(Default)]
pub struct Cursor {
    pub visibility: bool,
    pub location: Location,
}

impl Cursor {
    /// Moves the caret position using the location
    /// * `code`: `KeyCode` event listened by the repl
    pub fn move_caret(&mut self, code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = get_size()?;

        match code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }

            _ => (),
        }

        self.location = Location { x, y };
        Ok(())
    }

    /// Moves the cursor position
    /// * `position`: `Position` struct where the cursor should go
    pub fn move_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::cast_possible_truncation)]
        queue_command(MoveTo(position.x as u16, position.y as u16))
    }

    pub fn show(&mut self) -> Result<(), Error> {
        self.visibility = true;
        queue_command(Show)
    }

    pub fn hide(&mut self) -> Result<(), Error> {
        self.visibility = false;
        queue_command(Hide)
    }
}
