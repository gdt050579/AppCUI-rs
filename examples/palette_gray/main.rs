use appcui::backend::{ColorPalette, Palette};
use appcui::prelude::*;

/// Shows the 16 named colors, each one as a swatch with its own name written inside.
#[CustomControl(overwrite=OnPaint)]
struct Swatches {}
impl Swatches {
    fn new() -> Self {
        Self {
            base: ControlBase::new(layout!("l:1,t:1,r:1,h:8"), false),
        }
    }
}
impl OnPaint for Swatches {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        // first 8 colors on the left column, last 8 on the right column
        for i in 0..16 {
            let color = Color::from_value(i).unwrap();
            let x = (i / 8) * 22;
            let y = i % 8;
            let attr = CharAttribute::with_color(color.contrast_color(), color);
            surface.fill_horizontal_line_with_size(x, y, 20, Character::with_attributes(' ', attr));
            surface.write_string(x + 1, y, color.name(), attr, false);
        }
    }
}

#[Window(events = ButtonEvents)]
struct MyWin {
    close: Handle<Button>,
}
impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: Window::new("Gray palette", layout!("a:c,w:50,h:22"), window::Flags::None),
            close: Handle::None,
        };
        w.add(Swatches::new());

        let mut p = panel!("'Some controls',l:1,t:9,r:1,h:9");
        p.add(label!("'Name:',x:1,y:1,w:6"));
        p.add(textfield!("'Gray palette',x:8,y:1,w:23"));
        p.add(checkbox!("'Checkbox &selected',x:1,y:3,w:23,checked:true"));
        p.add(radiobox!("'RadioBox',x:26,y:3,w:15,selected:true"));
        p.add(button!("'&Button',x:1,y:5,w:14"));
        p.add(button!("'Disabled',x:17,y:5,w:14,enabled:false"));
        w.add(p);

        w.close = w.add(button!("'&Close',r:1,b:0,w:12"));
        w
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.close {
            self.close();
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}

/// A grayscale palette. Each named color is a pure gray (`R = G = B`) with its own
/// brightness, so the default theme stays readable while every slot is gray.
fn custom_palette() -> ColorPalette {
    ColorPalette::from_pairs(&[
        (Color::Black, 28, 28, 28),
        (Color::DarkBlue, 52, 52, 52),
        (Color::DarkGreen, 68, 68, 68),
        (Color::Teal, 84, 84, 84),
        (Color::DarkRed, 96, 96, 96),
        (Color::Magenta, 112, 112, 112),
        (Color::Olive, 128, 128, 128),
        (Color::Silver, 210, 210, 210),
        (Color::Gray, 140, 140, 140),
        (Color::Blue, 156, 156, 156),
        (Color::Green, 196, 196, 196),
        (Color::Aqua, 224, 224, 224),
        (Color::Red, 168, 168, 168),
        (Color::Pink, 184, 184, 184),
        (Color::Yellow, 236, 236, 236),
        (Color::White, 248, 248, 248),
    ])
}


fn main() -> Result<(), appcui::system::Error> {
    App::new()
        //.backend(appcui::backend::Type::CrossTerm)
        .palette(Palette::Custom(custom_palette()))
        .window(MyWin::new)
        .run()
}
