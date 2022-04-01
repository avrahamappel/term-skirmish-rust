use std::error::Error;

use terminal_size;
// use terminal_size::{Height, Width};

use crate::helpers::Position;

pub fn hide_cursor() {
    print!("\033[?25l")
}

pub fn show_cursor() {
    print!("\033[?25h")
}

pub fn move_cursor(p: Position) {
    print!("\033[{};{}H", p[1], p[0]);
}

pub fn clear() {
    print!("\033[2J")
}

pub fn draw(str: String) {
    print!("{}", str)
}

pub fn render() {
    // screen.Flush()
}

pub fn get_size() -> Result<(i32, i32), dyn Error> {
    if let Some(width, height) = terminal_size::terminal_size()? {
        Ok((width, height - 1))
    }
}
