use appcui::prelude::*;
mod color_custom_control;
mod my_window;
mod text_custom_control;
use color_custom_control::ColorCustomControl;
use my_window::MyWindow;
use text_custom_control::TextCustomControl;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .size(Size::new(60, 24))
        .window(|| {
            let mut win = MyWindow::new("Win-1", layout!("x:1,y:2,w:18,h:10"));
            win.add(ColorCustomControl::new(layout!("x:1,y:1,w:10")));
            win.add(button!("Button,x:1,y:3,w:10"));
            win
        })
        .window(|| {
            let mut win = MyWindow::new("Win-2", layout!("x:20,y:2,w:18,h:15"));
            win.add(TextCustomControl::new(layout!("x:1,y:1,w:10")));
            win.add(button!("Button,x:1,y:3,w:10"));
            win
        })
        .app_bar()
        .run()
}
