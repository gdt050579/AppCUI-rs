use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint)]
pub struct ChildControl {
}

impl ChildControl {
    pub fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::new(layout, true),
        }
    }

    pub fn set_layout(&mut self, layout: Layout) {
        // For the layout tester, we'll recreate the control with the new layout
        // This is a simplified approach for demonstration purposes
        self.base = ControlBase::new(layout, true);
    }
}

impl OnPaint for ChildControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        let size = self.client_size();
        
        // Fill with red background
        surface.clear(Character::new(' ', Color::Black, Color::Red, CharFlags::None));
        
        // Add a border for better visibility
        let rect = Rect::with_size(0, 0, size.width as u16, size.height as u16);
        surface.draw_rect(
            rect,
            LineType::Single,
            CharAttribute::with_fore_color(Color::White),
        );
        
        // Display dimensions in the center
        let width_str = size.width.to_string();
        let height_str = size.height.to_string();
        let dimension_text = format!("{}x{}", width_str, height_str);
        
        let text_x = (size.width as i32 - dimension_text.len() as i32) / 2;
        let text_y = size.height as i32 / 2;
        
        if text_x >= 0 && text_y >= 0 && text_x < size.width as i32 && text_y < size.height as i32 {
            for (i, ch) in dimension_text.chars().enumerate() {
                let x = text_x + i as i32;
                if x < size.width as i32 {
                    surface.write_char(
                        x, text_y,
                        Character::new(ch, Color::White, Color::Red, CharFlags::None)
                    );
                }
            }
        }
    }
} 