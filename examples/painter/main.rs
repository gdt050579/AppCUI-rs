use appcui::prelude::*;
mod painter_window;
use painter_window::PainterWindow;
mod painter_control;

#[Desktop(events = [MenuEvents, DesktopEvents],  
          overwrite = OnPaint,
          commands = [New, Exit])]
struct PainterDesktop {
    index: u32,
    menu_file: Handle<Menu>,
}

impl PainterDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            menu_file: Handle::None,
        }
    }
}

impl OnPaint for PainterDesktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(theme.desktop.character);
    }
}

impl DesktopEvents for PainterDesktop {
    fn on_start(&mut self) { 
        self.menu_file = self.register_menu(menu!("
            &File,class: PainterDesktop, items:[
                {'&New',cmd: New},
                {-},
                {'E&xit',cmd: Exit}
            ]
        "));
    }
}

impl MenuEvents for PainterDesktop {
    fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: painterdesktop::Commands) {
        match command {
            painterdesktop::Commands::New => {
                let name = format!("Painting─{}", self.index);
                self.index += 1;
                self.add_window(PainterWindow::new(&name));
            }
            painterdesktop::Commands::Exit => self.close(),
        }
    }

    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.menu_file);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .desktop(PainterDesktop::new())
        .menu_bar()
        .build()?
        .run();
    Ok(())
} 