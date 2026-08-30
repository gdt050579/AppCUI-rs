use appcui::prelude::*;
mod my_custom_control;
use my_custom_control::MyCustomControl;
mod my_window;
use my_window::MyWindow;

fn main() -> Result<(), appcui::system::Error> {
    App::new().size(Size::new(80, 24)).app_bar().window(|| MyWindow::new()).run()
}  