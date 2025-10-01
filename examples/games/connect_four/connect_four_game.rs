use appcui::prelude::*;
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Playing,
    GameOver,
    Victory,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Player {
    Red,
    Yellow,
}

impl Player {
    fn color(&self) -> Color {
        match self {
            Player::Red => Color::DarkRed,
            Player::Yellow => Color::Yellow,
        }
    }

    fn next(&self) -> Player {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum CellState {
    Empty,
    Occupied(Player),
}

impl CellState {
    fn is_empty(&self) -> bool {
        matches!(self, CellState::Empty)
    }
}

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

fn create_circle_pattern() -> BitTileU64 {
    let pattern: u64 = 0b00000000_00011000_00111100_01111110_01111110_00111100_00011000_00000000;
    BitTileU64::from_u64(8, 8, pattern).unwrap()
}

#[CustomControl(overwrite = OnPaint+OnKeyPressed, events = TimerEvents)]
pub struct ConnectFourGame {
    board: [[CellState; BOARD_WIDTH]; BOARD_HEIGHT],
    current_player: Player,
    state: GameState,
    falling_piece: Option<(Player, i32, f32)>,
    winner: Option<Player>,
}

impl ConnectFourGame {
    pub fn new() -> Self {
        let mut game = Self {
            base: ControlBase::new(layout!("d:f"), true),
            board: [[CellState::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
            current_player: Player::Red,
            state: GameState::Playing,
            falling_piece: None,
            winner: None,
        };

        if let Some(timer) = game.timer() {
            timer.start(Duration::from_millis(50));
        }

        game
    }

    fn drop_piece(&mut self, column: usize) -> bool {
        if self.state != GameState::Playing || self.falling_piece.is_some() {
            return false;
        }

        if column >= BOARD_WIDTH {
            return false;
        }

        for row in (0..BOARD_HEIGHT).rev() {
            if self.board[row][column].is_empty() {
                self.falling_piece = Some((self.current_player, column as i32, -1.0));
                return true;
            }
        }
        false
    }

    fn update_falling_piece(&mut self) {
        if let Some((player, col, y)) = self.falling_piece {
            let new_y = y + 0.25;

            let next_row = (new_y as i32) + 1;
            let should_stop = if next_row >= BOARD_HEIGHT as i32 {
                true
            } else if next_row >= 0 && !self.board[next_row as usize][col as usize].is_empty() {
                true
            } else {
                false
            };

            if should_stop {
                let final_y = if next_row >= BOARD_HEIGHT as i32 {
                    BOARD_HEIGHT - 1
                } else {
                    (new_y as i32) as usize
                };

                self.board[final_y][col as usize] = CellState::Occupied(player);
                self.falling_piece = None;

                if self.check_win(final_y, col as usize, player) {
                    self.state = GameState::Victory;
                    self.winner = Some(player);
                } else if self.is_board_full() {
                    self.state = GameState::GameOver;
                } else {
                    self.current_player = self.current_player.next();
                }
            } else {
                self.falling_piece = Some((player, col, new_y));
            }
        }
    }

    fn check_win(&self, row: usize, col: usize, player: Player) -> bool {
        let mut count = 1;
        for x in (0..col).rev() {
            if matches!(self.board[row][x], CellState::Occupied(p) if p == player) {
                count += 1;
            } else {
                break;
            }
        }
        for x in (col + 1)..BOARD_WIDTH {
            if matches!(self.board[row][x], CellState::Occupied(p) if p == player) {
                count += 1;
            } else {
                break;
            }
        }
        if count >= 4 {
            return true;
        }

        count = 1;
        for y in (0..row).rev() {
            if matches!(self.board[y][col], CellState::Occupied(p) if p == player) {
                count += 1;
            } else {
                break;
            }
        }
        for y in (row + 1)..BOARD_HEIGHT {
            if matches!(self.board[y][col], CellState::Occupied(p) if p == player) {
                count += 1;
            } else {
                break;
            }
        }
        if count >= 4 {
            return true;
        }

        count = 1;
        let mut x = col as i32 - 1;
        let mut y = row as i32 - 1;
        while x >= 0 && y >= 0 {
            if matches!(self.board[y as usize][x as usize], CellState::Occupied(p) if p == player) {
                count += 1;
            } else {
                break;
            }
            x -= 1;
            y -= 1;
        }
        x = col as i32 + 1;
        y = row as i32 + 1;
        while x < BOARD_WIDTH as i32 && y < BOARD_HEIGHT as i32 {
            if matches!(self.board[y as usize][x as usize], CellState::Occupied(p) if p == player) {
                count += 1;
            } else {
                break;
            }
            x += 1;
            y += 1;
        }
        if count >= 4 {
            return true;
        }

        count = 1;
        x = col as i32 + 1;
        y = row as i32 - 1;
        while x < BOARD_WIDTH as i32 && y >= 0 {
            if matches!(self.board[y as usize][x as usize], CellState::Occupied(p) if p == player) {
                count += 1;
            } else {
                break;
            }
            x += 1;
            y -= 1;
        }
        x = col as i32 - 1;
        y = row as i32 + 1;
        while x >= 0 && y < BOARD_HEIGHT as i32 {
            if matches!(self.board[y as usize][x as usize], CellState::Occupied(p) if p == player) {
                count += 1;
            } else {
                break;
            }
            x -= 1;
            y += 1;
        }
        if count >= 4 {
            return true;
        }

        false
    }

    fn is_board_full(&self) -> bool {
        for col in 0..BOARD_WIDTH {
            if self.board[0][col].is_empty() {
                return false;
            }
        }
        true
    }

    fn reset_game(&mut self) {
        self.board = [[CellState::Empty; BOARD_WIDTH]; BOARD_HEIGHT];
        self.current_player = Player::Red;
        self.state = GameState::Playing;
        self.falling_piece = None;
        self.winner = None;
    }

    fn draw_circle(&self, surface: &mut Surface, x: i32, y: i32, color: Color) {
        let circle_pattern = create_circle_pattern();
        surface.draw_tile(x, y, &circle_pattern, color, Color::Black, BitTileRenderMethod::SmallBlocks);
    }

    fn paint_board(&self, surface: &mut Surface) {
        let board_x = 2;
        let board_y = 2;

        if let Some((player, col, y)) = self.falling_piece {
            if y >= 0.0 {
                let screen_x = board_x + (col * 8) as i32;
                let screen_y = board_y + (y * 4.0) as i32 + 1;
                self.draw_circle(surface, screen_x, screen_y, player.color());
            }
        }
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                let screen_x = board_x + (col * 8) as i32;
                let screen_y = board_y + (row * 4) as i32 + 1;

                if let CellState::Occupied(player) = self.board[row][col] {
                    self.draw_circle(surface, screen_x, screen_y, player.color());
                }
            }
        }
        let circle_pattern = create_circle_pattern();
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                let screen_x = board_x + (col * 8) as i32;
                let screen_y = board_y + (row * 4) as i32 + 1;

                surface.draw_tile(
                    screen_x,
                    screen_y,
                    &circle_pattern,
                    Color::Transparent,
                    Color::Gray,
                    BitTileRenderMethod::SmallBlocks,
                );
            }
        }
    }

