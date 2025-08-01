use std::path::PathBuf;

use crate::dialogs;
use crate::prelude::*;

use super::FileMask;
use super::OpenFileDialogFlags;
use super::SaveFileDialogFlags;
use super::SelectFolderDialogFlags;

#[Window(events=ButtonEvents, internal: true)]
struct CallbackWin {
    f: fn(),
}
impl CallbackWin {
    fn new(f: fn()) -> Self {
        let mut w = Self {
            base: window!("Test,a:c,w:20,h:6"),
            f,
        };
        w.add(button!("'Press Me',a:c,w:15,h:100%"));
        w
    }
}
impl ButtonEvents for CallbackWin {
    fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
        (self.f)();
        EventProcessStatus::Processed
    }
}

static VFS: &str = "
    r,C:\\,1000000,100000,SYSTEM,fixed   
    r,D:\\,123456,123,USB Drive,removable
    d,C:\\Program Files,0,2024-01-10 12:00:00,
    f,C:\\Program Files\\runme.exe,123,2024-01-10 12:31:55,
    f,C:\\Program Files\\readme.txt,123456,2023-02-05 09:12:25,
    d,C:\\Program Files\\Windows,0,2024-01-10 12:31:55,
    f,C:\\Program Files\\Windows\\picture.png,123456,2020-03-12 12:31:55,
    f,C:\\Program Files\\Windows\\melody.mp3,0,2019-03-12 12:31:55,
    f,C:\\Program Files\\Windows\\script.bat,10000,2023-08-11 11:11:11,
    d,C:\\Program Files\\Windows\\System32,0,2020-03-12 12:31:55,
    f,C:\\Program Files\\Windows\\System32\\cmd.exe,123456,2020-03-12 22:15:45,
    f,C:\\Program Files\\Windows\\System32\\notepad.exe,123456,2020-05-14 12:18:55,
    f,C:\\Program Files\\Windows\\System32\\calc.exe,123456,2022-05-14 12:19:35,
    d,C:\\Program Files\\Windows\\System32\\drivers,0,2022-05-14 12:19:35,
    f,C:\\Program Files\\Windows\\System32\\drivers\\file.sys,13579,2022-05-14 12:19:35,
    f,C:\\Program Files\\Windows\\System32\\drivers\\graphics.sys,12345,2021-08-14 12:19:35,
    f,C:\\Program Files\\Windows\\System32\\drivers\\network.sys,54321,2020-10-14 12:19:35,
    f,D:\\runme.exe,123,2024-01-10 12:31:55,
    f,D:\\readme.txt,123456,2023-02-05 09:12:25,
    d,D:\\Windows,0,2024-01-10 12:31:55,
    f,D:\\Windows\\picture.png,123456,2020-03-12 12:31:55,
    f,D:\\Windows\\melody.mp3,0,2019-03-12 12:31:55,
";

static FILE_MASK: &str = "Images = [jpg,png,bmp], 
                          Documents = [txt,docx], 
                          Executable and scripts = [exe,dll,js,py,ps1,sh,bat,cmd], 
                          Presentation = [ppt]";

enum OpenSaveTestWindowFlags {
    Save(dialogs::SaveFileDialogFlags),
    Open(dialogs::OpenFileDialogFlags),
}
#[Window(events = ButtonEvents, internal: true)]
struct OpenSaveTestWindow<'a> {
    title: String,
    location: dialogs::Location<'a>,
    file_name: String,
    flags: OpenSaveTestWindowFlags,
    info: Handle<Label>,
    mask: Option<&'static str>,
}

impl<'a> OpenSaveTestWindow<'a> {
    fn save(title: &str, file_name: &str, location: dialogs::Location<'a>, save_flags: dialogs::SaveFileDialogFlags) -> Self {
        let mut w = Self {
            base: window!("Test, d:f"),
            title: title.to_string(),
            location,
            file_name: file_name.to_string(),
            flags: OpenSaveTestWindowFlags::Save(save_flags),
            info: Handle::None,
            mask: Some(FILE_MASK),
        };
        w.add(button!("'Press Me',a:c,w:14,h:100%"));
        w.info = w.add(label!("'',x:0,y:0,w:100%,h:2"));
        w
    }
    fn open(title: &str, file_name: &str, location: dialogs::Location<'a>, open_flags: dialogs::OpenFileDialogFlags) -> Self {
        let mut w = Self {
            base: window!("Test, d:f"),
            title: title.to_string(),
            location,
            file_name: file_name.to_string(),
            flags: OpenSaveTestWindowFlags::Open(open_flags),
            info: Handle::None,
            mask: Some(FILE_MASK),
        };
        w.add(button!("'Press Me',a:c,w:14,h:100%"));
        w.info = w.add(label!("'',x:0,y:0,w:100%,h:2"));
        w
    }
    fn open_all(title: &str, file_name: &str, location: dialogs::Location<'a>, open_flags: dialogs::OpenFileDialogFlags) -> Self {
        let mut w = Self {
            base: window!("Test, d:f"),
            title: title.to_string(),
            location,
            file_name: file_name.to_string(),
            flags: OpenSaveTestWindowFlags::Open(open_flags),
            info: Handle::None,
            mask: None,
        };
        w.add(button!("'Press Me',a:c,w:14,h:100%"));
        w.info = w.add(label!("'',x:0,y:0,w:100%,h:2"));
        w
    }
}

