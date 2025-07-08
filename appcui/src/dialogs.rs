//! Dialog system for AppCUI applications.
//!
//! This module provides a set of predefined modal windows that are common when using a UI system:
//! * **Notification dialogs** - Show errors, warnings, messages, or ask for validation
//! * **File dialogs** - Allow users to select files to open or save
//! * **Folder selection dialogs** - Allow users to select folders
//!
//! # Notification Dialogs
//!
//! The module provides several functions for displaying notifications with different severity levels:
//! * [`error`] - Shows an error message with an "Ok" button
//! * [`retry`] - Shows an error message with "Retry" and "Cancel" buttons
//! * [`alert`] - Shows a warning message with an "Ok" button
//! * [`proceed`] - Shows a warning message with "Yes" and "No" buttons
//! * [`message`] - Shows an information message with an "Ok" button
//! * [`validate`] - Shows a question with "Yes" and "No" buttons
//! * [`validate_or_cancel`] - Shows a question with "Yes", "No", and "Cancel" buttons
//!
//! # File Dialogs
//!
//! For file operations, the module offers:
//! * [`open`] - A dialog for selecting a file to open
//! * [`save`] - A dialog for selecting a location to save a file
//!
//! # Folder Selection Dialogs
//!
//! For folder selection:
//! * [`select_folder`] - A dialog for selecting a folder
//!
//! # Examples
//!
//! ```rust,no_run
//! use appcui::dialogs;
//!
//! // Show a simple error message
//! dialogs::error("Error", "An error has occurred");
//!
//! // Ask the user a yes/no question
//! if dialogs::validate("Confirm", "Do you want to proceed?") {
//!     // User clicked "Yes"
//! } else {
//!     // User clicked "No" or closed the dialog
//! }
//!
//! // Open a file dialog
//! if let Some(file_path) = dialogs::open("Open File",
//!                                        "document.txt",
//!                                        dialogs::Location::Current,
//!                                        Some("Text files = [txt]"),
//!                                        dialogs::OpenFileDialogFlags::Icons)
//! {
//!     // User selected a file
//!     println!("Selected file: {:?}", file_path);
//! }
//! ```
mod dialog_buttons;
mod dialog_result;
mod file_mask;
mod folder_select_dialog;
mod generic_alert_dialog;
mod input_dialog;
mod open_save_dialog;
mod root_select_dialog;
#[cfg(test)]
mod tests;

use std::{path::{Path, PathBuf}, str::FromStr};

use crate::{
    prelude::{window, ModalWindowMethods},
    utils::{self, Navigator},
};
use dialog_buttons::DialogButtons;
use dialog_result::DialogResult;
use file_mask::FileMask;
use folder_select_dialog::{FolderExplorer, FolderSelectionDialogResult};
use generic_alert_dialog::GenericAlertDialog;
use input_dialog::StringImputDialog;
use open_save_dialog::{FileExplorer, OpenSaveDialogResult};
use EnumBitFlags::EnumBitFlags;

/// Result of a validation dialog with a cancel option.
///
/// This enum represents the possible outcomes when a dialog with "Yes", "No",
/// and "Cancel" buttons is displayed.
///
/// # Values
/// * `Yes` - The user clicked the "Yes" button.
/// * `No` - The user clicked the "No" button.
/// * `Cancel` - The user clicked the "Cancel" button or closed the dialog.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ValidateOrCancelResult {
    Yes,
    No,
    Cancel,
}

/// Displays an error dialog with an "Ok" button.
///
/// This function shows a modal error dialog with the specified title and message.
/// The dialog will have a single "Ok" button and will block until the user dismisses it.
///
/// # Arguments
/// * `title` - The title of the dialog.
/// * `caption` - The message to display in the dialog.
///
/// # Example
/// ```rust,no_run
/// use appcui::dialogs;
///
/// dialogs::error("Error", "An error has occurred during the last operation");
/// ```
pub fn error(title: &str, caption: &str) {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::Ok, window::Type::Error);
    w.show();
}

/// Displays an error dialog with "Retry" and "Cancel" buttons.
///
/// This function shows a modal error dialog with the specified title and message.
/// The dialog will have "Retry" and "Cancel" buttons and will block until the user
/// makes a selection.
///
/// # Arguments
/// * `title` - The title of the dialog.
/// * `caption` - The message to display in the dialog.
///
/// # Returns
/// * `true` - If the user clicked the "Retry" button.
/// * `false` - If the user clicked the "Cancel" button or closed the dialog.
///
/// # Example
/// ```rust,no_run
/// use appcui::dialogs;
///
/// if dialogs::retry("Error", "An error occurred while performing a copy operation.\nRetry again?") {
///     // Retry the operation
/// }
/// ```
pub fn retry(title: &str, caption: &str) -> bool {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::RetryCancel, window::Type::Error);
    if let Some(result) = w.show() {
        return result == DialogResult::Retry;
    }
    false
}

/// Displays an alert dialog with an "Ok" button.
///
/// This function shows a modal warning dialog with the specified title and message.
/// The dialog will have a single "Ok" button and will block until the user dismisses it.
///
/// # Arguments
/// * `title` - The title of the dialog.
/// * `caption` - The message to display in the dialog.
///
/// # Example
/// ```rust,no_run
/// use appcui::dialogs;
///
/// dialogs::alert("Warning", "Low disk space detected");
/// ```
pub fn alert(title: &str, caption: &str) {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::Ok, window::Type::Warning);
    w.show();
}

