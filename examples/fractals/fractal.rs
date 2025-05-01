use std::f64::consts::PI;

pub struct Fractal {
    pub points: Vec<(f64, f64)>,
    pub angle: f64,
    pub scale: f64,
    pub depth: u32,
    pub max_depth: u32,
}

impl Fractal {
    pub fn new(max_depth: u32) -> Self {
        Self {
            points: Vec::new(),
            angle: 0.0,
            scale: 1.0,
            depth: 0,
            max_depth,
        }
    }

    pub fn update(&mut self) {
        self.angle += 0.02;
        self.scale = 0.8 + 0.2 * (self.angle * 2.0).sin();
        self.points.clear();
        self.generate_points(0.0, 0.0, 1.0, 0.0);
    }

    fn generate_points(&mut self, x: f64, y: f64, length: f64, angle: f64) {
        if self.depth >= self.max_depth {
            return;
        }

        let end_x = x + length * angle.cos();
        let end_y = y + length * angle.sin();
        
        self.points.push((x, y));
        self.points.push((end_x, end_y));

        self.depth += 1;
        
        // Generate branches
        let branch_angle = PI / 4.0;
        let new_length = length * self.scale;
        
        self.generate_points(end_x, end_y, new_length, angle + branch_angle);
        self.generate_points(end_x, end_y, new_length, angle - branch_angle);
        
        self.depth -= 1;
    }
} 