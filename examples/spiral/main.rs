use appcui::prelude::*;
use appcui::ui::appbar::*;
use std::time::Duration;

mod spiral;
use spiral::Spiral;

#[Desktop(events    = [DesktopEvents, TimerEvents, MenuEvents, CommandBarEvents], 
          overwrite = OnPaint,
          commands  = [New, Exit])]
struct SpiralDesktop {
    spiral: Spiral,
    window_count: u32,
    main_menu: Handle<MenuEntry>,
}

impl SpiralDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            spiral: Spiral::new(200),
            window_count: 0,
            main_menu: Handle::None,
        }
    }
    
    fn add_new_window(&mut self) {
        self.window_count += 1;
        let title = format!("Spiral Window {}", self.window_count);
        let win = Window::new(&title, layout!("a:c,w:40,h:15"), window::Flags::None);
        self.add_window(win);
    }
}

impl OnPaint for SpiralDesktop {
    fn on_paint(&self, surface: &mut Surface, _: &Theme) {
        surface.clear(char!("' ',black,black"));
        
        let size = surface.size();
        let center_x = size.width as f64 / 2.0;
        let center_y = size.height as f64 / 2.0;
        let scale = (size.width.min(size.height) as f64) * 0.3;
        
        // Draw the spiral points
        for i in 0..self.spiral.points.len().saturating_sub(1) {
            let (x1, y1) = self.spiral.points[i];
            let (x2, y2) = self.spiral.points[i + 1];
            
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
                    // Use different colors based on the point's position in the spiral
                    let color = if i < self.spiral.points.len() / 3 {
                        Color::Gray
                    } else if i < 2 * self.spiral.points.len() / 3 {
                        Color::Silver
                    } else {
                        Color::White
                    };
                    
                    surface.write_char(
                        x,
                        y,
                        Character::new('█', color, Color::Black, CharFlags::None),
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

impl DesktopEvents for SpiralDesktop {
    fn on_start(&mut self) {
        if let Some(timer) = self.timer() {
            timer.start(Duration::from_millis(50));
        }
        
        self.main_menu = self.appbar_mut().add(MenuEntry::new("&File", menu!("
            class: SpiralDesktop, items:[
                {&New,cmd: New, key: Ctrl+N},
                {-},
                {&Exit,cmd: Exit, key: Escape}
            ]
        "),0,AppBarPosition::Left));
    }
}

impl MenuEvents for SpiralDesktop {
    fn on_command(&mut self, _:Handle<Menu>, _:Handle<menu::Command>, command:spiraldesktop::Commands) {
        match command {
            spiraldesktop::Commands::New => self.add_new_window(),
            spiraldesktop::Commands::Exit => self.close(),
        }
    }
    fn on_update_menubar(&self, menubar: &mut AppBar) {
        menubar.show(self.main_menu);
    }
}

impl CommandBarEvents for SpiralDesktop {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F1"), "New Window", spiraldesktop::Commands::New);
        commandbar.set(key!("Escape"), "Exit", spiraldesktop::Commands::Exit);
    }

    fn on_event(&mut self, command_id: spiraldesktop::Commands) {
        match command_id {
            spiraldesktop::Commands::New => self.add_new_window(),
            spiraldesktop::Commands::Exit => self.close(),
        }
    }
}

impl TimerEvents for SpiralDesktop {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        self.spiral.update();
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(SpiralDesktop::new()).menu_bar().command_bar().build()?.run();
    Ok(())
} 