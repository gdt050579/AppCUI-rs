use std::marker::PhantomData;

use dialogs::file_mask::FileMask;

use super::{DialogResult, FileInfo};
use crate::prelude::*;
use crate::utils::*;

#[ModalWindow(events = ToggleButtonEvents+ButtonEvents, response: DialogResult, internal: true)]
pub(super) struct FileExplorer<T, E, R>
where
    T: Navigator<E, R> + 'static,
    E: NavigatorEntry + 'static,
    R: NavigatorRoot + 'static,
{
    list: Handle<ListView<FileInfo>>,
    details: Handle<ToggleButton>,
    columns: Handle<ToggleButton>,
    name: Handle<TextField>,
    b_ok: Handle<Button>,
    b_cancel: Handle<Button>,
    mask: Handle<ComboBox>,
    extension_mask: Vec<FileMask>,
    nav: T,
    _e: PhantomData<E>,
    _r: PhantomData<R>,
}

// ====================================================================================================================
// ====================================================================================================================

impl<T, E, R> FileExplorer<T, E, R>
where
    T: Navigator<E, R>,
    E: NavigatorEntry,
    R: NavigatorRoot,
{
    pub(super) fn new(title: &str, extension_mask: Vec<FileMask>, nav: T) -> Self {
        let mut w = Self {
            base: ModalWindow::new(title, Layout::new("d:c,w:70,h:20"), window::Flags::Sizeable),
            list: Handle::None,
            details: Handle::None,
            columns: Handle::None,
            name: Handle::None,
            b_ok: Handle::None,
            b_cancel: Handle::None,
            mask: Handle::None,
            extension_mask,
            nav,
            _e: PhantomData,
            _r: PhantomData,
        };
        w.add(button!("Drive,x:1,y:1,w:7,type:Flat"));
        let mut p = panel!("l:1,t:3,r:1,b:5");
        let mut lv = listview!("FileInfo,d:c,w:100%,h:100%,flags: ScrollBars+SearchBar");
        p.add(lv);
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
        w.set_size_bounds(40, 10, u16::MAX, u16::MAX);
        w
    }
}
impl<T, E, R> ButtonEvents for FileExplorer<T, E, R>
where
    T: Navigator<E, R>,
    E: NavigatorEntry,
    R: NavigatorRoot,
{
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl<T, E, R> ToggleButtonEvents for FileExplorer<T, E, R>
where
    T: Navigator<E, R>,
    E: NavigatorEntry,
    R: NavigatorRoot,
{
    fn on_selection_changed(&mut self, _handle: Handle<ToggleButton>, _selected: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
