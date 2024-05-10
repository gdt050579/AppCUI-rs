use appcui::prelude::*;

use crate::dizzy::DIZZY_PIXELS;
use crate::mywin::MyWin;

#[Desktop(events = MenuEvents+DesktopEvents, commands  = Dizzy+Exit+Grid+Vertical+Horizontal+Cascade)]
pub struct MyDesktop {
    index: u32,
    menu_windows: Handle<Menu>,
    menu_arrange: Handle<Menu>,
}
impl MyDesktop {
    pub fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            menu_windows: Handle::None,
            menu_arrange: Handle::None,
        }
    }
    fn open_dizzy(&mut self) {
        let mut dizzy = Image::new(256, 192).unwrap();
        for y in 0..192 {
            for x in 0..256 {
                dizzy.set_pixel(x, y, Pixel::from(DIZZY_PIXELS[(y * 256 + x) as usize]));
            }
        }
        self.add_window(MyWin::new("Dizzy", dizzy));
    }
}

impl DesktopEvents for MyDesktop {
    fn on_start(&mut self) {
        // define and register a menu
        self.menu_windows = self.register_menu(menu!(
            "&Windows,class: MyDesktop, items:[
                {'&Dizzy',Alt+1, cmd: Dizzy},
                {---},
                {'E&xit',cmd: Exit},
            ]"
        ));
        self.menu_arrange = self.register_menu(menu!(
          "&Arrange,class: MyDesktop, items:[
              {'&Grid',cmd: Grid},
              {'&Vertical',cmd: Vertical},
              {'&Horizontal',cmd: Horizontal},
              {'&Cascade',cmd: Cascade},
          ]"
      ));
    }
}

impl MenuEvents for MyDesktop {
    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.menu_windows);
        menubar.add(self.menu_arrange);
    }

    fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: mydesktop::Commands) {
        match command {
            mydesktop::Commands::Dizzy => self.open_dizzy(),
            mydesktop::Commands::Exit => self.close(),
            mydesktop::Commands::Grid => self.arrange_windows(desktop::ArrangeWindowsMethod::Grid),
            mydesktop::Commands::Vertical => self.arrange_windows(desktop::ArrangeWindowsMethod::Vertical),
            mydesktop::Commands::Horizontal => self.arrange_windows(desktop::ArrangeWindowsMethod::Horizontal),
            mydesktop::Commands::Cascade => self.arrange_windows(desktop::ArrangeWindowsMethod::Cascade),
            
        }
    }
}
