use appcui::prelude::*;

struct Starfield {
    time: f32,
}

impl Starfield {
    fn new() -> Self {
        Self { time: 0.0 }
    }
    fn length(x: f32, y: f32) -> f32 {
        (x * x + y * y).sqrt()
    }
    fn fract(x: f32) -> f32 {
        x - x.floor()
    }
    fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
    fn myrandom(seed_x: f32, seed_y: f32) -> f32 {
        Self::fract(((seed_x * seed_y) * 0.001).sin() * 100.0)
    }
    fn my_cell(coord_x: f32, coord_y: f32) -> f32 {
        let cell_x = Self::fract(coord_x) * 0.5;
        let cell_y = Self::fract(coord_y) * 2.0 - 0.5;
        let mask = if Self::myrandom(coord_x.floor(), coord_y.floor()) <= 0.04 {
            1.0
        } else {
            0.0
        };
        (1.0 - Self::length(cell_x * 2.0 - 1.0, cell_y * 2.0 - 1.0)) * mask * 2.0
    }
    fn fragment(&self, sx: f32, sy: f32) -> Color {
        let d = Self::length(sx, sy);
        let mut coord_x = d.powf(0.04) * 250.0;
        let mut coord_y = Self::fract(sx.atan2(sy) / 3.2) * 250.0;
        let delta_x = self.time * (-0.02) * 256.0;
        let delta_y = 0.5;

        let mut c: f32 = 0.0;
        coord_x += delta_x;
        coord_y += delta_y;
        c = c.max(Self::my_cell(coord_x, coord_y));
        coord_x += delta_x;
        coord_y += delta_y;
        c = c.max(Self::my_cell(coord_x, coord_y));
        coord_x += delta_x;
        coord_y += delta_y;
        c = c.max(Self::my_cell(coord_x, coord_y));

        Self::rgb(0.0, (c * d).clamp(0.0, 1.0), (c * d).clamp(0.0, 1.0))
    }
}

impl FrameApp for Starfield {
    fn on_update(&mut self, ticks: u64) {
        self.time = ticks as f32;
    }

    fn on_paint(&self, surface: &mut Surface) {
        let size = surface.size();
        if size.width == 0 || size.height == 0 {
            return;
        }
        let w = size.width as f32;
        let h = (size.height * 2) as f32;
        let aspect = w / h;

        for y in 0..size.height {
            for x in 0..size.width {
                let sx = ((x as f32 + 0.5) / w * 2.0 - 1.0) * aspect;
                let top = self.fragment(sx, 1.0 - ((y * 2) as f32 + 0.5) / h * 2.0);
                let bottom = self.fragment(sx, 1.0 - ((y * 2 + 1) as f32 + 0.5) / h * 2.0);
                surface.write_char(
                    x as i32,
                    y as i32,
                    Character::new(SpecialChar::BlockUpperHalf, top, bottom, CharFlags::None),
                );
            }
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    {
        App::frame_app(Starfield::new())
            .fps(5)
            .title("Starfield")
            .clear_char(None)
            .color_schema(false)
            .backend(appcui::backend::Type::WindowsVT)
            .run()
    }
    #[cfg(not(target_os = "windows"))]
    {
        App::frame_app(Starfield::new())
            .fps(5)
            .title("Starfield")
            .clear_char(None)
            .color_schema(false)
            .run()
    }
}
