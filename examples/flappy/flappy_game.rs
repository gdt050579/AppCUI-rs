use appcui::prelude::*;
use flappygame::Events;
use rand::Rng;
use std::time::Duration;

const GAME_HEIGHT: usize = 15;
const BIRD_X_POS: usize = 10;
const PIPE_GAP_SIZE: usize = 5;
const PIPE_WIDTH: usize = 3;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Countdown,
    Playing,
    Paused,
    GameOver,
}

struct Pipe {
    x_pos: i32,
    gap_y_pos: i32,
}

#[CustomControl(overwrite = OnPaint+OnKeyPressed, events = TimerEvents, emit: UpdateScore)]
pub struct FlappyGame {
    state: GameState,
    bird_y: f32,
    bird_velocity: f32,
    pipes: Vec<Pipe>,
    score: u32,
    high_score: u32,
    countdown: u32,
    gravity: f32,
    jump_force: f32,
    initialized: bool,
    pipe_spawn_timer: u32,
    game_speed: f32,
    next_pipe_distance: u32,
}

impl FlappyGame {
    pub fn new() -> Self {
        let mut o = Self {
            base: ControlBase::new(layout!("d:f"), true),
            state: GameState::Menu,
            bird_y: GAME_HEIGHT as f32 / 2.0,
            bird_velocity: 0.0,
            pipes: Vec::new(),
            score: 0,
            high_score: 0,
            countdown: 3,
            gravity: 0.05,
            jump_force: -0.6,
            initialized: false,
            pipe_spawn_timer: 0,
            game_speed: 1.0,
            next_pipe_distance: 15,
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
        self.bird_y = GAME_HEIGHT as f32 / 2.0;
        self.bird_velocity = 0.0;
        self.pipes.clear();
        self.score = 0;
        self.countdown = 3;
        self.pipe_spawn_timer = 0;
        self.game_speed = 1.0;
        self.state = GameState::Playing;
        self.raise_event(Events::UpdateScore);
    }

    fn update_bird(&mut self) {
        // Apply gravity to bird's velocity
        self.bird_velocity += self.gravity;
        
        // Update bird position
        self.bird_y += self.bird_velocity;
        
        // Keep bird within game boundaries
        if self.bird_y < 0.0 {
            self.bird_y = 0.0;
            self.bird_velocity = 0.0;
        } else if self.bird_y >= GAME_HEIGHT as f32 {
            self.bird_y = GAME_HEIGHT as f32 - 1.0;
            self.state = GameState::GameOver;
            if self.score > self.high_score {
                self.high_score = self.score;
            }
        }
    }
    
    fn update_pipes(&mut self) {
        // Move pipes to the left
        for pipe in &mut self.pipes {
            pipe.x_pos -= 1;
        }
        
        // Remove pipes that are off-screen
        self.pipes.retain(|pipe| pipe.x_pos + PIPE_WIDTH as i32 > 0);
        
        // Add new pipe if it's time
        self.pipe_spawn_timer += 1;
        if self.pipe_spawn_timer >= self.next_pipe_distance {
            self.pipe_spawn_timer = 0;
            self.add_pipe();
        }
        
        // Check for collisions with pipes
        self.check_pipe_collisions();
        
        // Check for score (when bird passes a pipe)
        self.check_score();
    }
    
    fn add_pipe(&mut self) {
        let mut rng = rand::thread_rng();
        let gap_y_pos = rng.gen_range(2..GAME_HEIGHT as i32 - PIPE_GAP_SIZE as i32 - 2);
        let game_width = self.size().width as i32;
        
        self.pipes.push(Pipe {
            x_pos: game_width,
            gap_y_pos,
        });
    }
    
    fn check_pipe_collisions(&mut self) {
        let bird_x = BIRD_X_POS as i32;
        let bird_y = self.bird_y as i32;
        
        for pipe in &self.pipes {
            // Only check pipes that the bird can possibly collide with
            if bird_x >= pipe.x_pos && bird_x <= pipe.x_pos + PIPE_WIDTH as i32 {
                // Check if bird is within the gap
                if bird_y < pipe.gap_y_pos || bird_y >= pipe.gap_y_pos + PIPE_GAP_SIZE as i32 {
                    self.state = GameState::GameOver;
                    if self.score > self.high_score {
                        self.high_score = self.score;
                    }
                    break;
                }
            }
        }
    }
    
    fn check_score(&mut self) {
        let bird_x = BIRD_X_POS as i32;
        
        for pipe in &self.pipes {
            // When the bird just passes the pipe
            if pipe.x_pos + PIPE_WIDTH as i32 == bird_x {
                self.score += 1;
                self.raise_event(Events::UpdateScore);
                
                // Increase game speed after every 5 points
                if self.score % 5 == 0 {
                    self.game_speed += 0.1;
                    // Reduce the distance between pipes as game progresses
                    if self.next_pipe_distance > 10 {
                        self.next_pipe_distance -= 1;
                    }
                }
                break;
            }
        }
    }
    
    fn make_bird_jump(&mut self) {
        self.bird_velocity = self.jump_force;
    }
}

impl OnPaint for FlappyGame {
    fn on_paint(&self, surface: &mut Surface, _: &Theme) {
        let size = self.size();
        surface.clear(char!("' ',blue,black"));

        match self.state {
            GameState::Menu => {
                surface.write_string((size.width / 2 - 10) as i32, 6, "Press SPACE to start", charattr!("Yellow,black"), true);
                if self.high_score > 0 {
                    let text = format!("High Score: {}", self.high_score);
                    surface.write_string((size.width / 2 - text.len() as u32 / 2) as i32, 8, &text, charattr!("White,black"), true);
                }
                
                // Draw a simple bird animation in the menu
                let bird_char = Character::new('>', Color::Yellow, Color::Transparent, CharFlags::None);
                surface.write_char(BIRD_X_POS as i32, (GAME_HEIGHT / 2) as i32, bird_char);
            }
            GameState::Countdown => {
                let text = format!("{}", self.countdown);
                let x = (size.width - text.len() as u32) / 2;
                let y = size.height / 2;
                let format = TextFormatBuilder::new()
                    .position(x as i32, y as i32)
                    .attribute(CharAttribute::with_color(Color::White, Color::Black))
                    .align(TextAlignment::Center)
                    .wrap_type(WrapType::SingleLine)
                    .build();
                surface.write_text(&text, &format);
            }
            GameState::Playing | GameState::GameOver | GameState::Paused => {
                // Draw the sky (already done with the clear)
                
                // Draw ground
                let ground_char = Character::new(SpecialChar::BlockLowerHalf, Color::Green, Color::Yellow, CharFlags::None);
                for x in 0..size.width as i32 {
                    surface.write_char(x, GAME_HEIGHT as i32, ground_char);
                }
                
                // Draw bird
                let bird_char = if self.bird_velocity < 0.0 {
                    // Bird is going up - use '^'
                    Character::new('^', Color::Yellow, Color::Transparent, CharFlags::None)
                } else {
                    // Bird is going down - use '>'
                    Character::new('>', Color::Yellow, Color::Transparent, CharFlags::None)
                };
                surface.write_char(BIRD_X_POS as i32, self.bird_y as i32, bird_char);
                
                // Draw pipes
                let pipe_char = Character::new(SpecialChar::Block100, Color::Green, Color::Transparent, CharFlags::None);
                for pipe in &self.pipes {
                    // Draw top part of pipe
                    for y in 0..pipe.gap_y_pos {
                        for x in 0..PIPE_WIDTH {
                            surface.write_char(pipe.x_pos + x as i32, y, pipe_char);
                        }
                    }
                    
                    // Draw bottom part of pipe
                    for y in (pipe.gap_y_pos + PIPE_GAP_SIZE as i32)..GAME_HEIGHT as i32 {
                        for x in 0..PIPE_WIDTH {
                            surface.write_char(pipe.x_pos + x as i32, y, pipe_char);
                        }
                    }
                }
                
                // If game is paused, draw pause overlay
                if self.state == GameState::Paused {
                    let pause_overlay = Character::with_color(Color::Gray, Color::Black);
                    surface.clear(pause_overlay);
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
                
                // If game over, draw game over overlay
                if self.state == GameState::GameOver {
                    let text = format!("Game Over! Score: {}", self.score);
                    surface.write_string((size.width / 2 - text.len() as u32 / 2) as i32, 6, &text, charattr!("Yellow,black"), true);
                    surface.write_string((size.width / 2 - 11) as i32, 8, "Press SPACE to restart", charattr!("Yellow,black"), true);
                }
            }
        }
    }
}

impl TimerEvents for FlappyGame {
    fn on_update(&mut self, ticks: u64) -> EventProcessStatus {
        match self.state {
            GameState::Playing => {
                self.update_bird();
                self.update_pipes();
                EventProcessStatus::Processed
            }
            GameState::Countdown => {
                if ticks % 20 == 0 {
                    self.countdown = self.countdown.saturating_sub(1);
                    if self.countdown == 0 {
                        self.reset_game();
                    }
                }
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Processed,
        }
    }
}

impl OnKeyPressed for FlappyGame {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match self.state {
            GameState::Menu => {
                if key.code == KeyCode::Space {
                    self.state = GameState::Countdown;
                }
            }
            GameState::Playing => match key.value() {
                key!("space") => self.make_bird_jump(),
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