#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]

/*
 * This is a terminal based editor make as a practice project for Rust
 * It is based on hecto, the tutorial provided by https://philippflenker.com/hecto/
 */

mod editor;
use editor::Editor;

fn main() {
    Editor::default().run();
}
