use crate::tui::*;
use std::{fmt::Debug};

#[derive(Debug)]
pub struct Button {
    string: String,
    pub vertical_alignment: VerticalAlignment,
    pub horizontal_alignment: HorizontalAlignment,
    pub size: Vec<PosTypes>,
    border: bool,
}

impl Objecttrait<Object> for Button {}

impl Button {
    pub fn new(
        input: String,
        vertical_alignment: VerticalAlignment,
        horizontal_alignment: HorizontalAlignment,
        size: Vec<PosTypes>,
        border: bool,
    ) -> Button {
        Button {
            string: input,
            vertical_alignment,
            horizontal_alignment,
            size,
            border,
        }
    }

    fn border(&self, vertical_alignment: u16, horizontal_alignment: u16) {
        for n in vertical_alignment as i16..getPosTypeValue(&self.size[0])+vertical_alignment as i16 {
            execute!(
                stdout(),
                MoveTo(n as u16, horizontal_alignment+getPosTypeValue(&self.size[1]) as u16)
            )
            .unwrap();
            println!("\u{2500}");

            execute!(
                stdout(),
                MoveTo(n as u16, horizontal_alignment as u16)
            )
            .unwrap();
            println!("\u{2500}");
        }

        for n in horizontal_alignment as i16..getPosTypeValue(&self.size[1])+horizontal_alignment as i16 {
            execute!(
                stdout(),
                MoveTo(vertical_alignment as u16 + getPosTypeValue(&self.size[0]) as u16, n as u16)
            )
            .unwrap();
            println!("\u{2502}");

            execute!(
                stdout(),
                MoveTo(vertical_alignment as u16, n as u16)
            )
            .unwrap();
            println!("\u{2502}");
        }
    }

    pub fn display(&self) {
        let verticalalignment = match &self.vertical_alignment {
            VerticalAlignment::Left() => (1 as u16, getPosTypeValue(&self.size[0])),
            VerticalAlignment::Right() => (size().unwrap().0 - getPosTypeValue(&self.size[0]) as u16, getPosTypeValue(&self.size[0])),
        };

        let horizontalalignment = match &self.horizontal_alignment {
            HorizontalAlignment::Top() => 1,
            HorizontalAlignment::Bottom() => size().unwrap().1 - getPosTypeValue(&self.size[1]) as u16,
        };

        if self.border {
            //self.border(verticalalignment, horizontalalignment);
            execute!(
                stdout(),
                MoveTo(verticalalignment+1, horizontalalignment+1)
            )
            .unwrap();
        } else {
            execute!(
                stdout(),
                MoveTo(verticalalignment, horizontalalignment)
            )
            .unwrap();
        }

        println!("test: {}", self.string);
    }
}
