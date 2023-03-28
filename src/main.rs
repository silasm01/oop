use tui::{Tui, text::*, button::*, VerticalAlignment::*, HorizontalAlignment::*, PosTypes::*};

mod tui;

fn main() {

    let mut tui = Tui::new();
    tui.add_obj(tui::Object::Buttonobj(Button::new("test".to_string(), Left(), Top(), vec![Pixel(75), Pixel(10)], true)));
    tui.display();

    loop {
        tui.reload_tui()
    }
}