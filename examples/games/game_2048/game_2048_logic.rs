use appcui::prelude::*;
use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Playing,
    GameOver,
    Won,
}

#[derive(Copy, Clone)]
struct Tile {
    value: u32,
    color: Color,
}

impl Tile {
    fn new(value: u32) -> Self {
        let color = match value {
            0 => Color::Black,
            2 => Color::White,
            4 => Color::Silver,
            8 => Color::Yellow,
            16 => Color::Yellow,
            32 => Color::Yellow,
            64 => Color::Red,
            128 => Color::Red,
            256 => Color::Pink,
            512 => Color::Pink,
            1024 => Color::Magenta,
            2048 => Color::Magenta,
            _ => Color::Aqua,
        };
        Self { value, color }
    }
}

#[CustomControl(overwrite = OnPaint+OnKeyPressed)]
pub struct Game2048Logic {
    grid: [[Tile; 4]; 4],
    score: u32,
    game_state: GameState,
    moved: bool,
}

impl Game2048Logic {
    pub fn new() -> Self {
        let mut game = Self {
            base: ControlBase::new(layout!("d:f"), true),
            grid: [[Tile::new(0); 4]; 4],
            score: 0,
            game_state: GameState::Playing,
            moved: false,
        };
        
        game.add_random_tile();
        game.add_random_tile();
        game
    }

    fn add_random_tile(&mut self) {
        let mut empty_cells = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if self.grid[i][j].value == 0 {
                    empty_cells.push((i, j));
                }
            }
        }

        if !empty_cells.is_empty() {
            let mut rng = rand::thread_rng();
            let (i, j) = empty_cells[rng.gen_range(0..empty_cells.len())];
            self.grid[i][j] = Tile::new(if rng.gen_bool(0.9) { 2 } else { 4 });
        }
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;
        for i in 0..4 {
            let row = self.grid[i];
            let original_row = row;
            
            let non_zero: Vec<Tile> = row.iter().filter(|t| t.value != 0).cloned().collect();
            
            let mut merged = Vec::new();
            let mut j = 0;
            while j < non_zero.len() {
                if j + 1 < non_zero.len() && non_zero[j].value == non_zero[j + 1].value {
                    let new_value = non_zero[j].value * 2;
                    merged.push(Tile::new(new_value));
                    self.score += new_value;
                    if new_value == 2048 && self.game_state == GameState::Playing {
                        self.game_state = GameState::Won;
                    }
                    j += 2;
                } else {
                    merged.push(non_zero[j]);
                    j += 1;
                }
            }
            
            while merged.len() < 4 {
                merged.push(Tile::new(0));
            }
            
            for (k, tile) in merged.iter().enumerate() {
                self.grid[i][k] = *tile;
            }
            
            if original_row.iter().zip(self.grid[i].iter()).any(|(a, b)| a.value != b.value) {
                moved = true;
            }
        }
        moved
    }

    fn move_right(&mut self) -> bool {
        let mut moved = false;
        for i in 0..4 {
            let row = self.grid[i];
            let original_row = row;
            
            let non_zero: Vec<Tile> = row.iter().filter(|t| t.value != 0).cloned().collect();
            
            let mut merged = Vec::new();
            let mut j = non_zero.len();
            while j > 0 {
                j -= 1;
                if j > 0 && non_zero[j].value == non_zero[j - 1].value {
                    let new_value = non_zero[j].value * 2;
                    merged.insert(0, Tile::new(new_value));
                    self.score += new_value;
                    if new_value == 2048 && self.game_state == GameState::Playing {
                        self.game_state = GameState::Won;
                    }
                    j -= 1;
                } else {
                    merged.insert(0, non_zero[j]);
                }
            }
            
            while merged.len() < 4 {
                merged.insert(0, Tile::new(0));
            }
            
            for (k, tile) in merged.iter().enumerate() {
                self.grid[i][k] = *tile;
            }
            
            if original_row.iter().zip(self.grid[i].iter()).any(|(a, b)| a.value != b.value) {
                moved = true;
            }
        }
        moved
    }

    fn move_up(&mut self) -> bool {
        let mut moved = false;
        for j in 0..4 {
            let col = [self.grid[0][j], self.grid[1][j], self.grid[2][j], self.grid[3][j]];
            let original_col = col;
            
            let non_zero: Vec<Tile> = col.iter().filter(|t| t.value != 0).cloned().collect();
            
            let mut merged = Vec::new();
            let mut i = 0;
            while i < non_zero.len() {
                if i + 1 < non_zero.len() && non_zero[i].value == non_zero[i + 1].value {
                    let new_value = non_zero[i].value * 2;
                    merged.push(Tile::new(new_value));
                    self.score += new_value;
                    if new_value == 2048 && self.game_state == GameState::Playing {
                        self.game_state = GameState::Won;
                    }
                    i += 2;
                } else {
                    merged.push(non_zero[i]);
                    i += 1;
                }
            }
            
            while merged.len() < 4 {
                merged.push(Tile::new(0));
            }
            
            for (k, tile) in merged.iter().enumerate() {
                self.grid[k][j] = *tile;
            }
            
            if original_col.iter().zip([self.grid[0][j], self.grid[1][j], self.grid[2][j], self.grid[3][j]].iter()).any(|(a, b)| a.value != b.value) {
                moved = true;
            }
        }
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved = false;
        for j in 0..4 {
            let col = [self.grid[0][j], self.grid[1][j], self.grid[2][j], self.grid[3][j]];
            let original_col = col;
            
            let non_zero: Vec<Tile> = col.iter().filter(|t| t.value != 0).cloned().collect();
            
            let mut merged = Vec::new();
            let mut i = non_zero.len();
            while i > 0 {
                i -= 1;
                if i > 0 && non_zero[i].value == non_zero[i - 1].value {
                    let new_value = non_zero[i].value * 2;
                    merged.insert(0, Tile::new(new_value));
                    self.score += new_value;
                    if new_value == 2048 && self.game_state == GameState::Playing {
                        self.game_state = GameState::Won;
                    }
                    i -= 1;
                } else {
                    merged.insert(0, non_zero[i]);
                }
            }
            
            while merged.len() < 4 {
                merged.insert(0, Tile::new(0));
            }
            
            for (k, tile) in merged.iter().enumerate() {
                self.grid[k][j] = *tile;
            }
            
            if original_col.iter().zip([self.grid[0][j], self.grid[1][j], self.grid[2][j], self.grid[3][j]].iter()).any(|(a, b)| a.value != b.value) {
                moved = true;
            }
        }
        moved
    }

    fn is_game_over(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.grid[i][j].value == 0 {
                    return false;
                }
            }
        }
        
        for i in 0..4 {
            for j in 0..4 {
                let current = self.grid[i][j].value;
                if j < 3 && self.grid[i][j + 1].value == current {
                    return false;
                }
                if i < 3 && self.grid[i + 1][j].value == current {
                    return false;
                }
            }
        }
        
        true
    }

    fn reset_game(&mut self) {
        self.grid = [[Tile::new(0); 4]; 4];
        self.score = 0;
        self.game_state = GameState::Playing;
        self.add_random_tile();
        self.add_random_tile();
    }
}

