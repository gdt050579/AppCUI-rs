mod dialog_buttons;
mod dialog_result;
mod file_info;
mod file_mask;
mod generic_alert_dialog;
mod open_save_dialog;
#[cfg(test)]
mod tests;

use dialog_buttons::DialogButtons;
use dialog_result::DialogResult;
use file_info::FileInfo;
use file_mask::FileMask;
use generic_alert_dialog::GenericAlertDialog;
use open_save_dialog::FileExplorer;

use crate::prelude::{window, ModalWindowMethods};

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

pub fn save(file_name: &str, root: &str, extension_mask: &str /*flags: u32*/) -> Option<String> {
    match FileMask::parse(extension_mask) {
        Ok(mask_list) => {
            //let w = FileExplorer::new("Save", mask_list);
            //w.show();
            None
        }
        Err(err_msg) => {
            panic!(
                "Error parsing file mask: '{}'. It should be in the format 'name1 = [ext1, ext2, ... extn], name2 = [...], ...'.\n{}",
                extension_mask, err_msg
            );
        }
    }
}
