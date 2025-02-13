use std::path::{Path, PathBuf};

use appcui::prelude::*;

#[Window(events = ButtonEvents)]
struct MyWin {

}
impl MyWin {
    fn new()->Self {
        let mut w = Self {
            base: window!("Test,d:c")
        };
        w.add(button!("Press,d:c,w:14"));
        w
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
        //let p = PathBuf::from("C:\\");
        let p = PathBuf::from("E:\\Lucru\\Altele");
        log!("INFO", "Selected folder: {:?}", p);
        dialogs::select_folder("Folder", dialogs::Location::Path(&p));
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().log_file("debug.log",false).build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
