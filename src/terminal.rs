use std::io::{self, Write};

use terminal_size::terminal_size;
use terminal_size::{Height, Width};

use crate::helpers::Position;

pub fn hide_cursor() {
    print!("\x1B[?25l")
}

pub fn show_cursor() {
    print!("\x1B[?25h")
}

pub fn move_cursor(p: Position) {
    print!("\x1B[{};{}H", p.1, p.0);
}

pub fn clear() {
    print!("\x1B[2J")
}

pub fn draw(str: &str) {
    print!("{}", str)
}

pub fn render() {
    io::stdout().flush().unwrap();
}

/// Get the current size of the display
pub fn get_size() -> (u16, u16) {
    terminal_size()
        .map(|(Width(width), Height(height))| (width, height - 1))
        .expect("Couldn't determine display size. Are you using a TTY?")
}
