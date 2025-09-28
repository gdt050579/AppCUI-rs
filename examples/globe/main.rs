use appcui::prelude::*;
use std::time::Duration;
mod globe;

// Image taken from: https://opengameart.org/content/rotating-pixel-art-earth

#[Desktop(events    = [DesktopEvents, TimerEvents], overwrite = OnPaint)]
struct GlobeDesktop {
    idx: usize,
    sprites: Vec<Image>,
    opt: RenderOptions,
}

impl GlobeDesktop {
    fn new() -> Self {
        let mut sprites = Vec::new();
        let mut px = 0;
        let mut py = 0;
        while sprites.len() < 94 {
            sprites.push(GlobeDesktop::build_image(px, py));
            px += 48;
            if px >= 480 {
                px = 0;
                py += 48;
            }
        }
        Self {
            base: Desktop::new(),
            idx: 0,
            sprites,
            opt: RenderOptionsBuilder::new().character_set(CharacterSet::SmallBlocks).build(),
        }
    }
    fn build_image(sprite_x: u32, sprite_y: u32) -> Image {
        let mut image = Image::new(48, 48).unwrap();
        for y in 0..48 {
            for x in 0..48 {
                let poz = ((sprite_y + y) * 480 + sprite_x + x) as usize;
                let pixel = Pixel::from(globe::GLOBE_IMAGE[poz]);
                image.set_pixel(x, y, pixel);
            }
        }
        image
    }
}

impl OnPaint for GlobeDesktop {
    fn on_paint(&self, surface: &mut Surface, _: &Theme) {
        surface.clear(char!("' ',black,black"));
        let sprite_size = self.sprites[self.idx].render_size(&self.opt);
        let x = self.size().width as i32 / 2 - sprite_size.width as i32 / 2;
        let y = self.size().height as i32 / 2 - sprite_size.height as i32 / 2;
        surface.draw_image(x, y, &self.sprites[self.idx], &self.opt);
    }
}

impl DesktopEvents for GlobeDesktop {
    fn on_start(&mut self) {
        if let Some(timer) = self.timer() {
            timer.start(Duration::from_millis(50));
        }
    }
}

impl TimerEvents for GlobeDesktop {
    fn on_update(&mut self, ticks: u64) -> EventProcessStatus {
        self.idx = ticks as usize % self.sprites.len();
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    App::with_backend(appcui::backend::Type::WindowsVT).desktop(GlobeDesktop::new()).color_schema(false).build()?.run();
    #[cfg(not(target_os = "windows"))]
    App::new().desktop(GlobeDesktop::new()).color_schema(false).build()?.run();
    Ok(())
}