impl ButtonEvents for OpenSaveTestWindow<'_> {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        let nav = crate::utils::fs::NavSimulator::with_csv(VFS, true, "C:\\Program Files\\");
        let result = match self.flags {
            OpenSaveTestWindowFlags::Save(flags) => {
                dialogs::inner_save(self.title.as_str(), self.file_name.as_str(), self.location.clone(), self.mask, flags, nav)
            }
            OpenSaveTestWindowFlags::Open(flags) => {
                dialogs::inner_open(self.title.as_str(), self.file_name.as_str(), self.location.clone(), self.mask, flags, nav)
            }
        };
        let txt = format!("{result:?}");
        let h = self.info;
        if let Some(info) = self.control_mut(h) {
            info.set_caption(&txt);
        }
        EventProcessStatus::Processed
    }
}

#[Window(events = ButtonEvents, internal: true)]
struct FolderSelectDialog {
    loc: String,
    flags: SelectFolderDialogFlags,
}
impl FolderSelectDialog {
    fn new(loc: &str, flags: SelectFolderDialogFlags) -> Self {
        let mut w = Self {
            base: window!("Test,d:f"),
            loc: loc.to_string(),
            flags,
        };
        w.add(button!("Press,a:c,w:14,h:100%"));
        w
    }
}
impl ButtonEvents for FolderSelectDialog {
    fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
        let nav = crate::utils::fs::NavSimulator::with_csv(VFS, true, "C:\\");
        let p = PathBuf::from(self.loc.as_str());
        let loc = match self.loc.as_str() {
            "" => dialogs::Location::Last,
            "." => dialogs::Location::Current,
            _ => dialogs::Location::Path(&p),
        };
        if let Some(result) = dialogs::inner_select_folder("Folder", loc, self.flags, nav) {
            self.set_title(&format!("{result:?}"));
        } else {
            self.set_title("Folder selection canceled !");
        }
        EventProcessStatus::Processed
    }
}

#[test]
fn check_small_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0x847C1B71CF1BAB79)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::error("Error", "123")));
    a.run();
}

#[test]
fn check_large_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0x563DF7AC2DDD7DAE)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| {
        dialogs::error("Error", "An error has occured while running the code.")
    }));
    a.run();
}

#[test]
fn check_very_large_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0xF83F2AE0FC4EC4ED)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| {
        dialogs::error(
            "Error",
            "An error has occured while running the code. Because of this certain operations are no longer possible.",
        )
    }));
    a.run();
}

#[test]
fn check_too_large_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        // should print:
        // An error has occured while running the code.
        // Because of this certain operations are no 
        // longer possible. All connection to the
        Paint('Message box');
        CheckHash(0xD8F2736351150900)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::error("Error", "An error has occured while running the code. Because of this certain operations are no longer possible. All connection to the database have been stop and the file system has been reverted to its original state before this operation has started !")));
    a.run();
}

#[test]
fn check_multi_line_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x7948D22070789869)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0x3E34FCDDB9B49649)
    ";
    let mut a = App::debug(60, 16, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::error("Error", "An error has occured during the last operarion. To recover perform the following:\n1. Run the diagnostics\n2. Restart the computer\n3. Reboot")));
    a.run();
}

#[test]
fn check_return_from_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0x847C1B71CF1BAB79)
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::error("Error", "123")));
    a.run();
}

#[test]
fn check_retry_error() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry');
        CheckHash(0x7C19591705072D6D)
        Key.Pressed(Enter)
        Paint('Back to initial State (result is cancel)')   
        CheckHash(0x937CE126B66578D9)
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry (second time)');
        CheckHash(0x7C19591705072D6D)
        Key.Pressed(Escape)
        Paint('Back to initial State (after escape)')   
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry (third time)');
        CheckHash(0x7C19591705072D6D)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('Now we need to retry');
        CheckHash(0xC88A5ABECB445F81)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| {
        if dialogs::retry("Error", "An error occured. Retry ?") {
            dialogs::message("Response", "We should retry.")
        } else {
            dialogs::message("Response", "Stop the action.")
        }
    }));
    a.run();
}

#[test]
fn check_alert() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0xBCA6A406AE5AE98E)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::alert("Error", "A problem occured while running the code.")));
    a.run();
}

#[test]
fn check_proceed() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('No (do not continue)');
        CheckHash(0x24B39A0A49793368)
        Key.Pressed(Enter)
        Paint('Back to initial State (result is cancel)')   
        CheckHash(0x937CE126B66578D9)
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry (second time)');
        CheckHash(0x24B39A0A49793368)
        Key.Pressed(Escape)
        Paint('Back to initial State (after escape)')   
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry (third time)');
        CheckHash(0x24B39A0A49793368)
        Key.Pressed(Tab)
        Paint('Yes button selected')
        CheckHash(0x7DB1B8B269F1DAB8)
        Key.Pressed(Enter)
        Paint('Now we should continue');
        CheckHash(0xF8045E482E522D83)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| {
        if dialogs::proceed("Alert", "An problem occured.\nContinue anyway ?") {
            dialogs::message("Response", "We should continue.")
        } else {
            dialogs::message("Response", "Stop the action.")
        }
    }));
    a.run();
}

#[test]
fn check_message() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Message box');
        CheckHash(0xE2E128A51D518819)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| dialogs::message("Success", "Operation completed succesifully.")));
    a.run();
}

