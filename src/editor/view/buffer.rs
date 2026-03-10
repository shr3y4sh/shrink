use super::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn init_buffer(content: &str) -> Self {
        let mut lines = Vec::new();

        for x in content.lines() {
            lines.push(Line::from(x));
        }

        Self { lines }
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
