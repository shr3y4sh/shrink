use std::{env, fs};

use crate::editor::view::line::Line;

use super::{
    cursor::Position,
    editorcommand::{Direction, EditorCommand},
    terminal::{self as term, Size},
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod buffer;
mod line;
pub mod location;

use location::Location;

use buffer::Buffer;

pub struct View {
    buff: Buffer,
    needs_redraw: bool,
    location: Location,
    scroll_offset: Location,
    size: Size,
}

impl View {
    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    /// Checks if file exists, if not, initialize empty buffer,
    /// else init buffer with contents
    pub fn load_file(&mut self) {
        let args: Vec<String> = env::args().collect();

        if let Some(file) = args.get(1) {
            let file_contents = fs::read_to_string(file).unwrap_or(String::new());

            self.load_buffer(&file_contents);
        } else {
            self.load_buffer("");
        }
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.change_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {}
        }
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let Size { width, height } = self.size;

        if height == 0 || width == 0 {
            return;
        }

        let vertical_center = height / 3;
        let top = self.scroll_offset.y;

        for curr in 0..height {
            if let Some(line) = self.buff.lines.get(curr.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);

                Self::render_line(curr, &line.get(left..right));
            } else if curr == vertical_center && self.buff.is_empty() {
                Self::render_line(curr, &Self::build_welcome_msg(width));
            } else {
                Self::render_line(curr, "~");
            }
        }

        self.needs_redraw = false;
    }

    /// Moves the caret position using the location
    /// * `code`: `KeyCode` event listened by the repl
    fn change_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let height = self.size.height;

        let doc_length = self.buff.lines.len();
        let line_length: usize = self.buff.lines.get(y).map_or(0, Line::len);

        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
                if let Some(line) = self.buff.lines.get(y) {
                    x = x.min(line.len());
                }
            }

            Direction::Down => {
                if let Some(next) = self.buff.lines.get(y + 1) {
                    y += 1;
                    x = x.min(next.len());
                }
            }

            Direction::Left => {
                if y > 0 && x == 0 {
                    y -= 1;
                    x = self.buff.lines.get(y).map_or(0, Line::len);
                } else {
                    x = x.saturating_sub(1);
                }
            }
            Direction::Right => {
                if x < line_length {
                    x += 1;
                } else if y < doc_length {
                    x = 0;
                    y += 1;
                }
            }
            Direction::End => x = line_length,
            Direction::Home => x = 0,
            Direction::PageUp => y = y.saturating_sub(height),
            Direction::PageDown => y = (y + height).min(doc_length),
        }

        self.location = Location { x, y };
        self.scroll_location_into_view();
    }

    fn render_line(at: usize, line_text: &str) {
        let result = term::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render this line");
    }

    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }

        self.needs_redraw = offset_changed;
    }

    /// if file contents is not empty, load into buffer
    fn load_buffer(&mut self, file: &str) {
        if file.is_empty() {
            return;
        }

        self.buff = Buffer::init_buffer(file);
        self.needs_redraw = true;
    }

    fn resize(&mut self, size: Size) {
        self.size = size;
        self.scroll_location_into_view();
        self.needs_redraw = true;
    }

    fn build_welcome_msg(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }

        let mut welcome_msg = format!("{NAME} editor -- version {VERSION}");

        let len = welcome_msg.len();
        if width < len {
            return "~".to_string();
        }

        let padding = (width.saturating_sub(len)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_msg = format!("~{spaces}{welcome_msg}");

        welcome_msg.truncate(width);

        welcome_msg
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buff: Buffer::default(),
            location: Location::default(),
            scroll_offset: Location::default(),
            needs_redraw: true,
            size: term::get_size().unwrap_or_default(),
        }
    }
}
