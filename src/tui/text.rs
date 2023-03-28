use std::io::stdout;

use crossterm::{execute, cursor::MoveTo};

use super::PosTypes;

#[derive(Debug)]
pub struct Text {
    string: String,
    x: u16,
    y: u16,
    size: Vec<PosTypes>
}

impl Text {
    pub fn new(input: String, x: u16, y: u16, size: Vec<PosTypes>) -> Text {
        Text {
            string: input,
            x,
            y,
            size,
        }
    }

    pub fn display(&self) {
        execute!(stdout(), MoveTo(self.x, self.y)).unwrap();
        println!("{}", self.string)
    }
}