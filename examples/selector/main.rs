use appcui::prelude::*;

mod shape;
use shape::Shape;

#[Window(events = SelectorEvents<Shape>)]
struct MyWin {
    h_img: Handle<ImageViewer>,
}
impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("a:c,w:30,h:12,caption:Win"),
            h_img: Handle::None,
        };
        let mut img = ImageViewer::new(
            Shape::Square.image(),
            layout!("x:1,y:3,w:26,h:7"),
            image::RenderOptionsBuilder::new().character_set(CharacterSet::SmallBlocks).build(),
            imageviewer::Flags::None,
        );
        img.set_enabled(false);
        img.set_backgound(Character::new(' ', Color::Black, Color::Black, CharFlags::None));
        w.h_img = w.add(img);
        w.add(selector!("enum: Shape,value:Square,x:1,y:1,w:26"));
        w
    }
}
impl SelectorEvents<Shape> for MyWin {
    fn on_selection_changed(&mut self, _: Handle<Selector<Shape>>, value: Option<Shape>) -> EventProcessStatus {
        let value = value.unwrap();
        let h = self.h_img;
        if let Some(img) = self.control_mut(h) {
            img.set_image(value.image());
        }
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
