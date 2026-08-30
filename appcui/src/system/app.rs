use std::marker::PhantomData;
use std::sync::Mutex;

use super::Error;
use super::ErrorKind;
use super::RuntimeManager;
use super::Theme;
use super::ThemeMethods;

static APP_CREATED_MUTEX: Mutex<bool> = Mutex::new(false);

/// HTML message displayed at the end of the application for WASM targets.
#[cfg(target_arch = "wasm32")]
const WEBTERMINAL_END_MESSAGE_HTML: &str = "<h1>{} has ended</h1><p>To re-start the application, please refresh the page.</p>";

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
    pub(super) fn create(builder: crate::system::InternalBuilder) -> Result<Self, Error> {
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
    pub fn new() -> crate::system::MultiWindowAppBuilder {
        crate::system::MultiWindowAppBuilder::new()
    }

    /// Runs the current appcui application. This command will display all windows, and allow you to run the cod that perform the event logic for every control.
    pub(crate) fn start_app(self) -> Result<(), crate::system::Error> {
        #[cfg(target_arch = "wasm32")]
        #[allow(unused_imports)]
        {
            use wasm_bindgen_rayon::init_thread_pool; // Explicitly import for WASM to export this function
            console_error_panic_hook::set_once();
        }
        // must pe self so that after a run a second call will not be possible
        RuntimeManager::get().run();
        // close the backend
        RuntimeManager::get().backend_mut().on_close();
        // clear the mutex from open_save_dialog to clear the last path
        crate::dialogs::clear_last_path();

        // clear the mutex so that other apps can be created after this step
        #[cfg(not(target_arch = "wasm32"))]
        {
            RuntimeManager::destroy();
            let mut app_created = APP_CREATED_MUTEX.lock().unwrap();
            *app_created = false;
        }
        // For WASM, APP_CREATED_MUTEX is reset via drop_app
        // called from RuntimeManager's animation loop when it terminates.
        Ok(())
    }

    /// Sets the theme for the current application.
    pub fn set_theme(theme: Theme) {
        if !App::is_created() {
            panic!("App::set_theme can only be called after the App has been created !");
        }
        RuntimeManager::get().set_theme(theme);
    }

    pub(crate) fn drop_app() {
        if APP_CREATED_MUTEX.is_poisoned() {
            APP_CREATED_MUTEX.clear_poison();
        }
        if RuntimeManager::is_instantiated() {
            RuntimeManager::destroy();
        }
        let mut app_created = APP_CREATED_MUTEX.lock().unwrap();
        *app_created = false;

        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            if let Some(win) = window() {
                if let Some(doc) = win.document() {
                    if let Some(body) = doc.body() {
                        body.set_inner_html(&WEBTERMINAL_END_MESSAGE_HTML.replace("{}", &doc.title()));
                    }
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for App {
    fn drop(&mut self) {
        Self::drop_app();
    }
}
