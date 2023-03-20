use crate::tui::*;

#[derive(Debug)]
pub struct Button {
    string: String,
    pub vertical_alignment: VerticalAlignment,
    pub horizontal_alignment: HorizontalAlignment,
    border: bool,
}

impl Objecttrait for Button {}

impl Button {
    pub fn new(
        input: String,
        vertical_alignment: VerticalAlignment,
        horizontal_alignment: HorizontalAlignment,
        border: bool,
    ) -> Button {
        Button {
            string: input,
            vertical_alignment,
            horizontal_alignment,
            border,
        }
    }

    pub fn display(&self) {
        let vertalignment = match &self.vertical_alignment {
            VerticalAlignment::Left(u) => (u, 0),
            VerticalAlignment::Right(u) => (u, size().unwrap().0 as i16 * -1),
        };

        let verticalpostype: i16 = match vertalignment.0 {
            PosTypes::Pixel(u) => (vertalignment.1 + u).abs(),
            PosTypes::Percent(u) => size().unwrap().0 as i16 * *u / 100,
            PosTypes::Weighted(_) => todo!(),
        };

        let horalignment = match &self.horizontal_alignment {
            HorizontalAlignment::Top(u) => (u, 0),
            HorizontalAlignment::Bottom(u) => (u, size().unwrap().1 as i16 * -1),
        };

        let horticalpostype: i16 = match horalignment.0 {
            PosTypes::Pixel(u) => (horalignment.1 + u).abs(),
            PosTypes::Percent(u) => size().unwrap().1 as i16 * *u / 100,
            PosTypes::Weighted(_) => todo!(),
        };

        execute!(
            stdout(),
            MoveTo(verticalpostype as u16, horticalpostype as u16)
        )
        .unwrap();

        println!("{}", self.string)
    }
}
