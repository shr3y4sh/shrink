use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};

mod terminal;
use terminal::{clear_screen, initialize, move_cursor, terminal_size, terminate};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        initialize().unwrap();
        let result = self.repl();
        terminate().unwrap();
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
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }

        Ok(())
    }

    pub fn draw_rows() -> Result<(), std::io::Error> {
        let height = terminal_size().1;

        for curr in 0..height {
            print!("~");

            if curr + 1 < height {
                print!("\r\n");
            }
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            move_cursor(0, 0)?;
        }

        Ok(())
    }
}
