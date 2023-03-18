use std::io::stdout;

use crossterm::{cursor::MoveTo, execute, terminal::size};

#[derive(Debug)]
pub enum Object {
    Text(Text),
    Button(Button),
}

#[derive(Debug)]
pub struct Tui {
    objects: Vec<Object>,
}

impl Tui {
    pub fn new() -> Tui {
        Tui { objects: vec![] }
    }

    pub fn add_obj(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn display(&self) {
        print!("\x1B[2J\x1B[1;1H");

        self.objects.iter().for_each(|x| match x {
            Object::Text(t) => t.display(),
            Object::Button(b) => b.display(),
        })
    }
}

#[derive(Debug)]
pub enum PosTypes {
    Pixel(i16),
    Percent(i16),
}

#[derive(Debug)]
pub enum VerticalAlignment {
    Left(PosTypes),
    Right(PosTypes),
}

#[derive(Debug)]
pub enum HorizontalAlignment {
    Top(PosTypes),
    Bottom(PosTypes),
}

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

#[derive(Debug)]
pub struct Button {
    string: String,
    vertical_alignment: VerticalAlignment,
    horizontal_alignment: HorizontalAlignment,
}

impl Button {
    pub fn new(
        input: String,
        vertical_alignment: VerticalAlignment,
        horizontal_alignment: HorizontalAlignment,
    ) -> Button {
        Button {
            string: input,
            vertical_alignment,
            horizontal_alignment,
        }
    }

    pub fn display(&self) {
        //println!("{}", obj.string)
        let vertalignment = match &self.vertical_alignment {
            VerticalAlignment::Left(u) => (u, 0),
            VerticalAlignment::Right(u) => (u, size().unwrap().0 as i16 * -1),
        };

        let verticalpostype: i16 = match vertalignment.0 {
            PosTypes::Pixel(u) => (vertalignment.1 + u).abs(),
            PosTypes::Percent(u) => size().unwrap().0 as i16 * *u / 100,
        };

        let horalignment = match &self.horizontal_alignment {
            HorizontalAlignment::Top(u) => (u, 0),
            HorizontalAlignment::Bottom(u) => (u, size().unwrap().1 as i16 * -1),
        };

        let horticalpostype: i16 = match horalignment.0 {
            PosTypes::Pixel(u) => (horalignment.1 + u).abs(),
            PosTypes::Percent(u) => size().unwrap().1 as i16 * *u / 100,
        };

        execute!(
            stdout(),
            MoveTo(verticalpostype as u16, horticalpostype as u16)
        )
        .unwrap();

        println!("{:?}, {:?}", verticalpostype as u16, size())
    }
}
