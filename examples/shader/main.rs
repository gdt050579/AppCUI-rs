use appcui::prelude::*;

const FPS: u32 = 30;
const PI: f32 = std::f32::consts::PI;
const DARK: (f32, f32, f32) = (0.071, 0.071, 0.071); // rgb(18, 18, 18)
const PURPLE: (f32, f32, f32) = (0.561, 0.380, 0.682); // rgb(143, 96, 173)

#[derive(Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
    fn scale(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s)
    }
    fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }
    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

struct Shader {
    ticks: u64,
    size: Size,
    mouse: Vec2,
}

impl Shader {
    fn new() -> Self {
        Self {
            ticks: 0,
            size: Size::new(1, 1),
            mouse: Vec2::new(0.0, 0.0),
        }
    }

    fn set_mouse(&mut self, x: i32, y: i32) {
        let w = self.size.width.max(1) as f32;
        let h = self.size.height.max(1) as f32;
        self.mouse = Vec2::new(x as f32 / w, 1.0 - y as f32 / h);
    }

    fn fragment(&self, px: u32, py: u32, res: Vec2, aspect: f32, alpha: f32) -> Color {
        let uv = Vec2::new((px as f32 + 0.5) / res.x, 1.0 - (py as f32 + 0.5) / res.y);
        let mut st = Vec2::new(uv.x * aspect, uv.y);
        st = rot(st, -PI / 4.0);

        let n = psrdnoise(st.scale(1.5), alpha);
        let lines = ((st.x + n * 0.12 + self.mouse.x + 0.2) * PI).cos();
        let t = bounce_out(lines * 0.5 + 0.5);

        rgb(mix(DARK.0, PURPLE.0, t), mix(DARK.1, PURPLE.1, t), mix(DARK.2, PURPLE.2, t))
    }
}

impl FrameApp for Shader {
    fn on_resize(&mut self, new_size: Size) {
        self.size = new_size;
    }

    fn on_update(&mut self, ticks: u64) {
        self.ticks = ticks;
    }

    fn on_mouse_event(&mut self, ev: &MouseEvent) {
        match ev {
            MouseEvent::Over(p) => self.set_mouse(p.x, p.y),
            MouseEvent::Pressed(d) | MouseEvent::Released(d) | MouseEvent::Drag(d) => self.set_mouse(d.x, d.y),
            _ => {}
        }
    }

    fn on_paint(&self, surface: &mut Surface) {
        let size = surface.size();
        if size.width == 0 || size.height == 0 {
            return;
        }
        let res = Vec2::new(size.width as f32, (size.height * 2) as f32);
        let aspect = res.x / res.y;
        let time = self.ticks as f32 / FPS as f32;
        let alpha = 1.2 * time + self.mouse.y * PI;

        for y in 0..size.height {
            for x in 0..size.width {
                let top = self.fragment(x, y * 2, res, aspect, alpha);
                let bottom = self.fragment(x, y * 2 + 1, res, aspect, alpha);
                surface.write_char(
                    x as i32,
                    y as i32,
                    Character::new(SpecialChar::BlockUpperHalf, top, bottom, CharFlags::None),
                );
            }
        }
    }
}

fn rot(v: Vec2, a: f32) -> Vec2 {
    let (c, s) = (a.cos(), a.sin());
    Vec2::new(c * v.x - s * v.y, s * v.x + c * v.y)
}

fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::from_rgb(
        (r * 255.0).clamp(0.0, 255.0) as u8,
        (g * 255.0).clamp(0.0, 255.0) as u8,
        (b * 255.0).clamp(0.0, 255.0) as u8,
    )
}

fn glsl_mod(x: f32, y: f32) -> f32 {
    x - y * (x / y).floor()
}

/// Periodic 2D simplex noise with rotating gradients (psrdnoise, period = 0).
/// Port of https://github.com/stegu/psrdnoise — MIT License.
fn psrdnoise(x: Vec2, alpha: f32) -> f32 {
    let uv = Vec2::new(x.x + x.y * 0.5, x.y);
    let i0 = uv.floor();
    let f0 = uv.sub(i0);
    let cmp = if f0.x >= f0.y { 1.0 } else { 0.0 };
    let o1 = Vec2::new(cmp, 1.0 - cmp);
    let i1 = i0.add(o1);
    let i2 = i0.add(Vec2::new(1.0, 1.0));

    let v0 = Vec2::new(i0.x - i0.y * 0.5, i0.y);
    let v1 = Vec2::new(v0.x + o1.x - o1.y * 0.5, v0.y + o1.y);
    let v2 = Vec2::new(v0.x + 0.5, v0.y + 1.0);

    let mut n = 0.0;
    for (i, xi) in [(i0, x.sub(v0)), (i1, x.sub(v1)), (i2, x.sub(v2))] {
        let mut hash = glsl_mod(i.x, 289.0);
        hash = glsl_mod((hash * 51.0 + 2.0) * hash + i.y, 289.0);
        hash = glsl_mod((hash * 34.0 + 10.0) * hash, 289.0);
        let psi = hash * 0.07482 + alpha;
        let g = Vec2::new(psi.cos(), psi.sin());
        let w = (0.8 - xi.dot(xi)).max(0.0);
        let w2 = w * w;
        n += w2 * w2 * g.dot(xi);
    }
    10.9 * n
}

fn bounce_out(t: f32) -> f32 {
    const A: f32 = 4.0 / 11.0;
    const B: f32 = 8.0 / 11.0;
    const C: f32 = 9.0 / 10.0;
    let t2 = t * t;
    if t < A {
        7.5625 * t2
    } else if t < B {
        9.075 * t2 - 9.9 * t + 3.4
    } else if t < C {
        (4356.0 / 361.0) * t2 - (35442.0 / 1805.0) * t + (16061.0 / 1805.0)
    } else {
        10.8 * t2 - 20.52 * t + 10.72
    }
}

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    {
        App::frame_app(Shader::new())
            .fps(FPS)
            .title("Shader")
            .clear_char(None)
            .color_schema(false)
            .backend(appcui::backend::Type::WindowsVT)
            .run()
    }
    #[cfg(not(target_os = "windows"))]
    {
        App::frame_app(Shader::new())
            .fps(FPS)
            .title("Shader")
            .background_char(None)
            .color_schema(false)
            .run()
    }
}
