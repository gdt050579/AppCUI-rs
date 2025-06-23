use appcui::prelude::*;
use ramitgame::Events;
use rand::Rng;
use std::time::Duration;

const NUM_ROWS: usize = 16;
const BAR_COLORS: [Color; 6] = [Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Magenta, Color::White];

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Countdown,
    Playing,
    Paused,
    GameOver,
}

#[CustomControl(overwrite = OnPaint+OnKeyPressed, events = TimerEvents, emit: UpdateScore)]
pub struct RamItGame {
    state: GameState,
    player_row: usize, // 0..NUM_ROWS-1
    left_bars: [u32; NUM_ROWS],
    right_bars: [u32; NUM_ROWS],
    score: u32,
    high_score: u32,
    countdown: u32,
    initialized: bool,
    move_timer: u32,
    on_left: bool,
    bullet_x: i32,
    bullet_add: i32,
    bullet_y: i32,
}

impl RamItGame {
    pub fn new() -> Self {
        let mut o = Self {
            base: ControlBase::new(Layout::new("d:c"), true),
            state: GameState::Menu,
            player_row: NUM_ROWS / 2,
            left_bars: [0; NUM_ROWS],
            right_bars: [0; NUM_ROWS],
            score: 0,
            high_score: 0,
            countdown: 3,
            initialized: false,
            move_timer: 0,
            on_left: true,
            bullet_add: 0,
            bullet_x: 0,
            bullet_y: 0,
        };
        if let Some(timer) = o.timer() {
            timer.start(Duration::from_millis(50));
        }
        o
    }

    #[inline(always)]
    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn reset_game(&mut self) {
        self.player_row = NUM_ROWS / 2;
        self.score = 0;
        self.countdown = 3;
        self.move_timer = 0;
        self.state = GameState::Countdown;
        self.left_bars = [2; NUM_ROWS];
        self.right_bars = [2; NUM_ROWS];
        self.on_left = true;
        self.bullet_add = 0;
        self.bullet_x = 0;
        self.bullet_y = 0;
        self.raise_event(Events::UpdateScore);
    }

    fn update_bars(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.4) {
            let idx = rng.gen_range(0..NUM_ROWS * 2);
            let val = if idx < NUM_ROWS {
                self.left_bars[idx] += 1;
                self.left_bars[idx]
            } else {
                self.right_bars[idx - NUM_ROWS] += 1;
                self.right_bars[idx - NUM_ROWS]
            };
            if val >= (self.size().width / 2).saturating_sub(1) {
                self.state = GameState::GameOver;
            }
        }
    }

    fn update_bullet(&mut self) {
        if self.bullet_add == 0 {
            return;
        }
        self.bullet_x += self.bullet_add;
        if self.bullet_add < 0 {
            let px = self.left_bars[self.bullet_y as usize] as i32;
            if px >= self.bullet_x {
                self.left_bars[self.bullet_y as usize] = self.left_bars[self.bullet_y as usize].saturating_sub(1);
                self.bullet_add = 0;
                self.score += 1;
                self.raise_event(Events::UpdateScore);
            }
        } else {
            let px = self.size().width as i32 - self.right_bars[self.bullet_y as usize] as i32 - 1;
            if px <= self.bullet_x {
                self.right_bars[self.bullet_y as usize] = self.right_bars[self.bullet_y as usize].saturating_sub(1);
                self.bullet_add = 0;
                self.score += 1;
                self.raise_event(Events::UpdateScore);
            }
        }
    }

    fn move_up(&mut self) {
        if self.player_row > 0 {
            self.player_row -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.player_row + 1 < NUM_ROWS {
            self.player_row += 1;
        }
    }

    fn paint_game(&self, surface: &mut Surface) {
        let size = self.size();
        let mut ch = Character::new(SpecialChar::BlockCentered, Color::White, Color::Transparent, CharFlags::None);
        for i in 0..NUM_ROWS * 2 {
            ch.foreground = BAR_COLORS[i % BAR_COLORS.len()];
            if i < NUM_ROWS {
                surface.fill_horizontal_line_with_size(0, i as i32, self.left_bars[i], ch);
            } else {
                let y = (i - NUM_ROWS) as i32;
                let x = size.width as i32 - self.right_bars[i - NUM_ROWS] as i32 - 1;
                surface.fill_horizontal_line_with_size(x, y, self.right_bars[i - NUM_ROWS], ch);
            }
        }
        let x = (size.width / 2) as i32;
        let mut c = Character::new('o', Color::White, Color::Black, CharFlags::None);
        surface.draw_vertical_line_with_size(x, 0, NUM_ROWS as u32, LineType::Single, charattr!("white,black"));

        surface.write_char(x, self.player_row as i32, c);
        if !self.on_left {
            c.code = '>';
            surface.write_char(x + 1, self.player_row as i32, c);
        } else {
            c.code = '<';
            surface.write_char(x - 1, self.player_row as i32, c);
        }

        if self.bullet_add != 0 {
            surface.write_char(
                self.bullet_x,
                self.bullet_y,
                Character::new(SpecialChar::CircleFilled, Color::Yellow, Color::Black, CharFlags::None),
            );
        }
    }
}

