use appcui::prelude::*;

mod my_window;
mod show_tab_modal;
use my_window::MyWindow;
use show_tab_modal::ShowTabModal;

fn main() -> Result<(), appcui::system::Error> {
    App::new().window(|| MyWindow::new()).run()
}
