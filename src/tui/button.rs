use crate::tui::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Button {
    tui: Tui,
    string: String,
    pub vertical_alignment: Alignment,
    pub horizontal_alignment: Alignment,
    pub size: Vec<PosTypes>,
    border: bool,
}

impl Objecttrait for Button {}

impl Button {
    pub fn new(
        tui: Tui,
        input: String,
        vertical_alignment: Alignment,
        horizontal_alignment: Alignment,
        size: Vec<PosTypes>,
        border: bool,
    ) -> Button {
        Button {
            tui,
            string: input,
            vertical_alignment,
            horizontal_alignment,
            size,
            border,
        }
    }

    fn border(&self, vertical_alignment: u16, horizontal_alignment: u16) {}

    pub fn display(&self) {
        execute!(
            stdout(),
            MoveTo(self.horizontal_alignment.get_type(&self.size).0 as u16, self.vertical_alignment.get_type(&self.size).1 as u16)
        )
        .unwrap();
        println!("{:?}", self.tui.objects)
    }
}
