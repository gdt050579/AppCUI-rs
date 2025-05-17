use appcui::prelude::*;
use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq)]
enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Copy, Clone)]
struct Cell {
    state: CellState,
    is_mine: bool,
    adjacent_mines: u8,
}

#[CustomControl(overwrite = OnPaint+OnKeyPressed+OnMouseEvent)]
pub struct MinesweeperGame {
    grid: Vec<Vec<Cell>>,
    cursor_x: usize,
    cursor_y: usize,
    remaining_mines: usize,
    width: usize,
    height: usize,
    game_over: bool,
    win: bool,
}

impl MinesweeperGame {
    pub fn new(width: usize, height: usize, num_mines: usize) -> Self {
        let mut game = Self {
            base: ControlBase::new(Layout::new("d:c,w:100%,h:100%"), true),
            grid: Vec::new(),
            cursor_x: 0,
            cursor_y: 0,
            remaining_mines: num_mines,
            width,
            height,
            game_over: false,
            win: false,
        };
        
        game.initialize_game();
        game
    }

    fn initialize_game(&mut self) {
        let mut rng = rand::thread_rng();
        self.grid = vec![vec![
            Cell {
                state: CellState::Hidden,
                is_mine: false,
                adjacent_mines: 0,
            };
            self.width
        ]; self.height];

        // Place mines randomly
        let mut mines_placed = 0;
        while mines_placed < self.remaining_mines {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            if !self.grid[y][x].is_mine {
                self.grid[y][x].is_mine = true;
                mines_placed += 1;
            }
        }

        // Calculate adjacent mines
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.grid[y][x].is_mine {
                    self.grid[y][x].adjacent_mines = self.count_adjacent_mines(x, y);
                }
            }
        }

        self.cursor_x = 0;
        self.cursor_y = 0;
        self.game_over = false;
        self.win = false;
    }

    fn count_adjacent_mines(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 && self.grid[ny as usize][nx as usize].is_mine {
                    count += 1;
                }
            }
        }
        count
    }

    fn reveal_cell(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height || self.game_over || self.win {
            return;
        }

        let cell = &mut self.grid[y][x];
        if cell.state != CellState::Hidden {
            return;
        }

        cell.state = CellState::Revealed;

        if cell.is_mine {
            self.game_over = true;
            
            // Show all mines for game over
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.grid[y][x].is_mine {
                        self.grid[y][x].state = CellState::Revealed;
                    }
                }
            }
            
            // Show the error message
            appcui::dialogs::error("Game Over", "You hit a mine!");
            
            return;
        }

        if cell.adjacent_mines == 0 {
            // Reveal adjacent cells for empty cells
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                        self.reveal_cell(nx as usize, ny as usize);
                    }
                }
            }
        }

        // Check for win condition
        let mut all_non_mines_revealed = true;
        for row in &self.grid {
            for cell in row {
                if !cell.is_mine && cell.state != CellState::Revealed {
                    all_non_mines_revealed = false;
                    break;
                }
            }
        }
        
        if all_non_mines_revealed {
            self.win = true;
            
            // Show all mines as flagged for win
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.grid[y][x].is_mine {
                        self.grid[y][x].state = CellState::Flagged;
                    }
                }
            }
            
            // Show congratulations message
            appcui::dialogs::message("Congratulations!", "You have cleared all mines!");
        }
    }

    fn toggle_flag(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height || self.game_over || self.win {
            return;
        }

        let cell = &mut self.grid[y][x];
        match cell.state {
            CellState::Hidden => {
                cell.state = CellState::Flagged;
                self.remaining_mines = self.remaining_mines.saturating_sub(1);
            }
            CellState::Flagged => {
                cell.state = CellState::Hidden;
                self.remaining_mines += 1;
            }
            _ => {}
        }
    }

    fn move_cursor(&mut self, dx: i32, dy: i32) {
        if self.game_over || self.win {
            return;
        }
        
        let new_x = (self.cursor_x as i32 + dx).clamp(0, self.width as i32 - 1) as usize;
        let new_y = (self.cursor_y as i32 + dy).clamp(0, self.height as i32 - 1) as usize;
        self.cursor_x = new_x;
        self.cursor_y = new_y;
    }
}

