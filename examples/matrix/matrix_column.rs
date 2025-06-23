use rand::Rng;

pub struct MatrixColumn {
    pub chars: Vec<char>,
    pub positions: Vec<i32>,
    pub speeds: Vec<i32>,
    pub active: Vec<bool>,
    pub head_pos: i32,
    pub column_pos: i32,
    pub length: usize,
}

impl MatrixColumn {
    const CHAR_SET: &'static str = "01lI|!-_+=:;.'`01010101";
    pub fn new(column_pos: i32, max_height: i32) -> Self {
        let mut rng = rand::thread_rng();
        // Increase the length range for more variation
        let length = rng.gen_range(5..20);
        // Stagger the initial positions more for a less uniform look
        let head_pos = -rng.gen_range(1..max_height * 2);

        let mut chars = Vec::with_capacity(length);
        let mut positions = Vec::with_capacity(length);
        let mut speeds = Vec::with_capacity(length);
        let mut active = Vec::with_capacity(length);

        // Initialize column with random characters
        for i in 0..length {
            chars.push(Self::random_matrix_char());
            positions.push(head_pos - (i as i32));
            // Add some speed variation to create a more dynamic effect
            speeds.push(if i == 0 { rng.gen_range(1..3) } else { 1 });
            active.push(i == 0); // Only the head is initially active
        }

        Self {
            chars,
            positions,
            speeds,
            active,
            head_pos,
            column_pos,
            length,
        }
    }

    pub fn random_matrix_char() -> char {
        let mut rng = rand::thread_rng();
        let count = MatrixColumn::CHAR_SET.chars().count();
        MatrixColumn::CHAR_SET.chars().nth(rng.gen_range(0..count)).unwrap_or('0')
    }

    pub fn update(&mut self, max_height: i32) {
        let mut rng = rand::thread_rng();
        
        // Update head position
        self.head_pos += self.speeds[0];
        self.positions[0] = self.head_pos;
        
        // Increase character change frequency for head (20% instead of 10%)
        if rng.gen_bool(0.2) {
            self.chars[0] = Self::random_matrix_char();
        }
        
        // Activate next character in trail once head moves down enough
        for i in 1..self.length {
            if self.active[i-1] && (self.positions[i-1] - self.positions[i]) >= 1 {
                self.active[i] = true;
            }
            
            if self.active[i] {
                self.positions[i] += self.speeds[i];
            }
            
            // Increase character change frequency in trail (10% instead of 5%)
            if self.active[i] && rng.gen_bool(0.1) {
                self.chars[i] = Self::random_matrix_char();
            }
        }
        
        // Reset column if it's completely off-screen
        if self.positions[self.length - 1] > max_height {
            *self = MatrixColumn::new(self.column_pos, max_height);
        }
    }
} 