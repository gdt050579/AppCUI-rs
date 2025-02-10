use std::path::{Component, PathBuf};
use std::sync::{Mutex, OnceLock};

use super::Location;
use crate::prelude::*;
use crate::ui::pathfinder::GenericPathFinder;
use crate::utils::fs::{Entry, Root};
use crate::utils::Navigator;
use EnumBitFlags::EnumBitFlags;

pub(super) enum FolderSelectionDialogResult {
    Path(PathBuf),
    Cancel,
}

#[EnumBitFlags(bits = 8)]
pub(super) enum InnerFlags {
    Save = 1,
    Icons = 2,
    MultipleOpen = 4,
    ValidateOverwrite = 8,
    CheckIfFileExists = 16,
}

#[derive(ListItem)]
struct FolderName {
    #[Column(name: "&Name", width: 200)]
    value: String,
}

pub(super) static FOLDER_LAST_PATH: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

#[ModalWindow(events = ButtonEvents+WindowEvents+PathFinderEvents+TreeViewEvents<FolderName>, response: FolderSelectionDialogResult, internal: true)]
pub(super) struct FolderExplorer<T>
where
    T: Navigator<Entry, Root, PathBuf> + 'static,
{
    path_viewer: Handle<GenericPathFinder<T>>,
    tv: Handle<TreeView<FolderName>>,
    b_ok: Handle<Button>,
    b_cancel: Handle<Button>,
    nav: T,
    path: PathBuf,
    flags: InnerFlags,
}

