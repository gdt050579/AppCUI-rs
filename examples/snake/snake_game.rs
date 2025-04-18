use appcui::prelude::*;
use std::{collections::VecDeque, time::Duration};
use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Countdown,
    Playing,
    Paused,
    GameOver,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum FoodType {
    Regular,
    Golden,
    Speed,
    Slow,
}

struct Food {
    position: (i32, i32),
    food_type: FoodType,
}

impl Food {
    fn new(x: i32, y: i32, food_type: FoodType) -> Self {
        Self {
            position: (x, y),
            food_type,
        }
    }

    fn get_char(&self) -> char {
        match self.food_type {
            FoodType::Regular => '🍎',
            FoodType::Golden => '🌟',
            FoodType::Speed => '⚡',
            FoodType::Slow => '🐌',
        }
    }

    fn get_points(&self) -> u32 {
        match self.food_type {
            FoodType::Regular => 10,
            FoodType::Golden => 50,
            FoodType::Speed => 20,
            FoodType::Slow => 15,
        }
    }
}

#[CustomControl(overwrite = OnPaint+OnKeyPressed, events = TimerEvents)]
pub struct SnakeGame {
    snake: VecDeque<(i32, i32)>, // Each position represents a single character snake segment
    direction: Direction,
    food: Food,
    state: GameState,
    score: u32,
    high_score: u32,
    speed: u64, // milliseconds between moves
    countdown: u32,
    base_speed: u64,
    speed_effect: Option<u64>,
    speed_effect_timer: u32,
    initialized: bool,
}

impl SnakeGame {
    pub fn new() -> Self {
        let mut o = Self {
            base: ControlBase::new(Layout::new("d:c"), true),
            snake: VecDeque::new(),
            direction: Direction::Right,
            food: Food::new(0, 0, FoodType::Regular),
            state: GameState::Menu,
            score: 0,
            high_score: 0,
            speed: 300,
            countdown: 3,
            base_speed: 300,
            speed_effect: None,
            speed_effect_timer: 0,
            initialized: true,
        };
        
        let speed = o.speed;
        if let Some(timer) = o.timer() {
            timer.start(Duration::from_millis(speed));
        }
        o
    }
    
    fn initialize_if_needed(&mut self) {
        if !self.initialized {
            let size = self.size();
            if size.width > 2 && size.height > 2 {
                self.reset_game();
                self.initialized = true;
            }
        }
    }

    pub fn reset_game(&mut self) {
        let size = self.size();
        if size.width < 3 || size.height < 3 {
            // Window size too small, skip initializing
            return;
        }
        
        self.snake.clear();
        let start_x = (size.width / 2 - 1) as i32;
        let start_y = (size.height / 2) as i32;
        self.snake.push_back((start_x, start_y));
        self.direction = Direction::Right;
        self.spawn_food();
        self.state = GameState::Countdown;
        self.score = 0;
        self.speed = self.base_speed;
        self.countdown = 3;
        self.speed_effect = None;
        self.speed_effect_timer = 0;
    }

    fn spawn_food(&mut self) {
        let size = self.size();
        if size.width < 3 || size.height < 3 {
            // Window size too small, use default food position
            self.food = Food::new(0, 0, FoodType::Regular);
            return;
        }
        
        let mut rng = rand::thread_rng();
        let food_type = match rng.gen_range(0..100) {
            0..=70 => FoodType::Regular,
            71..=85 => FoodType::Speed,
            86..=95 => FoodType::Slow,
            _ => FoodType::Golden,
        };

        // Try to find a valid position for the food
        let mut attempts = 0;
        let max_attempts = 100;
        
        while attempts < max_attempts {
            // Ensure food spawns within valid bounds
            let x = rng.gen_range(0..size.width) as i32;
            let y = rng.gen_range(0..size.height) as i32;
            
            // Check if the position is not on the snake
            let mut valid_position = true;
            for &(snake_x, snake_y) in &self.snake {
                if x == snake_x && y == snake_y {
                    valid_position = false;
                    break;
                }
            }
            
            if valid_position {
                self.food = Food::new(x, y, food_type);
                return;
            }
            attempts += 1;
        }

        // If we couldn't find a valid position after max attempts,
        // try to find any empty spot systematically
        for y in 0..size.height {
            for x in 0..size.width {
                let mut valid_position = true;
                for &(snake_x, snake_y) in &self.snake {
                    if x as i32 == snake_x && y as i32 == snake_y {
                        valid_position = false;
                        break;
                    }
                }
                if valid_position {
                    self.food = Food::new(x as i32, y as i32, food_type);
                    return;
                }
            }
        }

        // If we still couldn't find a position, just place it at (0,0)
        // This should only happen if the snake fills the entire board
        self.food = Food::new(0, 0, food_type);
    }

    fn move_snake(&mut self) {
        self.initialize_if_needed();
        
        if self.state != GameState::Playing {
            return;
        }

        // Update speed effects
        if self.speed_effect.is_some() {
            if self.speed_effect_timer > 0 {
                self.speed_effect_timer -= 1;
            } else {
                self.speed = self.base_speed;
                self.speed_effect = None;
                let speed = self.speed;
                if let Some(timer) = self.timer() {
                    timer.start(Duration::from_millis(speed));
                }
            }
        }

        let size = self.size();
        let (head_x, head_y) = match self.snake.front() {
            Some(&head) => head,
            None => return, // No snake head, skip movement
        };
        
        let new_head = match self.direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };

