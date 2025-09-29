use appcui::prelude::*;
use std::time::Duration;
use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
    Victory,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum CellType {
    Wall,
    Food,
    Cherry,
    Empty,
}

struct Ghost {
    position: (i32, i32),
    direction: Direction,
    last_move_time: u64,
}

impl Ghost {
    fn new(x: i32, y: i32) -> Self {
        Self {
            position: (x, y),
            direction: Direction::Up,
            last_move_time: 0,
        }
    }

    fn get_char(&self) -> char {
        '👻'
    }
}

#[CustomControl(overwrite = OnPaint+OnKeyPressed, events = TimerEvents)]
pub struct PacmanGame {
    board: Vec<Vec<CellType>>,
    board_width: usize,
    board_height: usize,
    pacman_pos: (i32, i32),
    pacman_direction: Direction,
    ghosts: Vec<Ghost>,
    state: GameState,
    score: u32,
    high_score: u32,
    food_count: u32,
    total_food: u32,
    game_ticks: u64,
    pacman_move_delay: u64,
    ghost_move_delay: u64,
    last_pacman_move: u64,
    initialized: bool,
}

impl PacmanGame {
    pub fn new() -> Self {
        let mut game = Self {
            base: ControlBase::new(layout!("d:f"), true),
            board: Vec::new(),
            board_width: 0,
            board_height: 0,
            pacman_pos: (1, 1),
            pacman_direction: Direction::Right,
            ghosts: Vec::new(),
            state: GameState::Playing,
            score: 0,
            high_score: 0,
            food_count: 0,
            total_food: 0,
            game_ticks: 0,
            pacman_move_delay: 4,
            ghost_move_delay: 10,
            last_pacman_move: 0,
            initialized: false,
        };
        
        if let Some(timer) = game.timer() {
            timer.start(Duration::from_millis(50));
        }
        
        game.create_board();
        game.initialized = true;
        
        game
    }

    fn initialize_if_needed(&mut self) {
        if !self.initialized {
            let size = self.size();
            if size.width > 10 && size.height > 10 {
                self.create_board();
                self.initialized = true;
            }
        }
    }

    fn create_board(&mut self) {
        self.board_width = 58;
        self.board_height = 21;
        
        self.board = vec![vec![CellType::Empty; self.board_width]; self.board_height];
        
        let maze_pattern = [
            "┌────────────────────────────────────────────────────────┐",
            "│. . . . . . . . . .  . . . .┌────┐. . . . . . . . . . . │",
            "│. ┌──────┐. ┌──────┐        │    │  ┌──────┐. ┌──────┐. │",
            "│o │      │. │      │        └────┘  │      │. │      │o │",
            "│. └──────┘. └──────┘                └──────┘. └──────┘. │",
            "│. . . . . . . . . . . . . . . . . . . . . . . . . . . . │",
            "│. ┌───────┐. ┌──────────      ──────────┐. ┌─────────┐. │",
            "│. └───────┘. └─────┐               ┌────┘. └─────────┘. │",
            "│. . . . . . . . . .                 . . . . . . . . . . │",
            "└───────────┐. ┌────┘   ┌────┐      └────┐. ┌────────────┘",
            "            │. │        │    │           │. │             ",
            "────────────┘. │        │    │           │. └─────────────",
            "             . │        └────┘           │.               ",
            "────────────┐. │                         │. ┌─────────────",
            "            │. │                         │. │             ",
            "┌───────────┘. └─────────────────────────┘. └────────────┐",
            "│. . . . . . . . . . . . . . . . . . . . . . . . . . . . │",
            "│. ┌──────┐. ┌──────┐  ┌──────────┐  ┌──────┐. ┌──────┐. │",
            "│o └──────┘. └──────┘  └──────────┘  └──────┘. └──────┘o │",
            "│. . . . . . . . . . . . . . . . . . . . . . . . . . . . │",
            "└────────────────────────────────────────────────────────┘"
        ];

        
        for (y, row) in maze_pattern.iter().enumerate() {
            if y >= self.board_height { break; }
            for (x, ch) in row.chars().enumerate() {
                if x >= self.board_width { break; }
                match ch {
                    '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴' | '┼' | '─' | '│' => {
                        self.board[y][x] = CellType::Wall;
                    },
                    '.' => self.board[y][x] = CellType::Food,
                    'o' => self.board[y][x] = CellType::Cherry,
                    ' ' => self.board[y][x] = CellType::Empty,
                    _ => self.board[y][x] = CellType::Empty,
                }
            }
        }
        
        self.food_count = 0;
        for y in 0..self.board_height {
            for x in 0..self.board_width {
                if self.board[y][x] == CellType::Food {
                    self.food_count += 1;
                }
            }
        }
        self.total_food = self.food_count;
        
        self.pacman_pos = (29, 16);
        if self.pacman_pos.1 < self.board_height as i32 && self.pacman_pos.0 < self.board_width as i32 {
            self.board[self.pacman_pos.1 as usize][self.pacman_pos.0 as usize] = CellType::Empty;
        }
        
        self.ghosts.clear();
        let ghost_positions = vec![
            (27, 12),
            (29, 12),
            (31, 12),
        ];
        
        for &(gx, gy) in &ghost_positions {
            if gx >= 0 && gy >= 0 && gx < self.board_width as i32 && gy < self.board_height as i32 {
                self.ghosts.push(Ghost::new(gx, gy));
            }
        }
    }

