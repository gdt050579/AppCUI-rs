use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use dialogs::root_select_dialog::RootSelectDialog;

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

pub(super) static LAST_PATH: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

#[ModalWindow(events = ButtonEvents+WindowEvents+PathFinderEvents, response: FolderSelectionDialogResult, internal: true)]
pub(super) struct FolderExplorer<T>
where
    T: Navigator<Entry, Root, PathBuf> + 'static,
{
    path_viewer: Handle<GenericPathFinder<T>>,
    tv: Handle<TreeView<FolderName>>,
    b_ok: Handle<Button>,
    b_cancel: Handle<Button>,
    b_drive: Handle<Button>,
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
            b_drive: Handle::None,
            path_viewer: Handle::None,
            nav: nav.clone(),
            path: PathBuf::new(),
            flags,
        };
        w.path = match location {
            Location::Current => nav.current_dir(),
            Location::Last => {
                let m = LAST_PATH.get_or_init(|| Mutex::new(None));
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
        w.b_drive = w.add(button!("&Drive,x:1,y:1,w:7,type:Flat"));
        let pf = GenericPathFinder::with_navigator(
            w.path.as_path().as_os_str().to_str().unwrap_or(""),
            Layout::new("l:9,t:1,r:1"),
            pathfinder::Flags::None,
            nav,
        );
        w.path_viewer = w.add(pf);
        let mut p = panel!("l:1,t:3,r:1,b:5");
        let mut tv = TreeView::with_capacity(
            256,
            Layout::new("d:c,w:100%,h:100%"),
            treeview::Flags::HideHeader | treeview::Flags::ScrollBars | treeview::Flags::SearchBar,
        );
        tv.set_components_toolbar_margins(2, 0);
        w.tv = p.add(tv);
        w.add(p);
        w.b_ok = w.add(button!("&OK,r:1,b:2,w:9"));
        w.b_cancel = w.add(button!("&Cancel,r:1,b:0,w:9"));
        w.set_size_bounds(40, 17, u16::MAX, u16::MAX);
        w
    }
    fn update_last_path(&self, last_path: &Path) {
        if let Some(dir) = last_path.parent() {
            let mut new_path = dir.to_path_buf();
            new_path.push(""); // make sure we have a trailing slash
            if let Ok(mut guard) = LAST_PATH.get_or_init(|| Mutex::new(None)).lock() {
                *guard = Some(new_path);
            }
        }
    }
    fn populate(&mut self) {
        // let is_root = self.path.is_absolute() && self.path.parent().is_none();
        // let mut entries = self.nav.entries(&self.path);
        // let filter_idx = if let Some(mask) = self.control(self.mask) {
        //     mask.index().unwrap_or_default() as usize
        // } else {
        //     0
        // };
        // if filter_idx < self.extension_mask.len() {
        //     // we need to filter the files
        //     let filter = &self.extension_mask[filter_idx];
        //     entries.retain(|e| (e.entry_type != EntryType::File) || filter.matches(&e.name));
        // }

        // let h = self.list;
        // let g_folders = self.g_folders;
        // let g_files = self.g_files;
        // let g_updir = self.g_updir;
        // let theme = self.theme();
        // let c_files = theme.text.normal;
        // let c_folders = theme.text.focused;
        // if let Some(lv) = self.control_mut(h) {
        //     lv.add_batch(|lv| {
        //         lv.clear();
        //         if !is_root {
        //             lv.add_item(listview::Item::new(
        //                 Entry::new("..", 0, chrono::NaiveDateTime::default(), crate::utils::fs::EntryType::UpDir),
        //                 false,
        //                 Some(c_folders),
        //                 ['🔙', ' '],
        //                 g_updir,
        //             ));
        //         }
        //         for e in entries {
        //             let is_folder = e.is_container();
        //             lv.add_item(listview::Item::new(
        //                 e,
        //                 false,
        //                 if is_folder { Some(c_folders) } else { Some(c_files) },
        //                 [if is_folder { '📁' } else { '📄' }, ' '],
        //                 if is_folder { g_folders } else { g_files },
        //             ));
        //         }
        //     });
        // }
    }
    fn populate_after_path_update(&mut self) {
        // self.path.push("");
        // let h = self.path_viewer;
        // let ts = TempString::<128>::new(self.path.to_str().unwrap_or_default());
        // if let Some(pv) = self.control_mut(h) {
        //     pv.set_path(Path::new(ts.as_str()));
        // }
        // let h = self.list;
        // if let Some(lst) = self.control_mut(h) {
        //     lst.clear_search();
        // }
        // self.populate();
    }

    fn return_result(&mut self) {
        
    }
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
            _ if handle == self.b_drive => {
                if let Some(path) = RootSelectDialog::new(self.nav.roots(), self.flags.contains(InnerFlags::Icons)).show() {
                    self.path = path;
                    self.populate_after_path_update();
                }
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
        self.populate();
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
                self.populate_after_path_update();
            }
        }
        EventProcessStatus::Processed
    }
}
