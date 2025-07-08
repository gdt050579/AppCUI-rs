use appcui::{dialogs::{OpenFileDialogFlags, SaveFileDialogFlags}, prelude::*};
mod painter_window;
use painter_window::PainterWindow;
mod painter_control;

#[Desktop(events = [MenuEvents, DesktopEvents],  
          overwrite = OnPaint,
          commands = [New, Exit, Open, Save])]
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
                {'&Open',cmd: Open},
                {'&Save',cmd: Save},
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
            painterdesktop::Commands::Open => {
                if let Some(file) = dialogs::open("Open surfaces", "", dialogs::Location::Last, Some("Surface image = [srf]"), OpenFileDialogFlags::Icons) {
                    if let Ok(painter) = PainterWindow::from_file(&file) {
                        self.add_window(painter);
                    } else {
                        dialogs::error("Error", "Failed to open the painting file.");
                    }
                }
            }
            painterdesktop::Commands::Save => {
                if let Some(path) = dialogs::save("Save surface", "", dialogs::Location::Last, Some("Surface image = [srf]"),SaveFileDialogFlags::Icons|SaveFileDialogFlags::ValidateOverwrite) {
                    if let Some(window) = self.active_window_handle() {
                        let h: Handle<PainterWindow> = unsafe { window.unsafe_cast() };
                        if let Some(painter) = self.window_mut(h) {
                            if let Err(err) = painter.save_to_file(&path) {
                                dialogs::error("Error", &format!("Failed to save the painting file: {err}"));
                            }
                        }   
                    }
                }
            }
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