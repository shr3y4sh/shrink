#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn init_buffer(content: &str) -> Self {
        let mut lines = Vec::new();

        for x in content.lines() {
            lines.push(x.to_string());
        }

        Self { lines }
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
