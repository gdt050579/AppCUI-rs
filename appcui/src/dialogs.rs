mod dialog_buttons;
mod dialog_result;
mod file_mask;
mod folder_select_dialog;
mod generic_alert_dialog;
mod open_save_dialog;
mod root_select_dialog;
#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use crate::{
    prelude::{window, ModalWindowMethods},
    utils::{self, Navigator},
};
use dialog_buttons::DialogButtons;
use dialog_result::DialogResult;
use file_mask::FileMask;
use folder_select_dialog::{FolderExplorer, FolderSelectionDialogResult};
use generic_alert_dialog::GenericAlertDialog;
use open_save_dialog::{FileExplorer, OpenSaveDialogResult};
use EnumBitFlags::EnumBitFlags;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ValidateOrCancelResult {
    Yes,
    No,
    Cancel,
}

pub fn error(title: &str, caption: &str) {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::Ok, window::Type::Error);
    w.show();
}
pub fn retry(title: &str, caption: &str) -> bool {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::RetryCancel, window::Type::Error);
    if let Some(result) = w.show() {
        return result == DialogResult::Retry;
    }
    false
}
pub fn alert(title: &str, caption: &str) {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::Ok, window::Type::Warning);
    w.show();
}
pub fn proceed(title: &str, caption: &str) -> bool {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::YesNo, window::Type::Warning);
    if let Some(result) = w.show() {
        return result == DialogResult::Yes;
    }
    false
}
pub fn message(title: &str, caption: &str) {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::Ok, window::Type::Notification);
    w.show();
}
pub fn validate(title: &str, caption: &str) -> bool {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::YesNo, window::Type::Notification);
    if let Some(result) = w.show() {
        return result == DialogResult::Yes;
    }
    false
}
pub fn validate_or_cancel(title: &str, caption: &str) -> ValidateOrCancelResult {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::YesNoCancel, window::Type::Notification);
    if let Some(result) = w.show() {
        match result {
            DialogResult::Yes => return ValidateOrCancelResult::Yes,
            DialogResult::No => return ValidateOrCancelResult::No,
            _ => return ValidateOrCancelResult::Cancel,
        }
    }
    ValidateOrCancelResult::Cancel
}

#[derive(Clone)]
pub enum Location<'a> {
    Current,
    Last,
    Path(&'a Path),
}

#[EnumBitFlags(bits = 8)]
pub enum SaveFileDialogFlags {
    Icons = 1,
    ValidateOverwrite = 2,
}
#[EnumBitFlags(bits = 8)]
pub enum OpenFileDialogFlags {
    Icons = 1,
    CheckIfFileExists = 2,
}

pub(super) fn inner_save<T>(
    title: &str,
    file_name: &str,
    location: Location,
    extension_mask: Option<&str>,
    flags: SaveFileDialogFlags,
    nav: T,
) -> Option<PathBuf>
where
    T: crate::utils::Navigator<crate::utils::fs::Entry, crate::utils::fs::Root, PathBuf> + 'static,
{
    let ext_mask = extension_mask.unwrap_or_default();
    match FileMask::parse(ext_mask) {
        Ok(mask_list) => {
            let mut inner_flags = open_save_dialog::InnerFlags::Save;
            if flags.contains(SaveFileDialogFlags::Icons) {
                inner_flags |= open_save_dialog::InnerFlags::Icons;
            }
            if flags.contains(SaveFileDialogFlags::ValidateOverwrite) {
                inner_flags |= open_save_dialog::InnerFlags::ValidateOverwrite;
            }

            let w = FileExplorer::new(file_name, title, location, mask_list, nav, inner_flags);
            let result = w.show();
            match result {
                Some(OpenSaveDialogResult::Path(path)) => Some(path),
                _ => None,
            }
        }
        Err(err_msg) => {
            panic!(
                "Error parsing file mask: '{}'. It should be in the format 'name1 = [ext1, ext2, ... extn], name2 = [...], ...'.\n{}",
                ext_mask, err_msg
            );
        }
    }
}

#[EnumBitFlags(bits = 8)]
pub enum SelectFolderDialogFlags {
    Icons = 1,
}
pub(super) fn inner_open<T>(
    title: &str,
    file_name: &str,
    location: Location,
    extension_mask: Option<&str>,
    flags: OpenFileDialogFlags,
    nav: T,
) -> Option<PathBuf>
where
    T: crate::utils::Navigator<crate::utils::fs::Entry, crate::utils::fs::Root, PathBuf> + 'static,
{
    let ext_mask = extension_mask.unwrap_or_default();
    match FileMask::parse(ext_mask) {
        Ok(mask_list) => {
            let mut inner_flags = open_save_dialog::InnerFlags::None;
            if flags.contains(OpenFileDialogFlags::Icons) {
                inner_flags |= open_save_dialog::InnerFlags::Icons;
            }
            if flags.contains(OpenFileDialogFlags::CheckIfFileExists) {
                inner_flags |= open_save_dialog::InnerFlags::CheckIfFileExists;
            }

            let w = FileExplorer::new(file_name, title, location, mask_list, nav, inner_flags);
            let result = w.show();
            match result {
                Some(OpenSaveDialogResult::Path(path)) => Some(path),
                _ => None,
            }
        }
        Err(err_msg) => {
            panic!(
                "Error parsing file mask: '{}'. It should be in the format 'name1 = [ext1, ext2, ... extn], name2 = [...], ...'.\n{}",
                ext_mask, err_msg
            );
        }
    }
}

