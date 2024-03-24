mod dialog_buttons;
mod dialog_result;
mod generic_alert_dialog;
#[cfg(test)]
mod tests;

use dialog_buttons::DialogButtons;
use dialog_result::DialogResult;
use generic_alert_dialog::GenericAlertDialog;

use crate::prelude::{window, ModalWindowMethods};

pub fn error(title: &str, caption: &str)  {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::Ok, window::Type::Error);
    w.show();
}
pub fn retry(title: &str, caption: &str)->bool  {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::RetryCancel, window::Type::Error);
    if let Some(result) = w.show() {
        return result == DialogResult::Retry;
    }
    return false;
}
pub fn alert(title: &str, caption: &str)  {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::Ok, window::Type::Warning);
    w.show();
}
pub fn message(title: &str, caption: &str)  {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::Ok, window::Type::Notification);
    w.show();
}