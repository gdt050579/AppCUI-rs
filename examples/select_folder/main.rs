use appcui::{dialogs::SelectFolderDialogFlags, prelude::*};

#[Desktop(events  = CommandBarEvents, commands  = [SelectFolder,Exit])]
struct MyDesktop {}
impl MyDesktop {
    fn new() -> Self {
        Self { base: Desktop::new() }
    }
}

impl CommandBarEvents for MyDesktop {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F1"), "Select Folder", mydesktop::Commands::SelectFolder);
        commandbar.set(key!("Escape"), "Exit", mydesktop::Commands::Exit);
    }

    fn on_event(&mut self, command_id: mydesktop::Commands) {
        match command_id {
            mydesktop::Commands::SelectFolder => {
                let res = dialogs::select_folder("Select Folder", dialogs::Location::Current, SelectFolderDialogFlags::Icons);
                if let Some(path) = res {
                    let p = path.to_str().unwrap_or("???");
                    dialogs::message("Path", p);
                }
            }
            mydesktop::Commands::Exit => self.close(),
            _ => {}
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    //App::new().desktop(MyDesktop::new()).command_bar().build()?.run();
    App::new().desktop(MyDesktop::new()).log_file("log.txt",true).command_bar().build()?.run();
    Ok(())
}
