use appcui::prelude::*;
use appcui::ui::menubar::*;
use crate::CSVFile;
use crate::Viewer;

const LOGO: [&str; 6] = [
    " ██████╗███████╗██╗▒▒▒██╗▒▒▒▒██╗▒▒▒██╗██╗███████╗██╗▒▒▒▒██╗███████╗██████╗▒",
    "██╔════╝██╔════╝██║▒▒▒██║▒▒▒▒██║▒▒▒██║██║██╔════╝██║▒▒▒▒██║██╔════╝██╔══██╗",
    "██║▒▒▒▒▒███████╗██║▒▒▒██║▒▒▒▒██║▒▒▒██║██║█████╗▒▒██║▒█╗▒██║█████╗▒▒██████╔╝",
    "██║▒▒▒▒▒╚════██║╚██╗▒██╔╝▒▒▒▒╚██╗▒██╔╝██║██╔══╝▒▒██║███╗██║██╔══╝▒▒██╔══██╗",
    "╚██████╗███████║▒╚████╔╝▒▒▒▒▒▒╚████╔╝▒██║███████╗╚███╔███╔╝███████╗██║▒▒██║",
    "▒╚═════╝╚══════╝▒▒╚═══╝▒▒▒▒▒▒▒▒╚═══╝▒▒╚═╝╚══════╝▒╚══╝╚══╝▒╚══════╝╚═╝▒▒╚═╝",                                                                   
];

#[Desktop(events    = [MenuEvents,DesktopEvents], 
          overwrite = OnPaint, 
          commands  = [Open,Exit, NoArrange, Cascade, Vertical, Horizontal, Grid])]
pub struct MyDesktop {
    index: u32,
    arrange_method: Option<desktop::ArrangeWindowsMethod>,
    menu_arrange: Handle<MenuEntry>,
    menu_file: Handle<MenuEntry>,
}
impl MyDesktop {
    pub fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            arrange_method: None,
            menu_arrange: Handle::None,
            menu_file: Handle::None,
        }
    }
}
impl OnPaint for MyDesktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(theme.desktop.character);
        let attr = CharAttribute::with_color(theme.desktop.character.foreground,theme.desktop.character.background);
        let x = ((surface.size().width as i32) / 2 ) - 38;
        let mut y = ((surface.size().height as i32) / 2 ) - 3;
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
        self.menu_file = self.menubar_mut().add(MenuEntry::new(menu!("
            &File, class: MyDesktop, items:[
                {'&Open',cmd: Open, key: F3},
                {'E&xit',cmd: Exit, key: Escape},
            ]
        "),0,MenuBarPosition::Left));
        self.menu_arrange = self.menubar_mut().add(MenuEntry::new(menu!("
            &Windows, class: MyDesktop, items:[
                {'&No arrangament',cmd: NoArrange, select: true},
                {&Cascade,cmd: Cascade, select: false},
                {&Vertical,cmd: Vertical, select: false},
                {&Horizontal,cmd: Horizontal, select: false},
                {&Grid,cmd: Grid, select: false},
            ]
        "),0,MenuBarPosition::Left));
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

    fn on_command(&mut self, _:Handle<Menu>, _:Handle<menu::Command>, command:mydesktop::Commands) {
        match command {
            mydesktop::Commands::Open => {
                if let Some(file_path) = dialogs::open("Open",
                    "",
                    dialogs::Location::Last,
                    Some("Comma separated values = [csv]"),
                    dialogs::OpenFileDialogFlags::Icons
                ) {
                    if let Some(csv) = CSVFile::from_file(&file_path) {
                        self.add_window(Viewer::new(&file_path, csv));
                    } else {
                        dialogs::error("Error", format!("Invalid CSV file: {}", file_path.display()).as_str());
                    }
                }
            }
            mydesktop::Commands::Exit => self.close(),
            _ => {}
        }
    }

    fn on_update_menubar(&self,menubar: &mut MenuBar)
    {
        menubar.show(self.menu_file);
        menubar.show(self.menu_arrange);
    }
}