use terminal_size::terminal_size;
use terminal_size::{Height, Width};

use crate::helpers::Position;

pub fn hide_cursor() {
    print!("\033[?25l")
}

pub fn show_cursor() {
    print!("\033[?25h")
}

pub fn move_cursor(p: Position) {
    print!("\033[{};{}H", p.1, p.0);
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

pub fn get_size() -> Option<(u16, u16)> {
    match terminal_size() {
        Some((Width(width), Height(height))) => Some((width, height - 1)),
        _ => None,
    }
}
