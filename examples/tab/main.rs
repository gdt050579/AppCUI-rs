use appcui::prelude::*;

mod my_window;
mod show_tab_modal;
use my_window::MyWindow;
use show_tab_modal::ShowTabModal;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWindow::new());
    a.run();
    Ok(())
}