impl OnKeyPressed for Game2048Logic {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Enter") | key!("Space") => {
                if self.game_state == GameState::GameOver || self.game_state == GameState::Won {
                    self.reset_game();
                }
                EventProcessStatus::Processed
            }
            key!("Left") | key!("A") => {
                if self.game_state == GameState::Playing {
                    self.moved = self.move_left();
                    if self.moved {
                        self.add_random_tile();
                        if self.is_game_over() {
                            self.game_state = GameState::GameOver;
                        }
                    }
                }
                EventProcessStatus::Processed
            }
            key!("Right") | key!("D") => {
                if self.game_state == GameState::Playing {
                    self.moved = self.move_right();
                    if self.moved {
                        self.add_random_tile();
                        if self.is_game_over() {
                            self.game_state = GameState::GameOver;
                        }
                    }
                }
                EventProcessStatus::Processed
            }
            key!("Up") | key!("W") => {
                if self.game_state == GameState::Playing {
                    self.moved = self.move_up();
                    if self.moved {
                        self.add_random_tile();
                        if self.is_game_over() {
                            self.game_state = GameState::GameOver;
                        }
                    }
                }
                EventProcessStatus::Processed
            }
            key!("Down") | key!("S") => {
                if self.game_state == GameState::Playing {
                    self.moved = self.move_down();
                    if self.moved {
                        self.add_random_tile();
                        if self.is_game_over() {
                            self.game_state = GameState::GameOver;
                        }
                    }
                }
                EventProcessStatus::Processed
            }
            key!("R") => {
                self.reset_game();
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Processed
        }
    }
}

