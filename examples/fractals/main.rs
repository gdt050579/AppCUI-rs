use appcui::prelude::*;
use appcui::ui::menubar::*;
use std::time::Duration;

mod fractal;
use fractal::Fractal;

#[Desktop(events    = [DesktopEvents, TimerEvents, MenuEvents, CommandBarEvents], 
          overwrite = OnPaint,
          commands  = [New, Exit])]
struct FractalDesktop {
    fractal: Fractal,
    window_count: u32,
    main_menu: Handle<MenuEntry>,
}

impl FractalDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            fractal: Fractal::new(8),
            window_count: 0,
            main_menu: Handle::None,
        }
    }
    
    fn add_new_window(&mut self) {
        self.window_count += 1;
        let title = format!("Fractal Window {}", self.window_count);
        let win = Window::new(&title, layout!("a:c,w:40,h:15"), window::Flags::None);
        self.add_window(win);
    }
}

impl OnPaint for FractalDesktop {
    fn on_paint(&self, surface: &mut Surface, _: &Theme) {
        surface.clear(char!("' ',black,black"));
        
        let size = surface.size();
        let center_x = size.width as f64 / 2.0;
        let center_y = size.height as f64 / 2.0;
        let scale = (size.width.min(size.height) as f64) * 0.6;
        
        // Draw the fractal lines
        for i in (0..self.fractal.points.len()).step_by(2) {
            if i + 1 < self.fractal.points.len() {
                let (x1, y1) = self.fractal.points[i];
                let (x2, y2) = self.fractal.points[i + 1];
                
                let start_x = (center_x + x1 * scale) as i32;
                let start_y = (center_y + y1 * scale) as i32;
                let end_x = (center_x + x2 * scale) as i32;
                let end_y = (center_y + y2 * scale) as i32;
                
                // Draw line using Bresenham's algorithm
                let dx = (end_x - start_x).abs();
                let dy = (end_y - start_y).abs();
                let sx = if start_x < end_x { 1 } else { -1 };
                let sy = if start_y < end_y { 1 } else { -1 };
                let mut err = dx - dy;
                
                let mut x = start_x;
                let mut y = start_y;
                
                while x != end_x || y != end_y {
                    if x >= 0 && x < size.width as i32 && y >= 0 && y < size.height as i32 {
                        surface.write_char(
                            x,
                            y,
                            char!("*,green,black"),
                        );
                    }
                    
                    let e2 = 2 * err;
                    if e2 > -dy {
                        err -= dy;
                        x += sx;
                    }
                    if e2 < dx {
                        err += dx;
                        y += sy;
                    }
                }
            }
        }
    }
}

impl DesktopEvents for FractalDesktop {
    fn on_start(&mut self) {
        if let Some(timer) = self.timer() {
            timer.start(Duration::from_millis(50));
        }
        
        self.main_menu = self.menubar_mut().add(MenuEntry::new(menu!("
            &File,class: FractalDesktop, items:[
                {&New,cmd: New, key: Ctrl+N},
                {-},
                {&Exit,cmd: Exit, key: Escape}
            ]
        "),0,MenuBarPosition::Left));
    }
}

impl MenuEvents for FractalDesktop {
    fn on_command(&mut self, _:Handle<Menu>, _:Handle<menu::Command>, command:fractaldesktop::Commands) {
        match command {
            fractaldesktop::Commands::New => self.add_new_window(),
            fractaldesktop::Commands::Exit => self.close(),
        }
    }
    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.show(self.main_menu);
    }
}

impl CommandBarEvents for FractalDesktop {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F1"), "New Window", fractaldesktop::Commands::New);
        commandbar.set(key!("Escape"), "Exit", fractaldesktop::Commands::Exit);
    }

    fn on_event(&mut self, command_id: fractaldesktop::Commands) {
        match command_id {
            fractaldesktop::Commands::New => self.add_new_window(),
            fractaldesktop::Commands::Exit => self.close(),
        }
    }
}

impl TimerEvents for FractalDesktop {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        self.fractal.update();
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(FractalDesktop::new()).menu_bar().command_bar().build()?.run();
    Ok(())
} 