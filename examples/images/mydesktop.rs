use appcui::prelude::*;

use crate::mywin::MyWin;
use crate::dizzy::DIZZY_PIXELS;

#[Desktop(events = MenuEvents+DesktopEvents, commands  = Dizzy+Exit)]
pub struct MyDesktop {
    index: u32,
    menu_windows: Handle<Menu>,
}
impl MyDesktop {
    pub fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            menu_windows: Handle::None,
        }
    }
    fn open_dizzy(&mut self) {
        let mut dizzy = Image::new(256, 192).unwrap();
        for y in 0..192 {
            for x in 0..256 {
                dizzy.set_pixel(x, y, Pixel::from(DIZZY_PIXELS[(y * 256 + x) as usize]));
            }
        }
        self.add_window(MyWin::new("Dizzy",dizzy));
    }
}

impl DesktopEvents for MyDesktop {
    fn on_update_window_count(&mut self, _count: usize) {
        // let m = self.arrange_method;
        // if let Some(method) = m {
        //     self.arrange_windows(method);
        // }
    }

    fn on_start(&mut self) {
        // define and register a menu
        self.menu_windows = self.register_menu(menu!(
            "&Windows,class: MyDesktop, items:[
                {'&Dizzy',Alt+1, cmd: Dizzy},
                {---},
                {'E&xit',cmd: Exit},
            ]"
        ));
    }
}

impl MenuEvents for MyDesktop {
    fn on_select(&mut self, _menu: Handle<Menu>, _item: Handle<menu::SingleChoice>, command: mydesktop::Commands) {
        // match command {
        //     mydesktop::Commands::NoArrange => self.arrange_method = None,
        //     mydesktop::Commands::Cascade => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Cascade),
        //     mydesktop::Commands::Vertical => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Vertical),
        //     mydesktop::Commands::Horizontal => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Horizontal),
        //     mydesktop::Commands::Grid => self.arrange_method = Some(desktop::ArrangeWindowsMethod::Grid),
        //     _ => {}
        // }
        // let m = self.arrange_method;
        // if let Some(method) = m {
        //     self.arrange_windows(method);
        // }
    }

    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.menu_windows);
    }

    fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: mydesktop::Commands) {
        match command {
            mydesktop::Commands::Dizzy => self.open_dizzy(),
            mydesktop::Commands::Exit => self.close(),
        }
    }
}
