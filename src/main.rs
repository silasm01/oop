use tui::{Tui, text::*, button::*, Alignment::*, PosTypes::*};

mod tui;

fn main() {

    let mut tui = Tui::new();
    tui.add_obj(tui::Object::Buttonobj(Button::new(tui, "test".to_string(), RightBottom(), LeftTop(), vec![Pixel(75), Pixel(10)], true)));
    tui.display();

    loop {
        tui.reload_tui()
    }
}