use std::arch::x86_64;

use appcui::prelude::*;

#[CustomContainer(overwrite = OnPaint)]
pub struct ParentControl {
    error_message: String,
    show_child: bool,
}

impl ParentControl {
    pub fn new(layout: Layout) -> Self {
        let mut me = Self {
            base: ContainerBase::new(layout, true),
            error_message: String::new(),
            show_child: true,
        };
        me
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
    pub fn paint_horizontal_rule(&self, surface: &mut Surface) {
        let w = self.size().width as i32;
        let mut pos = 0;
        let attr = charattr!("black,gray");
        surface.draw_horizontal_line(4, 1, w, LineType::Single, attr);
        while pos < w {
            let x = pos + 4;
            if pos < 10 {
                surface.write_char(x, 0, Character::with_attributes(((pos + 48) as u8) as char, attr));
            } else if pos < 100 {
                surface.write_char(x, 0, Character::with_attributes(((pos / 10 + 48) as u8) as char, attr));
                surface.write_char(x + 1, 0, Character::with_attributes(((pos % 10 + 48) as u8) as char, attr));
            } else {
                surface.write_char(x - 1, 0, Character::with_attributes(((pos / 100 + 48) as u8) as char, attr));
                surface.write_char(x, 0, Character::with_attributes((((pos % 100) / 10 + 48) as u8) as char, attr));
                surface.write_char(x + 1, 0, Character::with_attributes(((pos % 10 + 48) as u8) as char, attr));
            }
            surface.write_char(x, 1, Character::with_attributes('┬', attr));
            pos += 5;
        }
    }
    pub fn paint_vertical_rule(&self, surface: &mut Surface) {
        let h = self.size().height as i32;
        let mut pos = 0;
        let attr = charattr!("black,gray");
        surface.draw_vertical_line(3, 2, h, LineType::Single, attr);
        while pos < h {
            let y = pos + 2;
            if pos < 10 {
                surface.write_char(2, y, Character::with_attributes(((pos + 48) as u8) as char, attr));
            } else if pos < 100 {
                surface.write_char(1, y, Character::with_attributes(((pos / 10 + 48) as u8) as char, attr));
                surface.write_char(2, y, Character::with_attributes(((pos % 10 + 48) as u8) as char, attr));
            } else {
                surface.write_char(0, y, Character::with_attributes(((pos / 100 + 48) as u8) as char, attr));
                surface.write_char(1, y, Character::with_attributes((((pos % 100) / 10 + 48) as u8) as char, attr));
                surface.write_char(2, y, Character::with_attributes(((pos % 10 + 48) as u8) as char, attr));
            }
            surface.write_char(3, y, Character::with_attributes('├', attr));
            pos += 5;
        }
    }
    pub fn paint_grid_lines(&self, surface: &mut Surface) {
        let w = self.size().width as i32;
        let h = self.size().height as i32;
        let attr = charattr!("gray,black");
        let mut pos = 0;
        while pos < w {
            surface.draw_vertical_line(pos + 4, 3, h, LineType::Single, attr);
            pos += 5;
        }
        pos = 0;
        while pos < h {
            surface.draw_horizontal_line(5, pos + 2, w, LineType::Single, attr);
            pos += 5;
        }
        let mut y = 0;
        let ch = Character::new(SpecialChar::BoxCrossSingleLine, Color::Gray, Color::Black, CharFlags::None);
        while y < h {
            let mut x = 0;
            while x < w {
                surface.write_char(x + 4, y + 2, ch);
                x += 5;
            }
            y += 5;
        }
    }
    fn paint_error_message(&self, surface: &mut Surface) {
        let sz = self.size();
        let format = TextFormatBuilder::new()
            .align(TextAlignment::Center)
            .wrap_type(WrapType::WordWrap((sz.width as u16).saturating_sub(4)))
            .position((sz.width as i32) / 2 + 2, (sz.height as i32) / 2 + 1)
            .attribute(CharAttribute::with_color(Color::Red, Color::Black))
            .build();
        surface.write_text(&self.error_message, &format);
    }
}

impl OnPaint for ParentControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        let size = self.client_size();
        let width = size.width as i32;
        let height = size.height as i32;

        surface.clear(Character::new(' ', Color::Black, Color::Black, CharFlags::None));
        surface.fill_rect(Rect::new(0, 0, width, 1), char!("' ',black,gray"));
        surface.fill_rect(Rect::new(0, 0, 3, height), char!("' ',black,gray"));

        self.paint_horizontal_rule(surface);
        self.paint_vertical_rule(surface);
        surface.write_char(
            3,
            1,
            Character::new(SpecialChar::BoxTopLeftCornerSingleLine, Color::Black, Color::Gray, CharFlags::None),
        );

        if self.error_message.is_empty() {
            self.paint_grid_lines(surface);
        } else {
            self.paint_error_message(surface);
        }
    }
}