    pub fn start_game(&mut self) {
        self.initialize_if_needed();
        if self.initialized {
            self.state = GameState::Playing;
            self.score = 0;
            self.game_ticks = 0;
            self.last_pacman_move = 0;
            
            for ghost in &mut self.ghosts {
                ghost.last_move_time = 0;
            }
            
            if self.board[self.pacman_pos.1 as usize][self.pacman_pos.0 as usize] == CellType::Wall {
                for y in 1..self.board_height-1 {
                    for x in 1..self.board_width-1 {
                        if self.board[y][x] != CellType::Wall {
                            self.pacman_pos = (x as i32, y as i32);
                            return;
                        }
                    }
                }
            }
        }
    }

    fn move_pacman(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        let (dx, dy) = match self.pacman_direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_x = self.pacman_pos.0 + dx;
        let new_y = self.pacman_pos.1 + dy;

        if new_x >= 0 && new_y >= 0 && 
           new_x < self.board_width as i32 && 
           new_y < self.board_height as i32 &&
           self.board[new_y as usize][new_x as usize] != CellType::Wall {
            
            match self.board[new_y as usize][new_x as usize] {
                CellType::Food => {
                    self.score += 10;
                    self.food_count -= 1;
                    self.board[new_y as usize][new_x as usize] = CellType::Empty;
                    
                    if self.food_count == 0 {
                        self.state = GameState::Victory;
                        if self.score > self.high_score {
                            self.high_score = self.score;
                        }
                    }
                }
                CellType::Cherry => {
                    self.score += 50;
                    self.board[new_y as usize][new_x as usize] = CellType::Empty;
                }
                _ => {}
            }

            self.pacman_pos = (new_x, new_y);
            
            for ghost in &self.ghosts {
                if ghost.position == self.pacman_pos {
                    self.state = GameState::GameOver;
                    if self.score > self.high_score {
                        self.high_score = self.score;
                    }
                    return;
                }
            }
        }
    }

    fn move_ghosts(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        for ghost in &mut self.ghosts {
            if self.game_ticks - ghost.last_move_time < self.ghost_move_delay {
                continue;
            }
            
            ghost.last_move_time = self.game_ticks;
            
            let mut rng = rand::thread_rng();
            let mut possible_moves = Vec::new();
            
            let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
            
            for &dir in &directions {
                let (dx, dy) = match dir {
                    Direction::Up => (0, -1),
                    Direction::Down => (0, 1),
                    Direction::Left => (-1, 0),
                    Direction::Right => (1, 0),
                };
                
                let new_x = ghost.position.0 + dx;
                let new_y = ghost.position.1 + dy;
                
                if new_x >= 0 && new_y >= 0 && 
                   new_x < self.board_width as i32 && 
                   new_y < self.board_height as i32 &&
                   self.board[new_y as usize][new_x as usize] != CellType::Wall {
                    possible_moves.push((new_x, new_y, dir));
                }
            }
            
            if !possible_moves.is_empty() {
                let target_move = if rng.gen_bool(0.7) {
                    let mut best_move = possible_moves[0];
                    let mut best_distance = (best_move.0 - self.pacman_pos.0).abs() + (best_move.1 - self.pacman_pos.1).abs();
                    
                    for &(x, y, dir) in &possible_moves {
                        let distance = (x - self.pacman_pos.0).abs() + (y - self.pacman_pos.1).abs();
                        if distance < best_distance {
                            best_distance = distance;
                            best_move = (x, y, dir);
                        }
                    }
                    best_move
                } else {
                    possible_moves[rng.gen_range(0..possible_moves.len())]
                };
                
                ghost.position = (target_move.0, target_move.1);
                ghost.direction = target_move.2;
                
                if ghost.position == self.pacman_pos {
                    self.state = GameState::GameOver;
                    if self.score > self.high_score {
                        self.high_score = self.score;
                    }
                    return;
                }
            }
        }
    }

