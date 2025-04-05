use appcui::prelude::*;
use std::time::Duration;
use rand;

#[derive(Clone)]
struct Bar {
    height: i32,
    color: Color,
    x: i32,  // Add x position to track movement
}

#[Window(events = TimerEvents)]
pub(crate) struct Win {
    bars: Vec<Bar>,
    last_update: u64,
    canvas: Handle<Canvas>,
}

impl Win {
    pub(crate) fn new() -> Self {
        let mut w = Self {
            base: window!("Animation,d:c,w:50,h:20"),
            bars: Vec::new(),
            last_update: 0,
            canvas: Handle::None,
        };

        // Add canvas using the public add method
        w.canvas = w.add(canvas!("l:0,t:0,r:0,b:0,size:100x100"));
        if let Some(timer) = w.timer() {
            timer.start(Duration::from_millis(300)); // Update every 0.3 seconds
        }
        w
    }

    fn get_height_change(&self, current_height: i32, max_height: i32) -> i32 {
        let quarter_height = max_height / 4;
        let three_quarters_height = (max_height * 3) / 4;
        
        if current_height < quarter_height {
            // When height is less than 25%, higher chance to increase
            match rand::random::<u32>() % 10 {
                0..=1 => -1,  // 20% chance
                2..=4 => 0,   // 30% chance
                5..=9 => 1,   // 50% chance
                _ => 0,
            }
        } else if current_height > three_quarters_height {
            // When height is more than 75%, higher chance to decrease
            match rand::random::<u32>() % 10 {
                0..=4 => -1,  // 50% chance
                5..=7 => 0,   // 30% chance
                8..=9 => 1,   // 20% chance
                _ => 0,
            }
        } else {
            // Normal case: equal probability
            rand::random::<i32>() % 3 - 1
        }
    }
}

impl TimerEvents for Win {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        let canvas_handle = self.canvas;
        
        // Get canvas size first
        let size = if let Some(canvas) = self.control(canvas_handle) {
            canvas.size()
        } else {
            return EventProcessStatus::Processed;
        };

        // Move all existing bars to the left
        for bar in &mut self.bars {
            bar.x -= 2; // Move 2 positions to leave space between columns
        }

        // Remove bars that have moved off screen
        self.bars.retain(|bar| bar.x >= 0);
        
        // Add a new random bar at the right edge
        let colors = [
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Aqua,
            Color::Yellow,
            Color::Pink,
            Color::Gray,
        ];
        let random_color = colors[rand::random::<usize>() % colors.len()];
        
        // Calculate new height based on previous bar or random if first bar
        let new_height = if let Some(last_bar) = self.bars.last() {
            let change = self.get_height_change(last_bar.height, size.height as i32);
            (last_bar.height + change).clamp(1, size.height as i32 - 2)
        } else {
            // First bar - random height
            (rand::random::<u32>() % (size.height as u32 - 2)) as i32 + 1
        };

        self.bars.push(Bar {
            height: new_height,
            color: random_color,
            x: size.width as i32 - 1,
        });

        // Prepare bar data for drawing
        let bars_data: Vec<_> = self.bars.iter()
            .map(|bar| (bar.x, bar.height, bar.color))
            .collect();

        // Draw bars
        if let Some(canvas) = self.control_mut(canvas_handle) {
            let surface = canvas.get_drawing_surface();
            surface.clear(Character::with_char(' '));
            
            // Draw marker lines
            let quarter_height = size.height as i32 / 4;
            let half_height = size.height as i32 / 2;
            let three_quarters_height = (size.height as i32 * 3) / 4;

            // Draw horizontal lines for each threshold
            for x in 0..size.width as i32 {
                // 25% line
                surface.write_char(
                    x,
                    size.height as i32 - 1 - quarter_height,
                    Character::new('─', Color::White, Color::Black, CharFlags::None),
                );
                // 50% line
                surface.write_char(
                    x,
                    size.height as i32 - 1 - half_height,
                    Character::new('─', Color::White, Color::Black, CharFlags::None),
                );
                // 75% line
                surface.write_char(
                    x,
                    size.height as i32 - 1 - three_quarters_height,
                    Character::new('─', Color::White, Color::Black, CharFlags::None),
                );
            }
            
            // Draw each bar using the prepared data
            for (x, height, color) in bars_data {
                if x >= 0 {
                    for y in 0..height {
                        surface.write_char(
                            x,
                            size.height as i32 - 1 - y,
                            Character::new('█', color, Color::Black, CharFlags::None),
                        );
                    }
                }
            }
        }

        EventProcessStatus::Processed
    }
} 