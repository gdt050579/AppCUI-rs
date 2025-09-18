use appcui::prelude::*;

use crate::regular_menus::RegularMenus;
mod regular_menus;


#[Desktop(events    = [CommandBarEvents,DesktopEvents,AppBarEvents], 
          commands  = [Next, A])]
struct MyDesktop {
    index: u32,
    regular_menus: Option<RegularMenus>,
}
impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            regular_menus: None,
        }
    }
}

impl DesktopEvents for MyDesktop {
    fn on_start(&mut self) { 
        // define and register a menu
        self.regular_menus = Some(RegularMenus::new(self.appbar_mut(), mydesktop::Commands::A));
    }
        
}
impl CommandBarEvents for MyDesktop {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F1"), "Next AppBar cofiguration", mydesktop::Commands::Next);
    }

    fn on_event(&mut self, command_id: mydesktop::Commands) {
        match command_id {
            mydesktop::Commands::Next => {
                self.index += 1;
            }
            _ => {}

        }
    }
}
impl AppBarEvents for MyDesktop {
    fn on_update(&self,appbar: &mut AppBar){
        self.regular_menus.as_ref().unwrap().activate(appbar);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MyDesktop::new()).command_bar().app_bar().build()?.run();
    Ok(())
}
