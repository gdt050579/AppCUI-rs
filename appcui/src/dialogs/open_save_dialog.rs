use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use dialogs::file_mask::FileMask;
use dialogs::root_select_dialog::RootSelectDialog;
use fs::EntryType;

use super::Location;
use crate::prelude::*;
use crate::ui::pathfinder::GenericPathFinder;
use crate::utils::fs::{Entry, Root};
use crate::utils::Navigator;
use EnumBitFlags::EnumBitFlags;

pub(super) enum OpenSaveDialogResult {
    Path(PathBuf),
    //MultiplePaths(Vec<PathBuf>),
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

pub(super) static LAST_PATH: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

#[ModalWindow(events = ButtonEvents+WindowEvents+ListViewEvents<Entry>+ComboBoxEvents+PathFinderEvents, response: OpenSaveDialogResult, internal: true)]
pub(super) struct FileExplorer<T>
where
    T: Navigator<Entry, Root, PathBuf> + 'static,
{
    list: Handle<ListView<Entry>>,
    path_viewer: Handle<GenericPathFinder<T>>,
    details: Handle<ToggleButton>,
    columns: Handle<ToggleButton>,
    name: Handle<TextField>,
    b_ok: Handle<Button>,
    b_cancel: Handle<Button>,
    b_drive: Handle<Button>,
    mask: Handle<ComboBox>,
    extension_mask: Vec<FileMask>,
    nav: T,
    g_updir: listview::Group,
    g_files: listview::Group,
    g_folders: listview::Group,
    path: PathBuf,
    flags: InnerFlags,
}

impl<T> FileExplorer<T>
where
    T: Navigator<Entry, Root, PathBuf> + 'static,
{
    pub(super) fn new(file_name: &str, title: &str, location: Location, extension_mask: Vec<FileMask>, nav: T, flags: InnerFlags) -> Self {
        let mut w = Self {
            base: ModalWindow::new(title, Layout::new("d:c,w:70,h:20"), window::Flags::Sizeable),
            list: Handle::None,
            details: Handle::None,
            columns: Handle::None,
            name: Handle::None,
            b_ok: Handle::None,
            b_cancel: Handle::None,
            b_drive: Handle::None,
            mask: Handle::None,
            path_viewer: Handle::None,
            extension_mask,
            nav: nav.clone(),
            g_updir: listview::Group::None,
            g_files: listview::Group::None,
            g_folders: listview::Group::None,
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
        let mut lv: ListView<Entry> = ListView::new(
            Layout::new("d:c,w:100%,h:100%"),
            listview::Flags::SearchBar
                | listview::Flags::ScrollBars
                | if flags.contains(InnerFlags::Icons) {
                    listview::Flags::LargeIcons
                } else {
                    listview::Flags::None
                }
                | if !flags.contains(InnerFlags::MultipleOpen) {
                    listview::Flags::NoSelection
                } else {
                    listview::Flags::None
                },
        );
        lv.set_components_toolbar_margins(2, 0);
        w.g_updir = lv.add_group("UpDir");
        w.g_folders = lv.add_group("Folders");
        w.g_files = lv.add_group("Files");
        w.list = p.add(lv);
        w.add(p);
        w.add(label!("&Name,l:1,b:3,w:4"));
        let mut fname = TextField::new(file_name, Layout::new("l:6,b:3,r:11"), textfield::Flags::None);
        fname.set_hotkey(key!("Alt+N"));
        w.name = w.add(fname);
        w.b_ok = w.add(button!("&OK,r:1,b:2,w:9"));
        w.add(label!("&Type,l:1,b:1,w:4"));
        let mut mask = ComboBox::new(Layout::new("l:6,b:1,r:11"), combobox::Flags::None);
        for m in &w.extension_mask {
            mask.add(m.name());
        }
        mask.add("All files");
        mask.set_index(0);
        mask.set_hotkey(key!("Alt+T"));
        w.mask = w.add(mask);
        w.b_cancel = w.add(button!("&Cancel,r:1,b:0,w:9"));
        w.set_size_bounds(40, 17, u16::MAX, u16::MAX);
        let h = w.name;
        w.request_focus_for_control(h);
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
        let is_root = self.path.is_absolute() && self.path.parent().is_none();
        let mut entries = self.nav.entries(&self.path);
        let filter_idx = if let Some(mask) = self.control(self.mask) {
            mask.index().unwrap_or_default() as usize
        } else {
            0
        };
        if filter_idx < self.extension_mask.len() {
            // we need to filter the files
            let filter = &self.extension_mask[filter_idx];
            entries.retain(|e| (e.entry_type != EntryType::File) || filter.matches(&e.name));
        }

        let h = self.list;
        let g_folders = self.g_folders;
        let g_files = self.g_files;
        let g_updir = self.g_updir;
        let theme = self.theme();
        let c_files = theme.text.normal;
        let c_folders = theme.text.focused;
        if let Some(lv) = self.control_mut(h) {
            lv.add_batch(|lv| {
                lv.clear();
                if !is_root {
                    lv.add_item(listview::Item::new(
                        Entry::new("..", 0, chrono::NaiveDateTime::default(), crate::utils::fs::EntryType::UpDir),
                        false,
                        Some(c_folders),
                        ['🔙', ' '],
                        g_updir,
                    ));
                }
                for e in entries {
                    let is_folder = e.is_container();
                    lv.add_item(listview::Item::new(
                        e,
                        false,
                        if is_folder { Some(c_folders) } else { Some(c_files) },
                        [if is_folder { '📁' } else { '📄' }, ' '],
                        if is_folder { g_folders } else { g_files },
                    ));
                }
            });
        }
    }
    fn populate_after_path_update(&mut self) {
        self.path.push("");
        let h = self.path_viewer;
        let ts = TempString::<128>::new(self.path.to_str().unwrap_or_default());
        if let Some(pv) = self.control_mut(h) {
            pv.set_path(Path::new(ts.as_str()));
        }
        let h = self.list;
        if let Some(lst) = self.control_mut(h) {
            lst.clear_search();
        }
        self.populate();
    }
    fn return_result_from_save(&mut self) {
        // get the file name
        let mut entry = Entry::default();
        if let Some(tf) = self.control(self.name) {
            if tf.text().trim().is_empty() {
                return;
            }
            entry.name.push_str(tf.text());
        } else {
            return;
        };
        if let Some(result) = self.nav.join(&self.path, &entry) {
            if self.flags.contains(InnerFlags::ValidateOverwrite) {
                match self.nav.exists(&result) {
                    Some(true) => {
                        if !crate::dialogs::validate("Overwrite", format!("Do you want to overwrite the file: '{}'", result.display()).as_str()) {
                            return;
                        }
                    }
                    Some(false) => {
                        // do nothing - the file does not exist
                    }
                    None => {
                        crate::dialogs::error("Error", format!("Fail to check if file exists: '{}'", result.display()).as_str());
                        return;
                    }
                }
            }
            self.update_last_path(&result);
            self.exit_with(OpenSaveDialogResult::Path(result));
        } else {
            crate::dialogs::error(
                "Error",
                format!(
                    "Fail to join current path: '{}' with file name: '{}'",
                    self.path.display(),
                    entry.name.as_str()
                )
                .as_str(),
            );
        }
    }

    fn return_result_from_open(&mut self) {
        // get the file name
        let mut entry = Entry::default();
        if let Some(tf) = self.control(self.name) {
            if tf.text().trim().is_empty() {
                return;
            }
            entry.name.push_str(tf.text());
        } else {
            return;
        };
        if let Some(result) = self.nav.join(&self.path, &entry) {
            if self.flags.contains(InnerFlags::CheckIfFileExists) {
                match self.nav.exists(&result) {
                    Some(true) => {
                        // do nothing --> all is good
                    }
                    Some(false) => {
                        crate::dialogs::error("Error", format!("File '{}' does not exists !", result.display()).as_str());
                        return;
                    }
                    None => {
                        crate::dialogs::error("Error", format!("Fail to check if file exists: '{}'", result.display()).as_str());
                        return;
                    }
                }
            }
            self.update_last_path(&result);
            self.exit_with(OpenSaveDialogResult::Path(result));
        } else {
            crate::dialogs::error(
                "Error",
                format!(
                    "Fail to join current path: '{}' with file name: '{}'",
                    self.path.display(),
                    entry.name.as_str()
                )
                .as_str(),
            );
        }
    }
    fn return_result(&mut self) {
        if self.flags.contains(InnerFlags::Save) {
            self.return_result_from_save()
        } else {
            self.return_result_from_open()
        }
    }
}
impl<T> ButtonEvents for FileExplorer<T>
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
                self.exit_with(OpenSaveDialogResult::Cancel);
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
impl<T> WindowEvents for FileExplorer<T>
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
impl<T> ComboBoxEvents for FileExplorer<T>
where
    T: Navigator<Entry, Root, PathBuf> + 'static,
{
    fn on_selection_changed(&mut self, handle: Handle<ComboBox>) -> EventProcessStatus {
        if handle == self.mask {
            self.populate();
        }
        EventProcessStatus::Processed
    }
}
impl<T> ListViewEvents<Entry> for FileExplorer<T>
where
    T: Navigator<Entry, Root, PathBuf> + 'static,
{
    fn on_item_action(&mut self, handle: Handle<ListView<Entry>>, item_index: usize) -> EventProcessStatus {
        let (data, etype): (Option<TempString<128>>, Option<EntryType>) = if let Some(lv) = self.control(handle) {
            if let Some(e) = lv.item(item_index) {
                if e.is_container() {
                    (Some(TempString::new(e.name())), Some(e.entry_type))
                } else {
                    (None, Some(e.entry_type))
                }
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        match etype {
            Some(EntryType::UpDir) => {
                self.path.pop();
                self.populate_after_path_update();
            }
            Some(EntryType::Folder) => {
                if let Some(name) = data {
                    self.path.push(name.as_str());
                    self.populate_after_path_update();
                }
            }
            Some(EntryType::File) => {
                self.return_result();
            }
            None => {}
        }
        EventProcessStatus::Processed
    }

    fn on_current_item_changed(&mut self, handle: Handle<ListView<Entry>>) -> EventProcessStatus {
        let current_item = if let Some(lv) = self.control(handle) {
            if let Some(e) = lv.current_item() {
                if e.entry_type == EntryType::File {
                    e.name()
                } else {
                    ""
                }
            } else {
                ""
            }
        } else {
            ""
        };
        let temp_string: TempString<128> = TempString::new(current_item);
        if !temp_string.is_empty() {
            let h = self.name;
            if let Some(tf) = self.control_mut(h) {
                tf.set_text(temp_string.as_str());
            }
        }
        EventProcessStatus::Processed
    }
}

impl<T> PathFinderEvents for FileExplorer<T>
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
