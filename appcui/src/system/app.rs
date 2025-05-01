use std::marker::PhantomData;
use std::sync::Mutex;

use super::Error;
use super::ErrorKind;
use super::Handle;
use super::RuntimeManager;
use super::Theme;
use super::ThemeMethods;
use crate::graphics::Size;
use crate::terminals::TerminalType;
use crate::ui::common::traits::*;

static APP_CREATED_MUTEX: Mutex<bool> = Mutex::new(false);

/// Represents the main application object for AppCUI.
/// 
/// This struct is used to create and manage the main application. It provides methods to add windows, set the theme, and run the application.
pub struct App {
    _phantom: PhantomData<*mut ()>,
}

impl App {
    pub(super) fn is_created() -> bool {
        let app_created = APP_CREATED_MUTEX.lock().unwrap();
        *app_created
    }
    pub(super) fn create(builder: crate::system::Builder) -> Result<Self, Error> {
        if APP_CREATED_MUTEX.is_poisoned() {
            APP_CREATED_MUTEX.clear_poison();
        }
        let mut app_created = APP_CREATED_MUTEX.lock().unwrap();
        if *app_created {
            return Err(Error::new(
                ErrorKind::InitializationFailure,
                "App has already been created ! There can only be one instance of an Application at one time. If you have more, make sure that you have only one !".to_string(),
            ));
        }
        RuntimeManager::create(builder)?;
        *app_created = true;
        Ok(App {
            _phantom: Default::default(),
        })
    }
    /// Creates a new builder object using the default terminal for the current operating system
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> crate::system::Builder {
        crate::system::Builder::new()
    }
    /// Creates a new builder object using a specified terminal from the list of terminals available
    /// for the current operating system.
    pub fn with_terminal(terminal: TerminalType) -> crate::system::Builder {
        let mut builder = crate::system::Builder::new();
        builder.terminal = Some(terminal);
        builder
    }
    /// Creates a builder designed for unit testing.
    /// The provided parameters indicated:
    /// * `width` and `height` : the size of the simulated terminal
    /// * `script` : a script with multiple commands (one command per line) that will be executed one after another simulating events that could be send to the AppCUI. Once all commands are being executed, the application will end.
    ///
    /// ## Debug commands
    /// The following list of commands are supported for the script:
    ///
    /// **Mouse related commands**
    /// * `Mouse.Hold(x,y,button)` simulates an event where the mouse button is being pressed while the mouse is located at a specific position on screen. The parameters `x` and `y` are a screen position, while the parameter `button` is one of `left`, `right` or `center`
    /// * `Mouse.Release(x,y)` simulates the release of all mouse buttons while the mouse is located at a specific screen position.
    /// * `Mouse.Click(x,y,button)` simulates a click (hold an release)
    /// * `Mouse.Move(x,y)` simulates the movement of a mouse to coordonates (x,y). No mouse button are being pressed.
    /// * `Mouse.Drag(x1,y1,x2,y2)` simulates the movement of a mouse from (x1,y1) to (x2,y2) while the left button is being pressed
    /// * `Mouse.Wheel(x,y,direction,times)` simulates the wheel mouse being rotated into a direction (one of `top`, `bottom`, `left`, `right`) for a number of times. The `times` parameter must be biggen than 0.
    ///
    /// **Key related commands**
    /// * `Key.Pressed(key)` where key can be any combination of keys
    ///
    /// **Paint related commands**
    /// * `Paint(name)` paints the current virtual screen into the current screen using ANSI codes.
    /// * `Paint.Enable(value)` enables or disables painting. `value` is a boolean value (**true** or **false**). If set to **false** all subsequent calls to command `Paint` will be ignored.
    ///
    /// **System events**
    /// * `Resize(width,height)` simulates a resize of the virtual terminal to the size represented by `width` and `height` parameters
    ///
    /// **Validation commands**
    /// * `CheckHash(hash)` checks if the hash computer over the current virtual screen is as expected. If not it will panic. This is useful for unit testing.
    pub fn debug(width: u16, height: u16, script: &str) -> crate::system::Builder {
        let mut builder = crate::system::Builder::new();
        builder.size = Some(Size::new(width as u32, height as u32));
        builder.debug_script = Some(String::from(script));
        builder
    }

    /// Runs the current appcui application. This command will display all windows, and allow you to run the cod that perform the event logic for every control.
    pub fn run(self) {
        // must pe self so that after a run a second call will not be possible
        RuntimeManager::get().run();
        // clear the mutex from open_save_dialog to clear the last path
        crate::dialogs::clear_last_path();
        // clear the mutex so that other apps can be created after this step
        RuntimeManager::destroy();
        let mut app_created = APP_CREATED_MUTEX.lock().unwrap();
        *app_created = false;
    }

    /// Adds a new window to AppCUI framework and returns a Handle towords it.
    /// Later on you can use that handle to manipulate that window in a safe way.
    pub fn add_window<T>(&mut self, window: T) -> Handle<T>
    where
        T: Control + WindowControl + NotModalWindow + 'static,
    {
        RuntimeManager::get().add_window(window)
    }

    /// Sets the theme for the current application.
    pub fn set_theme(theme: Theme) {
        if !App::is_created() {
            panic!("App::set_theme can only be called after the App has been created !");
        }
        RuntimeManager::get().set_theme(theme);
    }
}

impl Drop for App {
    fn drop(&mut self) {
        if APP_CREATED_MUTEX.is_poisoned() {
            APP_CREATED_MUTEX.clear_poison();
        }
        if RuntimeManager::is_instantiated() {
            RuntimeManager::destroy();
        }
        let mut app_created = APP_CREATED_MUTEX.lock().unwrap();
        *app_created = false;
    }
}