impl OnPaint for Game2048Logic {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("' ',black,black"));
        
        surface.write_string(1, 1, &format!("Score: {}", self.score), charattr!("white"), false);
        
        let start_x = 2;
        let start_y = 2;
        let cell_width = 8;
        let cell_height = 4;
        let grid_width = cell_width * 4 + 1;
        
        for i in 0..5 {
            let y = start_y + i * cell_height;
            for x in start_x..start_x + grid_width {
                if i == 0 {
                    if x == start_x {
                        surface.write_char(x, y, Character::new('┌', Color::Gray, Color::Black, CharFlags::None));
                    } else if x == start_x + grid_width - 1 {
                        surface.write_char(x, y, Character::new('┐', Color::Gray, Color::Black, CharFlags::None));
                    } else if (x - start_x) % cell_width == 0 {
                        surface.write_char(x, y, Character::new('┬', Color::Gray, Color::Black, CharFlags::None));
                    } else {
                        surface.write_char(x, y, Character::new('─', Color::Gray, Color::Black, CharFlags::None));
                    }
                } else if i == 4 {
                    if x == start_x {
                        surface.write_char(x, y, Character::new('└', Color::Gray, Color::Black, CharFlags::None));
                    } else if x == start_x + grid_width - 1 {
                        surface.write_char(x, y, Character::new('┘', Color::Gray, Color::Black, CharFlags::None));
                    } else if (x - start_x) % cell_width == 0 {
                        surface.write_char(x, y, Character::new('┴', Color::Gray, Color::Black, CharFlags::None));
                    } else {
                        surface.write_char(x, y, Character::new('─', Color::Gray, Color::Black, CharFlags::None));
                    }
                } else {
                    if x == start_x {
                        surface.write_char(x, y, Character::new('├', Color::Gray, Color::Black, CharFlags::None));
                    } else if x == start_x + grid_width - 1 {
                        surface.write_char(x, y, Character::new('┤', Color::Gray, Color::Black, CharFlags::None));
                    } else if (x - start_x) % cell_width == 0 {
                        surface.write_char(x, y, Character::new('┼', Color::Gray, Color::Black, CharFlags::None));
                    } else {
                        surface.write_char(x, y, Character::new('─', Color::Gray, Color::Black, CharFlags::None));
                    }
                }
            }
        }
        
        for j in 0..5 {
            let x = start_x + j * cell_width;
            for i in 1..5 {
                let y = start_y + i * cell_height;
                for dy in 1..cell_height {
                    surface.write_char(x, y - dy, Character::new('│', Color::Gray, Color::Black, CharFlags::None));
                }
            }
        }
        
        for i in 0..4 {
            for j in 0..4 {
                let tile_x = start_x + 1 + j * cell_width;
                let tile_y = start_y + 1 + i * cell_height;
                
                let tile = self.grid[i as usize][j as usize];
                let bg_color = match tile.value {
                    0 => Color::Black,
                    2 => Color::DarkBlue,
                    4 => Color::DarkGreen,
                    8 => Color::DarkRed,
                    16 => Color::Magenta,
                    32 => Color::Olive,
                    64 => Color::Teal,
                    128 => Color::Blue,
                    256 => Color::Green,
                    512 => Color::Red,
                    1024 => Color::Pink,
                    2048 => Color::Yellow,
                    _ => Color::Aqua,
                };
                
                for dy in 0..3 {
                    for dx in 0..7 {
                        surface.write_char(tile_x + dx, tile_y + dy, Character::new(' ', Color::White, bg_color, CharFlags::None));
                    }
                }
                
                if tile.value > 0 {
                    let value_str = tile.value.to_string();
                    let text_x = tile_x + (7 - value_str.len() as i32) / 2;
                    let text_y = tile_y + 1;
                    
                    let color_attr = match tile.color {
                        Color::White => charattr!("white"),
                        Color::Yellow => charattr!("yellow"),
                        Color::Red => charattr!("red"),
                        Color::Pink => charattr!("magenta"),
                        Color::Magenta => charattr!("magenta"),
                        Color::Aqua => charattr!("aqua"),
                        _ => charattr!("white"),
                    };
                    surface.write_string(text_x, text_y, &value_str, color_attr, false);
                }
            }
        }
        
        match self.game_state {
            GameState::Won => {
                surface.write_string(1, 20, "Congratulations! You reached 2048!", charattr!("green"), false);
                surface.write_string(1, 21, "Press Enter or Space to play again", charattr!("yellow"), false);
            }
            GameState::GameOver => {
                surface.write_string(1, 20, "Game Over! No more moves available.", charattr!("red"), false);
                surface.write_string(1, 21, "Press Enter or Space to play again", charattr!("yellow"), false);
            }
            GameState::Playing => {
            }
        }
    }
}
