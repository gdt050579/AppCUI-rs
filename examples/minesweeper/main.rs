use appcui::prelude::*;
mod minesweeper_game;
use minesweeper_game::MinesweeperGame;

#[Desktop(events = [CommandBarEvents, MenuEvents, DesktopEvents], 
          overwrite = OnPaint, 
          commands = [Easy, Medium, Hard, Exit])]
struct MyDesktop {
    menu_game: Handle<Menu>,
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
    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.menu_game);
    }
    
    fn on_command(&mut self, _: Handle<Menu>, _: Handle<menu::Command>, command: mydesktop::Commands) {
        match command {
            mydesktop::Commands::Easy => {
                // Fixed window size for 5x5 grid with 3 bombs
                let mut win = Window::new(
                    "Minesweeper - Easy", 
                    Layout::new("d:c,w:23,h:14"), 
                    window::Flags::None
                );
                win.add(MinesweeperGame::new(5, 5, 3));
                self.add_window(win);
            },
            mydesktop::Commands::Medium => {
                // Fixed window size for 7x7 grid with 10 bombs
                let mut win = Window::new(
                    "Minesweeper - Medium", 
                    Layout::new("d:c,w:33,h:19"), 
                    window::Flags::None
                );
                win.add(MinesweeperGame::new(7, 7, 10));
                self.add_window(win);
            },
            mydesktop::Commands::Hard => {
                // Fixed window size for 10x10 grid with 25 bombs
                let mut win = Window::new(
                    "Minesweeper - Hard", 
                    Layout::new("d:c,w:45,h:25"), 
                    window::Flags::None
                );
                win.add(MinesweeperGame::new(10, 10, 25));
                self.add_window(win);
            },
            mydesktop::Commands::Exit => self.close(),
            _ => {}
        }
    }
}

impl DesktopEvents for MyDesktop {
    fn on_start(&mut self) {
        self.menu_game = self.register_menu(menu!("
            &Game, class: MyDesktop, items:[
                {&Easy, cmd: Easy},
                {&Medium, cmd: Medium},
                {&Hard, cmd: Hard},
                {---},
                {&Exit, cmd: Exit}
            ]
        "));
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MyDesktop::new()).menu_bar().command_bar().build()?.run();
    Ok(())
} 