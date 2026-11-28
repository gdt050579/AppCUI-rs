use crate::{
    benchmark::{Benchmark, BenchmarkList},
    params::Params,
};
use appcui::prelude::*;

struct TestSurfaceWriteChar {
    surface: Surface,
}
impl Benchmark for TestSurfaceWriteChar {
    fn init(&mut self, params: &Params) {
        self.surface = Surface::new(params.screen_size.width, params.screen_size.height);
    }
    fn run(&mut self, _params: &Params) {
        let size = self.surface.size();
        for x in 0..size.width as i32 {
            for y in 0..size.height as i32 {
                self.surface.write_char(x, y, char!("A,white,black"));
            }
        }
    }
    fn name(&self) -> &'static str {
        "Surface::WriteChar"
    }
    fn description(&self) -> &'static str {
        "Write 1M chars to a surface"
    }
    fn size(&self) -> Size {
        self.surface.size()
    }
}
struct TestSurfaceWriteTransparentChar {
    surface: Surface,
}
impl Benchmark for TestSurfaceWriteTransparentChar {
    fn init(&mut self, params: &Params) {
        self.surface = Surface::new(params.screen_size.width, params.screen_size.height);
    }
    fn run(&mut self, _params: &Params) {
        let size = self.surface.size();
        for x in 0..size.width as i32 {
            for y in 0..size.height as i32 {
                self.surface
                    .write_char(x, y, Character::new(0, Color::Transparent, Color::Transparent, CharFlags::None));
            }
        }
    }
    fn name(&self) -> &'static str {
        "Surface::WriteTransparentChar"
    }
    fn description(&self) -> &'static str {
        "Write 1M transparent chars to a surface"
    }
    fn size(&self) -> Size {
        self.surface.size()
    }
}

struct TestSpriteWriteChar {
    sprite: Sprite,
}
impl Benchmark for TestSpriteWriteChar {
    fn init(&mut self, params: &Params) {
        self.sprite = Sprite::new(params.screen_size.width, params.screen_size.height);
    }
    fn run(&mut self, _params: &Params) {
        let size = self.sprite.size();
        for x in 0..size.width {
            for y in 0..size.height {
                self.sprite.set(x, y, char!("A,white,black"));
            }
        }
    }
    fn name(&self) -> &'static str {
        "Sprite::set"
    }
    fn description(&self) -> &'static str {
        "Write 1M chars to a sprite"
    }
    fn size(&self) -> Size {
        self.sprite.size()
    }
}

pub(crate) fn test_list() -> BenchmarkList {
    let mut benchmark_list = BenchmarkList::new();
    benchmark_list.add(Box::new(TestSurfaceWriteChar { surface: Surface::new(1, 1) }));
    benchmark_list.add(Box::new(TestSurfaceWriteTransparentChar { surface: Surface::new(1, 1) }));
    benchmark_list.add(Box::new(TestSpriteWriteChar { sprite: Sprite::new(1, 1) }));
    benchmark_list
}
