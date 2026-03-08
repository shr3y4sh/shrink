use std::io::Error;

use crate::editor::terminal::{self as term, Size};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod buffer;
use buffer::Buffer;

pub struct View {
    buff: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        let Size { width, height } = self.size;

        if height == 0 || width == 0 {
            return Ok(());
        }

        let welcome_msg_position = height / 3;

        for curr in 0..height {
            if let Some(line) = self.buff.lines.get(curr) {
                let truncated_line = line.get(0..width).unwrap_or(line);
                Self::render_line(curr, truncated_line)?;
            } else if curr == welcome_msg_position && self.buff.is_empty() {
                Self::render_line(curr, &Self::build_welcome_msg(width))?;
            } else {
                Self::render_line(curr, "~")?;
            }
        }

        self.needs_redraw = false;

        Ok(())
    }

    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        term::move_caret(term::Position { column: 0, row: at })?;
        term::clear_line()?;
        term::print(line_text)
    }

    /// if file contents is not empty, load into buffer
    pub fn load_buffer(&mut self, file: &str) {
        if file.is_empty() {
            return;
        }

        self.buff = Buffer::init_buffer(file);
        self.needs_redraw = true;
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
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
            needs_redraw: true,
            size: term::get_size().unwrap_or_default(),
        }
    }
}
