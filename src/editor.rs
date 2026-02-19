use std::io::Error;

use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};

use crate::editor::terminal as term;

mod terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

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

            if curr == height / 3 {
                Self::add_title()?;
            } else {
                term::print("~")?;
            }

            if curr + 1 < height {
                term::print("\r\n")?;
            }
        }

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

    pub fn add_title() -> Result<(), Error> {
        let termsize = term::get_size()?;

        let mut welcome_msg = format!("{NAME} editor -- version {VERSION}");

        let width = termsize.width as usize;

        let padding = (width.saturating_sub(welcome_msg.len())) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_msg = format!("~{spaces}{welcome_msg}");

        welcome_msg.truncate(width);

        term::print(&welcome_msg)?;

        Ok(())
    }
}
