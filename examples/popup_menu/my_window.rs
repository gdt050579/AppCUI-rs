use appcui::prelude::*;

#[Window(events: CheckBoxEvents)]
pub struct MyWindow {
    hc: Handle<super::MyCustomControl>,
    cb: Handle<CheckBox>,
}
impl MyWindow {
    pub fn new() -> Self {
        let mut w = MyWindow {
            base: Window::new("Test", layout!("a:c,w:76,h:10"), window::Flags::None),
            hc: Handle::None,
            cb: Handle::None,
        };
        w.hc = w.add(super::MyCustomControl::new(layout!("x:50%,y:6,p:c,w:16,h:4")));
        w.add(label!("'Press the right mouse button on the square below to show a popup menu',x:37,y:1,p:c,w:70,h:1"));
        w.cb = w.add(checkbox!("'&Limit the meniu size to 3 items',x:2,y:2,w:30,checked:false"));

        w
    }
}
impl CheckBoxEvents for MyWindow {
    fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
        if handle == self.cb {
            let h = self.hc;
            if let Some(obj) = self.control_mut(h) {
                obj.enable_small_menu(checked);
            }
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}