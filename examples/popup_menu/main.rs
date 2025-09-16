use appcui::prelude::*;
mod my_custom_control;
use my_custom_control::MyCustomControl;
mod my_window;
use my_window::MyWindow;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(80, 24)).app_bar().build()?;
    a.add_window(MyWindow::new());
    a.run();
    Ok(())
}  