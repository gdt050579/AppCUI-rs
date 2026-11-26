use appcui::prelude::*;
use crate::benchmark::{Benchmark, BenchmarkList};

struct TestSurfaceWriteChar {
    surface: Surface,
}
impl Benchmark for TestSurfaceWriteChar {
    fn init(&mut self) {
        self.surface = Surface::new(1000, 1000);
    }
    fn run(&mut self) {
        for x in 0..1000 {
            for y in 0..1000 {
                self.surface.write_char(x,y, char!("A,white,black"));
            }
        }
    }
    fn name(&self) -> &'static str {
        "Surface::WriteCharr"
    }
    fn description(&self) -> &'static str {
        "Test the performance of Surface::write_char over a 1000x1000 surface"
    }
}

struct TestSpriteWriteChar {
    sprite: Sprite,
}
impl Benchmark for TestSpriteWriteChar {
    fn init(&mut self) {
        self.sprite = Sprite::new(1000, 1000);
    }
    fn run(&mut self) {
        for x in 0..1000 {
            for y in 0..1000 {
                self.sprite.set(x,y, char!("A,white,black"));
            }
        }
    }
    fn name(&self) -> &'static str {
        "Sprite::set"
    }
    fn description(&self) -> &'static str {
        "Test the performance of Sprite::set over a 1000x1000 sprite"
    }
}

pub(crate) fn test_list() -> BenchmarkList {
    let mut benchmark_list = BenchmarkList::new();
    benchmark_list.add(Box::new(TestSurfaceWriteChar { surface: Surface::new(1, 1) }));
    benchmark_list.add(Box::new(TestSpriteWriteChar { sprite: Sprite::new(1, 1) }));
    benchmark_list
}
