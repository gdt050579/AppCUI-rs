use appcui::prelude::*;
use appcui::ui::appbar::*;

mod simple_menu;
mod aligned_menus;
mod disabled_menu;
mod progress_bar;
mod navigation;

const LOGO: [&str; 11] = [
    "   █████████                       ███████████                     ",
    "  ███░░░░░███                     ░░███░░░░░███                    ",
    " ░███    ░███  ████████  ████████  ░███    ░███  ██████   ████████ ",
    " ░███████████ ░░███░░███░░███░░███ ░██████████  ░░░░░███ ░░███░░███",
    " ░███░░░░░███  ░███ ░███ ░███ ░███ ░███░░░░░███  ███████  ░███ ░░░ ",
    " ░███    ░███  ░███ ░███ ░███ ░███ ░███    ░███ ███░░███  ░███     ",
    " █████   █████ ░███████  ░███████  ███████████ ░░████████ █████    ",
    "░░░░░   ░░░░░  ░███░░░   ░███░░░  ░░░░░░░░░░░   ░░░░░░░░ ░░░░░     ",
    "               ░███      ░███                                      ",
    "               █████     █████                                     ",
    "              ░░░░░     ░░░░░                                      ",
];

#[Desktop(events    = [MenuEvents,DesktopEvents,AppBarEvents], 
          overwrite = OnPaint, 
          commands  = [ShowSimpleMenus, ShowAlignedMenus, ShowDisableMenu, ShowProgressBar, ShowNavigation,
                       Exit, About, 
                       NoArrange, Cascade, Vertical, Horizontal, Grid])]
struct MyDesktop {
    index: u32,
    arrange_method: Option<desktop::ArrangeWindowsMethod>,
    menu_arrange: Handle<MenuButton>,
    menu_example: Handle<MenuButton>,
}
impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            arrange_method: None,
            menu_arrange: Handle::None,
            menu_example: Handle::None,
        }
    }
    fn update_arrange_windows_method(&mut self, method: Option<desktop::ArrangeWindowsMethod>) {
        self.arrange_method = method;
        if let Some(method) = method {
            self.arrange_windows(method);
        }
    }  
}
impl OnPaint for MyDesktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let attr = CharAttribute::with_color(theme.desktop.character.foreground,theme.desktop.character.background);
        surface.clear(Character::with_attributes(' ', attr));
        let x = (surface.size().width as i32 - 50)/2;
        let mut y = (surface.size().height as i32  - 12)/2;
        for line in LOGO {
            surface.write_string(x, y, line, attr, false);
            y += 1;
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
        self.menu_arrange = self.appbar().add(MenuButton::new("&Windows",menu!("
            class: MyDesktop, items:[
                {'&No arrangament',cmd: NoArrange, select: true},
                {&Cascade,cmd: Cascade, select: false},
                {&Vertical,cmd: Vertical, select: false},
                {&Horizontal,cmd: Horizontal, select: false},
                {&Grid,cmd: Grid, select: false},
            ]
        "),0,Side::Left));
        self.menu_example = self.appbar().add(MenuButton::new("&Examples",menu!("
            class: MyDesktop, items:[
                {'Simple menus',cmd: ShowSimpleMenus},
                {'Menus with alignment', cmd: ShowAlignedMenus },
                {'Disabled menu', cmd: ShowDisableMenu },
                {'Progress bar', cmd: ShowProgressBar },
                {'Navigation', cmd: ShowNavigation },
            ]
        "),0,Side::Left));
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
            mydesktop::Commands::ShowSimpleMenus => { 
                self.add_window(simple_menu::Win::new());
            },          
            mydesktop::Commands::ShowAlignedMenus => { 
                self.add_window(aligned_menus::Win::new());
            },     
            mydesktop::Commands::ShowDisableMenu => { 
                self.add_window(disabled_menu::Win::new());
            },       
            mydesktop::Commands::ShowProgressBar => { 
                self.add_window(progress_bar::Win::new());
            },     
            mydesktop::Commands::ShowNavigation => { 
                self.add_window(navigation::Win::new());
            },                                    
            mydesktop::Commands::Exit => self.close(),   
            _ => { }      
        }
    }


}
impl AppBarEvents for MyDesktop {
    fn on_update(&self,appbar: &mut AppBar) {
        appbar.show(self.menu_example);
        appbar.show(self.menu_arrange);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MyDesktop::new()).app_bar().build()?.run();
    Ok(())
}