/// Displays an alert dialog with "Yes" and "No" buttons.
///
/// This function shows a modal warning dialog with the specified title and message.
/// The dialog will have "Yes" and "No" buttons and will block until the user
/// makes a selection.
///
/// # Arguments
/// * `title` - The title of the dialog.
/// * `caption` - The message to display in the dialog.
///
/// # Returns
/// * `true` - If the user clicked the "Yes" button.
/// * `false` - If the user clicked the "No" button or closed the dialog.
///
/// # Example
/// ```rust,no_run
/// use appcui::dialogs;
///
/// if dialogs::proceed("Warning", "An error occurred while performing a copy operation.\nContinue anyway?") {
///     // Continue with the operation
/// }
/// ```
pub fn proceed(title: &str, caption: &str) -> bool {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::YesNo, window::Type::Warning);
    if let Some(result) = w.show() {
        return result == DialogResult::Yes;
    }
    false
}

/// Displays a notification dialog with an "Ok" button.
///
/// This function shows a modal notification dialog with the specified title and message.
/// The dialog will have a single "Ok" button and will block until the user dismisses it.
///
/// # Arguments
/// * `title` - The title of the dialog.
/// * `caption` - The message to display in the dialog.
///
/// # Example
/// ```rust,no_run
/// use appcui::dialogs;
///
/// dialogs::message("Success", "All files have been copied");
/// ```
pub fn message(title: &str, caption: &str) {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::Ok, window::Type::Notification);
    w.show();
}

/// Displays a validation dialog with "Yes" and "No" buttons.
///
/// This function shows a modal notification dialog with the specified title and message.
/// The dialog will have "Yes" and "No" buttons and will block until the user
/// makes a selection.
///
/// # Arguments
/// * `title` - The title of the dialog.
/// * `caption` - The message to display in the dialog.
///
/// # Returns
/// * `true` - If the user clicked the "Yes" button.
/// * `false` - If the user clicked the "No" button or closed the dialog.
///
/// # Example
/// ```rust,no_run
/// use appcui::dialogs;
///
/// if dialogs::validate("Question", "Are you sure you want to proceed?") {
///     // Start the action
/// }
/// ```
pub fn validate(title: &str, caption: &str) -> bool {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::YesNo, window::Type::Notification);
    if let Some(result) = w.show() {
        return result == DialogResult::Yes;
    }
    false
}

/// Displays a validation dialog with "Yes", "No", and "Cancel" buttons.
///
/// This function shows a modal notification dialog with the specified title and message.
/// The dialog will have "Yes", "No", and "Cancel" buttons and will block until the user
/// makes a selection.
///
/// # Arguments
/// * `title` - The title of the dialog.
/// * `caption` - The message to display in the dialog.
///
/// # Returns
/// A `ValidateOrCancelResult` indicating which button was clicked:
/// * `ValidateOrCancelResult::Yes` - If the user clicked the "Yes" button.
/// * `ValidateOrCancelResult::No` - If the user clicked the "No" button.
/// * `ValidateOrCancelResult::Cancel` - If the user clicked the "Cancel" button or closed the dialog.
///
/// # Example
/// ```rust,no_run
/// use appcui::dialogs;
/// use appcui::dialogs::ValidateOrCancelResult;
///
/// let result = dialogs::validate_or_cancel("Exit", "Do you want to save your files?");
/// match result {
///     ValidateOrCancelResult::Yes => { /* save files and then exit application */ },
///     ValidateOrCancelResult::No => { /* exit the application directly */ },
///     ValidateOrCancelResult::Cancel => { /* don't exit the application */ }
/// }
/// ```
pub fn validate_or_cancel(title: &str, caption: &str) -> ValidateOrCancelResult {
    let w = GenericAlertDialog::new(title, caption, DialogButtons::YesNoCancel, window::Type::Notification);
    match w.show() {
        Some(DialogResult::Yes) => ValidateOrCancelResult::Yes,
        Some(DialogResult::No) => ValidateOrCancelResult::No,
        _ => ValidateOrCancelResult::Cancel,
    }
}

/// Specifies the initial location for file and folder selection dialogs.
///
/// This enum represents different ways to specify where file and folder
/// selection dialogs should start browsing.
///
/// # Variants
/// * `Current` - Start in the current working directory.
/// * `Last` - Start in the last location used in a previous dialog. If no previous dialog
///   has been opened, falls back to the current directory.
/// * `Path` - Start in the specified path.
///
/// # Example
/// ```rust,no_run
/// use appcui::dialogs;
/// use std::path::Path;
///
/// // Start in a specific directory
/// let specific_path = Path::new("C:/Users/Documents");
/// let location = dialogs::Location::Path(specific_path);
///
/// // Start in the current directory
/// let current_location = dialogs::Location::Current;
///
/// // Start in the last used location
/// let last_location = dialogs::Location::Last;
/// ```
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

#[EnumBitFlags(bits = 8)]
pub enum SelectFolderDialogFlags {
    Icons = 1,
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
                "Error parsing file mask: '{ext_mask}'. It should be in the format 'name1 = [ext1, ext2, ... extn], name2 = [...], ...'.\n{err_msg}"
            );
        }
    }
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
                "Error parsing file mask: '{ext_mask}'. It should be in the format 'name1 = [ext1, ext2, ... extn], name2 = [...], ...'.\n{err_msg}"
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
/// ```
pub fn select_folder(title: &str, location: Location, flags: SelectFolderDialogFlags) -> Option<PathBuf> {
    inner_select_folder(title, location, flags, utils::fs::Navigator::new())
}

pub fn input<T>(title: &str, text: &str, value: Option<T>, validation: Option<fn(&T) -> Result<(), String>>) -> Option<T>
where
    T: FromStr + Sized + std::fmt::Display + 'static,
{
    StringImputDialog::new(title, text, value, validation).show()
}
