mod viewer;
use appcui::prelude::*;
use my_desktop::MyDesktop;
use viewer::Viewer;
mod my_desktop;

fn main() -> Result<(), appcui::system::Error> {
    // let mut app = App::new().build()?;
    // app.add_window(Viewer::new());
    let mut app = App::new().desktop(MyDesktop::new()).command_bar().menu_bar().build()?;

    app.run();
    Ok(())
}
