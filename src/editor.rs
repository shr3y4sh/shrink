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

#[derive(Default)]
pub struct Editor {
    cursor: Cursor,
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        term::initialize().unwrap();
        let result = self.repl();
        term::terminate().unwrap();
        result.unwrap();
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
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

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event)?;
        }

        Ok(())
    }

    pub fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = term::get_size()?;
        for curr in 0..height {
            term::clear_line()?;

            if curr == height / 3 {
                Self::draw_welcome_msg()?;
            } else {
                term::print("~")?;
            }

            if curr + 1 < height {
                term::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        self.cursor.hide()?;
        Cursor::move_to(Position::default())?;
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

        self.cursor.show()?;
        term::execute()?;
        Ok(())
    }

    pub fn draw_welcome_msg() -> Result<(), Error> {
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
