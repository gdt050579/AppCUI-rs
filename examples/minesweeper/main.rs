use appcui::prelude::*;
use appcui::ui::appbar::*;
mod minesweeper_game;
mod mywin;
use mywin::MyWin;

#[Desktop(events = [CommandBarEvents, MenuEvents, DesktopEvents], 
          overwrite = OnPaint, 
          commands = [Easy, Medium, Hard, Extreme, Exit])]
struct MyDesktop {
    menu_game: Handle<MenuButton>,
}

impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            menu_game: Handle::None,
        }
    }
}

impl OnPaint for MyDesktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(theme.desktop.character);
    }
}

impl CommandBarEvents for MyDesktop {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("Escape"), "Exit", mydesktop::Commands::Exit);
    }

    fn on_event(&mut self, command_id: mydesktop::Commands) {
        if command_id == mydesktop::Commands::Exit { self.close() }
    }
}

impl MenuEvents for MyDesktop {
    fn on_update_menubar(&self, menubar: &mut AppBar) {
        menubar.show(self.menu_game);
    }
    
    fn on_command(&mut self, _: Handle<Menu>, _: Handle<menu::Command>, command: mydesktop::Commands) {
        match command {
            mydesktop::Commands::Easy => {
                self.add_window(MyWin::new("Easy", layout!("a:c,w:23,h:14"), Size::new(5, 5), 3));
            },
            mydesktop::Commands::Medium => {
                self.add_window(MyWin::new("Medium", layout!("a:c,w:31,h:18"), Size::new(7, 7), 10));
            },
            mydesktop::Commands::Hard => {
                self.add_window(MyWin::new("Hard", layout!("a:c,w:43,h:24"), Size::new(10, 10), 25));
            },
            mydesktop::Commands::Extreme => {
                self.add_window(MyWin::new("Extreme", layout!("a:c,w:83,h:24"), Size::new(20, 10), 80));
            },
            mydesktop::Commands::Exit => self.close(),
        }
    }
}

impl DesktopEvents for MyDesktop {
    fn on_start(&mut self) {
        self.menu_game = self.appbar_mut().add(MenuButton::new("&Game", menu!("
            class: MyDesktop, items:[
                {&Easy, cmd: Easy},
                {&Medium, cmd: Medium},
                {&Hard, cmd: Hard},
                {&Extreme, cmd: Extreme},
                {---},
                {&Exit, cmd: Exit}
            ]
        "),0,Side::Left));
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MyDesktop::new()).menu_bar().command_bar().build()?.run();
    Ok(())
} 