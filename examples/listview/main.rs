use appcui::prelude::*;

const LOGO: [&str; 6] = [
    "██╗     ██╗███████╗████████╗██╗   ██╗██╗███████╗██╗    ██╗",
    "██║     ██║██╔════╝╚══██╔══╝██║   ██║██║██╔════╝██║    ██║",
    "██║     ██║███████╗   ██║   ██║   ██║██║█████╗  ██║ █╗ ██║",
    "██║     ██║╚════██║   ██║   ╚██╗ ██╔╝██║██╔══╝  ██║███╗██║",
    "███████╗██║███████║   ██║    ╚████╔╝ ██║███████╗╚███╔███╔╝",
    "╚══════╝╚═╝╚══════╝   ╚═╝     ╚═══╝  ╚═╝╚══════╝ ╚══╝╚══╝ ",
];

#[Desktop(events    = [MenuEvents,DesktopEvents], 
          overwrite = OnPaint, 
          commands  = [ShowCountries, Exit, About, NoArrange, Cascade, Vertical, Horizontal, Grid])]
struct MyDesktop {
    index: u32,
    arrange_method: Option<desktop::ArrangeWindowsMethod>,
    menu_arrange: Handle<Menu>,
    menu_example: Handle<Menu>,
    menu_help: Handle<Menu>,
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
impl OnPaint for MyDesktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(theme.desktop.character);
        let attr = CharAttribute::with_color(theme.desktop.character.foreground,theme.desktop.character.background);
        let x = (surface.size().width as i32) - 57;
        let mut y = (surface.size().height as i32)  - 7;
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
        self.menu_arrange = self.register_menu(menu!("
            &Windows,class: MyDesktop, items:[
                {'&No arrangament',cmd: NoArrange, select: true},
                {&Cascade,cmd: Cascade, select: false},
                {&Vertical,cmd: Vertical, select: false},
                {&Horizontal,cmd: Horizontal, select: false},
                {&Grid,cmd: Grid, select: false},
            ]
        "));
        self.menu_example = self.register_menu(menu!("
            &Example,class: MyDesktop, items:[
                {&Countries,cmd: ShowCountries},
            ]
        "));
        self.menu_help = self.register_menu(menu!("
            &Help,class: MyDesktop, items:[
                {&About,cmd: About},
                {E&xit,cmd: Exit},
            ]
        "));
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
            mydesktop::Commands::ShowCountries => todo!(),
            mydesktop::Commands::Exit => todo!(),
            mydesktop::Commands::About => {
                dialogs::message("List View Example", "This is an example of a list view control in AppCUI");
            },            
        }
    }

    fn on_update_menubar(&self,menubar: &mut MenuBar) {
        menubar.add(self.menu_arrange);
        menubar.add(self.menu_example);
        menubar.add(self.menu_help);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MyDesktop::new()).menu_bar().build()?.run();
    Ok(())
}
