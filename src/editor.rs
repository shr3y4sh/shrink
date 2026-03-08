/*
 * Starting with the Editor struct
 * */

use std::{env, fs, io::Error};

use core::cmp::min;

use crossterm::event::{
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read,
};

use crate::editor::{term::Position, terminal as term, view::View};

mod terminal;
mod view;

/// ### Represents the location in the text of the file
/// Different from the Caret position (denoted by struct Position)
#[derive(Clone, Copy, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct Editor {
    location: Location,
    should_quit: bool,
    view: View,
}

impl Editor {
    /// This is the entry point into the program
    /// The terminal is initialized and call to repl starts the infinite loop to listen for events
    /// Result is stored in `result` the terminal is terminated
    pub fn run(&mut self) {
        term::initialize().unwrap();

        self.load_file();

        let result = self.repl();
        term::terminate().unwrap();
        result.unwrap();
    }

    /// Checks if file exists, if not, initialize empty buffer,
    /// else init buffer with contents
    fn load_file(&mut self) {
        let args: Vec<String> = env::args().collect();

        if let Some(file) = args.get(1) {
            let file_contents = fs::read_to_string(file).unwrap_or(String::new());

            self.view.load_buffer(&file_contents);
        } else {
            self.view.load_buffer("");
        }
    }

    /// Any key press or other event will be evaluated by this
    ///
    /// * `event` - from crossterm `Event` type which specifies the type of event
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
                    self.change_location(*code)?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    /// Moves the caret position using the location
    /// * `code`: `KeyCode` event listened by the repl
    fn change_location(&mut self, code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let term::Size { width, height } = term::get_size()?;

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

    /// Refresh the screen at each iteration. `refresh_screen` flushes the stdout buffer each time,
    /// and any other command which is necessary to maintain the screen
    /// It also checks for `should_quit` variable, if the keypress event has signalled quitting
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

    /// This does most of the heavy lifting
    /// Executes all the queued commands, check for `should_quit`, clears screens and `draw_rows`
    fn refresh_screen(&mut self) -> Result<(), Error> {
        // to avoid weird cursor blinking, we hide it
        term::hide_caret()?;
        term::move_caret(Position::default())?;

        // any cursor moving happens actually at screen refresh
        if self.should_quit {
            term::clear_screen()?;
            term::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            term::move_caret(Position {
                x: self.location.x,
                y: self.location.y,
            })?;
        }

        // show the cursor at the end of all the refreshing
        term::show_caret()?;

        // flush the stdout buffer
        term::execute()
    }
}