impl<T> FolderExplorer<T>
where
    T: Navigator<Entry, Root, PathBuf> + 'static,
{
    pub(super) fn new(title: &str, location: Location, nav: T, flags: InnerFlags) -> Self {
        let mut w = Self {
            base: ModalWindow::new(title, Layout::new("d:c,w:70,h:20"), window::Flags::Sizeable),
            tv: Handle::None,
            b_ok: Handle::None,
            b_cancel: Handle::None,
            path_viewer: Handle::None,
            nav: nav.clone(),
            path: PathBuf::new(),
            flags,
        };
        w.path = match location {
            Location::Current => nav.current_dir(),
            Location::Last => {
                let m = FOLDER_LAST_PATH.get_or_init(|| Mutex::new(None));
                if let Ok(m) = m.lock() {
                    if let Some(p) = m.as_ref() {
                        p.clone()
                    } else {
                        nav.current_dir()
                    }
                } else {
                    nav.current_dir()
                }
            }
            Location::Path(p) => p.to_path_buf(),
        };
        w.add(label!("&Path,x:1,y:1,w:4"));
        let mut pf = GenericPathFinder::with_navigator(
            w.path.as_path().as_os_str().to_str().unwrap_or(""),
            Layout::new("l:6,t:1,r:1"),
            pathfinder::Flags::None,
            nav,
        );
        pf.set_hotkey(key!("Alt+P"));
        w.path_viewer = w.add(pf);
        let mut p = panel!("l:1,t:2,r:1,b:3");
        let mut tv = TreeView::with_capacity(
            256,
            Layout::new("d:c,w:100%,h:100%"),
            treeview::Flags::HideHeader | treeview::Flags::ScrollBars | treeview::Flags::SearchBar,
        );
        tv.set_components_toolbar_margins(2, 0);
        tv.sort(0, true);
        w.tv = p.add(tv);
        w.add(p);
        w.b_ok = w.add(button!("&OK,r:1,b:0,w:13"));
        w.b_cancel = w.add(button!("&Cancel,r:15,b:0,w:13"));
        w.set_size_bounds(40, 17, u16::MAX, u16::MAX);
        let h = w.tv;
        w.request_focus_for_control(h);
        w
    }

    fn populate_node(
        &mut self,
        path: &PathBuf,
        parent_node: Handle<treeview::Item<FolderName>>,
        child: &str,
    ) -> Option<Handle<treeview::Item<FolderName>>> {
        let h = self.tv;
        let entries = self.nav.entries(path);
        let mut result = None;
        log!("INFO", "Populate Node: Path={:?}, search='{}'",path, child);

        if let Some(tv) = self.control_mut(h) {
            tv.add_batch(|tv| {
                for e in entries {
                    if !e.is_container() {
                        continue;
                    }
                    log!("INFO", " - Add: {}", e.name);
                    if e.name.eq_ignore_ascii_case(child) {
                        result = Some(tv.add_item_to_parent(treeview::Item::expandable(FolderName { value: e.name }, false), parent_node));
                    } else {
                        tv.add_item_to_parent(treeview::Item::expandable(FolderName { value: e.name }, true), parent_node);
                    }
                }
            });
        }
        result
    }
    fn populate_root(&mut self, search: &str) -> Option<Handle<treeview::Item<FolderName>>> {
        log!("INFO", "Populate root with search: '{}'", search);
        let h = self.tv;
        let roots = self.nav.roots();
        if let Some(tv) = self.control_mut(h) {
            let mut result = None;
            for root in roots {
                let found = (search.len() > 0) && search[0..1].eq_ignore_ascii_case(&root.path[0..1]);
                let handle = tv.add_item(treeview::Item::expandable(
                    FolderName {
                        value: root.path.to_string(),
                    },
                    !found,
                ));
                if found {
                    result = Some(handle);
                }
            }
            result
        } else {
            None
        }
    }
    fn populate_from_path(&mut self) {
        let mut cp = PathBuf::new();
        let current_path = self.path.clone();
        let mut first = true;
        let mut parent_handle = Handle::None;
        let h = self.tv;
        self.control_mut(h).map(|tv| tv.clear());
        log!("INFO", "Populate from path: {:?}", current_path);

        for component in current_path.components() {
            if cfg!(target_os = "windows") && component == Component::RootDir {
                continue; // Skip RootDir only  on Windows
            }
            let c = component.as_os_str().to_str().unwrap_or_default();
            if first {
                first = false;
                if let Some(handle) = self.populate_root(c) {
                    parent_handle = handle;
                    cp.push(component);
                    if cfg!(target_os = "windows") {
                        cp.push("\\");
                    }
                } else {
                    break;
                }
            } else {
                if let Some(handle) = self.populate_node(&cp, parent_handle, c) {
                    parent_handle = handle;
                    cp.push(component);
                } else {
                    break;
                }
            }
        }
        if !parent_handle.is_none() {
            let h = self.tv;
            if let Some(tv) = self.control_mut(h) {
                tv.move_cursor_to(parent_handle);
            }
        }
    }

    fn return_result(&mut self) {}
}
impl<T> ButtonEvents for FolderExplorer<T>
where
    T: Navigator<Entry, Root, PathBuf> + 'static,
{
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        match () {
            _ if handle == self.b_ok => {
                self.return_result();
                EventProcessStatus::Processed
            }
            _ if handle == self.b_cancel => {
                self.exit_with(FolderSelectionDialogResult::Cancel);
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}
impl<T> WindowEvents for FolderExplorer<T>
where
    T: Navigator<Entry, Root, PathBuf> + 'static,
{
    fn on_activate(&mut self) {
        self.populate_from_path();
    }
    fn on_accept(&mut self) {
        self.return_result();
    }
    fn on_cancel(&mut self) -> ActionRequest {
        ActionRequest::Allow
    }
}

impl<T> PathFinderEvents for FolderExplorer<T>
where
    T: Navigator<Entry, Root, PathBuf> + 'static,
{
    fn on_path_updated(&mut self, handle: Handle<PathFinder>) -> EventProcessStatus {
        if handle == self.path_viewer {
            if let Some(pv) = self.control(self.path_viewer) {
                self.path = pv.path().to_path_buf();
                self.populate_from_path();
            }
        }
        EventProcessStatus::Processed
    }
}

impl<T> TreeViewEvents<FolderName> for FolderExplorer<T>
where
    T: Navigator<Entry, Root, PathBuf> + 'static,
{
    fn on_current_item_changed(
        &mut self,
        handle: Handle<TreeView<FolderName>>,
        item_handle: Handle<treeview::Item<FolderName>>,
    ) -> EventProcessStatus {
        let p = if let Some(tv) = self.control(handle) {
            let mut a: [Handle<treeview::Item<FolderName>>; 256] = [Handle::None; 256];
            let mut pos = 255;
            let mut h = item_handle;
            while let Some(item) = tv.item(h) {
                a[pos] = h;
                h = item.parent().unwrap_or(Handle::None);
                pos -= 1;
                if (pos == 0) || (h.is_none()) {
                    break;
                }
            }
            if pos > 0 {
                let mut path = PathBuf::new();
                for i in (pos + 1)..256 {
                    if let Some(item) = tv.item(a[i]) {
                        path.push(item.value().value.as_str());
                    }
                }
                Some(path)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(path) = p {
            let h = self.path_viewer;
            if let Some(pv) = self.control_mut(h) {
                pv.set_path(&path);
            }
            self.path = path;
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }

    fn on_item_expanded(&mut self, tv: Handle<TreeView<FolderName>>, item_handle: Handle<treeview::Item<FolderName>>, _: bool) -> EventProcessStatus {
        if let Some(tv) = self.control_mut(tv) {
            tv.clear_search();
            tv.delete_item_children(item_handle);
        }
        let p = self.path.clone();
        log!("INFO", "Item expanded: {:?}, Handle:{:?}", p, item_handle);
        self.populate_node(&p, item_handle, "");
        EventProcessStatus::Processed
    }

    fn on_item_action(&mut self, _: Handle<TreeView<FolderName>>, _: Handle<treeview::Item<FolderName>>) -> EventProcessStatus {
        self.return_result();
        EventProcessStatus::Processed
    }
}
