use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint)]
pub struct ParentControl {
    error_message: String,
    show_child: bool,
}

impl ParentControl {
    pub fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::new(layout, true),
            error_message: String::new(),
            show_child: true,
        }
    }

    pub fn set_error_message(&mut self, message: String) {
        self.error_message = message;
    }

    pub fn clear_error(&mut self) {
        self.error_message.clear();
    }

    pub fn hide_child(&mut self) {
        self.show_child = false;
    }

    pub fn show_child(&mut self) {
        self.show_child = true;
    }
}

impl OnPaint for ParentControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        let size = self.client_size();
        let width = size.width as i32;
        let height = size.height as i32;

        // Clear with black background
        surface.clear(Character::new(' ', Color::Black, Color::Black, CharFlags::None));

        // Draw top ruler (horizontal)
        for x in 0..width {
            if x % 5 == 0 {
                // Draw tick mark
                surface.write_char(
                    x, 0,
                    Character::new('|', Color::Gray, Color::Black, CharFlags::None)
                );
                // Draw number every 10 units
                if x % 10 == 0 && x > 0 {
                    let num_str = x.to_string();
                    for (i, ch) in num_str.chars().enumerate() {
                        if x + (i as i32) < width {
                            surface.write_char(
                                x + i as i32, 1,
                                Character::new(ch, Color::Gray, Color::Black, CharFlags::None)
                            );
                        }
                    }
                }
            } else {
                // Draw ruler line
                surface.write_char(
                    x, 0,
                    Character::new('-', Color::Gray, Color::Black, CharFlags::None)
                );
            }
        }

        // Draw left ruler (vertical)
        for y in 0..height {
            if y % 5 == 0 && y > 0 {
                // Draw tick mark
                surface.write_char(
                    0, y,
                    Character::new('-', Color::Gray, Color::Black, CharFlags::None)
                );
                // Draw number every 10 units
                if y % 10 == 0 {
                    let num_str = y.to_string();
                    for (i, ch) in num_str.chars().enumerate() {
                        if i + 1 < width as usize {
                            surface.write_char(
                                1 + i as i32, y,
                                Character::new(ch, Color::Gray, Color::Black, CharFlags::None)
                            );
                        }
                    }
                }
            } else if y > 0 {
                // Draw ruler line
                surface.write_char(
                    0, y,
                    Character::new('|', Color::Gray, Color::Black, CharFlags::None)
                );
            }
        }

        // Draw grid lines every 5 units (faint)
        for x in (5..width).step_by(5) {
            for y in 3..height {
                surface.write_char(
                    x, y,
                    Character::new('·', Color::Gray, Color::Black, CharFlags::None)
                );
            }
        }

        for y in (5..height).step_by(5) {
            for x in 3..width {
                surface.write_char(
                    x, y,
                    Character::new('·', Color::Gray, Color::Black, CharFlags::None)
                );
            }
        }

        // Display error message if there is one
        if !self.error_message.is_empty() {
            let lines: Vec<&str> = self.error_message.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                let y = height / 2 + i as i32 - lines.len() as i32 / 2;
                if y >= 0 && y < height {
                    let x_start = std::cmp::max(5, (width - line.len() as i32) / 2);
                    for (j, ch) in line.chars().enumerate() {
                        if x_start + (j as i32) < width {
                            surface.write_char(
                                x_start + j as i32, y,
                                Character::new(ch, Color::Red, Color::Black, CharFlags::None)
                            );
                        }
                    }
                }
            }
        }
    }
} 