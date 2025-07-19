use appcui::prelude::*;

#[Window(events = ButtonEvents+WindowEvents)]
struct MyWindow {
    info: Handle<Label>,
    number: Handle<TextField>,
}
impl MyWindow {
    fn new() -> Self {
        let mut w = Self {
            base: window!("title:'Square root',a:c"),
            info: Handle::None,
            number: Handle::None,
        };
        w.number = w.add(textfield!("l:1,t:1,r:1,h:1"));
        w.info = w.add(label!("'',l:1,t:3,r:1,h:1"));
        w.add(button!("Compute,x:50%,y:100%,w:20,p:b"));

        w
    }
}
impl ButtonEvents for MyWindow {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        let value: Option<f64> = if let Some(txt) = self.control(self.number) {
            txt.text().parse().ok()
        } else {
            None
        };
        let h = self.info;
        if let (Some(v), Some(i)) = (value, self.control_mut(h)) {
            i.set_caption(format!("SQRT({})={}",v,v.sqrt()).as_str());
        }
        EventProcessStatus::Processed
    }
}
impl WindowEvents for MyWindow {
    fn on_cancel(&mut self) -> ActionRequest {
        if dialogs::validate("Close", "Do you want to close the application ?") {
            ActionRequest::Allow
        } else {
            ActionRequest::Deny
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(40, 10)).single_window().build()?;
    a.add_window(MyWindow::new());
    a.run();
    Ok(())
}
