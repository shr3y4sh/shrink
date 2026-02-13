use std::io::stdout;

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::size;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};

pub fn initialize() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    clear_screen()
}

pub fn clear_screen() -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))
}

pub fn terminate() -> Result<(), std::io::Error> {
    disable_raw_mode()
}

pub fn draw_rows() {
    const TILDE: char = '~';

    let mut cursor_pos = 0;
    let mut stdout = stdout();

    let screen_size = size().unwrap().1;

    for _ in 1..screen_size {
        let _ = execute!(stdout, MoveTo(0, cursor_pos));
        println!("{TILDE}");
        cursor_pos += 1;
    }

    let _ = execute!(stdout, MoveTo(0, 0));
}
