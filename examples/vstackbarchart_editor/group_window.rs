use appcui::prelude::*;

/// Group labels are copied into a 22-character buffer on the chart.
const MAX_LABEL_LEN: usize = 22;

#[derive(Clone, ListItem)]
pub struct XAxisGroup {
    #[Column(name: "&Label", width: 16)]
    pub label: String,
    #[Column(name: "&Start index", width: 12, align: right)]
    pub start: u32,
    #[Column(name: "&Count", width: 8, align: right)]
    pub count: u32,
}

#[ModalWindow(events = [ButtonEvents, WindowEvents], response = XAxisGroup)]
pub struct GroupWindow {
    label: Handle<TextField>,
    start: Handle<NumericSelector<u32>>,
    count: Handle<NumericSelector<u32>>,
    ok: Handle<Button>,
    cancel: Handle<Button>,
}

impl GroupWindow {
    pub fn new(title: &str, group: &XAxisGroup) -> Self {
        let mut win = Self {
            base: ModalWindow::new(title, layout!("a:c,w:46,h:11"), window::Flags::None),
            label: Handle::None,
            start: Handle::None,
            count: Handle::None,
            ok: Handle::None,
            cancel: Handle::None,
        };
        win.add(label!("'Label',l:1,t:1,w:12"));
        win.label = win.add(TextField::new(&group.label, layout!("l:14,t:1,r:1"), textfield::Flags::None));
        win.add(label!("'Start index',l:1,t:3,w:12"));
        win.start = win.add(NumericSelector::new(
            group.start,
            0,
            10_000,
            1,
            layout!("l:14,t:3,r:1"),
            numericselector::Flags::None,
        ));
        win.add(label!("'Count',l:1,t:5,w:12"));
        win.count = win.add(NumericSelector::new(
            group.count.max(1),
            1,
            10_000,
            1,
            layout!("l:14,t:5,r:1"),
            numericselector::Flags::None,
        ));
        win.ok = win.add(button!("'&Ok',l:1,b:0,w:14"));
        win.cancel = win.add(button!("'&Cancel',r:1,b:0,w:14"));
        win
    }

    fn read_group(&self) -> Option<XAxisGroup> {
        let label = self.control(self.label).map(|field| field.text().trim().to_string()).unwrap_or_default();
        if label.chars().count() > MAX_LABEL_LEN {
            dialogs::message("Group", "A group label can be at most 22 characters.");
            return None;
        }
        let start = self.control(self.start).map(|selector| selector.value()).unwrap_or(0);
        let count = self.control(self.count).map(|selector| selector.value()).unwrap_or(1).max(1);
        Some(XAxisGroup { label, start, count })
    }

    fn accept(&mut self) {
        let Some(group) = self.read_group() else {
            return;
        };
        self.exit_with(group);
    }
}

impl WindowEvents for GroupWindow {
    fn on_accept(&mut self) {
        self.accept();
    }
}

impl ButtonEvents for GroupWindow {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.ok {
            self.accept();
            EventProcessStatus::Processed
        } else if handle == self.cancel {
            self.exit();
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