#[test]
fn check_validate() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('No (do not start the action)');
        CheckHash(0xA081FA6B7B9761A1)
        Key.Pressed(Enter)
        Paint('Back to initial State (result is cancel)')   
        CheckHash(0x937CE126B66578D9)
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry (second time)');
        CheckHash(0xA081FA6B7B9761A1)
        Key.Pressed(Escape)
        Paint('Back to initial State (after escape)')   
        Key.Pressed(Enter)
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Retry (third time)');
        CheckHash(0xA081FA6B7B9761A1)
        Key.Pressed(Tab)
        Paint('Yes button selected')
        CheckHash(0x1D8F2EAA023E3949)
        Key.Pressed(Enter)
        Paint('Now we should start the action');
        CheckHash(0x893A6F5432DC3312)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| {
        if dialogs::validate("Alert", "Are you sure that you want to start the action ?") {
            dialogs::message("Response", "Start the action.")
        } else {
            dialogs::message("Response", "Stop the action.")
        }
    }));
    a.run();
}

#[test]
fn check_validate_or_cancel() {
    let script = "
        Paint.Enable(false)
        Paint('Initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Save all files ?');
        CheckHash(0xAEC8093ED340EABD)    
        Key.Pressed(Enter)
        Paint('Cancel exit')  
        CheckHash(0xE75AE21563966113)
        Key.Pressed(Enter)   
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Save all files ?');
        CheckHash(0xAEC8093ED340EABD) 
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('Save, then exit')
        CheckHash(0xB4DED0FD1E127409) 
        Key.Pressed(Enter)   
        Paint('Back to initial State')   
        CheckHash(0x90DB478C0FC0C3A9)
        Key.Pressed(Enter)
        Paint('Save all files ?');
        CheckHash(0xAEC8093ED340EABD) 
        Key.Pressed(Tab,2)
        Key.Pressed(Enter)
        Paint('Exit without saving')
        CheckHash(0x1E7B8615E4DBEF3F)  
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(CallbackWin::new(|| {
        let result = dialogs::validate_or_cancel("Exit", "Save all files ?");
        match result {
            dialogs::ValidateOrCancelResult::Yes => dialogs::message("Response", "Save, then exit"),
            dialogs::ValidateOrCancelResult::No => dialogs::message("Response", "Exit without saving"),
            dialogs::ValidateOrCancelResult::Cancel => dialogs::message("Response", "Cancel exit"),
        }
    }));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_save_dialog_select_existent() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show save dialog');
        CheckHash(0x21756A7594358988)    
        Key.Pressed(Alt+T)
        Paint('3. Open type list');
        CheckHash(0x45D8222BE460849C)   
        Key.Pressed(End)
        Key.Pressed(Enter) 
        Paint('4. All files selected');
        CheckHash(0x5F99B748EA7DF5CD)
        Key.Pressed(Tab,4)   
        Paint('5. File list has focus');
        CheckHash(0xFF7E9BC315253CE1)
        Key.Pressed(Down,3)   
        Paint('6. readme.txt is selected');
        CheckHash(0x66FB1C04540FF3B8)
        Key.Pressed(Enter)   
        Paint('7. readme.txt is chosen');
        CheckHash(0xEB21471DE6FDA1EA)
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::save(
        "Save",
        "blabla.exe",
        dialogs::Location::Current,
        SaveFileDialogFlags::None,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_save_dialog_cancelt_existent() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show save dialog');
        CheckHash(0x21756A7594358988)    
        Key.Pressed(Alt+T)
        Paint('3. Open type list');
        CheckHash(0x45D8222BE460849C)   
        Key.Pressed(End)
        Key.Pressed(Enter) 
        Paint('4. All files selected');
        CheckHash(0x5F99B748EA7DF5CD)
        Key.Pressed(Tab,4)   
        Paint('5. File list has focus');
        CheckHash(0xFF7E9BC315253CE1)
        Key.Pressed(Down,3)   
        Paint('6. readme.txt is selected');
        CheckHash(0x66FB1C04540FF3B8)
        Key.Pressed(Escape)   
        Paint('7. readme.txt is chosen');
        CheckHash(0xAD065263787B818A)
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::save(
        "Save",
        "blabla.exe",
        dialogs::Location::Current,
        SaveFileDialogFlags::None,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_save_dialog_select_existent_with_validate_overwrite() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show save dialog');
        CheckHash(0x21756A7594358988)    
        Key.Pressed(Alt+T)
        Paint('3. Open type list');
        CheckHash(0x45D8222BE460849C)   
        Key.Pressed(End)
        Key.Pressed(Enter) 
        Paint('4. All files selected');
        CheckHash(0x5F99B748EA7DF5CD)
        Key.Pressed(Tab,4)   
        Paint('5. File list has focus');
        CheckHash(0xFF7E9BC315253CE1)
        Key.Pressed(Down,3)   
        Paint('6. readme.txt is selected');
        CheckHash(0x66FB1C04540FF3B8)
        Key.Pressed(Enter)   
        Paint('7. Validate overwrite question');
        CheckHash(0xCA110EB5BB5ADEAA)
        Key.Pressed(Tab)
        Key.Pressed(Enter)  
        Key.Pressed(Tab)
        Paint('8. readme.txt is chosen');
        CheckHash(0xEB21471DE6FDA1EA)
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::save(
        "Save",
        "blabla.exe",
        dialogs::Location::Current,
        SaveFileDialogFlags::ValidateOverwrite,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_open_dialog_hardcoded_relative_path() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show open dialog');
        CheckHash(0x301AEE3E32DC3466)    
        Key.Pressed(Enter)
        Paint('3. Selected path: Some(C:\\abc.exe)');
        CheckHash(0x66405B20EE6A5135)            
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::open(
        "Open",
        "../abc.exe",
        dialogs::Location::Current,
        OpenFileDialogFlags::None,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_open_dialog_hardcoded_absolute_path() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show open dialog');
        CheckHash(0x9D35411CA46289A9)    
        Key.Pressed(Enter)
        Paint('3. Selected path: Some(E:\\abc.exe)');
        CheckHash(0x1F861B0B7CF0B263)            
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::open(
        "Open",
        "E:/abc.exe",
        dialogs::Location::Current,
        OpenFileDialogFlags::None,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_open_dialog_invalid_path_with_validation_flag() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show open dialog');
        CheckHash(0x9D35411CA46289A9)    
        Key.Pressed(Enter)
        Paint('3. Error (File E:\\abc.exe does not exists)');
        CheckHash(0xEF1EBEE444B0A935) 
        Key.Pressed(Enter)
        Paint('4. back to open dialog window');
        CheckHash(0x9D35411CA46289A9)    
        Key.Pressed(Escape)
        Paint('5. No file selected (None)');
        CheckHash(0xAD065263787B818A)    
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::open(
        "Open",
        "E:/abc.exe",
        dialogs::Location::Current,
        OpenFileDialogFlags::CheckIfFileExists,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_open_dialog_last_path() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show open dialog');
        CheckHash(0x6D0409A7FCA6AE9B)    
        Key.Pressed(Tab,6)
        Paint('3. Focus on File list');
        CheckHash(0x7E2C45D95E26E3FB) 
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Key.Pressed(Tab)
        Paint('4. Back on the file name list - folder is C:\\Program Files\\Windows');
        CheckHash(0x830F7746A03258A2) 
        Key.Pressed(Enter)
        Paint('5. Selected file is Some(C:\\Program Files\\Windows\\myfile.exe)');
        CheckHash(0x3703567C51F0C71A)                    
        Key.Pressed(Enter)
        Paint('6. Open the file dialog again (Directory should be C:\\Program Files\\Windows)');
        CheckHash(0xC8E329F1E80B6D04)                    
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::open(
        "Open",
        "myfile.exe",
        dialogs::Location::Last,
        OpenFileDialogFlags::None,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_open_dialog_select_drive() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show open dialog');
        CheckHash(0x2328A3E49BBF7A06)    
        Key.Pressed(Alt+D)
        Paint('3. Drive selection window is chosen');
        CheckHash(0xB3EBC64EAA555682) 
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('4. Now the folder is D:\');
        CheckHash(0x9C98C24AA885FA47) 
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::open_all(
        "Open",
        "myfile.exe",
        dialogs::Location::Current,
        OpenFileDialogFlags::None,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_open_dialog_change_path_manually() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show open dialog');
        CheckHash(0x2328A3E49BBF7A06)    
        Key.Pressed(Tab,5)
        Paint('3. PathFinder is selected (C:\\Program Files)');
        CheckHash(0xCE4C8945F94D4A52) 
        // delete the entire content
        Key.Pressed(Backspace,100)
        Key.TypeText('C:\\Program Files\\Windows\\System32\\drivers')
        Key.Pressed(Enter)
        Key.Pressed(Tab);
        Paint('4. Showing files from C:\\Program Files\\Windows\\System32\\drivers');
        CheckHash(0xB593E849F4871BAD) 
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::open_all(
        "Open",
        "myfile.exe",
        dialogs::Location::Current,
        OpenFileDialogFlags::None,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_open_dialog_select_drive_with_mouse() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show open dialog');
        CheckHash(0x2328A3E49BBF7A06)    
        Key.Pressed(Alt+D)
        Paint('3. Drive selection window is chosen');
        CheckHash(0xB3EBC64EAA555682) 
        Mouse.Click(45,20,left)
        Paint('4. Back to open dialog');
        CheckHash(0x6EA9497308742D9A) 
        Mouse.Click(10,7,left)    
        Paint('5. Back to drive selection dialog');
        CheckHash(0xB3EBC64EAA555682)  
        Mouse.Click(22,13,left)
        Mouse.Click(30,20,left)          
        Paint('6. Now the folder is D:\');
        CheckHash(0x9C98C24AA885FA47) 
    ";

    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::open_all(
        "Open",
        "myfile.exe",
        dialogs::Location::Last,
        OpenFileDialogFlags::None,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_save_dialog_navigate() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show save dialog');
        CheckHash(0x6D6E678D8633D6C1)   
        Key.Pressed(Tab,6) 
        Paint('3. Focus on file list');
        CheckHash(0x78626038BB410A89)
        Key.Pressed(Enter)  
        Paint('4. Now path is C:\\');
        CheckHash(0x680204A38D010F47)
        Key.Pressed(Alt+T)
        Paint('5. Open type list');
        CheckHash(0x787D5122AD4B01F4)
        Key.Pressed(Down,4)
        Paint('6. Select all files');
        CheckHash(0x16EAEE304AD5967A)
        Key.Pressed(Enter)
        Key.Pressed(Tab,4)
        Paint('7. File list has focus');
        CheckHash(0x3235F6FF0D20FA17)
        Key.Pressed(Enter)
        Paint('8. C:\\Program Files selected');
        CheckHash(0x492F4FCA68F37B10)
        Key.Pressed(End)
        Key.Pressed(Enter)
        Paint('9. Check is readme.txt can be overwritten');
        CheckHash(0xCA110EB5BB5ADEAA)  
        Key.Pressed(Escape)
        Key.TypeText('run')
        Paint('10. Now focus on runme.exe');
        CheckHash(0xC00E126B14811E14)   
        Key.Pressed(Enter)
        Paint('11. Ask to overwrite Runme.exe');
        CheckHash(0xCA110EB5BB5ADEAA)
        Key.Pressed(Left)
        Key.Pressed(Enter)
        Key.Pressed(Tab)
        Paint('12. Return with Runme.exe as selection');
        CheckHash(0xEB21471DE6FDA1EA)
    ";

    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::save(
        "Save",
        "myfile.exe",
        dialogs::Location::Last,
        SaveFileDialogFlags::Icons | SaveFileDialogFlags::ValidateOverwrite,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_create_folder_select_dialog() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0xDC27AD6BE7A637F4)
        Key.Pressed(Enter)
        Paint('2. Folder Select Dialog shown')   
        CheckHash(0xD04C3E714AC5A212)
        Key.Pressed(Space)
        Paint('3. Program Files expanded')   
        CheckHash(0xB4761C157E0BC65C)
        Key.Pressed(Down)
        Paint('4. Windows selected')   
        CheckHash(0x5DD2E869CAAA2C2F)
        Key.Pressed(Enter)
        Paint('5. `C:\\Program Files\\Windows` returned')   
        CheckHash(0x57FDC0A388354481)
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(FolderSelectDialog::new("C:\\Program Files\\", SelectFolderDialogFlags::None));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_expand_collapse_select_dialog() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0xDC27AD6BE7A637F4)
        Key.Pressed(Enter)
        Paint('2. Folder Select Dialog shown')   
        CheckHash(0xD04C3E714AC5A212)
        Key.Pressed(Space)
        Paint('3. Program Files expanded')   
        CheckHash(0xB4761C157E0BC65C)
        Key.Pressed(Down)
        Paint('4. Windows selected')   
        CheckHash(0x5DD2E869CAAA2C2F)
        Key.Pressed(Space)
        Paint('5. `Windows` expanded')   
        CheckHash(0xE04DF44556D7054B)
        Key.Pressed(Down)
        Key.Pressed(Space)
        Paint('6. `System32` expanded (no children)')   
        CheckHash(0xEC7FE1642903A5BE)
        Key.Pressed(Down,2)
        Key.Pressed(Space)
        Paint('7. `D:` expanded')   
        CheckHash(0x965AB5F9A62E278C)
        Key.Pressed(Down)
        Key.Pressed(Space)
        Paint('8. `Windows` from D expanded (no children)')   
        CheckHash(0xCC5E647E8E8B8379)
        Key.Pressed(Home)
        Key.Pressed(Space)
        Paint('9. `C` is collapsed')   
        CheckHash(0x1EF4952E6256D245)
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(FolderSelectDialog::new("C:\\Program Files\\", SelectFolderDialogFlags::None));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_folder_select_dialog_with_icons() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0xDC27AD6BE7A637F4)
        Key.Pressed(Enter)
        Key.Pressed(Home)
        Paint('2. Folder Select Dialog shown')   
        CheckHash(0x541A8C33A6975193)
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(FolderSelectDialog::new(
        "C:\\Program Files\\Windows\\System32\\drivers",
        SelectFolderDialogFlags::Icons,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_folder_select_dialog_cancel() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0xDC27AD6BE7A637F4)
        Key.Pressed(Enter)
        Paint('2. Folder Select Dialog shown')   
        CheckHash(0xD04C3E714AC5A212)
        Key.Pressed(Space)
        Paint('3. Program Files expanded')   
        CheckHash(0xB4761C157E0BC65C)
        Key.Pressed(Down)
        Paint('4. Windows selected')   
        CheckHash(0x5DD2E869CAAA2C2F)
        Key.Pressed(Escape)
        Paint('5. Nothing selected')   
        CheckHash(0x4BA44613FC503131)
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(FolderSelectDialog::new("C:\\Program Files\\", SelectFolderDialogFlags::None));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_folder_select_dialog_cancel_via_button() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0xDC27AD6BE7A637F4)
        Key.Pressed(Enter)
        Paint('2. Folder Select Dialog shown')   
        CheckHash(0xD04C3E714AC5A212)
        Key.Pressed(Space)
        Paint('3. Program Files expanded')   
        CheckHash(0xB4761C157E0BC65C)
        Key.Pressed(Down)
        Paint('4. Windows selected')   
        CheckHash(0x5DD2E869CAAA2C2F)
        Key.Pressed(Tab,2)
        Key.Pressed(Enter)
        Paint('5. Nothing selected')   
        CheckHash(0x4BA44613FC503131)
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(FolderSelectDialog::new("C:\\Program Files\\", SelectFolderDialogFlags::None));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_folder_select_dialog_ok_via_button() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0xDC27AD6BE7A637F4)
        Key.Pressed(Enter)
        Paint('2. Folder Select Dialog shown')   
        CheckHash(0xD04C3E714AC5A212)
        Key.Pressed(Space)
        Paint('3. Program Files expanded')   
        CheckHash(0xB4761C157E0BC65C)
        Key.Pressed(Down)
        Paint('4. Windows selected')   
        CheckHash(0x5DD2E869CAAA2C2F)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('5. C:\\Program Files\\Windows selected')   
        CheckHash(0x57FDC0A388354481)
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(FolderSelectDialog::new("C:\\Program Files\\", SelectFolderDialogFlags::None));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_folder_select_dialog_navigator() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0xDC27AD6BE7A637F4)
        Key.Pressed(Enter)
        Paint('2. Folder Select Dialog shown')   
        CheckHash(0x9F2DE0204475624D)
        Key.Pressed(Alt+P)
        Key.TypeText('Program Files\\Windows\\')
        Key.Pressed(Enter)
        Paint('3. C:\\Program Files\\Windows selected from Navigator')   
        CheckHash(0x2D7EA2EA5CC21257)
        Key.Pressed(Tab)
        Key.Pressed(Down)
        Paint('4. D:\\ selected')   
        CheckHash(0xBCF70255B3E29322)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('5. D:\\ selected')   
        CheckHash(0xE2640A216D7BFEDC)
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(FolderSelectDialog::new("", SelectFolderDialogFlags::None));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_folder_select_dialog_mouse_usage() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0xDC27AD6BE7A637F4)
        Key.Pressed(Enter)
        Mouse.Wheel(40,15,up,2)
        Paint('2. Folder Select Dialog shown')   
        CheckHash(0xDE086D26B8B708B7)
        Mouse.Click(9,9,left)
        Paint('3. C:\\ expanded')   
        CheckHash(0xDD33C7F7498AD706)
        Mouse.Click(15,10,left)
        Paint('4. C:\\Program Files\\ expanded')   
        CheckHash(0x6F576C7EE1F145D7)
    ";
    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(FolderSelectDialog::new("D:\\Windows", SelectFolderDialogFlags::None));
    a.run();
}

#[test]
fn check_file_mask_errors() {
    let fm = FileMask::parse("abc");
    assert!(fm.is_err());
    assert!(fm
        .err()
        .unwrap()
        .starts_with("Unexpecting end of file mask definition. Expecting a assignment operator ('=' or ':')"));

    let fm = FileMask::parse("###");
    assert!(fm.is_err());
    assert!(fm
        .err()
        .unwrap()
        .starts_with("Expected word character (A-Z, a-z, 0-9, _, -, .) but got invalid character"));

    let fm = FileMask::parse("test:123");
    assert!(fm.is_err());
    assert!(fm
        .err()
        .unwrap()
        .starts_with("Expected open square bracket ('[') but got word character (A-Z, a-z, 0-9, _, -, .)"));

    let fm = FileMask::parse("test:[1,,3]");
    assert!(fm.is_err());
    assert!(fm
        .err()
        .unwrap()
        .starts_with("Expected comma (',') separator or close square bracket (']') but got word character (A-Z, a-z, 0-9, _, -, .)"));

    let fm = FileMask::parse("test:[1,");
    assert!(fm.is_err());
    assert!(fm
        .err()
        .unwrap()
        .starts_with("Unexpecting end of file mask definition. Expecting either a comma (',') separator or a close square bracket (']')"));

    // println!("{}",fm.err().unwrap());
}

#[test]
fn check_file_mask_empty() {
    let fm = FileMask::parse("");
    assert!(fm.is_ok());
    assert!(fm.ok().unwrap().is_empty());
    let fm = FileMask::parse("   ").unwrap();
    assert!(fm.is_empty());

    let fm = FileMask::parse("key = []").unwrap();
    assert!(fm.len() == 1);
    assert!(fm[0].extensions_count() == 0);
    assert!(fm[0].name() == "key");
    assert!(fm[0].matches("test.txt"));
    assert!(fm[0].matches("C:\\windows\\test.txt"));
    assert!(fm[0].matches("/etc/test.txt"));
    assert!(fm[0].matches("test"));
    assert!(fm[0].matches("C:\\windows\\test"));
    assert!(fm[0].matches("/etc/test"));
}

#[test]
fn check_file_mask_array() {
    let fm = FileMask::parse("  first key = [value1,  value2 , value3  ], key2 = [1,2,3,4,5]").unwrap();
    assert_eq!(fm.len(), 2);
    assert_eq!(fm[0].name(), "first key");
    assert_eq!(fm[0].extensions_count(), 3);
    assert_eq!(fm[1].name(), "key2");
    assert_eq!(fm[1].extensions_count(), 5);
}

#[test]
fn check_file_mask_ignore_case() {
    let fm = FileMask::parse("images = [png,JPG]").unwrap();
    assert!(fm.len() == 1);
    assert!(fm[0].extensions_count() == 2);
    assert!(fm[0].name() == "images");
    assert!(fm[0].matches("test.png"));
    assert!(fm[0].matches("test.pNG"));
    assert!(fm[0].matches("test.jpg"));
    assert!(fm[0].matches("test.JPG"));
    assert!(!fm[0].matches("test.png123"));
    assert!(!fm[0].matches("test.JpG123"));
}

#[test]
fn check_extension_list() {
    let fm = FileMask::parse("images = [png,JPG, gif, bmp],video = [avi, mp3, hd_video]").unwrap();
    assert_eq!(fm.len() , 2);
    assert_eq!(fm[0].extensions_count() , 4);
    assert_eq!(fm[0].name() , "images");
    assert_eq!(fm[0].extension(0) , "gif");
    assert_eq!(fm[0].extension(1) , "png");
    assert_eq!(fm[0].extension(2) , "bmp");
    assert_eq!(fm[0].extension(3) , "JPG");
    assert_eq!(fm[1].name() , "video");
    assert_eq!(fm[1].extension(0) , "avi");
    assert_eq!(fm[1].extension(1) , "hd_video");
    assert_eq!(fm[1].extension(2) , "mp3");
}

#[test]
fn check_input_no_validation() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x126FE1C3874DA20C)
        Key.Pressed(Space)
        Paint('2. InputBox opened')   
        CheckHash(0xFDF1E1D1B8DDE540)
        Key.TypeText('123')
        Paint('3. <123> value added')   
        CheckHash(0x43B3BF19A2CA7D30)
        Key.Pressed(Enter)
        Paint('4. Back to main window (title is 123)')   
        CheckHash(0x1E02C66F2D12D285)
        Key.Pressed(Space)
        Paint('5. InputBox opened')   
        CheckHash(0xFDF1E1D1B8DDE540)
        Key.Pressed(Escape)
        Paint('6. Back to main window (title is canceled)')   
        CheckHash(0x42E05DD410E2F1AD)
        Key.Pressed(Space)
        Paint('7. InputBox opened')   
        CheckHash(0xFDF1E1D1B8DDE540)
        Key.TypeText('blablabla')
        Paint('8. <blablabla> invalid value added')   
        CheckHash(0x41A0D938CD104527)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('9. Conversion error showed')   
        CheckHash(0xC1D875ED3682CBF8)
        Key.Pressed(Enter)
        Paint('10. Back to the input box')   
        CheckHash(0xC038D5EFF53F966E)
        Key.Pressed(Tab,2)
        Key.Pressed(Ctrl+A)
        Key.TypeText('-1234')
        Paint('11. <-1234> invalid value added')   
        CheckHash(0x3D95BDAB623C7CD5)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('12. Back to main window (title is -1234)')   
        CheckHash(0x925677BC32B5898)
        Key.Pressed(Enter)
        Paint('13. Input dialog showed')   
        CheckHash(0xFDF1E1D1B8DDE540)
        Key.Pressed(Tab,2)
        Paint('14. Focus on cancel button')   
        CheckHash(0x534E994BE4EED6C9)
        Key.Pressed(Enter)
        Paint('15. Back to main window (title is canceled)')   
        CheckHash(0x42E05DD410E2F1AD)
    ";

    #[Window(events = ButtonEvents, internal: true)]
    struct MyWin {}

    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: window!("Test,a:c,w:30,h:7"),
            };
            me.add(button!("Click,a:c,w:15,h:100%"));
            me
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            let s = if let Some(res) = dialogs::input::<i32>("i32", "Enter a i32 value bigger in the textfield below and press enter", None, None) {
                format!("{res}")
            } else {
                "canceled".to_string()
            };
            self.set_title(&s);
            EventProcessStatus::Processed
        }
    }

    let mut a = App::debug(80, 12, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_input_validation() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x126FE1C3874DA20C)
        Key.Pressed(Space)
        Paint('2. InputBox opened')   
        CheckHash(0x97A2D438E0E074B1)
        Key.TypeText('123')
        Paint('3. <123> value added')   
        CheckHash(0xAEA3A85BCAAAE961)
        Key.Pressed(Enter)
        Paint('4. Validation fails with error')   
        CheckHash(0xA3ED5C76A4A16AF6)
        Key.Pressed(Enter)
        Paint('5. Back to input window')   
        CheckHash(0x117DCE6F7053870E)
        Key.TypeText(50)
        Key.Pressed(Enter)
        Paint('6. Back t original window (title is 50)')   
        CheckHash(0x635EC263AD67AB3)
    ";

    #[Window(events = ButtonEvents, internal: true)]
    struct MyWin {}

    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: window!("Test,a:c,w:30,h:7"),
            };
            me.add(button!("Click,a:c,w:15,h:100%"));
            me
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            let s = if let Some(res) = dialogs::input::<i32>(
                "i32",
                "Enter a i32 value bigger than 10 and smaller than 100",
                None,
                Some(|x: &i32| {
                    if *x > 10 && *x < 100 {
                        Ok(())
                    } else {
                        Err("Value should be between 10 and 123".to_string())
                    }
                }),
            ) {
                format!("{res}")
            } else {
                "canceled".to_string()
            };
            self.set_title(&s);
            EventProcessStatus::Processed
        }
    }

    let mut a = App::debug(80, 12, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_input_with_initial_value() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x126FE1C3874DA20C)
        Key.Pressed(Space)
        Paint('2. InputBox opened (with value 95 written)')   
        CheckHash(0xD718042D771F9CCC)
    ";

    #[Window(events = ButtonEvents, internal: true)]
    struct MyWin {}

    impl MyWin {
        fn new() -> Self {
            let mut me = Self {
                base: window!("Test,a:c,w:30,h:7"),
            };
            me.add(button!("Click,a:c,w:15,h:100%"));
            me
        }
    }
    impl ButtonEvents for MyWin {
        fn on_pressed(&mut self, _: Handle<Button>) -> EventProcessStatus {
            let s = if let Some(res) = dialogs::input::<i32>("i32", "Enter a value:", Some(95), None) {
                format!("{res}")
            } else {
                "canceled".to_string()
            };
            self.set_title(&s);
            EventProcessStatus::Processed
        }
    }

    let mut a = App::debug(80, 12, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_save_select_extension() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show save dialog');
        CheckHash(0x6D6E678D8633D6C1)   
        Key.TypeText('myfile')
        Paint('3. myfile without extension is typed');
        CheckHash(0x276FB959EE1AE207)   
        Key.Pressed(Enter)
        Paint('4. Validate extension selection');
        CheckHash(0xDD59640B190846EE)
        Key.Pressed(Enter)
        Paint('5. combobox opened');
        CheckHash(0x2840F03ECF7FFA00)           
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('6. selected extension: png -> file is: C:\\Program Files\\myfile.png');
        CheckHash(0x5AEDD66519EB7207)           
    ";

    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::save(
        "Save",
        "myfile.exe",
        dialogs::Location::Last,
        SaveFileDialogFlags::Icons | SaveFileDialogFlags::ValidateOverwrite,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_save_select_extension_but_quit() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show save dialog');
        CheckHash(0x6D6E678D8633D6C1)   
        Key.TypeText('myfile')
        Paint('3. myfile without extension is typed');
        CheckHash(0x276FB959EE1AE207)   
        Key.Pressed(Enter)
        Paint('4. Validate extension selection');
        CheckHash(0xDD59640B190846EE)
        Key.Pressed(Enter)
        Paint('5. combobox opened');
        CheckHash(0x2840F03ECF7FFA00)           
        Key.Pressed(Tab)
        Key.Pressed(Escape)
        Paint('6. selected extension: None -> file is: C:\\Program Files\\myfile');
        CheckHash(0x7F769C57E0A16CE4)           
    ";

    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::save(
        "Save",
        "myfile.exe",
        dialogs::Location::Last,
        SaveFileDialogFlags::Icons | SaveFileDialogFlags::ValidateOverwrite,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_save_select_extension_different() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show save dialog');
        CheckHash(0x6D6E678D8633D6C1)   
        Key.TypeText('myfile')
        Paint('3. myfile without extension is typed');
        CheckHash(0x276FB959EE1AE207)   
        Key.Pressed(Enter)
        Paint('4. Validate extension selection');
        CheckHash(0xDD59640B190846EE)
        Key.Pressed(Enter)
        Key.Pressed(Down,2)
        Paint('5. combobox opened + select jpg');
        CheckHash(0xCCFB713ECDD5FFB8)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('6. selected extension: jpg -> file is: C:\\Program Files\\myfile.jpg');
        CheckHash(0xA446E306398B35EF)           
    ";

    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::save(
        "Save",
        "myfile.exe",
        dialogs::Location::Last,
        SaveFileDialogFlags::Icons | SaveFileDialogFlags::ValidateOverwrite,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_save_select_extension_do_not_change() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show save dialog');
        CheckHash(0x6D6E678D8633D6C1)   
        Key.TypeText('myfile')
        Paint('3. myfile without extension is typed');
        CheckHash(0x276FB959EE1AE207)   
        Key.Pressed(Enter)
        Paint('4. Validate extension selection');
        CheckHash(0xDD59640B190846EE)
        Key.Pressed(Enter)
        Key.Pressed(Down,3)
        Paint('5. combobox opened + select do_not_change');
        CheckHash(0x403BD91BBB561AEF)
        Key.Pressed(Tab)
        Key.Pressed(Enter)
        Paint('6. selected extension: None -> file is: C:\\Program Files\\myfile');
        CheckHash(0x7F769C57E0A16CE4)           
    ";

    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::save(
        "Save",
        "myfile.exe",
        dialogs::Location::Last,
        SaveFileDialogFlags::Icons | SaveFileDialogFlags::ValidateOverwrite,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_save_select_extension_not_called_due_to_correct_extension() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show save dialog');
        CheckHash(0x6D6E678D8633D6C1)   
        Key.TypeText('myfile.png')
        Paint('3. myfile.png typed');
        CheckHash(0x1AC52ACDDC935620)   
        Key.Pressed(Enter)
        Paint('4. selected extension: png -> file is: C:\\Program Files\\myfile.png');
        CheckHash(0x5AEDD66519EB7207)           
    ";

    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::save(
        "Save",
        "myfile.exe",
        dialogs::Location::Last,
        SaveFileDialogFlags::Icons | SaveFileDialogFlags::ValidateOverwrite,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_save_select_extension_not_called_due_one_extension() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show save dialog');
        CheckHash(0x6D6E678D8633D6C1)   
        Key.TypeText('myfile')
        Paint('3. myfile typed (no extension0');
        CheckHash(0x276FB959EE1AE207)   
        Key.Pressed(Tab, 2)
        Key.Pressed(Down,3)
        Paint('4. Selected Presentation type')
        CheckHash(0x7992999EBAE045ED)           
        Key.Pressed(Alt+O)
        Key.Pressed(Tab)
        // selection dialog should not be triggered because there is only one selection in the Presentation category
        Paint('5. selected extension: ppt -> file is: C:\\Program Files\\myfile.ppt');
        CheckHash(0x4C351F165DC22042)           
    ";

    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::save(
        "Save",
        "myfile.exe",
        dialogs::Location::Last,
        SaveFileDialogFlags::Icons | SaveFileDialogFlags::ValidateOverwrite,
    ));
    a.run();
}

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[test]
fn check_save_select_extension_not_called_due_all_files_mask() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial State')   
        CheckHash(0x5ED47A4336110FC4)
        Key.Pressed(Enter)
        Paint('2. Show save dialog');
        CheckHash(0x6D6E678D8633D6C1)   
        Key.TypeText('myfile')
        Paint('3. myfile typed (no extension0');
        CheckHash(0x276FB959EE1AE207)   
        Key.Pressed(Tab, 2)
        // make sure that we select the 'All files' mask
        Key.Pressed(Down,100)
        Paint('4. Selected All Files type')
        CheckHash(0xB9275C623A71C4E6)           
        Key.Pressed(Alt+O)
        Key.Pressed(Tab)
        // selection dialog should not be trigered because we have used the All Files mask
        Paint('5. selected extension: None -> file is: C:\\Program Files\\myfile');
        CheckHash(0x7F769C57E0A16CE4)           
    ";

    let mut a = App::debug(80, 30, script).build().unwrap();
    a.add_window(OpenSaveTestWindow::save(
        "Save",
        "myfile.exe",
        dialogs::Location::Last,
        SaveFileDialogFlags::Icons | SaveFileDialogFlags::ValidateOverwrite,
    ));
    a.run();
}