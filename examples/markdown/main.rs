mod viewer;
use appcui::prelude::*;
use my_desktop::MyDesktop;
use viewer::Viewer;
mod my_desktop;

fn main() -> Result<(), appcui::system::Error> {
    let app = App::new().desktop(MyDesktop::new()).command_bar().app_bar().build()?;
    app.run();
    Ok(())
}