use std::io::stdout;

use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};

use crate::terminal::draw_rows;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        draw_rows();
        let result = self.repl();
        Self::terminate().unwrap();
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

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }

        Ok(())
    }
}
