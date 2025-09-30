use appcui::prelude::*;
use rand::Rng;
use std::time::Duration;

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
    Wall(char),
    Food,
    Cherry,
    Empty,
}

impl CellType {
    #[inline(always)]
    fn is_wall(&self) -> bool {
        matches!(self, CellType::Wall(_))
    }
}

struct Ghost {
    position: Point,
    direction: Direction,
}

impl Ghost {
    fn new(x: i32, y: i32) -> Self {
        Self {
            position: Point::new(x, y),
            direction: Direction::Up,
        }
    }
}

const BOARD_WIDTH: usize = 55;
const BOARD_HEIGHT: usize = 22;
static MAZE_PATTERN: &[&str; BOARD_HEIGHT] = &[
    "┌────────────┬┬────────────┐",
    "│..c.........││............│",
    "│.┌──┐.┌───┐.└┘.┌───┐.┌──┐.│",
    "│.│  │.│   │....│   │.│  │.│",
    "│.└──┘.└───┘.┌┐.└───┘.└──┘.│",
    "│............││.c..........│",
    "│.┌──┐.┌─────┘└─────┐.┌──┐.│",
    "│.└──┘.└────────────┘.└──┘.│",
    "│..........................│",
    "├──┐.┌───┐.┌  G ┐.┌───┐.┌──┤",
    "├──┘.└───┘.│G  G│.└───┘.└──┤",
    "│..........└────┘..........│",
    "│.┌──┐.┌─┐    P   ┌─┐.┌──┐.│",
    "│.└──┘.└─┘.┌────┐.└─┘.└──┘.│",
    "│..c.......│    │..........│",
    "│.┌──┐.┌───┘    └───┐.┌──┐.│",
    "│.└──┘.└────────────┘.└──┘.│",
    "│.......................c..│",
    "├──┐.┌───┐.┌────┐.┌───┐.┌──┤",
    "├──┘.└───┘.└────┘.└───┘.└──┤",
    "│..........................│",
    "└──────────────────────────┘",
];

#[CustomControl(overwrite = OnPaint+OnKeyPressed, events = TimerEvents)]
pub struct PacmanGame {
    board: [[CellType; BOARD_WIDTH]; BOARD_HEIGHT],
    pacman_pos: Point,
    pacman_direction: Direction,
    ghosts: Vec<Ghost>,
    state: GameState,
    score: u32,
    high_score: u32,
    food_count: u32,
    total_food: u32,
}

