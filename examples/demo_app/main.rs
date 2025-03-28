use appcui::prelude::*;

const LOGO: [&str; 15] = [
    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒",
    "▒▒█████┐▒▒██████┐▒▒██████┐▒▒",
    "▒██┌──██┐▒██┌──██┐▒██┌──██┐▒",
    "▒███████│▒██████┌┘▒██████┌┘▒",
    "▒██┌──██│▒██┌───┘▒▒██┌───┘▒▒",
    "▒██│▒▒██│▒██│▒▒▒▒▒▒██│▒▒▒▒▒▒",
    "▒└─┘▒▒└─┘▒└─┘▒▒▒▒▒▒└─┘▒▒▒▒▒▒",
    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒",
    "▒▒▒▒█████┐▒▒██┐▒▒▒██┐▒██┐▒▒▒",
    "▒▒▒██┌──██┐▒██│▒▒▒██│▒██│▒▒▒",
    "▒▒▒██│▒▒└─┘▒██│▒▒▒██│▒██│▒▒▒",
    "▒▒▒██│▒▒██┐▒██│▒▒▒██│▒██│▒▒▒",
    "▒▒▒└█████┌┘▒└██████┌┘▒██│▒▒▒",
    "▒▒▒▒└────┘▒▒▒└─────┘▒▒└─┘▒▒▒",
    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒",
];

#[Desktop(events    = [CommandBarEvents,MenuEvents,DesktopEvents], 
          overwrite = OnPaint, 
          commands  = [AddWindow,Exit, NoArrange, Cascade, Vertical, Horizontal, Grid])]
struct MyDesktop {
    index: u32,
    arrange_method: Option<desktop::ArrangeWindowsMethod>,
    menu_arrange: Handle<Menu>,
}
impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            arrange_method: None,
            menu_arrange: Handle::None,
        }
    }
}
impl OnPaint for MyDesktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(theme.desktop.character);
        let attr = CharAttribute::with_color(theme.desktop.character.foreground,theme.desktop.character.background);
        let x = ((surface.size().width as i32) / 2 ) - 15;
        let mut y = ((surface.size().height as i32) / 2 ) - 7;
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
    }
        
}
impl CommandBarEvents for MyDesktop {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("Insert"), "Add new_window", mydesktop::Commands::AddWindow);
        commandbar.set(key!("Escape"), "Exit", mydesktop::Commands::Exit);
    }

    fn on_event(&mut self, command_id: mydesktop::Commands) {
        match command_id {
            mydesktop::Commands::AddWindow => {
                // create a new window
            }
            mydesktop::Commands::Exit => self.close(), 
            _ => {}

        }
    }
}
impl MenuEvents for MyDesktop {
    fn on_select(&mut self,_menu:Handle<Menu>,_item:Handle<menu::SingleChoice>,command:mydesktop::Commands){
        match command {
            mydesktop::Commands::NoArrange => self.arrange_method = None,
            mydesktop::Commands::Cascade => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Cascade),
            mydesktop::Commands::Vertical => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Vertical),
            mydesktop::Commands::Horizontal => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Horizontal),
            mydesktop::Commands::Grid => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Grid),
            _ => {}
        }
        let m = self.arrange_method;
        if let Some(method) = m {
            self.arrange_windows(method);
        }
    }

    fn on_update_menubar(&self,menubar: &mut MenuBar){
        menubar.add(self.menu_arrange);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MyDesktop::new()).command_bar().menu_bar().build()?.run();
    Ok(())
}
