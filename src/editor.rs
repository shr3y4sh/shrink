use std::{io::Error, panic};

use crossterm::event::{Event, KeyEvent, KeyEventKind, read};

use editorcommand::EditorCommand;

use {terminal as term, view::View};

mod cursor;
mod editorcommand;
mod terminal;
mod view;

pub struct Editor {
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
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            match EditorCommand::try_from(event) {
                Ok(command) => {
                    if matches!(command, EditorCommand::Quit) {
                        self.should_quit = true;
                    } else {
                        self.view.handle_command(command);
                    }
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not handle command: {err}");
                    }
                }
            }
        } else {
            #[cfg(debug_assertions)]
            {
                panic!("Recieved and discarded unsupported type event");
            }
        }
    }

    /// This does most of the heavy lifting
    /// Executes all the queued commands, check for `should_quit`, clears screens and `draw_rows`
    fn refresh_screen(&mut self) -> Result<(), Error> {
        cursor::hide_caret()?;

        self.view.render();
        cursor::move_caret(self.view.get_position())?;

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