pub(super) fn inner_select_folder<T>(title: &str, location: Location, flags: SelectFolderDialogFlags, nav: T) -> Option<PathBuf>
where
    T: crate::utils::Navigator<crate::utils::fs::Entry, crate::utils::fs::Root, PathBuf> + 'static,
{
    let w = FolderExplorer::new(title, location, nav, flags);
    let result = w.show();
    match result {
        Some(FolderSelectionDialogResult::Path(path)) => Some(path),
        _ => None,
    }
}

pub(crate) fn clear_last_path() {
    if let Some(m) = open_save_dialog::LAST_PATH.get() {
        if let Ok(mut guard) = m.lock() {
            *guard = None;
        }
    }
    if let Some(m) = folder_select_dialog::FOLDER_LAST_PATH.get() {
        if let Ok(mut guard) = m.lock() {
            *guard = None;
        }
    }
}

/// Opens a file dialog for saving a file and returns the path of the file selected by the user or None if the user canceled the operation.
/// # Arguments
/// * `title` - The title of the dialog.
/// * `file_name` - The default file name.
/// * `location` - The initial location of the dialog (one of Current, Last or Path). If Last is used, the dialog will open in the last location used by the user.
/// * `extension_mask` - A string that specifies the file extensions that can be selected by the user. The format is `name1 = [ext1, ext2, ... extn], name2 = [...], ...`. If None is provided, all files will be displayed.
/// * `flags` - Flags that specify the behavior of the dialog.
///
/// # Example
/// ```rust,no_run
/// use appcui::dialogs;
///
/// if let Some(path) = dialogs::save("Save file",
///                                   "file.txt",
///                                   dialogs::Location::Current,
///                                   Some("Text files = [txt]"),
///                                   dialogs::SaveFileDialogFlags::Icons)
/// {
///    println!("File saved at: {:?}", path);
/// }
/// ```
pub fn save(title: &str, file_name: &str, location: Location, extension_mask: Option<&str>, flags: SaveFileDialogFlags) -> Option<PathBuf> {
    inner_save(title, file_name, location, extension_mask, flags, utils::fs::Navigator::new())
}

/// Opens a file dialog for opening a file and returns the path of the file selected by the user or None if the user canceled the operation.
/// # Arguments
/// * `title` - The title of the dialog.
/// * `file_name` - The default file name.
/// * `location` - The initial location of the dialog (one of Current, Last or Path). If Last is used, the dialog will open in the last location used by the user.
/// * `extension_mask` - A string that specifies the file extensions that can be selected by the user. The format is `name1 = [ext1, ext2, ... extn], name2 = [...], ...`. If None is provided, all files will be displayed.
/// * `flags` - Flags that specify the behavior of the dialog.
///
/// # Example
/// ```rust,no_run
/// use appcui::dialogs;
///
/// if let Some(path) = dialogs::open("Open file",
///                                   "file.txt",
///                                   dialogs::Location::Current,
///                                   Some("Text files = [txt]"),
///                                   dialogs::OpenFileDialogFlags::Icons)
/// {
///   println!("File opened: {:?}", path);
/// }
/// ```
pub fn open(title: &str, file_name: &str, location: Location, extension_mask: Option<&str>, flags: OpenFileDialogFlags) -> Option<PathBuf> {
    inner_open(title, file_name, location, extension_mask, flags, utils::fs::Navigator::new())
}

/// Opens a dialog for selecting a folder and returns the path of the folder selected by the user or None if the user canceled the operation.
/// # Arguments
/// * `title` - The title of the dialog.
/// * `location` - The initial location of the dialog (one of Current, Last or Path). If Last is used, the dialog will open in the last location used by the user.
/// * `flags` - Flags that specify the behavior of the dialog (ex: display icons).
/// 
/// # Example
/// ```rust,no_run
/// use appcui::dialogs;
/// 
/// if let Some(path) = dialogs::select_folder("Select folder", 
///                                            dialogs::Location::Current, 
///                                            dialogs::SelectFolderDialogFlags::Icons) 
/// {
///    println!("Folder selected: {:?}", path);
/// }
pub fn select_folder(title:&str, location: Location, flags: SelectFolderDialogFlags) -> Option<PathBuf> {
    inner_select_folder(title, location, flags, utils::fs::Navigator::new())
}