    fn get_pacman_char(&self) -> char {
        match self.pacman_direction {
            Direction::Right => '>',
            Direction::Left => '<', 
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }

    fn can_move_in_direction(&self, direction: Direction) -> bool {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_x = self.pacman_pos.0 + dx;
        let new_y = self.pacman_pos.1 + dy;

        if new_x < 0 || new_y < 0 || 
           new_x >= self.board_width as i32 || 
           new_y >= self.board_height as i32 {
            return false;
        }

        self.board[new_y as usize][new_x as usize] != CellType::Wall
    }

    fn get_original_wall_char(&self, x: usize, y: usize) -> char {
        let maze_pattern = [
            "┌────────────────────────────────────────────────────────┐",
            "│. . . . . . . . . .  . . . .┌────┐. . . . . . . . . . . │",
            "│. ┌──────┐. ┌──────┐        │    │  ┌──────┐. ┌──────┐. │",
            "│o │      │. │      │        └────┘  │      │. │      │o │",
            "│. └──────┘. └──────┘                └──────┘. └──────┘. │",
            "│. . . . . . . . . . . . . . . . . . . . . . . . . . . . │",
            "│. ┌───────┐. ┌──────────      ──────────┐. ┌─────────┐. │",
            "│. └───────┘. └─────┐               ┌────┘. └─────────┘. │",
            "│. . . . . . . . . .                 . . . . . . . . . . │",
            "└───────────┐. ┌────┘   ┌────┐      └────┐. ┌────────────┘",
            "            │. │        │    │           │. │             ",
            "────────────┘. │        │    │           │. └─────────────",
            "             . │        └────┘           │.               ",
            "────────────┐. │                         │. ┌─────────────",
            "            │. │                         │. │             ",
            "┌───────────┘. └─────────────────────────┘. └────────────┐",
            "│. . . . . . . . . . . . . . . . . . . . . . . . . . . . │",
            "│. ┌──────┐. ┌──────┐  ┌──────────┐  ┌──────┐. ┌──────┐. │",
            "│o └──────┘. └──────┘  └──────────┘  └──────┘. └──────┘o │",
            "│. . . . . . . . . . . . . . . . . . . . . . . . . . . . │",
            "└────────────────────────────────────────────────────────┘"
        ];
        
        if y < maze_pattern.len() && x < maze_pattern[y].len() {
            if let Some(ch) = maze_pattern[y].chars().nth(x) {
                match ch {
                    '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴' | '┼' | '─' | '│' => ch,
                    _ => '█',
                }
            } else {
                '█'
            }
        } else {
            '█'
        }
    }

}

impl OnPaint for PacmanGame {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(char!("' ',black,black"));
        
        if !self.initialized {
            return;
        }
        
