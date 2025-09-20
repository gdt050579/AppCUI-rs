#[cfg(target_family = "windows")]
use appcui::backend;
use appcui::prelude::*;

mod dizzy;
mod ferris;
mod hello_rust;
mod mydesktop;
mod mywin;
mod shapes;

use mydesktop::MyDesktop;

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_family = "windows")]
    App::with_backend(backend::Type::WindowsVT)
        .desktop(MyDesktop::new())
        .command_bar()
        .app_bar()
        .build()?
        .run();

    #[cfg(not(target_family = "windows"))]
    App::new().desktop(MyDesktop::new()).command_bar().app_bar().build()?.run();
    Ok(())
}
