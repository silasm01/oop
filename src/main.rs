use std::{time::Duration, thread};

use tui::{Tui, text::*, button::*, VerticalAlignment::*, HorizontalAlignment::*, PosTypes::*};

mod tui;

fn main() {

    let mut tui = Tui::new();
    tui.add_obj(tui::Object::Text(Text::new("Hello".to_string(), 5, 5)));
    tui.display();
    thread::sleep(Duration::from_millis(1000));
    tui.add_obj(tui::Object::Text(Text::new("Hello2".to_string(), 10, 10)));
    tui.add_obj(tui::Object::Buttonobj(Button::new("hello3".to_string(), Left(Percent(25)) , Top(Pixel(10)), false)));
    tui.add_obj(tui::Object::Buttonobj(Button::new("test".to_string(), Left(Pixel(25)), Bottom(Percent(75)), true)));
    tui.display();

    loop {
        tui.reload_tui()
    }
}