mod my_desktop;
mod csv;
mod viewer;

use appcui::prelude::*;
use my_desktop::MyDesktop;
pub use csv::CSVFile;
pub use csv::CSVEntry;
pub use viewer::Viewer;


fn main() -> Result<(), appcui::system::Error> {    
    App::new().desktop(MyDesktop::new()).menu_bar().build()?.run();
    Ok(())
}
