use appcui::prelude::*;

use crate::dizzy::DIZZY_PIXELS;
use crate::ferris::FERRIS_PIXELS;
use crate::hello_rust::HELLO_RUST_PIXELS;
use crate::mywin::MyWin;
use crate::shapes::SHAPES_PIXELS;

#[Desktop(events    = MenuEvents+DesktopEvents, 
          commands  = Dizzy+Hello+Shapes+Ferris+Exit+Grid+Vertical+Horizontal+Cascade)]
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
    fn create_image(width: u32, height: u32, buf: &[u32]) -> Image {
        let mut img = Image::new(width, height).unwrap();
        for y in 0..height {
            for x in 0..width {
                img.set_pixel(x, y, Pixel::from(buf[(y * width + x) as usize]));
            }
        }
        img
    }
}

impl DesktopEvents for MyDesktop {
    fn on_start(&mut self) {
        // define and register a menu
        self.menu_windows = self.register_menu(menu!(
            "&Windows,class: MyDesktop, items:[
                {'&Dizzy',Alt+1, cmd: Dizzy},
                {'&Hello Rust',Alt+2, cmd: Hello},
                {'&Shapes',Alt+3, cmd: Shapes},
                {'&Ferris',Alt+4, cmd: Ferris},
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
            mydesktop::Commands::Dizzy => {
                self.add_window(MyWin::new("Dizzy", MyDesktop::create_image(256, 192, DIZZY_PIXELS)));
            }
            mydesktop::Commands::Hello => {
                self.add_window(MyWin::new("Hello Rust", MyDesktop::create_image(163, 41, HELLO_RUST_PIXELS)));
            }
            mydesktop::Commands::Shapes => {
                self.add_window(MyWin::new("Shapes", MyDesktop::create_image(120, 60, SHAPES_PIXELS)));
            }
            mydesktop::Commands::Ferris => {
                self.add_window(MyWin::new("Ferris", MyDesktop::create_image(172, 122, FERRIS_PIXELS)));
            }
            mydesktop::Commands::Exit => self.close(),
            mydesktop::Commands::Grid => self.arrange_windows(desktop::ArrangeWindowsMethod::Grid),
            mydesktop::Commands::Vertical => self.arrange_windows(desktop::ArrangeWindowsMethod::Vertical),
            mydesktop::Commands::Horizontal => self.arrange_windows(desktop::ArrangeWindowsMethod::Horizontal),
            mydesktop::Commands::Cascade => self.arrange_windows(desktop::ArrangeWindowsMethod::Cascade),
        }
    }
}
