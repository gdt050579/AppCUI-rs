mod dialog_buttons;
mod dialog_result;
mod file_mask;
mod generic_alert_dialog;
mod open_save_dialog;
mod root_select_dialog;
#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use crate::{
    prelude::{window, ModalWindowMethods},
    utils,
};
use dialog_buttons::DialogButtons;
use dialog_result::DialogResult;
use file_mask::FileMask;
use generic_alert_dialog::GenericAlertDialog;
use open_save_dialog::{FileExplorer, OpenSaveDialogResult, InnerFlags};
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

static VFS: &str = include_str!("E:\\Lucru\\Personal\\AppCUI-rs\\scripts\\vfs.csv");

pub enum Location<'a> {
    Current,
    Last,
    Path(&'a Path),
}

#[EnumBitFlags(bits = 8)]
pub enum SaveFileDialogFlags {
    Icons = 1,
    ValidateOverwrite = 2,
    SaveAs = 4,
}
// save:
//     - file_name: &str
//     - loction: Current, Last, Specific (path)
//     - extension_mask: Option<&str> (None inseamna all files)
//     - flags: u32 -> Icons, ValidateOverwrite, SaveAs (daca e setat, se va deschide cu numele save as, altfel cu save)
// returneaza: Option<PathBuf>
// DisableSelection trebuie adaugat si setat la ListView
// Button de new folder

// open:
//     - file_name: &str
//     - location: Current, Last, Specific (path)
//     - extension_mask: Option<&str> (None inseamna all files)
//     - flags: u32 -> Icons,
// returneaza: Option<PathBuf>
// DisableSelection trebuie adaugat si setat la ListView

// open_multiple:
//     - location: Current, Last, Specific (path)
//     - extension_mask: Option<&str> (None inseamna all files)
//     - flags: u32 -> Icons
// returneaza: Option<Vec<PathBuf>>
// nu are campul de file_name
pub(super) fn inner_save<T>(file_name: &str, location: Location, extension_mask: Option<&str>, flags: SaveFileDialogFlags, nav: T) -> Option<PathBuf>
where
    T: crate::utils::Navigator<crate::utils::fs::Entry, crate::utils::fs::Root, PathBuf> + 'static,
{
    let ext_mask = extension_mask.unwrap_or_default();
    match FileMask::parse(ext_mask) {
        Ok(mask_list) => {
            let title = if flags.contains(SaveFileDialogFlags::SaveAs) {
                "Save As"
            } else {
                "Save"
            };
            let mut inner_flags = InnerFlags::Save;
            if flags.contains(SaveFileDialogFlags::Icons) {
                inner_flags |= InnerFlags::Icons;
            }
            if flags.contains(SaveFileDialogFlags::ValidateOverwrite) {
                inner_flags |= InnerFlags::ValidateOverwrite;
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

pub fn save(file_name: &str, location: Location, extension_mask: Option<&str>, flags: SaveFileDialogFlags) -> Option<PathBuf> {
    inner_save(file_name, location, extension_mask, flags, utils::fs::NavSimulator::with_csv(VFS, true))
}
