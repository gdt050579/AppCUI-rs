use appcui::prelude::*;

mod mydesktop;
mod mywin;
mod dizzy;

use mydesktop::MyDesktop;


fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MyDesktop::new()).command_bar().menu_bar().build()?.run();
    Ok(())
}