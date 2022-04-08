use terminal_size::terminal_size;
use terminal_size::{Height, Width};

use crate::helpers::Position;

pub fn hide_cursor() {
    print!("\\033[?25l")
}

pub fn show_cursor() {
    print!("\\033[?25h")
}

pub fn move_cursor(p: Position) {
    print!("\\033[{};{}H", p.1, p.0);
}

pub fn clear() {
    print!("\\033[2J")
}

pub fn draw(str: &str) {
    print!("{}", str)
}

pub fn render() {
    // screen.Flush()
}

/// Get the current size of the display
pub fn get_size() -> (u16, u16) {
    terminal_size()
        .map(|(Width(width), Height(height))| (width, height - 1))
        .expect("Couldn't determine display size. Are you using a TTY?")
}
