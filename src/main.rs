use std::{time::Duration, thread};

use crossterm::terminal::size;
use tui::{Tui, Text, Button, VerticalAlignment::*, HorizontalAlignment::*, PosTypes::*};

mod tui;

fn main() {

    let mut tui = Tui::new();
    tui.add_obj(tui::Object::Text(Text::new("Hello".to_string(), 5, 5)));
    tui.display();
    thread::sleep(Duration::from_millis(1000));
    tui.add_obj(tui::Object::Text(Text::new("Hello2".to_string(), 10, 10)));
    tui.add_obj(tui::Object::Button(Button::new("hello3".to_string(), Left(Percent(25)) , Top(Pixel(10)))));
    tui.display();

    let mut old_size = size();
    loop {
        thread::sleep(Duration::from_millis(10));
        if size().unwrap() != old_size.unwrap() {
            tui.display();
        }
        old_size = size();
    }
}