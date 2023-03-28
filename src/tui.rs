use std::io::stdout;

use crossterm::{cursor::MoveTo, execute, terminal::size};

use button::*;
pub(crate) mod button;

use text::*;
pub(crate) mod text;

#[derive(Debug)]
pub enum Object {
    Text(Text),
    Buttonobj(Button),
}

#[derive(Debug)]
pub struct Tui {
    objects: Vec<Object>,
    terminal_size: Result<(u16, u16), std::io::Error>,
    total_weighted: i16,
}

impl Tui {
    pub fn new() -> Tui {
        Tui {
            objects: vec![],
            terminal_size: size(),
            total_weighted: 0,
        }
    }

    pub fn add_obj(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn reload_tui(&mut self) {
        if size().unwrap() != *self.terminal_size.as_ref().unwrap() {
            self.display();
            self.terminal_size = size();
        }

        // self.total_weighted = 0;
        // self.objects.iter().for_each(|x| match x {
        //     Object::Text(_) => (),O
        //     Object::Buttonobj(b) => match &b.vertical_alignment {
        //         VerticalAlignment::Left(x) => match x {
        //             PosTypes::Pixel(p) => self.total_weighted += p,
        //             PosTypes::Percent(p) => self.total_weighted += p * size().unwrap().0 as i16 / 100,
        //             PosTypes::Weighted(_) => (),
        //         },
        //         VerticalAlignment::Right(_) => (),
        //     },
        // });

        //println!("total_weighted = {:?}, {:?}", &self.total_weighted, size().unwrap().0)
    }

    pub fn display(&self) {
        print!("\x1B[2J\x1B[1;1H");

        self.objects.iter().for_each(|x| match x {
            Object::Text(t) => t.display(),
            Object::Buttonobj(b) => b.display(),
        })
    }
}

#[derive(Debug)]
pub enum PosTypes {
    Pixel(i16),
    Percent(i16),
    Weighted(i16),
}

#[derive(Debug)]
pub enum VerticalAlignment {
    Left(),
    Right(),
}

#[derive(Debug)]
pub enum HorizontalAlignment {
    Top(),
    Bottom(),
}


trait Objecttrait<Object> {
}

pub fn getPosTypeValue(postype: &PosTypes) -> i16 {
    let out = match *postype {
        PosTypes::Pixel(o) => o,
        PosTypes::Percent(_) => 0,
        PosTypes::Weighted(_) => 0,
    };
    return out;
}