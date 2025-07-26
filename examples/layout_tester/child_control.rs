use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint)]
pub struct ChildControl {}

impl ChildControl {
    pub fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::new(layout, true),
        }
    }
}

impl OnPaint for ChildControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        let size = self.client_size();
        surface.clear(Character::with_color(Color::Transparent, Color::DarkRed));
        let txt = format!("{}x{}", size.width, size.height);
        let len = txt.len() as i32;
        let px = (size.width as i32 - len) / 2;
        let py = size.height as i32 / 2;
        surface.write_string(px, py, &txt, charattr!("w,dr"), false);
    }
}
