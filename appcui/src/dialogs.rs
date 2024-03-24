mod dialog_buttons;
mod dialog_result;
mod generic_alert_dialog;
#[cfg(test)]
mod tests;

use dialog_buttons::DialogButtons;
use dialog_result::DialogResult;
use generic_alert_dialog::GenericAlertDialog;

use crate::prelude::{window, ModalWindowMethods};

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum ValidateOrCancelResult {
    Yes,
    No,
    Cancel
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
            _ => return ValidateOrCancelResult::Cancel
        }
    }
    ValidateOrCancelResult::Cancel
}