pub mod dialog_buttons;
pub mod dialog_result;
mod generic_alert_dialog;

pub use dialog_buttons::DialogButtons;
pub use dialog_result::DialogResult;
use generic_alert_dialog::GenericAlertDialog;

use crate::prelude::{window, ModalWindowMethods};

pub fn error(title: &str, caption: &str, buttons: DialogButtons) -> DialogResult {
    let w = GenericAlertDialog::new(title, caption, buttons, window::Type::Error);
    if let Some(result) = w.show() {
        return result;
    }
    return DialogResult::Cancel;
}
