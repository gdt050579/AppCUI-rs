use appcui::prelude::*;

#[ModalWindow(events: ButtonEvents)]
pub struct ShowTabModal {}
impl ShowTabModal {
    pub fn new(tab_type: tab::Type, tab_flags: tab::Flags, tab_width: u8) -> Self {
        let mut w = Self {
            base: ModalWindow::new("Example", layout!("a:c,w:70,h:10"), window::Flags::None),
        };
        let mut t = Tab::with_type(layout!("l:1,t:1,r:1,b:3"), tab_flags, tab_type);
        t.set_tab_width(tab_width);
        t.add_tab("&First");
        t.add_tab("&Second");
        t.add_tab("&Third");
        w.add(t);
        w.add(button!("Close,l:25,b:0,w:20"));
        w
    }
}
impl ButtonEvents for ShowTabModal {
    fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
        self.exit();
        EventProcessStatus::Processed
    }
}