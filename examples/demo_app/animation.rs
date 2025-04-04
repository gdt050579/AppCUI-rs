use appcui::prelude::*;
use std::time::Duration;

#[Window(events = TimerEvents)]
pub(crate) struct Win {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    last_update: u64,
    canvas: Handle<Canvas>,
}

impl Win {
    pub(crate) fn new() -> Self {
        let mut w = Self {
            base: window!("Animation,d:c,w:50,h:20"),
            x: 1,
            y: 1,
            dx: 1,
            dy: 1,
            last_update: 0,
            canvas: Handle::None,
        };

        // Add canvas using the public add method
        w.canvas = w.add(canvas!("l:0,t:0,r:0,b:0,size:100x100"));
        if let Some(timer) = w.timer() {
            timer.start(Duration::from_millis(16));
        }
        w
    }
}

impl TimerEvents for Win {
    fn on_update(&mut self, _ticks: u64) -> EventProcessStatus {
        let canvas_handle = self.canvas;
        let mut x = self.x;
        let mut y = self.y;
        let mut dx = self.dx;
        let mut dy = self.dy;

        if let Some(canvas) = self.control_mut(canvas_handle) {
            let size = canvas.size();
            x += dx;
            y += dy;

            if x <= 0 {
                x = 0;
                dx = 1;
            } else if x >= size.width as i32 - 1 {
                x = size.width as i32 - 1;
                dx = -1;
            }

            if y <= 0 {
                y = 0;
                dy = 1;
            } else if y >= size.height as i32 - 1 {
                y = size.height as i32 - 1;
                dy = -1;
            }

            // Get the drawing surface and draw the circle
            let surface = canvas.get_drawing_surface();
            surface.clear(Character::with_char(' '));
            surface.write_char(x, y, Character::new('O', Color::White, Color::Black, CharFlags::None));
        }

        self.x = x;
        self.y = y;
        self.dx = dx;
        self.dy = dy;
        EventProcessStatus::Processed
    }
} 