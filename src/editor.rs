/*
 * Starting with the Editor struct
 * */

use std::io::Error;

use crossterm::event::{
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read,
};

use crate::editor::{
    cursor::Cursor,
    term::{Position, Size},
    terminal as term,
};

mod cursor;
mod terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Editor struct
///     `cursor`: the implementation of cursor manipulation struct
///     `should_quit`: a boolean value to quit the editor
#[derive(Default)]
pub struct Editor {
    cursor: Cursor,
    should_quit: bool,
}

impl Editor {
    /// This is the entry point into the program
    /// initialize terminal, start the `repl`, terminate and check the result
    pub fn run(&mut self) {
        term::initialize().unwrap();
        let result = self.repl();
        term::terminate().unwrap();
        result.unwrap();
    }

    /// Any key press or other event will be evaluated by this
    ///     event: crossterm event enum
    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                // Ctrl+q keypress quits the editor
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                // Any cursor movement keypress go here
                KeyCode::Right
                | KeyCode::Left
                | KeyCode::Down
                | KeyCode::Up
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::Home
                | KeyCode::End => {
                    self.cursor.move_caret(*code)?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    /// This start the repl loop, which breaks only when `should_quit` is true
    /// it listens for any event and evaluates it, the execution happen in `refresh_screen` fn
    fn repl(&mut self) -> Result<(), Error> {
        loop {
            // Any pending command in the queue gets flushed
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event)?;
        }

        Ok(())
    }

    /// At the start we draw all the rows with the tilde '~' like vim does
    /// starting from top it clears each line, prints ~, and check if we are not at end
    pub fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = term::get_size()?;
        for curr in 0..height {
            term::clear_line()?;

            // One third of the way we draw the Welcome Message
            if curr == height / 3 {
                Self::draw_welcome_msg()?;
            } else {
                term::print("~")?;
            }

            // for height just at max, we do not add newline to avoid scroll off
            if curr + 1 < height {
                term::print("\r\n")?;
            }
        }

        Ok(())
    }

    /// This does most of the heavy lifting
    /// Executes all the queued commands, check for `should_quit`, clears screens and `draw_rows`
    fn refresh_screen(&mut self) -> Result<(), Error> {
        // to avoid weird cursor blinking, we hide it
        self.cursor.hide()?;
        Cursor::move_to(Position::default())?;

        // any cursor moving happens actually at screen refresh
        if self.should_quit {
            term::clear_screen()?;
            term::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Cursor::move_to(Position {
                x: self.cursor.location.x,
                y: self.cursor.location.y,
            })?;
        }

        // show the cursor at the end of all the refreshing
        self.cursor.show()?;

        // flush the stdout buffer
        term::execute()
    }

    /// draws the welcome message third of the way from top and at half width
    pub fn draw_welcome_msg() -> Result<(), Error> {
        let termsize = term::get_size()?;

        let mut welcome_msg = format!("{NAME} editor -- version {VERSION}");

        let width = termsize.width as usize;

        let padding = (width.saturating_sub(welcome_msg.len())) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_msg = format!("~{spaces}{welcome_msg}");

        welcome_msg.truncate(width);

        term::print(&welcome_msg)
    }
}
