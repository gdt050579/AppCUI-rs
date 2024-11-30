use dialogs::file_mask::FileMask;

use super::DialogResult;
use crate::prelude::*;
use crate::utils::fs::{Entry, Root};
use crate::utils::Navigator;

#[ModalWindow(events = ToggleButtonEvents+ButtonEvents+WindowEvents, response: DialogResult, internal: true)]
pub(super) struct FileExplorer<T>
where
    T: Navigator<Entry, Root> + 'static,
{
    list: Handle<ListView<Entry>>,
    path: Handle<TextField>,
    details: Handle<ToggleButton>,
    columns: Handle<ToggleButton>,
    name: Handle<TextField>,
    b_ok: Handle<Button>,
    b_cancel: Handle<Button>,
    mask: Handle<ComboBox>,
    extension_mask: Vec<FileMask>,
    nav: T,
    g_updir: listview::Group,
    g_files: listview::Group,
    g_folders: listview::Group,
}

// ====================================================================================================================
// ====================================================================================================================

impl<T> FileExplorer<T>
where
    T: Navigator<Entry, Root> + 'static,
{
    pub(super) fn new(title: &str, path: &str, extension_mask: Vec<FileMask>, nav: T) -> Self {
        let mut w = Self {
            base: ModalWindow::new(title, Layout::new("d:c,w:70,h:20"), window::Flags::Sizeable),
            list: Handle::None,
            details: Handle::None,
            columns: Handle::None,
            name: Handle::None,
            b_ok: Handle::None,
            b_cancel: Handle::None,
            mask: Handle::None,
            path: Handle::None,
            extension_mask,
            nav,
            g_updir: listview::Group::None,
            g_files: listview::Group::None,
            g_folders: listview::Group::None,
        };
        w.add(button!("Drive,x:1,y:1,w:7,type:Flat"));
        w.path = w.add(TextField::new(path, Layout::new("l:9,t:1,r:1"), textfield::Flags::Readonly));
        let mut p = panel!("l:1,t:3,r:1,b:5");
        let mut lv = listview!("Entry,d:c,w:100%,h:100%,flags: ScrollBars+SearchBar+LargeIcons");
        w.g_updir = lv.add_group("UpDir");
        w.g_folders = lv.add_group("Folders");
        w.g_files = lv.add_group("Files");
        w.list = p.add(lv);
        w.add(p);
        w.add(label!("&Name,l:1,b:3,w:4"));
        w.name = w.add(TextField::new("", Layout::new("l:6,b:3,r:11"), textfield::Flags::None));
        w.b_ok = w.add(button!("&OK,r:1,b:2,w:9"));
        w.add(label!("&Type,l:1,b:1,w:4"));
        let mut mask = ComboBox::new(Layout::new("l:6,b:1,r:11"), combobox::Flags::None);
        for m in &w.extension_mask {
            mask.add(m.name());
        }
        mask.add("All files");
        mask.set_index(0);
        w.mask = w.add(mask);
        w.b_cancel = w.add(button!("&Cancel,r:1,b:0,w:9"));
        w.set_size_bounds(40, 17, u16::MAX, u16::MAX);
        w
    }
    fn populate(&mut self) {
        let entries = if let Some(path) = self.control(self.path) {
            self.nav.entries(path.text())
        } else {
            Vec::new()
        };
        let h = self.list;
        let g_folders = self.g_folders;
        let g_files = self.g_files;
        if let Some(lv) = self.control_mut(h) {
            lv.add_batch(|lv| {
                //lv.clear();
                for e in entries {
                    let is_folder = e.is_container();
                    lv.add_item(listview::Item::new(
                        e,
                        false,
                        None,
                        [if is_folder { '📁' } else { '🗔' }, ' '],
                        if is_folder { g_folders } else { g_files },
                    ));
                }
            });
        }
    }
}
impl<T> ButtonEvents for FileExplorer<T>
where
    T: Navigator<Entry, Root> + 'static,
{
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl<T> ToggleButtonEvents for FileExplorer<T>
where
    T: Navigator<Entry, Root> + 'static,
{
    fn on_selection_changed(&mut self, _handle: Handle<ToggleButton>, _selected: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl<T> WindowEvents for FileExplorer<T>
where
    T: Navigator<Entry, Root> + 'static,
{
    fn on_layout_changed(&mut self, _old_layout: Rect, _new_layout: Rect) {}

    fn on_activate(&mut self) {
        self.populate();
    }

    fn on_deactivate(&mut self) {}

    fn on_accept(&mut self) {}

    fn on_cancel(&mut self) -> ActionRequest {
        ActionRequest::Allow
    }
}
