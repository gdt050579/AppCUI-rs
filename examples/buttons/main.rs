use appcui::prelude::*;

#[Window(events = ButtonEvents)]
struct MyWin {
    b1: Handle<Button>,
    b2: Handle<Button>,
    b3: Handle<Button>,
    b4: Handle<Button>,
    lb: Handle<Label>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: Window::new("Buttons example", layout!("a:c,w:60,h:7"), window::Flags::None),
            //..Default::default()
            b1: Handle::None,
            b2: Handle::None,
            b3: Handle::None,
            b4: Handle::None,
            lb: Handle::None,
        };
        win.b1 = win.add(button!("&Button,x:2,y:2,w:15"));
        win.b2 = win.add(button!("'&Inactive',x:19,y:2,w:15,enabled:false"));
        win.b3 = win.add(button!("&Flat,x:2,y:4,w:32,type:flat"));
        win.b4 = win.add(button!("&Raised,x:36,y:2,w:20,type:raised"));
        win.lb = win.add(Label::new("",layout!("x:2,y:0,w:35")));
        win
    }
    fn set_label_text(&mut self, txt: &str) {
        let h = self.lb;
        if let Some(label) = self.control_mut(h) {
            label.set_caption(txt);
        }
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, button_handle: Handle<Button>) -> EventProcessStatus {
        if (button_handle == self.b1) || (button_handle == self.b2) || (button_handle == self.b3) {
            let button = self.control(button_handle).unwrap();
            let new_text = format!("Button ('{}') was pressed !", button.caption());
            self.set_label_text(new_text.as_str());
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}
    
fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    let mut app = App::with_backend(appcui::backend::Type::WindowsVT).color_schema(false).build()?;
    #[cfg(not(target_os = "windows"))]
    let mut app = App::new().color_schema(false).build()?;
    
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
