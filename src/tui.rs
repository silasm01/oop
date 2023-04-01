use std::{io::stdout};

use crossterm::{cursor::MoveTo, execute, terminal::size};

use button::*;
pub(crate) mod button;

use text::*;
pub(crate) mod text;

#[derive(Debug, Clone)]
pub enum Object {
    Text(Text),
    Buttonobj(Button),
}

#[derive(Debug, Clone)]
pub struct Tui {
    objects: Vec<Object>,
    terminal_size: (u16, u16),
    total_weighted: i16,
}

impl Tui {
    pub fn new() -> Tui {
        Tui {
            objects: vec![],
            terminal_size: size().unwrap(),
            total_weighted: 0,
        }
    }

    pub fn add_obj(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn reload_tui(&mut self) {
        if size().unwrap() != self.terminal_size {
            self.display();
            self.terminal_size = size().unwrap();
        }
    }

    pub fn display(&self) {
        print!("\x1B[2J\x1B[1;1H");

        self.objects.iter().for_each(|x| match x {
            Object::Text(t) => t.display(),
            Object::Buttonobj(b) => b.display(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Alignment {
    LeftTop(),
    RightBottom(),
}

impl Alignment {
    pub fn get_type(&self, obj: &Object) -> i16{
        let i = match obj {
            Object::Text(_) => todo!(),
            Object::Buttonobj(_) => todo!(),
        };
        let out = match self {
            Alignment::LeftTop() => 0,
            Alignment::RightBottom() => 0,
        };
        out
    }
}

trait Objecttrait<Object> {
}

#[derive(Debug, Clone)]
pub enum PosTypes {
    Pixel(i16),
    Percent(i16),
    Weighted(i16),
}

impl PosTypes {
    pub fn get_type(&self, window_size: i16) -> i16 {
        let out = match *self {
            PosTypes::Pixel(o) => o,
            PosTypes::Percent(o) => window_size/o,
            PosTypes::Weighted(_) => 0,
        };
        return out;
    }
}