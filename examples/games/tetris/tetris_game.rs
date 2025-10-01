use appcui::prelude::*;
use rand::Rng;
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum SquareState {
    Empty,
    Filled(Color),
}

impl SquareState {
    fn is_empty(&self) -> bool {
        matches!(self, SquareState::Empty)
    }
}

#[derive(Copy, Clone)]
struct Tetromino {
    shape: [[bool; 4]; 4],
    color: Color,
}

impl Tetromino {
    fn new(tetromino_type: TetrominoType) -> Self {
        let (shape, color) = match tetromino_type {
            TetrominoType::I => (
                [
                    [false, false, false, false],
                    [true, true, true, true],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
                Color::Aqua,
            ),
            TetrominoType::O => (
                [
                    [false, false, false, false],
                    [false, true, true, false],
                    [false, true, true, false],
                    [false, false, false, false],
                ],
                Color::Yellow,
            ),
            TetrominoType::T => (
                [
                    [false, false, false, false],
                    [false, true, false, false],
                    [true, true, true, false],
                    [false, false, false, false],
                ],
                Color::Magenta,
            ),
            TetrominoType::S => (
                [
                    [false, false, false, false],
                    [false, true, true, false],
                    [true, true, false, false],
                    [false, false, false, false],
                ],
                Color::Green,
            ),
            TetrominoType::Z => (
                [
                    [false, false, false, false],
                    [true, true, false, false],
                    [false, true, true, false],
                    [false, false, false, false],
                ],
                Color::Red,
            ),
            TetrominoType::J => (
                [
                    [false, false, false, false],
                    [true, false, false, false],
                    [true, true, true, false],
                    [false, false, false, false],
                ],
                Color::Blue,
            ),
            TetrominoType::L => (
                [
                    [false, false, false, false],
                    [false, false, true, false],
                    [true, true, true, false],
                    [false, false, false, false],
                ],
                Color::Olive,
            ),
        };
        Self { shape, color }
    }

    fn rotate(&self) -> Self {
        let mut rotated = [[false; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                rotated[j][3 - i] = self.shape[i][j];
            }
        }
        Self {
            shape: rotated,
            color: self.color,
        }
    }
}

const BOARD_WIDTH: usize = 12;
const BOARD_HEIGHT: usize = 10;

#[CustomControl(overwrite = OnPaint+OnKeyPressed, events = TimerEvents)]
pub struct TetrisGame {
    board: [[SquareState; BOARD_WIDTH]; BOARD_HEIGHT],
    current_piece: Option<Tetromino>,
    current_x: i32,
    current_y: i32,
    next_piece: Tetromino,
    state: GameState,
    score: u32,
    lines_cleared: u32,
    level: u32,
    drop_timer: u64,
    drop_counter: u64,
}

impl TetrisGame {
    pub fn new() -> Self {
        let mut game = Self {
            base: ControlBase::new(layout!("d:f"), true),
            board: [[SquareState::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
            current_piece: None,
            current_x: 0,
            current_y: 0,
            next_piece: Tetromino::new(TetrominoType::I),
            state: GameState::Playing,
            score: 0,
            lines_cleared: 0,
            level: 1,
            drop_timer: 1000,
            drop_counter: 0,
        };

        if let Some(timer) = game.timer() {
            timer.start(Duration::from_millis(50));
        }

        game.spawn_new_piece();
        game
    }

    fn spawn_new_piece(&mut self) {
        let tetromino_types = [
            TetrominoType::I,
            TetrominoType::O,
            TetrominoType::T,
            TetrominoType::S,
            TetrominoType::Z,
            TetrominoType::J,
            TetrominoType::L,
        ];
        let mut rng = rand::thread_rng();
        let random_type = tetromino_types[rng.gen_range(0..tetromino_types.len())];

        self.current_piece = Some(self.next_piece);
        self.next_piece = Tetromino::new(random_type);
        self.current_x = (BOARD_WIDTH / 2) as i32 - 2;
        self.current_y = 0;

        if self.check_collision(self.current_piece.unwrap(), self.current_x, self.current_y) {
            self.state = GameState::GameOver;
        }
    }

    fn check_collision(&self, piece: Tetromino, x: i32, y: i32) -> bool {
        for py in 0..4 {
            for px in 0..4 {
                if piece.shape[py][px] {
                    let board_x = x + px as i32;
                    let board_y = y + py as i32;

                    if board_x < 0 || board_x >= BOARD_WIDTH as i32 || board_y >= BOARD_HEIGHT as i32 {
                        return true;
                    }

                    if board_y >= 0 && !self.board[board_y as usize][board_x as usize].is_empty() {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn place_piece(&mut self) {
        if let Some(piece) = self.current_piece {
            for py in 0..4 {
                for px in 0..4 {
                    if piece.shape[py][px] {
                        let board_x = (self.current_x + px as i32) as usize;
                        let board_y = (self.current_y + py as i32) as usize;
                        if board_y < BOARD_HEIGHT && board_x < BOARD_WIDTH {
                            self.board[board_y][board_x] = SquareState::Filled(piece.color);
                        }
                    }
                }
            }
        }
        self.clear_lines();
        self.spawn_new_piece();
    }

    fn clear_lines(&mut self) {
        let mut lines_to_clear = Vec::new();

        for y in 0..BOARD_HEIGHT {
            let mut is_full = true;
            for x in 0..BOARD_WIDTH {
                if self.board[y][x].is_empty() {
                    is_full = false;
                    break;
                }
            }
            if is_full {
                lines_to_clear.push(y);
            }
        }

        for &line_y in lines_to_clear.iter().rev() {
            for y in (1..=line_y).rev() {
                for x in 0..BOARD_WIDTH {
                    self.board[y][x] = self.board[y - 1][x];
                }
            }
            for x in 0..BOARD_WIDTH {
                self.board[0][x] = SquareState::Empty;
            }
        }

        let cleared_count = lines_to_clear.len() as u32;
        if cleared_count > 0 {
            self.lines_cleared += cleared_count;
            self.score += match cleared_count {
                1 => 100,
                2 => 300,
                3 => 500,
                4 => 800,
                _ => 0,
            } * (self.level + 1);

            self.level = (self.lines_cleared / 10) + 1;
            self.drop_timer = (1000.0 / (1.0 + self.level as f64 * 0.1)) as u64;
        }
    }

    fn move_piece(&mut self, dx: i32, dy: i32) -> bool {
        if let Some(piece) = self.current_piece {
            let new_x = self.current_x + dx;
            let new_y = self.current_y + dy;

            if !self.check_collision(piece, new_x, new_y) {
                self.current_x = new_x;
                self.current_y = new_y;
                return true;
            }
        }
        false
    }

    fn rotate_piece(&mut self) -> bool {
        if let Some(piece) = self.current_piece {
            let rotated = piece.rotate();
            if !self.check_collision(rotated, self.current_x, self.current_y) {
                self.current_piece = Some(rotated);
                return true;
            }
        }
        false
    }

    fn drop_piece(&mut self) {
        if !self.move_piece(0, 1) {
            self.place_piece();
        }
    }

    fn hard_drop(&mut self) {
        while self.move_piece(0, 1) {}
        self.place_piece();
    }

    fn draw_square(&self, surface: &mut Surface, x: i32, y: i32, color: Color) {
        let screen_x = x * 5;
        let screen_y = y * 3;

        let rect = Rect::with_size(screen_x, screen_y, 5, 3);
        surface.draw_rect(rect, LineType::Single, charattr!("gray,black"));

        let fill_char = Character::with_attributes(' ', CharAttribute::with_color(Color::Black, color));
        surface.fill_horizontal_line(screen_x + 1, screen_y + 1, screen_x + 3, fill_char);
    }

    fn paint_board(&self, surface: &mut Surface) {
        let game_width = BOARD_WIDTH * 5;
        let game_height = BOARD_HEIGHT * 3;

        surface.draw_vertical_line(game_width as i32 + 1, 0, game_height as i32, LineType::Single, charattr!("gray,black"));

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if let SquareState::Filled(color) = self.board[y][x] {
                    self.draw_square(surface, x as i32, y as i32, color);
                }
            }
        }

        if let Some(piece) = self.current_piece {
            for py in 0..4 {
                for px in 0..4 {
                    if piece.shape[py][px] {
                        let board_x = self.current_x + px as i32;
                        let board_y = self.current_y + py as i32;
                        if board_y >= 0 && board_y < BOARD_HEIGHT as i32 && board_x >= 0 && board_x < BOARD_WIDTH as i32 {
                            self.draw_square(surface, board_x, board_y, piece.color);
                        }
                    }
                }
            }
        }
    }

    fn paint_game_over(&self, surface: &mut Surface) {
        const X: i32 = 17;
        const Y: i32 = 8;
        const W: i32 = 40;
        let r = Rect::with_size(X, Y, W as u16, 6);
        surface.fill_rect(r, Character::new(' ', Color::White, Color::DarkRed, CharFlags::None));
        surface.write_string(X + W / 2 - 5, Y, "Game Over!", charattr!("white"), false);
        surface.draw_horizontal_line_with_size(X + 1, Y + 1, (W - 2) as u32, LineType::Single, charattr!("gray"));
        surface.write_string(X + 1, Y + 2, "Final Score", charattr!("silver"), false);
        surface.write_string(X + W - 8, Y + 2, format!("{:6}", self.score).as_str(), charattr!("yellow"), false);
        surface.write_string(X + 1, Y + 3, "Lines Cleared", charattr!("silver"), false);
        surface.write_string(X + W - 8, Y + 3, format!("{:6}", self.lines_cleared).as_str(), charattr!("yellow"), false);
        surface.draw_horizontal_line_with_size(X + 1, Y + 4, (W - 2) as u32, LineType::Single, charattr!("gray"));
        surface.write_string(X + 6, Y + 5, "Press SPACE to restart!", charattr!("white"), false);
    }
}

impl OnPaint for TetrisGame {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(char!("' ',black,black"));

        match self.state {
            GameState::GameOver => {
                self.paint_game_over(surface);
            }
            GameState::Playing | GameState::Paused => {
                self.paint_board(surface);

                let game_width = (BOARD_WIDTH * 5) as i32;
                surface.write_string(game_width + 2, 0, format!("Score: {}", self.score).as_str(), theme.symbol.checked, false);
                surface.write_string(game_width + 2, 1, format!("Level: {}", self.level).as_str(), theme.symbol.checked, false);
                surface.write_string(
                    game_width + 2,
                    2,
                    format!("Lines: {}", self.lines_cleared).as_str(),
                    theme.symbol.checked,
                    false,
                );

                surface.write_string(game_width + 2, 4, "Next:", charattr!("white"), false);

                let mut bit_tile = BitTileU16::new(4, 4).unwrap();
                for py in 0..4 {
                    for px in 0..4 {
                        if self.next_piece.shape[py][px] {
                            bit_tile.set(px as u32, py as u32, true);
                        }
                    }
                }

                let preview_x = game_width + 2;
                let preview_y = 6;
                surface.draw_tile(
                    preview_x,
                    preview_y,
                    &bit_tile,
                    self.next_piece.color,
                    Color::Black,
                    BitTileRenderMethod::SmallBlocks,
                );

                if self.state == GameState::Paused {
                    surface.write_string(game_width + 2, 15, "PAUSED - Press P to resume", charattr!("white,black"), false);
                }
            }
        }
    }
}

impl TimerEvents for TetrisGame {
    fn on_update(&mut self, _: u64) -> EventProcessStatus {
        if self.state == GameState::Playing {
            self.drop_counter += 50;
            if self.drop_counter >= self.drop_timer {
                self.drop_counter = 0;
                self.drop_piece();
            }
        }
        EventProcessStatus::Processed
    }
}

impl OnKeyPressed for TetrisGame {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") => {
                if self.state == GameState::Playing {
                    self.move_piece(-1, 0);
                }
                EventProcessStatus::Processed
            }
            key!("Right") => {
                if self.state == GameState::Playing {
                    self.move_piece(1, 0);
                }
                EventProcessStatus::Processed
            }
            key!("Down") => {
                if self.state == GameState::Playing {
                    self.drop_piece();
                }
                EventProcessStatus::Processed
            }
            key!("Up") => {
                if self.state == GameState::Playing {
                    self.rotate_piece();
                }
                EventProcessStatus::Processed
            }
            key!("Space") => {
                match self.state {
                    GameState::GameOver => {
                        *self = Self::new();
                    }
                    GameState::Playing => {
                        self.hard_drop();
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
