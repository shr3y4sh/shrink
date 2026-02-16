use std::io::Error;

use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};

use crate::editor::terminal as term;

mod terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        term::initialize().unwrap();
        let result = self.repl();
        term::terminate().unwrap();
        result.unwrap();
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }

                _ => (),
            }
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }

        Ok(())
    }

    pub fn draw_rows() -> Result<(), Error> {
        let term::Size { height, .. } = term::get_size()?;
        for curr in 0..height {
            term::clear_line()?;
            term::print("~")?;

            if curr + 1 < height {
                term::print("\r\n")?;
            }
        }

        term::execute()?;

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        term::hide_cursor()?;
        if self.should_quit {
            term::clear_screen()?;
            term::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            term::move_cursor_to(term::Position { x: 0, y: 0 })?;
        }

        term::show_cursor()?;
        term::execute()?;
        Ok(())
    }
}