impl OnPaint for MinesweeperGame {
    fn on_paint(&self, surface: &mut Surface, _: &Theme) {
        let grid_attr = CharAttribute::with_color(Color::Gray, Color::Black);
        
        // Draw top border
        surface.write_char(0, 0, Character::new('┌', Color::Gray, Color::Black, CharFlags::None));
        for x in 0..self.width {
            for i in 0..3 {
                surface.write_char((x * 4 + i + 1) as i32, 0, Character::new('─', Color::Gray, Color::Black, CharFlags::None));
            }
            if x < self.width - 1 {
                surface.write_char((x * 4 + 4) as i32, 0, Character::new('┬', Color::Gray, Color::Black, CharFlags::None));
            }
        }
        surface.write_char((self.width * 4) as i32, 0, Character::new('┐', Color::Gray, Color::Black, CharFlags::None));

        // Draw grid cells
        for y in 0..self.height {
            // First row of each cell
            surface.write_char(0, (y * 2 + 1) as i32, Character::new('│', Color::Gray, Color::Black, CharFlags::None));
            
            for x in 0..self.width {
                let cell = &self.grid[y][x];
                let cell_ch = match cell.state {
                    CellState::Hidden => Character::new('■', Color::Gray, Color::Black, CharFlags::None),
                    CellState::Revealed => {
                        if cell.is_mine {
                            Character::new('*', Color::Red, Color::Black, CharFlags::None)
                        } else {
                            match cell.adjacent_mines {
                                0 => Character::new(' ', Color::White, Color::Black, CharFlags::None),
                                n => Character::new((b'0' + n) as char, Color::Green, Color::Black, CharFlags::None),
                            }
                        }
                    }
                    CellState::Flagged => Character::new('⚑', Color::Yellow, Color::Black, CharFlags::None),
                };
                
                // Draw cell content with value centered
                surface.write_char((x * 4 + 1) as i32, (y * 2 + 1) as i32, Character::new(' ', Color::White, Color::Black, CharFlags::None));
                surface.write_char((x * 4 + 2) as i32, (y * 2 + 1) as i32, cell_ch);
                surface.write_char((x * 4 + 3) as i32, (y * 2 + 1) as i32, Character::new(' ', Color::White, Color::Black, CharFlags::None));
                
                if x < self.width - 1 {
                    surface.write_char((x * 4 + 4) as i32, (y * 2 + 1) as i32, Character::new('│', Color::Gray, Color::Black, CharFlags::None));
                }
            }
            surface.write_char((self.width * 4) as i32, (y * 2 + 1) as i32, Character::new('│', Color::Gray, Color::Black, CharFlags::None));
            
            // Draw horizontal line after each cell row (except the last)
            if y < self.height - 1 {
                surface.write_char(0, (y * 2 + 2) as i32, Character::new('├', Color::Gray, Color::Black, CharFlags::None));
                for x in 0..self.width {
                    for i in 0..3 {
                        surface.write_char((x * 4 + i + 1) as i32, (y * 2 + 2) as i32, Character::new('─', Color::Gray, Color::Black, CharFlags::None));
                    }
                    if x < self.width - 1 {
                        surface.write_char((x * 4 + 4) as i32, (y * 2 + 2) as i32, Character::new('┼', Color::Gray, Color::Black, CharFlags::None));
                    }
                }
                surface.write_char((self.width * 4) as i32, (y * 2 + 2) as i32, Character::new('┤', Color::Gray, Color::Black, CharFlags::None));
            }
        }

        // Draw bottom border
        surface.write_char(0, (self.height * 2) as i32, Character::new('└', Color::Gray, Color::Black, CharFlags::None));
        for x in 0..self.width {
            for i in 0..3 {
                surface.write_char((x * 4 + i + 1) as i32, (self.height * 2) as i32, Character::new('─', Color::Gray, Color::Black, CharFlags::None));
            }
            if x < self.width - 1 {
                surface.write_char((x * 4 + 4) as i32, (self.height * 2) as i32, Character::new('┴', Color::Gray, Color::Black, CharFlags::None));
            }
        }
        surface.write_char((self.width * 4) as i32, (self.height * 2) as i32, Character::new('┘', Color::Gray, Color::Black, CharFlags::None));

        // Draw cursor (only if game is still in progress)
        if !self.game_over && !self.win {
            let cursor_bg = Color::Blue;
            surface.write_char(
                (self.cursor_x * 4 + 1) as i32, 
                (self.cursor_y * 2 + 1) as i32,
                Character::new(' ', Color::White, cursor_bg, CharFlags::None)
            );
            
            // Determine what character to display at the cursor
            let cell = &self.grid[self.cursor_y][self.cursor_x];
            let cursor_ch = match cell.state {
                CellState::Hidden => Character::new('■', Color::Silver, cursor_bg, CharFlags::None),
                CellState::Revealed => {
                    if cell.is_mine {
                        Character::new('*', Color::Red, cursor_bg, CharFlags::None)
                    } else {
                        match cell.adjacent_mines {
                            0 => Character::new(' ', Color::White, cursor_bg, CharFlags::None),
                            n => Character::new((b'0' + n) as char, Color::Green, cursor_bg, CharFlags::None),
                        }
                    }
                }
                CellState::Flagged => Character::new('⚑', Color::Yellow, cursor_bg, CharFlags::None),
            };
            
            surface.write_char((self.cursor_x * 4 + 2) as i32, (self.cursor_y * 2 + 1) as i32, cursor_ch);
            surface.write_char(
                (self.cursor_x * 4 + 3) as i32, 
                (self.cursor_y * 2 + 1) as i32,
                Character::new(' ', Color::White, cursor_bg, CharFlags::None)
            );
        }

        // Draw remaining mines indicator
        let message = format!("Mines remaining: {}", self.remaining_mines);
        surface.write_string(0, (self.height * 2 + 1) as i32, &message, grid_attr, false);
    }
}

