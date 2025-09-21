use appcui::prelude::*;
use appcui::ui::appbar::*;
use rand::Rng;
use std::time::Duration;

mod matrix_column;
use matrix_column::MatrixColumn;

enum DisplayState {
    Loading,
    Running,
}

#[Desktop(events    = [DesktopEvents, TimerEvents, MenuEvents, CommandBarEvents, AppBarEvents], 
          overwrite = OnPaint,
          commands  = [New, Exit])]
struct MatrixDesktop {
    columns: Vec<MatrixColumn>,
    display_state: DisplayState,
    loading_chars_shown: usize,
    loading_start_time: u64,
    window_count: u32,
    main_menu: Handle<MenuButton>,
}

impl MatrixDesktop {
    const LOADING_TEXT: &'static str = "Loading the matrix...";
    const LOADING_WAIT_TIME: u64 = 6500; // 6.5 seconds

    fn new() -> Self {
        Self {
            base: Desktop::new(),
            columns: Vec::new(),
            display_state: DisplayState::Loading,
            loading_chars_shown: 0,
            loading_start_time: 0,
            window_count: 0,
            main_menu: Handle::None,
        }
    }
    
    fn init_matrix(&mut self) {
        let size = self.size();
        let width = size.width as i32;
        let height = size.height as i32;
        
        self.columns.clear();
        
        let mut rng = rand::thread_rng();
        for x in 0..width {
            if rng.gen_bool(0.8) {  // 80% chance to create a column at each position
                self.columns.push(MatrixColumn::new(x, height));
            }
        }        
    }
    
    fn add_new_window(&mut self) {
        self.window_count += 1;
        let title = format!("Matrix Window {}", self.window_count);
        let win = Window::new(&title, layout!("a:c,w:40,h:15"), window::Flags::None);
        self.add_window(win);
    }
}

impl OnPaint for MatrixDesktop {
    fn on_paint(&self, surface: &mut Surface, _: &Theme) {
        surface.clear(char!("' ',black,black"));
        
        match self.display_state {
            DisplayState::Loading => {
                // Draw the loading message with animation
                let size = surface.size();
                let x_pos = (size.width as i32 - MatrixDesktop::LOADING_TEXT.len() as i32) / 2;
                let y_pos = size.height as i32 / 2;
                
                // Only draw the characters that should be visible so far
                if self.loading_chars_shown > 0 {
                    let visible_text = &MatrixDesktop::LOADING_TEXT[0..self.loading_chars_shown.min(MatrixDesktop::LOADING_TEXT.len())];
                    surface.write_string(
                        x_pos,
                        y_pos,
                        visible_text,
                        CharAttribute::with_color(Color::Green, Color::Black),
                        false
                    );
                }
                
                if self.loading_chars_shown <= MatrixDesktop::LOADING_TEXT.len() {
                    surface.write_char(
                        x_pos + self.loading_chars_shown as i32,
                        y_pos,
                        char!("' ',black,green"),
                    );
                }
            },
            DisplayState::Running => {
                for column in &self.columns {
                    for i in 0..column.length {
                        if column.active[i] && column.positions[i] >= 0 && column.positions[i] < surface.size().height as i32 {
                            let x = column.column_pos;
                            let y = column.positions[i];
                            
                            let color = if i == 0 {
                                Color::Green
                            } else {
                                let alpha = i as f32 / column.length as f32;
                                if alpha > 0.5 {
                                    Color::Green
                                } else if alpha > 0.2 {
                                    Color::DarkGreen
                                } else {
                                    Color::Gray 
                                }
                            };
                            
                            surface.write_char(
                                x, 
                                y, 
                                Character::with_attributes(column.chars[i], CharAttribute::with_color(color, Color::Black))
                            );
                        }
                    }
                }
            }
        }
    }
}

impl DesktopEvents for MatrixDesktop {
    fn on_start(&mut self) {
        if let Some(timer) = self.timer() {
            timer.start(Duration::from_millis(50)); 
        }
        
        self.main_menu = self.appbar().add(MenuButton::new("&File", menu!("
            class: MatrixDesktop, items:[
                {&New,cmd: New, key: Ctrl+N},
                {-},
                {&Exit,cmd: Exit, key: Escape}
            ]
        "),0,Side::Left));
    }
}

impl MenuEvents for MatrixDesktop {
    fn on_command(&mut self, _:Handle<Menu>, _:Handle<menu::Command>, command:matrixdesktop::Commands) {
        match command {
            matrixdesktop::Commands::New => self.add_new_window(),
            matrixdesktop::Commands::Exit => self.close(),
        }
    }
}
impl AppBarEvents for MatrixDesktop {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.main_menu);
    }
}

impl CommandBarEvents for MatrixDesktop {
    fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
        commandbar.set(key!("F1"), "New Window", matrixdesktop::Commands::New);
        commandbar.set(key!("Escape"), "Exit", matrixdesktop::Commands::Exit);
    }

    fn on_event(&mut self, command_id: matrixdesktop::Commands) {
        match command_id {
            matrixdesktop::Commands::New => self.add_new_window(),
            matrixdesktop::Commands::Exit => self.close(),
        }
    }
}

impl TimerEvents for MatrixDesktop {
    fn on_update(&mut self, ticks: u64) -> EventProcessStatus {
        
        match self.display_state {
            DisplayState::Loading => {
                // Record the start time when we begin loading
                if self.loading_start_time == 0 {
                    self.loading_start_time = ticks;
                }
                
                // Animate the text one character at a time (every 5 ticks = 250ms)
                if ticks % 5 == 0 && self.loading_chars_shown < MatrixDesktop::LOADING_TEXT.len() {
                    self.loading_chars_shown += 1;
                }
                
                // After the text is fully shown, wait for the loading duration
                if self.loading_chars_shown >= MatrixDesktop::LOADING_TEXT.len() {
                    let elapsed = (ticks - self.loading_start_time) * 50; // Convert ticks to ms
                    if elapsed >= MatrixDesktop::LOADING_WAIT_TIME {
                        // Transition to running state
                        self.init_matrix();
                        self.display_state = DisplayState::Running;
                    }
                }
            },
            DisplayState::Running => {
                
                let size = self.size();
                let height = size.height as i32;
                
                for column in &mut self.columns {
                    column.update(height);
                }                
                let mut rng = rand::thread_rng();
                if self.columns.len() < size.width as usize && rng.gen_bool(0.05) {
                    let x = rng.gen_range(0..size.width as i32);
                    // Check if there's already a column at this position
                    if !self.columns.iter().any(|col| col.column_pos == x) {
                        self.columns.push(MatrixColumn::new(x, height));
                    }
                }
            }
        }
        
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MatrixDesktop::new()).app_bar().command_bar().build()?.run();
    Ok(())
} 