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
            base: Window::new("Palette test", layout!("a:c,w:50,h:22"), window::Flags::None),
            close: Handle::None,
        };
        w.add(Swatches::new());

        let mut p = panel!("'Some controls',l:1,t:9,r:1,h:9");
        p.add(label!("'Name:',x:1,y:1,w:6"));
        p.add(textfield!("'Palette test',x:8,y:1,w:23"));
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

/// A custom palette (dark purple/blue tones) that is clearly different from the AppCUI colors.
fn custom_palette() -> ColorPalette {
    ColorPalette::from_pairs(&[
        (Color::Black, 40, 42, 54),
        (Color::DarkBlue, 60, 80, 170),
        (Color::DarkGreen, 35, 134, 54),
        (Color::Teal, 21, 128, 140),
        (Color::DarkRed, 170, 50, 60),
        (Color::Magenta, 140, 70, 160),
        (Color::Olive, 170, 150, 50),
        (Color::Silver, 188, 190, 200),
        (Color::Gray, 108, 112, 130),
        (Color::Blue, 100, 140, 255),
        (Color::Green, 80, 250, 123),
        (Color::Aqua, 139, 233, 253),
        (Color::Red, 255, 85, 85),
        (Color::Pink, 255, 121, 198),
        (Color::Yellow, 241, 250, 140),
        (Color::White, 248, 248, 242),
    ])
}


fn main() -> Result<(), appcui::system::Error> {
    #[cfg(not(windows))]
    return App::new()
        .palette(Palette::Custom(custom_palette()))
        .window(MyWin::new)
        .run();
    
    #[cfg(windows)]
    return App::new()
        .backend(appcui::backend::Type::WindowsVT)
        .palette(Palette::Custom(custom_palette()))
        .window(MyWin::new)
        .run();
}
