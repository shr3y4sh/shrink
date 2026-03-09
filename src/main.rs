#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
//! ## A terminal based file editor written in rust (for learning purpose)
//! Follows the tutorial by Phillip Flenker (www.phillipflenker.com/hecto)
//!
//! Created by Shreyash

mod editor;

use editor::Editor;

fn main() {
    let mut editor = Editor::new().unwrap();
    editor.run();
}
