use crate::editor::cursor::Position;

/// ### Represents the location in the text of the file
/// Different from the Caret position (denoted by struct Position)
#[derive(Clone, Copy, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<Location> for Position {
    fn from(loc: Location) -> Self {
        Self {
            column: loc.x,
            row: loc.y,
        }
    }
}

impl Location {
    pub const fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}
