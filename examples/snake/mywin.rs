use appcui::prelude::*;
use std::{collections::VecDeque, time::Duration};
use rand::Rng; // Add this import for random number generation

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[CustomControl(overwrite = OnPaint+OnKeyPressed, events = TimerEvents)]
struct SnakeGame {
    snake: VecDeque<(i32, i32)>,
    direction: Direction,
    food: (i32, i32),
    width: i32,
    height: i32,
    game_over: bool,
}

impl SnakeGame {
    fn new(width: i32, height: i32) -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((width / 2, height / 2));
        let food = (width / 4, height / 4);
        let mut o = Self {
            base: ControlBase::new(Layout::new("d:c"), true),
            snake,
            direction: Direction::Right,
            food,
            width,
            height,
            game_over: false,
        };
        if let Some(timer) = o.timer() {
            timer.start(Duration::from_millis(300));
        }
        o
    }

    fn move_snake(&mut self) {
        if self.game_over {
            return;
        }

        let (head_x, head_y) = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (*head_x, *head_y - 1),
            Direction::Down => (*head_x, *head_y + 1),
            Direction::Left => (*head_x - 2, *head_y), // Move by 2 characters horizontally
            Direction::Right => (*head_x + 2, *head_y), // Move by 2 characters horizontally
        };

        // Check for collisions
        if new_head.0 < 0 || new_head.1 < 0 || new_head.0 >= self.width || new_head.1 >= self.height || self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        // Move snake
        self.snake.push_front(new_head);

        // Check if food is eaten
        if new_head == self.food {
            let mut rng = rand::thread_rng();
            self.food = (
                rng.gen_range(0..self.width),  // Generate new random x-coordinate
                rng.gen_range(0..self.height), // Generate new random y-coordinate
            );
        } else {
            self.snake.pop_back();
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if self.game_over {
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
        // paint the snake
        for (x, y) in &self.snake {
            surface.write_char(*x, *y, Character::with_attributes('O', theme.symbol.checked));
            surface.write_char(*x + 1, *y, Character::with_attributes('O', theme.symbol.checked)); // Extend cell horizontally
        }
        // paint the food
        surface.write_char(self.food.0, self.food.1, Character::with_attributes('X', theme.symbol.checked));
        surface.write_char(self.food.0 + 1, self.food.1, Character::with_attributes('X', theme.symbol.checked)); // Extend food horizontally
    }
}
impl TimerEvents for SnakeGame {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        if self.game_over {
            return EventProcessStatus::Ignored;
        }
        self.move_snake();
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
            _ => EventProcessStatus::Ignored,
        }
    }
}

#[Window()]
pub struct MyWin {
    game: Handle<SnakeGame>,
}

impl MyWin {
    pub fn new() -> Self {
        let mut win = MyWin {
            base: window!("'Snake Game',d:c,w:50,h:20"),
            game: Handle::None,
        };

        win.game = win.add(SnakeGame::new(50, 20));
        win
    }
}
