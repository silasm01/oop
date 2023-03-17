use std::{time::Duration, thread};

use tui::{Tui, Text};

mod tui;

fn main() {
    let mut tui = Tui::new();
    tui.add_obj(Text::new("Hello".to_string(), 5, 5));
    tui.display();
    thread::sleep(Duration::from_millis(1000));
    tui.add_obj(Text::new("Hello2".to_string(), 10, 10));
    //tui.add_obj("hellot543".to_string());
    tui.display();
    while true {
        
    }
}