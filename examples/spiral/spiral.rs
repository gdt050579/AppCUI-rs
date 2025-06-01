pub struct Spiral {
    pub points: Vec<(f64, f64)>,
    pub angle: f64,
    pub scale: f64,
    pub segments: u32,
}

impl Spiral {
    pub fn new(segments: u32) -> Self {
        Self {
            points: Vec::new(),
            angle: 0.0,
            scale: 1.0,
            segments,
        }
    }

    pub fn update(&mut self) {
        self.angle += 0.02;
        self.scale = 0.8 + 0.2 * (self.angle * 2.0).sin();
        self.points.clear();
        self.generate_points();
    }

    fn generate_points(&mut self) {
        let mut r = 0.0;
        let mut theta = self.angle;
        let r_step = 0.0625;
        let theta_step = 0.25;
        
        // Generate points for the spiral
        for _ in 0..self.segments {
            // Compensate for character aspect ratio (2:1)
            let x = r * theta.cos() * 2.0;
            let y = r * theta.sin(); 
            self.points.push((x, y));
            
            r += r_step * self.scale;
            theta += theta_step;
        }
    }
} 