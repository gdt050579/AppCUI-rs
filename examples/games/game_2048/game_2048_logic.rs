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
}

impl Tile {
    fn new(value: u32) -> Self {
        Self { value }
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

            if original_col
                .iter()
                .zip([self.grid[0][j], self.grid[1][j], self.grid[2][j], self.grid[3][j]].iter())
                .any(|(a, b)| a.value != b.value)
            {
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

            if original_col
                .iter()
                .zip([self.grid[0][j], self.grid[1][j], self.grid[2][j], self.grid[3][j]].iter())
                .any(|(a, b)| a.value != b.value)
            {
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
            _ => EventProcessStatus::Processed,
        }
    }
}

impl OnPaint for Game2048Logic {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("' ',black,black"));
        surface.write_string(1, 1, &format!("Score: {}", self.score), charattr!("white"), false);

        const CELL_WIDTH: i32 = 16;
        const CELL_HEIGHT: i32 = 6;
        let start_x = 2;
        let start_y = 2;
        let grid_width = CELL_WIDTH * 4 + 1;

        for i in 0..5 {
            let y = start_y + i * CELL_HEIGHT;
            for x in start_x..start_x + grid_width {
                if i == 0 {
                    if x == start_x {
                        surface.write_char(x, y, Character::new('┌', Color::Gray, Color::Black, CharFlags::None));
                    } else if x == start_x + grid_width - 1 {
                        surface.write_char(x, y, Character::new('┐', Color::Gray, Color::Black, CharFlags::None));
                    } else if (x - start_x) % CELL_WIDTH == 0 {
                        surface.write_char(x, y, Character::new('┬', Color::Gray, Color::Black, CharFlags::None));
                    } else {
                        surface.write_char(x, y, Character::new('─', Color::Gray, Color::Black, CharFlags::None));
                    }
                } else if i == 4 {
                    if x == start_x {
                        surface.write_char(x, y, Character::new('└', Color::Gray, Color::Black, CharFlags::None));
                    } else if x == start_x + grid_width - 1 {
                        surface.write_char(x, y, Character::new('┘', Color::Gray, Color::Black, CharFlags::None));
                    } else if (x - start_x) % CELL_WIDTH == 0 {
                        surface.write_char(x, y, Character::new('┴', Color::Gray, Color::Black, CharFlags::None));
                    } else {
                        surface.write_char(x, y, Character::new('─', Color::Gray, Color::Black, CharFlags::None));
                    }
                } else {
                    if x == start_x {
                        surface.write_char(x, y, Character::new('├', Color::Gray, Color::Black, CharFlags::None));
                    } else if x == start_x + grid_width - 1 {
                        surface.write_char(x, y, Character::new('┤', Color::Gray, Color::Black, CharFlags::None));
                    } else if (x - start_x) % CELL_WIDTH == 0 {
                        surface.write_char(x, y, Character::new('┼', Color::Gray, Color::Black, CharFlags::None));
                    } else {
                        surface.write_char(x, y, Character::new('─', Color::Gray, Color::Black, CharFlags::None));
                    }
                }
            }
        }

        for j in 0..5 {
            let x = start_x + j * CELL_WIDTH;
            for i in 1..5 {
                let y = start_y + i * CELL_HEIGHT;
                for dy in 1..CELL_HEIGHT {
                    surface.write_char(x, y - dy, Character::new('│', Color::Gray, Color::Black, CharFlags::None));
                }
            }
        }

        for y in 0..4 {
            for x in 0..4 {
                let px = start_x + 1 + x * CELL_WIDTH;
                let py = start_y + 1 + y * CELL_HEIGHT;

                let tile = self.grid[y as usize][x as usize];
                let (fg_color,bg_color) = match tile.value {
                    0 => (Color::Black,Color::Black),
                    2 => (Color::White,Color::DarkBlue),
                    4 => (Color::White,Color::DarkGreen),
                    8 => (Color::White,Color::DarkRed),
                    16 => (Color::White,Color::Magenta),
                    32 => (Color::White,Color::Olive),
                    64 => (Color::White,Color::Teal),
                    128 => (Color::White,Color::Blue),
                    256 => (Color::Black,Color::Green),
                    512 => (Color::White,Color::Red),
                    1024 => (Color::Black,Color::Pink),
                    2048 => (Color::Black,Color::Yellow),
                    _ => (Color::Black,Color::White),
                };

                surface.fill_rect(
                    Rect::with_size(px, py, 15, 5),
                    Character::new(' ', Color::White, bg_color, CharFlags::None),
                );

                if tile.value > 0 {
                    let tile_draw = BitTileU128::from_u128(
                        15,
                        5,
                        match tile.value {
                            0 => 0,
                            2 => 0x1C00080070008001C0,
                            4 => 0x80010007000200040,
                            8 => 0x1C0028007000A001C0,
                            16 => 0x7700A401C800980720,
                            32 => 0x77002801DC02200770,
                            64 => 0x27004A01DC00880170,
                            128 => 0x1DDC28907720A861DC8,
                            256 => 0x1DDC2A0877702281DDC,
                            512 => 0x1DDC092072708621C9C,
                            1024 => 0x277742A5DD48A299772,
                            2048 => 0x7277A4A3DD5E8AA7177,
                            _ => 0,
                        },
                    )
                    .unwrap();
                    surface.draw_tile(px, py + 1, &tile_draw, fg_color, bg_color, BitTileRenderMethod::SmallBlocks);
                }
            }
        }

        match self.game_state {
            GameState::Won => {
                surface.write_string(1, 30, "Congratulations! You reached 2048!", charattr!("green"), false);
                surface.write_string(1, 31, "Press Enter or Space to play again", charattr!("yellow"), false);
            }
            GameState::GameOver => {
                surface.write_string(1, 30, "Game Over! No more moves available.", charattr!("red"), false);
                surface.write_string(1, 31, "Press Enter or Space to play again", charattr!("yellow"), false);
            }
            GameState::Playing => {}
        }
    }
}
