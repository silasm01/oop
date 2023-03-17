use std::io::stdout;

use crossterm::{
    execute,
    cursor::MoveTo
};

#[derive(Debug)]
pub struct Tui<T> {
    objects: Vec<T>
}

impl<U: std::fmt::Debug> Tui<U> {
    pub fn new() -> Tui<U> {
        Tui { objects: vec![] }
    }

    pub fn add_obj(&mut self, object:U ) {
        self.objects.append(&mut vec![object])
    }

    pub fn display(&self) {
        print!("\x1B[2J\x1B[1;1H");
        for obj in &self.objects {
            println!("{:?}", obj)
            //obj.display(&obj)
        }
    }
}

#[derive(Debug)]
pub struct Text {
    string: String,
    x: u16,
    y: u16,
}

impl Text {
    pub fn new(input: String, xin: u16, yin: u16) -> Text {
        Text { string: input, x: xin, y: yin }
    }

    pub fn display(&self, obj: &Text) {
        execute!(stdout(), MoveTo(self.x, self.y)).unwrap();
        println!("{:?}", obj.string)
    }
}