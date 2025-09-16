use appcui::prelude::*;
mod color_custom_control;
mod text_custom_control;
mod my_window;
use color_custom_control::ColorCustomControl;
use text_custom_control::TextCustomControl;
use my_window::MyWindow;


fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(60, 24)).app_bar().build()?;
    // first window
    let mut w1 = MyWindow::new("Win-1",layout!("x:1,y:2,w:18,h:10"));
    w1.add(ColorCustomControl::new(layout!("x:1,y:1,w:10")));
    w1.add(button!("Button,x:1,y:3,w:10"));
    let mut w2 = MyWindow::new("Win-2",layout!("x:20,y:2,w:18,h:15"));
    w2.add(TextCustomControl::new(layout!("x:1,y:1,w:10")));
    w2.add(button!("Button,x:1,y:3,w:10"));
    a.add_window(w1);
    a.add_window(w2);
    a.run();
    Ok(())
}  