        // Check for wall collisions
        if new_head.0 < 0 || new_head.1 < 0 || 
           new_head.0 >= size.width as i32 || 
           new_head.1 >= size.height as i32 {
            self.state = GameState::GameOver;
            if self.score > self.high_score {
                self.high_score = self.score;
            }
            return;
        }

        // Check for snake collisions
        for &(snake_x, snake_y) in &self.snake {
            if new_head.0 == snake_x && new_head.1 == snake_y {
                self.state = GameState::GameOver;
                if self.score > self.high_score {
                    self.high_score = self.score;
                }
                return;
            }
        }

        // Move snake
        self.snake.push_front(new_head);

        // Check if food is eaten
        if new_head.0 == self.food.position.0 && new_head.1 == self.food.position.1 {
            self.score += self.food.get_points();
            
            // Handle food effects
            match self.food.food_type {
                FoodType::Speed => {
                    self.speed_effect = Some(self.speed);
                    self.speed = (self.speed as f32 * 0.5) as u64;
                    self.speed_effect_timer = 20;
                }
                FoodType::Slow => {
                    self.speed_effect = Some(self.speed);
                    self.speed = (self.speed as f32 * 1.5) as u64;
                    self.speed_effect_timer = 15;
                }
                _ => {
                    if self.score % 50 == 0 && self.base_speed > 50 {
                        self.base_speed -= 25;
                        if self.speed_effect.is_none() {
                            self.speed = self.base_speed;
                        }
                    }
                }
            }

            let speed = self.speed;
            if let Some(timer) = self.timer() {
                timer.start(Duration::from_millis(speed));
            }
            
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if self.state != GameState::Playing {
            return;
        }

        // Prevent reversing direction
        match (self.direction, new_direction) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => {}
            _ => self.direction = new_direction,
        }
    }
}

impl OnPaint for SnakeGame {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        // Clear the entire surface with black background
        surface.clear(char!("' ',black,black"));
        
        if !self.initialized {
            let size = self.size();
            if size.width > 2 && size.height > 2 {
                return;
            }
        }
        
        // Draw game elements
        match self.state {
            GameState::Menu => {
                let size = self.size();
                let title = "SNAKE GAME";
                let instructions = "Press SPACE to start";
                surface.write_string((size.width as i32 - title.len() as i32) / 2, size.height as i32 / 2 - 1, title, theme.symbol.checked, false);
                surface.write_string((size.width as i32 - instructions.len() as i32) / 2, size.height as i32 / 2 + 1, instructions, theme.symbol.checked, false);
            }
            GameState::GameOver => {
                let size = self.size();
                let title = "GAME OVER";
                let score = format!("Score: {}", self.score);
                let instructions = "Press SPACE to restart";
                surface.write_string((size.width as i32 - title.len() as i32) / 2, size.height as i32 / 2 - 2, title, theme.symbol.checked, false);
                surface.write_string((size.width as i32 - score.len() as i32) / 2, size.height as i32 / 2, score.as_str(), theme.symbol.checked, false);
                surface.write_string((size.width as i32 - instructions.len() as i32) / 2, size.height as i32 / 2 + 2, instructions, theme.symbol.checked, false);
            }
            GameState::Countdown => {
                let size = self.size();
                surface.write_string((size.width / 2 - 1) as i32, (size.height / 2) as i32, format!("{}", self.countdown).as_str(), theme.symbol.checked, false);
            }
            GameState::Playing | GameState::Paused => {
                // Draw score
                surface.write_string(0, 0, format!("Score: {}", self.score).as_str(), theme.symbol.checked, false);
                surface.write_string(0, 1, format!("High Score: {}", self.high_score).as_str(), theme.symbol.checked, false);
                
                // Draw snake
                for (x, y) in &self.snake {
                    surface.write_char(*x, *y + 2, Character::with_attributes('■', theme.symbol.checked));
                }
                // Draw food
                surface.write_char(self.food.position.0, self.food.position.1 + 2, Character::with_attributes(self.food.get_char(), theme.symbol.checked));

                if self.state == GameState::Paused {
                    let size = self.size();
                    surface.write_string(0, (size.height - 1) as i32, "PAUSED - Press P to resume", theme.symbol.checked, false);
                }
            }
        }
    }
}

impl TimerEvents for SnakeGame {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        match self.state {
            GameState::Countdown => {
                if self.countdown > 0 {
                    self.countdown -= 1;
                } else {
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                self.move_snake();
            }
            _ => {}
        }
        EventProcessStatus::Processed
    }
}

impl OnKeyPressed for SnakeGame {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Up") => {
                self.change_direction(Direction::Up);
                EventProcessStatus::Processed
            }
            key!("Down") => {
                self.change_direction(Direction::Down);
                EventProcessStatus::Processed
            }
            key!("Left") => {
                self.change_direction(Direction::Left);
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.change_direction(Direction::Right);
                EventProcessStatus::Processed
            }
            key!("Space") => {
                match self.state {
                    GameState::Menu => self.reset_game(),
                    GameState::GameOver => self.reset_game(),
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