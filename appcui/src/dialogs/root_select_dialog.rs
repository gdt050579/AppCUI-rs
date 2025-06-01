use crate::prelude::*;
use crate::utils::fs::Root;
use std::path::PathBuf;

#[ModalWindow(events: ButtonEvents+ListViewEvents<Root>, response: PathBuf, internal: true)]
pub(super) struct RootSelectDialog {
    list: Handle<ListView<Root>>,
    b_ok: Handle<Button>,
    b_cancel: Handle<Button>,
}

impl RootSelectDialog {
    pub(super) fn new(roots_list: Vec<Root>, icons: bool) -> Self {
        let mut w = Self {
            base: ModalWindow::new("Devices", Layout::new("d:c,w:50,h:15"), window::Flags::None),
            list: Handle::None,
            b_ok: Handle::None,
            b_cancel: Handle::None,
        };
        w.b_ok = w.add(button!("&Ok,l:11,b:0,w:11"));
        w.b_cancel = w.add(button!("&Cancel,l:25,b:0,w:11"));
        let mut lv = ListView::<Root>::new(
            Layout::new("d:c,w:100%,h:100%"),
            if icons { listview::Flags::LargeIcons } else { listview::Flags::None },
        );
        if icons {
            lv.add_batch(|l| {
                for root in roots_list {
                    let icon = root.root_type.icon();
                    l.add_item(listview::Item::new(root, false, None, [icon, ' '], listview::Group::None));
                }
            });
        } else {
            lv.add_items(roots_list);
        }
        let mut p = panel!("l:1,r:1,t:1,b:3");
        w.list = p.add(lv);
        w.add(p);
        let h = w.list;
        w.request_focus_for_control(h);
        w
    }
    fn return_result(&mut self) {
        if let Some(lv) = self.control(self.list) {
            if let Some(item) = lv.current_item() {
                self.exit_with(PathBuf::from(item.path.as_str()));
            }
        }
    }
}

impl ButtonEvents for RootSelectDialog {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.b_ok {
            self.return_result();
            return EventProcessStatus::Processed;
        }
        if handle == self.b_cancel {
            self.exit();
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}

impl ListViewEvents<Root> for RootSelectDialog {
    fn on_item_action(&mut self, _: Handle<ListView<Root>>, _: usize) -> EventProcessStatus {
        self.return_result();
        EventProcessStatus::Processed
    }
}
