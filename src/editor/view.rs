use std::io::Error;

use crate::editor::terminal::{self as term, Size};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod buffer;
use buffer::Buffer;

#[derive(Default)]
pub struct View {
    buff: Buffer,
}

impl View {
    pub fn render(&mut self) -> Result<(), Error> {
        let last_line = self.render_buffer_lines()?;

        Self::render_empty_lines(last_line, self.buff.is_empty())
    }

    /// if file contents is not empty, load into buffer
    pub fn load(&mut self, file: &str) {
        if file.is_empty() {
            return;
        }

        self.buff = Buffer::init_buffer(file);
    }

    /// We draw all the rows with the tilde '~' like vim does
    /// it clears each line, prints ~, and check if we are not at end
    fn render_empty_lines(start_from: usize, no_content: bool) -> Result<(), Error> {
        let Size { height, .. } = term::get_size()?;

        for curr in start_from..height {
            term::clear_line()?;
            // One third of the way we draw the Welcome Message
            if no_content && curr == height / 3 {
                Self::draw_welcome_msg()?;
            } else {
                term::print("~")?;
            }
            // for height just at max, we do not add newline to avoid scroll off
            if curr + 1 < height {
                term::print("\r\n")?;
            }
        }
        Ok(())
    }

    /// this returns number of buffer lines
    /// so that drawing of tildes happen after that line
    fn render_buffer_lines(&self) -> Result<usize, Error> {
        for line in &self.buff.lines {
            term::clear_line()?;
            term::print(line)?;
            term::print("\r\n")?;
        }

        Ok(self.buff.lines.len())
    }

    fn draw_welcome_msg() -> Result<(), Error> {
        let termsize = term::get_size()?;

        let mut welcome_msg = format!("{NAME} editor -- version {VERSION}");

        let width = termsize.width as usize;

        let padding = (width.saturating_sub(welcome_msg.len())) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_msg = format!("~{spaces}{welcome_msg}");

        welcome_msg.truncate(width);

        term::print(&welcome_msg)
    }
}
