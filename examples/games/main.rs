use appcui::prelude::*;

mod pacman;
mod tetris;
mod connect_four;
mod game_2048;



#[Desktop(events    = [MenuEvents,DesktopEvents,AppBarEvents], 
          commands  = [Exit, About, Pacman, Tetris, ConnectFour, Game2048,
                       NoArrange, Cascade, Vertical, Horizontal, Grid])]
struct MyDesktop {
    index: u32,
    arrange_method: Option<desktop::ArrangeWindowsMethod>,
    menu_arrange: Handle<appbar::MenuButton>,
    menu_example: Handle<appbar::MenuButton>,
    menu_help: Handle<appbar::MenuButton>,
}
impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            arrange_method: None,
            menu_arrange: Handle::None,
            menu_example: Handle::None,
            menu_help: Handle::None,
        }
    }
    fn update_arrange_windows_method(&mut self, method: Option<desktop::ArrangeWindowsMethod>) {
        self.arrange_method = method;
        if let Some(method) = method {
            self.arrange_windows(method);
        }
    }  
}

impl DesktopEvents for MyDesktop {
    fn on_update_window_count(&mut self, _count: usize) {
        let m = self.arrange_method;
        if let Some(method) = m {
            self.arrange_windows(method);
        }
    }
    
    fn on_start(&mut self) { 
        // define and register a menu
        self.menu_arrange = self.appbar().add(appbar::MenuButton::new("&Windows",menu!("
            class: MyDesktop, items:[
                {'&No arrangament',cmd: NoArrange, select: true},
                {&Cascade,cmd: Cascade, select: false},
                {&Vertical,cmd: Vertical, select: false},
                {&Horizontal,cmd: Horizontal, select: false},
                {&Grid,cmd: Grid, select: false},
            ]
        "),0,appbar::Side::Left));
        self.menu_example = self.appbar().add(appbar::MenuButton::new("&Example",menu!("
            class: MyDesktop, items:[
                {'&Pacman Game',cmd: Pacman},
                {'&Tetris Game',cmd: Tetris},
                {'&Connect Four Game',cmd: ConnectFour},
                {'&2048 Game',cmd: Game2048},
            ]
        "),0,appbar::Side::Left));
        self.menu_help = self.appbar().add(appbar::MenuButton::new("&Help",menu!("
            class: MyDesktop, items:[
                {&About,cmd: About},
                {E&xit,cmd: Exit},
            ]
        "),0,appbar::Side::Left));
    }  
}
impl MenuEvents for MyDesktop {
    fn on_select(&mut self,_menu:Handle<Menu>,_item:Handle<menu::SingleChoice>,command:mydesktop::Commands){
        match command {
            mydesktop::Commands::NoArrange => self.update_arrange_windows_method(None),
            mydesktop::Commands::Cascade => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Cascade)),
            mydesktop::Commands::Vertical => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Vertical)),
            mydesktop::Commands::Horizontal => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Horizontal)),
            mydesktop::Commands::Grid => self.update_arrange_windows_method(Some(desktop::ArrangeWindowsMethod::Grid)),
            _ => {}
        }
    }
    fn on_command(&mut self, _menu:Handle<Menu>, _item:Handle<menu::Command>,command:mydesktop::Commands) {
        match command {
            mydesktop::Commands::Pacman => { 
                self.add_window(pacman::Win::new());
            },
            mydesktop::Commands::Tetris => { 
                self.add_window(tetris::Win::new());
            },
            mydesktop::Commands::ConnectFour => { 
                self.add_window(connect_four::Win::new());
            },
            mydesktop::Commands::Game2048 => { 
                self.add_window(game_2048::Win::new());
            },          
            mydesktop::Commands::Exit => self.close(),
            mydesktop::Commands::About => {
                dialogs::message("Games Example", "This is an example of a games designed in AppCUI");
            },     
            _ => { }      
        }
    }   
}
impl AppBarEvents for MyDesktop {
    fn on_update(&self,appbar: &mut AppBar) {
        appbar.show(self.menu_example);
        appbar.show(self.menu_arrange);
        appbar.show(self.menu_help);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    App::with_backend(appcui::backend::Type::WindowsVT).desktop(MyDesktop::new()).app_bar().color_schema(false).build()?.run();
    #[cfg(not(target_os = "windows"))]
    App::new().desktop(MyDesktop::new()).app_bar().color_schema(false).build()?.run();
    Ok(())
}