        match self.state {
            GameState::GameOver => {
                let size = self.size();
                let title = "GAME OVER";
                let score = format!("Final Score: {}", self.score);
                let high_score = format!("High Score: {}", self.high_score);
                let restart_msg = "Press SPACE to play again";
                
                surface.write_string((size.width as i32 - title.len() as i32) / 2, size.height as i32 / 2 - 2, title, theme.symbol.checked, false);
                surface.write_string((size.width as i32 - score.len() as i32) / 2, size.height as i32 / 2, score.as_str(), theme.symbol.checked, false);
                surface.write_string((size.width as i32 - high_score.len() as i32) / 2, size.height as i32 / 2 + 1, high_score.as_str(), theme.symbol.checked, false);
                surface.write_string((size.width as i32 - restart_msg.len() as i32) / 2, size.height as i32 / 2 + 3, restart_msg, theme.symbol.checked, false);
            }
            GameState::Victory => {
                let size = self.size();
                let title = "VICTORY!";
                let score = format!("Final Score: {}", self.score);
                let high_score = format!("High Score: {}", self.high_score);
                let restart_msg = "Press SPACE to play again";
                
                surface.write_string((size.width as i32 - title.len() as i32) / 2, size.height as i32 / 2 - 2, title, theme.symbol.checked, false);
                surface.write_string((size.width as i32 - score.len() as i32) / 2, size.height as i32 / 2, score.as_str(), theme.symbol.checked, false);
                surface.write_string((size.width as i32 - high_score.len() as i32) / 2, size.height as i32 / 2 + 1, high_score.as_str(), theme.symbol.checked, false);
                surface.write_string((size.width as i32 - restart_msg.len() as i32) / 2, size.height as i32 / 2 + 3, restart_msg, theme.symbol.checked, false);
            }
            GameState::Playing | GameState::Paused => {
                surface.write_string(0, 0, format!("Score: {}", self.score).as_str(), theme.symbol.checked, false);
                surface.write_string(15, 0, format!("High Score: {}", self.high_score).as_str(), theme.symbol.checked, false);
                surface.write_string(35, 0, format!("Food Left: {}", self.food_count).as_str(), theme.symbol.checked, false);
                surface.write_string(0, 1, format!("Pos: ({},{})", self.pacman_pos.0, self.pacman_pos.1).as_str(), theme.symbol.checked, false);
                
                for y in 0..self.board_height {
                    for x in 0..self.board_width {
                        let screen_x = x as i32;
                        let screen_y = y as i32 + 2;
                        
                        let ch = match self.board[y][x] {
                            CellType::Wall => Character::with_attributes(self.get_original_wall_char(x, y), CharAttribute::with_color(Color::Blue, Color::Black)),
                            CellType::Food => Character::with_attributes('·', CharAttribute::with_color(Color::Gray, Color::Black)),
                            CellType::Cherry => Character::with_attributes('🍒', CharAttribute::with_color(Color::Red, Color::Black)),
                            CellType::Empty => Character::with_attributes(' ', theme.symbol.checked),
                        };
                        surface.write_char(screen_x, screen_y, ch);
                    }
                }
                
                surface.write_char(self.pacman_pos.0, self.pacman_pos.1 + 2, 
                    Character::with_attributes(self.get_pacman_char(), CharAttribute::with_color(Color::Yellow, Color::Black)));
                
                for ghost in &self.ghosts {
                    surface.write_char(ghost.position.0, ghost.position.1 + 2, 
                        Character::with_attributes(ghost.get_char(), CharAttribute::with_color(Color::Magenta, Color::Black)));
                }
                
                if self.state == GameState::Paused {
                    let size = self.size();
                    surface.write_string(0, (size.height - 1) as i32, "PAUSED - Press P to resume", theme.symbol.checked, false);
                }
            }
        }
    }
}

impl TimerEvents for PacmanGame {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        self.initialize_if_needed();
        
        if self.state == GameState::Playing {
            self.game_ticks += 1;
            
            if self.game_ticks - self.last_pacman_move >= self.pacman_move_delay {
                self.move_pacman();
                self.last_pacman_move = self.game_ticks;
            }
            
            self.move_ghosts();
        }
        
        EventProcessStatus::Processed
    }
}

impl OnKeyPressed for PacmanGame {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Up") => {
                if self.can_move_in_direction(Direction::Up) {
                    self.pacman_direction = Direction::Up;
                }
                EventProcessStatus::Processed
            }
            key!("Down") => {
                if self.can_move_in_direction(Direction::Down) {
                    self.pacman_direction = Direction::Down;
                }
                EventProcessStatus::Processed
            }
            key!("Left") => {
                if self.can_move_in_direction(Direction::Left) {
                    self.pacman_direction = Direction::Left;
                }
                EventProcessStatus::Processed
            }
            key!("Right") => {
                if self.can_move_in_direction(Direction::Right) {
                    self.pacman_direction = Direction::Right;
                }
                EventProcessStatus::Processed
            }
            key!("Space") => {
                match self.state {
                    GameState::GameOver => {
                        self.create_board();
                        self.start_game();
                    }
                    GameState::Victory => {
                        self.create_board();
                        self.start_game();
                    }
                    _ => {}
                }
                EventProcessStatus::Processed
            }
            key!("P") => {
                if self.state == GameState::Playing {
                    self.state = GameState::Paused;
                } else if self.state == GameState::Paused {
                    self.state = GameState::Playing;
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}