impl OnKeyPressed for MinesweeperGame {
    fn on_key_pressed(&mut self, key: Key, _: char) -> EventProcessStatus {
        if self.game_over || self.win {
            return EventProcessStatus::Ignored;
        }
        
        match key.value() {
            key!("up") => self.move_cursor(0, -1),
            key!("down") => self.move_cursor(0, 1),
            key!("left") => self.move_cursor(-1, 0),
            key!("right") => self.move_cursor(1, 0),
            key!("space") => self.reveal_cell(self.cursor_x, self.cursor_y),
            key!("f") => self.toggle_flag(self.cursor_x, self.cursor_y),
            _ => return EventProcessStatus::Ignored,
        }
        
        EventProcessStatus::Processed
    }
}

impl OnMouseEvent for MinesweeperGame {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.game_over || self.win {
            return EventProcessStatus::Ignored;
        }
        
        match event {
            MouseEvent::Over(point) => {
                // Convert mouse coordinates to grid coordinates
                let grid_x = (point.x - 1) / 4;
                let grid_y = (point.y - 1) / 2;
                
                // Check if within grid bounds
                if grid_x >= 0 && grid_x < self.width as i32 && grid_y >= 0 && grid_y < self.height as i32 {
                    self.cursor_x = grid_x as usize;
                    self.cursor_y = grid_y as usize;
                    return EventProcessStatus::Processed;
                }
            },
            MouseEvent::Pressed(data) => {
                // Convert mouse coordinates to grid coordinates
                let grid_x = (data.x - 1) / 4;
                let grid_y = (data.y - 1) / 2;
                
                // Check if within grid bounds
                if grid_x >= 0 && grid_x < self.width as i32 && grid_y >= 0 && grid_y < self.height as i32 {
                    match data.button {
                        MouseButton::Left => {
                            self.reveal_cell(grid_x as usize, grid_y as usize);
                            return EventProcessStatus::Processed;
                        },
                        MouseButton::Right => {
                            self.toggle_flag(grid_x as usize, grid_y as usize);
                            return EventProcessStatus::Processed;
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
        
        EventProcessStatus::Ignored
    }
}
