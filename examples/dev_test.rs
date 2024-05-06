use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint)]
struct MyControl {}
impl MyControl {
    fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::new(layout, true),
        }
    }
}

impl OnPaint for MyControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(char!("'X',Yellow,DarkRed"));
        let size = self.size();
        surface.draw_rect(
            Rect::with_point_and_size(Point::ORIGIN, size),
            LineType::Double,
            CharAttribute::with_fore_color(Color::White),
        );
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("caption:'Custom Control',d:c,w:30,h:10");
    w.add(MyControl::new(Layout::new("l:1,t:1,r:1,b:1")));
    a.add_window(w);
    a.run();
    Ok(())
}
