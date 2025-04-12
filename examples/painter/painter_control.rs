use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint + OnMouseEvent)]
pub struct PainterControl {
    drawing_char: Character,
    surface: Surface,
}

impl PainterControl {
    pub fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::new(layout, true),
            drawing_char: Character::with_char('█'),
            surface: Surface::new(100, 100),
        }
    }

    pub fn set_drawing_char(&mut self, ch: char) {
        self.drawing_char.code = ch;
    }

    pub fn set_foreground_color(&mut self, color: Color) {
        self.drawing_char.foreground = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.drawing_char.background = color;
    }

    pub fn clear_surface(&mut self) {
        self.surface.clear(char!("' ',black,black"));
    }
}

impl OnPaint for PainterControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.draw_surface(0, 0, &self.surface);
    }
}

impl OnMouseEvent for PainterControl {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Drag(mouse_event_data) => {
                if mouse_event_data.button == MouseButton::Left {
                    self.surface.write_char(mouse_event_data.x, mouse_event_data.y, self.drawing_char);
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            _ => EventProcessStatus::Ignored
        }
    }
}