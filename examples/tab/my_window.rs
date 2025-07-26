use appcui::prelude::*;

#[Window(events: RadioBoxEvents+CheckBoxEvents+ButtonEvents)]
pub struct MyWindow {
    run: Handle<Button>,
    exit: Handle<Button>,
    ontop: Handle<RadioBox>,
    onbottom: Handle<RadioBox>,
    onleft: Handle<RadioBox>,
    hidden: Handle<RadioBox>,
    trnsback: Handle<CheckBox>,
    tabsbar: Handle<CheckBox>,
    t_type: tab::Type,
    t_flags: tab::Flags,
    tw: u8,
}
impl MyWindow {
    pub fn new() -> Self {
        let mut w = Self {
            base: window!("'Tab Example',a:c,w:40,h:20"),
            run: Handle::None,
            exit: Handle::None,
            ontop: Handle::None,
            onbottom: Handle::None,
            onleft: Handle::None,
            hidden: Handle::None,
            trnsback: Handle::None,
            tabsbar: Handle::None,
            t_type: tab::Type::OnTop,
            t_flags: tab::Flags::None,
            tw: 0,
        };

        let mut t = tab!("tabs: [Type,Flags,Sizes],tw:9,l:1,t:1,r:1,b:3");
        w.ontop = t.add(
            0,
            radiobox!("'OnTop (tabs are located on top of the control)',x:1,y:1,w:35,h:2,select:true"),
        );
        w.onbottom = t.add(
            0,
            radiobox!("'OnBottom (tabs are located on the bottom of the control)',x:1,y:3,w:35,h:2"),
        );
        w.onleft = t.add(
            0,
            radiobox!("'OnLeft (tabs are located on the left side of the control)',x:1,y:5,w:35,h:2"),
        );
        w.hidden = t.add(0, radiobox!("'No tabs will be visible',x:1,y:7,w:35,h:2"));

        w.trnsback = t.add(1, checkbox!("'Transparent background',x:1,y:1,w:30"));
        w.tabsbar = t.add(1, checkbox!("'Show tabs bar',x:1,y:2,w:30"));

        t.add(2,radiobox!("3,x:1,y:1,w:30"));
        t.add(2,radiobox!("7,x:1,y:2,w:30"));
        t.add(2,radiobox!("10,x:1,y:3,w:30,select:true"));
        t.add(2,radiobox!("15,x:1,y:4,w:30"));
        t.add(2,radiobox!("20,x:1,y:5,w:30"));
        t.add(2,radiobox!("25,x:1,y:6,w:30"));

        w.add(t);
        w.run = w.add(button!("'Run example',l:1,b:0,w:18"));
        w.exit = w.add(button!("Close,l:20,b:0,w:18"));
        w
    }
}
impl RadioBoxEvents for MyWindow {
    fn on_selected(&mut self, handle: Handle<RadioBox>) -> EventProcessStatus {
        match () {
            _ if handle == self.ontop => self.t_type = tab::Type::OnTop,
            _ if handle == self.onbottom => self.t_type = tab::Type::OnBottom,
            _ if handle == self.onleft => self.t_type = tab::Type::OnLeft,
            _ if handle == self.hidden => self.t_type = tab::Type::HiddenTabs,
            _ => {
                // its a size
                if let Some(rb) = self.control(handle) {
                    self.tw = rb.caption().parse().unwrap();
                }
            }
        }
        EventProcessStatus::Processed
    }
}
impl CheckBoxEvents for MyWindow {
    fn on_status_changed(&mut self, handle: Handle<CheckBox>, checked: bool) -> EventProcessStatus {
        if handle == self.trnsback {
            if checked {
                self.t_flags |= tab::Flags::TransparentBackground
            } else {
                self.t_flags.remove(tab::Flags::TransparentBackground);
            }
        }
        if handle == self.tabsbar {
            if checked {
                self.t_flags |= tab::Flags::TabsBar
            } else {
                self.t_flags.remove(tab::Flags::TabsBar);
            }
        }
        EventProcessStatus::Processed
    }
}
impl ButtonEvents for MyWindow {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.exit {
            self.close();
            return EventProcessStatus::Processed;
        }
        if handle == self.run {
            // show a tab with those specifications
            super::ShowTabModal::new(self.t_type, self.t_flags, self.tw).show();
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}