impl OnPaint for RamItGame {
    fn on_paint(&self, surface: &mut Surface, _: &Theme) {
        let size = self.size();
        surface.clear(char!("' ',white,black"));

        match self.state {
            GameState::Menu => {
                surface.write_string((size.width / 2 - 10) as i32, 6, "Press SPACE to start", charattr!("Yellow,black"), true);
            }
            GameState::Countdown => {
                let text = format!("{}", self.countdown);
                let x = (size.width - text.len() as u32) / 2;
                let y = size.height / 2;
                let format = TextFormatBuilder::new()
                    .position(x as i32, y as i32)
                    .attribute(CharAttribute::with_color(Color::White, Color::Black))
                    .align(TextAlignament::Center)
                    .wrap_type(WrapType::SingleLine)
                    .build();
                surface.write_text(&text, &format);
            }
            GameState::Playing => {
                self.paint_game(surface);
            }
            GameState::Paused => {
                self.paint_game(surface);
                surface.clear(Character::with_color(Color::Gray, Color::Black));
                surface.write_string((size.width / 2 - 6) as i32, 6, "GAME IS PAUSED", charattr!("Yellow,black"), true);
                surface.write_string(
                    (size.width / 2 - 12) as i32,
                    8,
                    "Press any key to continue",
                    charattr!("Silver,black"),
                    true,
                );
                surface.write_string((size.width / 2 - 6) as i32, 9, "or ESC to exit", charattr!("Silver,black"), true);
            }
            GameState::GameOver => {
                let text = format!("Game Over! Score: {}", self.score);
                surface.write_string((size.width / 2 - text.len() as u32 / 2) as i32, 6, &text, charattr!("Yellow,black"), true);
                surface.write_string((size.width / 2 - 11) as i32, 8, "Press SPACE to restart", charattr!("Yellow,black"), true);
            }
        }
    }
}

impl TimerEvents for RamItGame {
    fn on_update(&mut self, ticks: u64) -> EventProcessStatus {
        match self.state {
            GameState::Playing => {
                self.update_bars();
                self.update_bullet();
                EventProcessStatus::Processed
            }
            GameState::Countdown => {
                if ticks % 20 == 0 {
                    self.countdown = self.countdown.saturating_sub(1);
                    if self.countdown == 0 {
                        self.reset_game();
                        self.state = GameState::Playing;
                    }
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Processed,
        }
    }
}

impl OnKeyPressed for RamItGame {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match self.state {
            GameState::Menu => {
                if key.code == KeyCode::Space {
                    self.state = GameState::Countdown;
                }
            }
            GameState::Playing => match key.value() {
                key!("up") => self.move_up(),
                key!("down") => self.move_down(),
                key!("left") => self.on_left = true,
                key!("right") => self.on_left = false,
                key!("space") => {
                    if self.bullet_add == 0 {
                        self.bullet_add = if self.on_left { -1 } else { 1 };
                        self.bullet_x = self.size().width as i32 / 2 + self.bullet_add * 2;
                        self.bullet_y = self.player_row as i32;
                    }
                }
                key!("esc") => self.state = GameState::Paused,
                _ => {}
            },
            GameState::Paused => {
                if key.code == KeyCode::Escape {
                    self.state = GameState::Menu;
                } else {
                    self.state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                if key.code == KeyCode::Space {
                    self.reset_game();
                }
            }
            _ => {}
        }
        EventProcessStatus::Processed
    }
}