    fn paint_game_over(&self, surface: &mut Surface) {
        const X: i32 = 20;
        const Y: i32 = 15;
        const W: i32 = 40;
        let r = Rect::with_size(X, Y, W as u16, 6);
        surface.fill_rect(r, Character::new(' ', Color::White, Color::DarkBlue, CharFlags::None));

        match self.state {
            GameState::Victory => {
                let winner_text = match self.winner {
                    Some(Player::Red) => "Red Player Wins!",
                    Some(Player::Yellow) => "Yellow Player Wins!",
                    None => "Someone Wins!",
                };
                surface.write_string(X + W / 2 - (winner_text.len() / 2) as i32, Y, winner_text, charattr!("white"), false);
            }
            GameState::GameOver => {
                surface.write_string(X + W / 2 - 4, Y, "Game Over!", charattr!("white"), false);
            }
            _ => {}
        }

        surface.draw_horizontal_line_with_size(X + 1, Y + 1, (W - 2) as u32, LineType::Single, charattr!("gray"));
        surface.write_string(X + 6, Y + 3, "Press SPACE to restart!", charattr!("white"), false);
    }
}

impl OnPaint for ConnectFourGame {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(char!("' ',black,black"));

        match self.state {
            GameState::GameOver | GameState::Victory => {
                self.paint_board(surface);
                self.paint_game_over(surface);
            }
            GameState::Playing => {
                self.paint_board(surface);

                // Draw current player indicator
                let current_player_text = match self.current_player {
                    Player::Red => "Red Player's Turn",
                    Player::Yellow => "Yellow Player's Turn",
                };
                surface.write_string(2, 0, current_player_text, theme.symbol.checked, false);

                for col in 0..BOARD_WIDTH {
                    let x = 2 + (col * 8) as i32 + 3;
                    surface.write_string(x, 1, format!("{}", col + 1).as_str(), charattr!("gray"), false);
                }
            }
        }
    }
}

impl TimerEvents for ConnectFourGame {
    fn on_update(&mut self, _: u64) -> EventProcessStatus {
        if self.state == GameState::Playing {
            self.update_falling_piece();
        }
        EventProcessStatus::Processed
    }
}

impl OnKeyPressed for ConnectFourGame {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("1") => {
                self.drop_piece(0);
                EventProcessStatus::Processed
            }
            key!("2") => {
                self.drop_piece(1);
                EventProcessStatus::Processed
            }
            key!("3") => {
                self.drop_piece(2);
                EventProcessStatus::Processed
            }
            key!("4") => {
                self.drop_piece(3);
                EventProcessStatus::Processed
            }
            key!("5") => {
                self.drop_piece(4);
                EventProcessStatus::Processed
            }
            key!("6") => {
                self.drop_piece(5);
                EventProcessStatus::Processed
            }
            key!("7") => {
                self.drop_piece(6);
                EventProcessStatus::Processed
            }
            key!("Space") => {
                match self.state {
                    GameState::GameOver | GameState::Victory => {
                        self.reset_game();
                    }
                    _ => {}
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