impl PacmanGame {
    pub fn new() -> Self {
        let mut game = Self {
            base: ControlBase::new(layout!("d:f"), true),
            board: [[CellType::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
            pacman_pos: Point::ORIGIN,
            pacman_direction: Direction::Right,
            ghosts: Vec::new(),
            state: GameState::Playing,
            score: 0,
            high_score: 0,
            food_count: 0,
            total_food: 0,
        };

        if let Some(timer) = game.timer() {
            timer.start(Duration::from_millis(250));
        }

        game.create_board();

        game
    }

    fn create_board(&mut self) {
        self.food_count = 0;
        self.ghosts.clear();
        for (y, row) in MAZE_PATTERN.iter().enumerate() {
            if y >= BOARD_HEIGHT {
                break;
            }
            for (x, ch) in row.chars().enumerate() {
                if x >= BOARD_WIDTH {
                    break;
                }
                let cell_type = match ch {
                    '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴' | '┼' | '─' | '│' => CellType::Wall(ch),
                    '.' => CellType::Food,
                    'c' => CellType::Cherry,
                    'G' => {
                        self.ghosts.push(Ghost::new(x as i32, y as i32));
                        CellType::Empty
                    }
                    'P' => {
                        self.pacman_pos = Point::new(x as i32, y as i32);
                        CellType::Empty
                    }
                    _ => CellType::Empty,
                };
                self.board[y][x] = cell_type;
                if cell_type == CellType::Food {
                    self.food_count += 1;
                }
            }
        }
        self.total_food = self.food_count;
    }

    pub fn start_game(&mut self) {
        self.state = GameState::Playing;
        self.score = 0;

        if self.board[self.pacman_pos.y as usize][self.pacman_pos.x as usize].is_wall() {
            for y in 1..BOARD_HEIGHT - 1 {
                for x in 1..BOARD_WIDTH - 1 {
                    if self.board[y][x].is_wall() == false {
                        self.pacman_pos = Point::new(x as i32, y as i32);
                        return;
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

        let new_x = self.pacman_pos.x + dx;
        let new_y = self.pacman_pos.y + dy;

        if new_x >= 0
            && new_y >= 0
            && new_x < BOARD_WIDTH as i32
            && new_y < BOARD_HEIGHT as i32
            && self.board[new_y as usize][new_x as usize].is_wall() == false
        {
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

            self.pacman_pos = Point::new(new_x, new_y);

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

                let new_x = ghost.position.x + dx;
                let new_y = ghost.position.y + dy;

                if new_x >= 0
                    && new_y >= 0
                    && new_x < BOARD_WIDTH as i32
                    && new_y < BOARD_HEIGHT as i32
                    && self.board[new_y as usize][new_x as usize].is_wall() == false
                {
                    possible_moves.push((new_x, new_y, dir));
                }
            }

            if !possible_moves.is_empty() {
                let target_move = if rng.gen_bool(0.7) {
                    let mut best_move = possible_moves[0];
                    let mut best_distance = (best_move.0 - self.pacman_pos.x).abs() + (best_move.1 - self.pacman_pos.y).abs();

                    for &(x, y, dir) in &possible_moves {
                        let distance = (x - self.pacman_pos.x).abs() + (y - self.pacman_pos.y).abs();
                        if distance < best_distance {
                            best_distance = distance;
                            best_move = (x, y, dir);
                        }
                    }
                    best_move
                } else {
                    possible_moves[rng.gen_range(0..possible_moves.len())]
                };

                ghost.position = Point::new(target_move.0, target_move.1);
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

    fn can_move_in_direction(&self, direction: Direction) -> bool {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_x = self.pacman_pos.x + dx;
        let new_y = self.pacman_pos.y + dy;

        if new_x < 0 || new_y < 0 || new_x >= BOARD_WIDTH as i32 || new_y >= BOARD_HEIGHT as i32 {
            return false;
        }

        !self.board[new_y as usize][new_x as usize].is_wall()
    }
    fn paint_board(&self, surface: &mut Surface) {
        let r = Rect::with_size(0, 2, (BOARD_WIDTH * 2) as u16, BOARD_HEIGHT as u16);
        surface.fill_rect(r, char!("' ',black,black"));
        let cherry = Character::with_attributes('🍒', CharAttribute::with_color(Color::Red, Color::Black));
        let food = Character::with_attributes('·', CharAttribute::with_color(Color::Gray, Color::Black));
        let line = Character::with_attributes(SpecialChar::BoxHorizontalSingleLine, charattr!("blue,black"));
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let screen_x = x as i32 * 2;
                let screen_y = y as i32 + 2;

                match self.board[y][x] {
                    CellType::Wall(ch) => {
                        let c = Character::with_attributes(ch, CharAttribute::with_color(Color::Blue, Color::Black));
                        surface.write_char(screen_x, screen_y, c);
                        if ch == '─' || ch == '┌' || ch == '└' || ch == '├' || ch == '┬' {
                            surface.write_char(screen_x + 1, screen_y, line);
                        }
                    }
                    CellType::Food => surface.write_char(screen_x, screen_y, food),
                    CellType::Cherry => surface.write_char(screen_x, screen_y, cherry),
                    CellType::Empty => {} //Character::with_attributes(' ', charattr!("black,black")),
                };
            }
        }
        surface.write_char(
            self.pacman_pos.x * 2,
            self.pacman_pos.y + 2,
            Character::with_attributes('⚫', CharAttribute::with_color(Color::Yellow, Color::Black)),
        );

        for ghost in &self.ghosts {
            surface.write_char(
                ghost.position.x * 2,
                ghost.position.y + 2,
                Character::with_attributes('👻', CharAttribute::with_color(Color::Aqua, Color::Black)),
            );
        }
    }
}

impl OnPaint for PacmanGame {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(char!("' ',black,black"));

        match self.state {
            GameState::GameOver => {
                let size = self.size();
                let title = "GAME OVER";
                let score = format!("Final Score: {}", self.score);
                let high_score = format!("High Score: {}", self.high_score);
                let restart_msg = "Press SPACE to play again";

                surface.write_string(
                    (size.width as i32 - title.len() as i32) / 2,
                    size.height as i32 / 2 - 2,
                    title,
                    theme.symbol.checked,
                    false,
                );
                surface.write_string(
                    (size.width as i32 - score.len() as i32) / 2,
                    size.height as i32 / 2,
                    score.as_str(),
                    theme.symbol.checked,
                    false,
                );
                surface.write_string(
                    (size.width as i32 - high_score.len() as i32) / 2,
                    size.height as i32 / 2 + 1,
                    high_score.as_str(),
                    theme.symbol.checked,
                    false,
                );
                surface.write_string(
                    (size.width as i32 - restart_msg.len() as i32) / 2,
                    size.height as i32 / 2 + 3,
                    restart_msg,
                    theme.symbol.checked,
                    false,
                );
            }
            GameState::Victory => {
                let size = self.size();
                let title = "VICTORY!";
                let score = format!("Final Score: {}", self.score);
                let high_score = format!("High Score: {}", self.high_score);
                let restart_msg = "Press SPACE to play again";

                surface.write_string(
                    (size.width as i32 - title.len() as i32) / 2,
                    size.height as i32 / 2 - 2,
                    title,
                    theme.symbol.checked,
                    false,
                );
                surface.write_string(
                    (size.width as i32 - score.len() as i32) / 2,
                    size.height as i32 / 2,
                    score.as_str(),
                    theme.symbol.checked,
                    false,
                );
                surface.write_string(
                    (size.width as i32 - high_score.len() as i32) / 2,
                    size.height as i32 / 2 + 1,
                    high_score.as_str(),
                    theme.symbol.checked,
                    false,
                );
                surface.write_string(
                    (size.width as i32 - restart_msg.len() as i32) / 2,
                    size.height as i32 / 2 + 3,
                    restart_msg,
                    theme.symbol.checked,
                    false,
                );
            }
            GameState::Playing | GameState::Paused => {
                surface.write_string(0, 0, format!("Score: {}", self.score).as_str(), theme.symbol.checked, false);
                surface.write_string(15, 0, format!("High Score: {}", self.high_score).as_str(), theme.symbol.checked, false);
                surface.write_string(35, 0, format!("Food Left: {}", self.food_count).as_str(), theme.symbol.checked, false);

                self.paint_board(surface);

                if self.state == GameState::Paused {
                    surface.clear(Character::with_color(Color::Gray, Color::Black));
                    surface.write_string(15, 11, "PAUSED - Press P to resume", charattr!("white,black"), false);
                }
            }
        }
    }
}

impl TimerEvents for PacmanGame {
    fn on_update(&mut self, _: u64) -> EventProcessStatus {
        if self.state == GameState::Playing {
            self.move_pacman();
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
