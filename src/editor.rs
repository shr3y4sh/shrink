/*
 * Starting with the Editor struct
 * */

use std::{env::args, fs, io::Error};

use crossterm::event::{
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read,
};

use crate::editor::{cursor::Cursor, term::Position, terminal as term, view::View};

mod cursor;
mod terminal;
mod view;

#[derive(Default)]
pub struct Editor {
    cursor: Cursor,
    should_quit: bool,
    view: View,
}

impl Editor {
    /// This is the entry point into the program
    /// The terminal is initialized and call to repl starts the infinite loop to listen for events
    /// Result is stored in `result` the terminal is terminated
    pub fn run(&mut self) {
        term::initialize().unwrap();

        let args: Vec<String> = args().collect();
        self.load_file(&args);

        let result = self.repl();
        term::terminate().unwrap();
        result.unwrap();
    }

    /// Checks if file exists, if not, initialize empty buffer,
    /// else init buffer with contents
    fn load_file(&mut self, args: &[String]) {
        if let Some(file) = args.get(1) {
            let file_contents = fs::read_to_string(file).unwrap_or(String::new());

            self.view.load(&file_contents);
        } else {
            self.view.load("");
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
                    self.cursor.move_caret(*code)?;
                }
                _ => (),
            }
        }

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
        self.cursor.hide()?;
        Cursor::move_to(Position::default())?;

        // any cursor moving happens actually at screen refresh
        if self.should_quit {
            term::clear_screen()?;
            term::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
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
}
