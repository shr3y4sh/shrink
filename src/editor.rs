/*
 * Starting with the Editor struct
 * */

use std::{io::Error, panic};

use core::cmp::min;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read};

use crate::editor::{terminal as term, view::View};

mod cursor;
mod terminal;
mod view;

/// ### Represents the location in the text of the file
/// Different from the Caret position (denoted by struct Position)
#[derive(Clone, Copy, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    location: Location,
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        term::initialize()?;

        let panic_hook = panic::take_hook();

        panic::set_hook(Box::new(move |panic_info| {
            let _ = term::terminate();

            panic_hook(panic_info);
        }));

        let mut view = View::default();
        view.load_file();

        Ok(Self {
            location: Location::default(),
            should_quit: false,
            view,
        })
    }

    /// This is the entry point into the program
    /// The terminal is initialized and call to run starts the infinite loop to listen for events
    /// Refresh the screen at each iteration. `refresh_screen` flushes the stdout buffer each time,
    /// and any other command which is necessary to maintain the screen
    /// It also checks for `should_quit` variable, if the keypress event has signalled quitting
    pub fn run(&mut self) {
        loop {
            let _ = self.refresh_screen();

            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    /// Any key press or other event will be evaluated by this
    ///
    /// * `event` - from crossterm `Event` type which specifies the type of event
    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Right
                    | KeyCode::Left
                    | KeyCode::Down
                    | KeyCode::Up
                    | KeyCode::PageDown
                    | KeyCode::PageUp
                    | KeyCode::Home
                    | KeyCode::End,
                    _,
                ) => {
                    self.change_location(code);
                }

                _ => (),
            },
            Event::Resize(width, height) => {
                let y = height as usize;
                let x = width as usize;

                self.view.resize(terminal::Size {
                    width: x,
                    height: y,
                });
            }
            _ => (),
        }
    }

    /// Moves the caret position using the location
    /// * `code`: `KeyCode` event listened by the repl
    fn change_location(&mut self, code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let term::Size { width, height } = term::get_size().unwrap_or_default();

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
    }

    /// This does most of the heavy lifting
    /// Executes all the queued commands, check for `should_quit`, clears screens and `draw_rows`
    fn refresh_screen(&mut self) -> Result<(), Error> {
        cursor::hide_caret()?;
        cursor::move_caret(cursor::Position::default())?;

        self.view.render();
        cursor::move_caret(cursor::Position {
            column: self.location.x,
            row: self.location.y,
        })?;

        cursor::show_caret()?;

        term::execute()
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = term::terminate();
        if self.should_quit {
            let _ = term::print("Goodbye\r\n");
        }
    }
}
