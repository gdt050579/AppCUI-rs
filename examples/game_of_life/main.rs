use appcui::prelude::*;
use rand::Rng;

/// Advance one generation every this many frames.
const TICKS_PER_GENERATION: u64 = 3;
/// Probability that a cell starts alive when the grid is randomised.
const ALIVE_CHANCE: f64 = 0.3;

struct GameOfLife {
    size: Size,
    cells: Vec<bool>,
    next: Vec<bool>,
    paused: bool,
    last_generation_tick: u64,
}

impl GameOfLife {
    fn new() -> Self {
        Self {
            size: Size::new(0, 0),
            cells: Vec::new(),
            next: Vec::new(),
            paused: false,
            last_generation_tick: 0,
        }
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        for cell in self.cells.iter_mut() {
            *cell = rng.gen_bool(ALIVE_CHANCE);
        }
    }

    /// Reallocates the grid for the new terminal size and fills it with a random pattern.
    fn resize_grid(&mut self, new_size: Size) {
        let count = new_size.width as usize * new_size.height as usize;
        self.size = new_size;
        self.cells = vec![false; count];
        self.next = vec![false; count];
        self.randomize();
    }

    /// Returns the state of a cell. The grid wraps around at its edges.
    fn is_alive(&self, x: i32, y: i32) -> bool {
        let w = self.size.width as i32;
        let h = self.size.height as i32;
        let x = (x + w) % w;
        let y = (y + h) % h;
        self.cells[(y * w + x) as usize]
    }

    fn step(&mut self) {
        let w = self.size.width as i32;
        let h = self.size.height as i32;
        for y in 0..h {
            for x in 0..w {
                let mut neighbours = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if (dx != 0 || dy != 0) && self.is_alive(x + dx, y + dy) {
                            neighbours += 1;
                        }
                    }
                }
                let alive = self.is_alive(x, y);
                self.next[(y * w + x) as usize] = matches!((alive, neighbours), (true, 2) | (true, 3) | (false, 3));
            }
        }
        std::mem::swap(&mut self.cells, &mut self.next);
    }
}

impl FrameApp for GameOfLife {
    fn on_resize(&mut self, new_size: Size) {
        self.resize_grid(new_size);
    }

    fn on_update(&mut self, ticks: u64) {
        if self.paused || self.cells.is_empty() {
            return;
        }
        if ticks.saturating_sub(self.last_generation_tick) >= TICKS_PER_GENERATION {
            self.step();
            self.last_generation_tick = ticks;
        }
    }

    fn on_key_event(&mut self, key: Key, ch: char) {
        if key.value() == key!("Space") {
            self.paused = !self.paused;
        } else if ch == 'r' || ch == 'R' {
            self.randomize();
        }
    }

    fn on_paint(&self, surface: &mut Surface) {
        let alive = char!("' ',black,green");
        for y in 0..self.size.height as i32 {
            for x in 0..self.size.width as i32 {
                let idx = (y * self.size.width as i32 + x) as usize;
                if self.cells.get(idx) == Some(&true) {
                    surface.write_char(x, y, alive);
                }
            }
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::frame_app(GameOfLife::new()).fps(30).title("Game of Life").run()
}
