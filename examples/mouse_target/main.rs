use appcui::prelude::*;

struct MouseTarget {
    pos: Point,
    tracking: bool,
}

impl MouseTarget {
    fn new() -> Self {
        Self {
            pos: Point::ORIGIN,
            tracking: false,
        }
    }

    fn set_pos(&mut self, x: i32, y: i32) -> EventProcessStatus {
        self.tracking = true;
        if self.pos.x == x && self.pos.y == y {
            EventProcessStatus::Ignored
        } else {
            self.pos = Point::new(x, y);
            EventProcessStatus::Processed
        }
    }
}

impl InputApp for MouseTarget {
    fn on_resize(&mut self, new_size: Size) {
        if !self.tracking {
            self.pos = Point::new((new_size.width / 2) as i32, (new_size.height / 2) as i32);
        }
    }

    fn on_mouse_event(&mut self, ev: &MouseEvent) -> EventProcessStatus {
        match ev {
            MouseEvent::Over(p) => self.set_pos(p.x, p.y),
            MouseEvent::Pressed(d) | MouseEvent::Released(d) | MouseEvent::Drag(d) => self.set_pos(d.x, d.y),
            _ => EventProcessStatus::Ignored,
        }
    }

    fn on_paint(&self, surface: &mut Surface) {
        let size = surface.size();
        if size.width == 0 || size.height == 0 {
            return;
        }

        let x = self.pos.x;
        let y = self.pos.y;
        let right = size.width.saturating_sub(1) as i32;
        let bottom = size.height.saturating_sub(1) as i32;
        let cross = charattr!("gray");

        surface.draw_horizontal_line(0, y, right, LineType::Single, cross);
        surface.draw_vertical_line(x, 0, bottom, LineType::Single, cross);

        for (width, height, attr) in [
            (21u16, 11u16, charattr!("white")),
            (15, 7, charattr!("aqua")),
            (9, 5, charattr!("yellow")),
            (5, 3, charattr!("red")),
        ] {
            surface.draw_rect(
                Rect::with_alignment(x, y, width, height, RectAlignment::Center),
                LineType::SingleRound,
                attr,
            );
        }

        surface.fill_rect(
            Rect::with_alignment(x, y, 3, 1, RectAlignment::Center),
            Character::new(' ', Color::Black, Color::Red, CharFlags::None),
        );
        surface.write_char(
            x,
            y,
            Character::new(SpecialChar::CircleFilled, Color::White, Color::Red, CharFlags::None),
        );

        let hud = format!("Target: ({}, {})   Esc to exit", x, y);
        surface.write_string(1, 0, &hud, charattr!("aqua"), false);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::input_app(MouseTarget::new()).title("Mouse Target").run()
}
