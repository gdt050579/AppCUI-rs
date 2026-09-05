use appcui::prelude::*;

struct Plasma {
    index: f32,
}

impl Plasma {
    fn new() -> Self {
        Self { index: 0.0 }
    }
    fn length(x: f32, y: f32) -> f32 {
        (x * x + y * y).sqrt().max(1e-6)
    }    
    fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
    fn fragment(&self, sx: f32, sy: f32) -> Color {
        let px = sx * 4.1 - 1.0;
        let py = sy * 4.1 - 1.0;
        let mut ix = px;
        let mut iy = py;

        let mut t = self.index * -0.1;
        let (nix, niy) = (px + (t - ix).cos() + (t + iy).sin(), py + (t - iy).sin() + (t + ix).cos());
        ix = nix;
        iy = niy;
        let mut c = 0.15 + 1.0 / Self::length(px / ((ix + t).sin() / 0.05), py / ((iy + t).cos() / 0.05));

        t = self.index * 0.45;
        let (nix, niy) = (px + (t - ix).cos() + (t + iy).sin(), py + (t - iy).sin() + (t + ix).cos());
        ix = nix;
        iy = niy;
        c += 1.0 / Self::length(px / ((ix + t).sin() / 0.05), py / ((iy + t).cos() / 0.05));
        c = 1.5 - (c / 2.0).sqrt();

        let gray = 0.6 * 0.4 * 1.1 * c;
        Self::rgb((gray + 0.1).clamp(0.0, 1.0), (gray + 0.1).clamp(0.0, 1.0), (gray + 0.2).clamp(0.0, 1.0))
    }
}

impl FrameApp for Plasma {
    fn on_update(&mut self, ticks: u64) {
        self.index = ticks as f32;
    }

    fn on_paint(&self, surface: &mut Surface) {
        let size = surface.size();
        if size.width == 0 || size.height == 0 {
            return;
        }
        let w = size.width as f32;
        let h = (size.height * 2) as f32;

        for y in 0..size.height {
            for x in 0..size.width {
                let sx = (x as f32 + 0.5) / w;
                let top = self.fragment(sx, 1.0 - ((y * 2) as f32 + 0.5) / h);
                let bottom = self.fragment(sx, 1.0 - ((y * 2 + 1) as f32 + 0.5) / h);
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
        App::frame_app(Plasma::new())
            .fps(10)
            .title("Plasma")
            .clear_char(None)
            .color_schema(false)
            .backend(appcui::backend::Type::WindowsVT)
            .run()
    }
    #[cfg(not(target_os = "windows"))]
    {
        App::frame_app(Plasma::new())
            .fps(10)
            .title("Plasma")
            .clear_char(None)
            .color_schema(false)
            .run()
    }
}
