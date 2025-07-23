use appcui::prelude::*;


#[CustomControl(overwrite=OnPaint)]
struct ColoredText {
    fore: Color,
    back: Color
}
impl ColoredText {
    fn new()->Self {
        Self {
            base: ControlBase::new(layout!("l:1,y:6,r:1,h:3,p:c"),false),
            fore: Color::White,
            back: Color::Black
        }
    }
}
impl OnPaint for ColoredText {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(Character::new(' ',self.fore,self.back,CharFlags::None));
        surface.write_string(1, 1, "Custom control", CharAttribute::with_color(self.fore, self.back), false);
    }
}

#[Window(events = ColorPickerEvents)]
struct MyWin {
    fore: Handle<ColorPicker>,
    back: Handle<ColorPicker>,
    ct: Handle<ColoredText>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: Window::new("ColorPicker example", layout!("a:c,w:40,h:10"), window::Flags::None),
            fore: Handle::None,
            back: Handle::None,
            ct: Handle::None,
        };
        win.add(label!("'Foreground:',x:1,y:1,w:12,h:1"));
        win.fore = win.add(colorpicker!("White,x:13,y:1,w:24"));
        win.add(label!("'Background:',x:1,y:3,w:12,h:1"));
        win.back = win.add(colorpicker!("Black,x:13,y:3,w:24"));
        win.ct = win.add(ColoredText::new());
        win
    }
}

impl ColorPickerEvents for MyWin {
    fn on_color_changed(&mut self, handle: Handle<ColorPicker>, color: Color) -> EventProcessStatus {
        let h = self.ct;
        let f = self.fore;
        let b = self.back;
        if let Some(coloredtext) = self.control_mut(h) {
            match () {
                _ if handle==f => coloredtext.fore = color,
                _ if handle==b => coloredtext.back = color,
                _ => {}
            };
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
