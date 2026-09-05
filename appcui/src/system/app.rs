use std::marker::PhantomData;
use std::sync::Mutex;

use crate::ui::common::traits::Control;
use crate::ui::common::traits::NotModalWindow;
use crate::ui::common::traits::WindowControl;

use super::Error;
use super::ErrorKind;
use super::RuntimeManager;
use super::Theme;
use super::ThemeMethods;

static APP_CREATED_MUTEX: Mutex<bool> = Mutex::new(false);

/// HTML message displayed at the end of the application for WASM targets.
#[cfg(target_arch = "wasm32")]
const WEBTERMINAL_END_MESSAGE_HTML: &str = "<h1>{} has ended</h1><p>To re-start the application, please refresh the page.</p>";

/// The main application object for AppCUI.
///
/// `App` is the entry point for creating and managing an AppCUI application. Use one of
/// the constructor methods to obtain a builder, configure the terminal and UI, then call
/// `run` on that builder to start the event loop.
///
/// Only one application can exist at a time. Creating a second instance while another
/// is still running returns an error.
///
/// # Application types
///
/// | Constructor | Builder | Use when |
/// |-------------|---------|----------|
/// | [`App::new`] | [`crate::system::MultiWindowAppBuilder`] | Classic desktop with one or more windows |
/// | [`App::single_window`] | [`crate::system::SingleWindowAppBuilder`] | A single window that fills the desktop |
/// | [`App::frame_app`] | [`crate::system::FrameAppBuilder`] | Games or animations that update every frame |
/// | [`App::input_app`] | [`crate::system::InputAppBuilder`] | Custom paint and input without window chrome |
///
/// # Examples
///
/// ```rust, no_run
/// use appcui::prelude::*;
///
/// fn main() -> Result<(), appcui::system::Error> {
///     App::new()
///         .window(|| {
///             let mut win = window!("'First Window',a:c,w:30,h:9");
///             win.add(label!("'Hello World !',a:c,w:13,h:1"));
///             win
///         })
///         .run()
/// }
/// ```
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
    /// Creates a builder for a multi-window desktop application.
    ///
    /// Chain configuration methods on the returned [`crate::system::MultiWindowAppBuilder`],
    /// register one or more windows with [`window`](crate::system::MultiWindowAppBuilder::window),
    /// then call [`run`](crate::system::MultiWindowAppBuilder::run) to start the event loop.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// fn main() -> Result<(), appcui::system::Error> {
    ///     App::new()
    ///         .window(|| window!("'First Window',a:c,w:30,h:9"))
    ///         .run()
    /// }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> crate::system::MultiWindowAppBuilder {
        crate::system::MultiWindowAppBuilder::new()
    }

    /// Creates a builder for a single-window application.
    ///
    /// The factory is invoked once, after the runtime is created, and must
    /// return a non-modal window. In this mode the window typically fills the desktop
    /// and closing it ends the application.
    ///
    /// The factory can be a **closure** or a **function** (`FnOnce() -> T`):
    /// * `App::single_window(|| window!("title:'Demo',d:f"))` — a closure, useful when
    ///   you build the window inline or `new` takes arguments (`|| MyWin::new("Title")`).
    /// * `App::single_window(MyWin::new)` — a constructor / function with no arguments.
    ///   Pass `MyWin::new` (no parentheses). Writing `MyWin::new()` would construct the
    ///   window immediately, before the runtime exists.
    ///
    /// # Parameters
    /// * `factory` - A `FnOnce() -> T` that constructs the main window (closure or function).
    ///
    /// # Type Constraints
    /// * `T` must implement [`Control`], [`WindowControl`], and [`NotModalWindow`].
    ///
    /// # Examples
    ///
    /// Closure:
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// fn main() -> Result<(), appcui::system::Error> {
    ///     App::single_window(|| window!("title:'Demo',d:f"))
    ///         .size(Size::new(40, 10))
    ///         .run()
    /// }
    /// ```
    ///
    /// Function / constructor (`new` takes no arguments):
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// fn demo_window() -> Window {
    ///     window!("title:'Demo',d:f")
    /// }
    ///
    /// fn main() -> Result<(), appcui::system::Error> {
    ///     App::single_window(demo_window).size(Size::new(40, 10)).run()
    /// }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn single_window<F, T>(factory: F) -> crate::system::SingleWindowAppBuilder
    where
        F: FnOnce() -> T + 'static,
        T: Control + WindowControl + NotModalWindow + 'static,
    {
        crate::system::SingleWindowAppBuilder::new(factory)
    }
    /// Creates a builder for a frame-based application.
    ///
    /// Use this when the application paints the full surface every tick (games,
    /// animations, visualizations). The provided type must implement
    /// [`crate::system::FrameApp`].
    ///
    /// # Parameters
    /// * `frame_app` - The object that receives start, update, input, paint, and close callbacks.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// struct HelloWorld;
    /// impl FrameApp for HelloWorld {
    ///     fn on_paint(&self, surface: &mut Surface) {
    ///         surface.write_string(0, 0, "Hello World !", charattr!("white"), false);
    ///     }
    /// }
    ///
    /// fn main() -> Result<(), appcui::system::Error> {
    ///     App::frame_app(HelloWorld {}).fps(1).run()
    /// }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn frame_app<T: crate::system::FrameApp + 'static>(frame_app: T) -> crate::system::FrameAppBuilder<T> {
        crate::system::FrameAppBuilder::new(frame_app)
    }

    /// Creates a builder for an input-driven application.
    ///
    /// Use this when you want to paint the surface and handle keyboard or mouse
    /// events without window chrome. The provided type must implement
    /// [`crate::system::InputApp`].
    ///
    /// # Parameters
    /// * `input_app` - The object that receives start, resize, input, paint, and close callbacks.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// struct HelloWorld;
    /// impl InputApp for HelloWorld {
    ///     fn on_paint(&self, surface: &mut Surface) {
    ///         surface.write_string(0, 0, "Hello World !", charattr!("white"), false);
    ///     }
    /// }
    ///
    /// fn main() -> Result<(), appcui::system::Error> {
    ///     App::input_app(HelloWorld {}).run()
    /// }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn input_app<T: crate::system::InputApp + 'static>(input_app: T) -> crate::system::InputAppBuilder<T> {
        crate::system::InputAppBuilder::new(input_app)
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

    /// Replaces the theme of the running application.
    ///
    /// Call this from an event handler (or any other code that runs after `run` has
    /// started) to change colors and styles at runtime. To set the theme before the
    /// event loop starts, use the builder [`theme`](crate::system::MultiWindowAppBuilder::theme)
    /// method instead.
    ///
    /// # Parameters
    /// * `theme` - The new [`Theme`] to apply to the desktop and all controls.
    ///
    /// # Panics
    /// Panics if no application has been created yet.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// App::set_theme(Theme::new(Themes::DarkGray));
    /// ```
    pub fn set_theme(theme: Theme) {
        if !App::is_created() {
            panic!("App::set_theme can only be called after the App has been created !");
        }
        RuntimeManager::get().set_theme(theme);
    }

    /// Requests that the current application terminate.
    ///
    /// The event loop exits after the current event is processed. If no application
    /// is running, this method does nothing.
    ///
    /// Frame and input applications also close automatically on `Escape` when
    /// [`auto_close`](crate::system::FrameAppBuilder::auto_close) is enabled (the default).
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// App::close();
    /// ```
    pub fn close() {
        if App::is_created() {
            RuntimeManager::get().close();
        }
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
