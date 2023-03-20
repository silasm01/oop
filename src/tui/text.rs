use std::io::stdout;

use crossterm::{execute, cursor::MoveTo};

#[derive(Debug)]
pub struct Text {
    string: String,
    x: u16,
    y: u16,
}

impl Text {
    pub fn new(input: String, x: u16, y: u16) -> Text {
        Text {
            string: input,
            x,
            y,
        }
    }

    pub fn display(&self) {
        execute!(stdout(), MoveTo(self.x, self.y)).unwrap();
        println!("{}", self.string)
    }
}