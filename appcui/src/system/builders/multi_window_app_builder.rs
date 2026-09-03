use super::impl_terminal_builder_methods;
use super::impl_ui_builder_methods;
use super::InternalBuilder;
use crate::system::RuntimeManager;
use crate::ui::common::traits::Control;
use crate::ui::common::traits::NotModalWindow;
use crate::ui::common::traits::WindowControl;

type WindowFactory = Box<dyn FnOnce(&mut RuntimeManager)>;

/// Builder for a multi-window AppCUI desktop application.
///
/// Obtain this builder with [`App::new`](crate::system::App::new). Register windows
/// with [`window`](Self::window), optionally configure the terminal and desktop UI,
/// then call [`run`](Self::run) to start the event loop.
///
/// # Examples
///
/// ```rust, no_run
/// use appcui::prelude::*;
///
/// fn main() -> Result<(), appcui::system::Error> {
///     App::new()
///         .window(|| window!("'First Window',a:c,w:30,h:9"))
///         .window(|| window!("'Second Window',x:2,y:2,w:24,h:8"))
///         .app_bar()
///         .run()
/// }
/// ```
pub struct MultiWindowAppBuilder {
    builder: InternalBuilder,
    window_factories: Vec<WindowFactory>,
}

impl MultiWindowAppBuilder {
    pub(crate) fn new() -> Self {
        Self {
            builder: InternalBuilder::new(),
            window_factories: Vec::new(),
        }
    }

    /// Replaces the default desktop with a custom desktop control.
    ///
    /// Use this to implement your own desktop background, window management,
    /// or desktop-level event handling.
    ///
    /// # Parameters
    /// * `desktop` - A control that implements [`crate::ui::common::traits::DesktopControl`].
    ///
    /// # Type Constraints
    /// * `T` must implement [`crate::ui::common::traits::Control`] and
    ///   [`crate::ui::common::traits::DesktopControl`].
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// App::new().desktop(Desktop::new());
    /// ```
    #[inline(always)]
    pub fn desktop<T>(mut self, desktop: T) -> Self
    where
        T: crate::ui::common::traits::Control + crate::ui::common::traits::DesktopControl + 'static,
    {
        self.builder.desktop(desktop);
        self
    }

    /// Registers a window that will be created when the application starts.
    ///
    /// Call this method once for each window. The factory closure runs after the
    /// runtime is initialized and must return a non-modal window. Windows are
    /// created in the order they were registered.
    ///
    /// # Parameters
    /// * `factory` - A `FnOnce() -> T` that constructs the window.
    ///
    /// # Type Constraints
    /// * `T` must implement [`Control`], [`WindowControl`], and [`NotModalWindow`].
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// fn main() -> Result<(), appcui::system::Error> {
    ///     App::new()
    ///         .window(|| {
    ///             let mut win = window!("'Demo',a:c,w:40,h:10");
    ///             win.add(label!("'Hello World !',a:c,w:13,h:1"));
    ///             win
    ///         })
    ///         .run()
    /// }
    /// ```
    pub fn window<F, T>(mut self, factory: F) -> Self
    where
        F: FnOnce() -> T + 'static,
        T: Control + WindowControl + NotModalWindow + 'static,
    {
        self.window_factories.push(Box::new(move |rt: &mut RuntimeManager| {
            let _ = rt.add_window(factory()); // T consumed here; handle discarded
        }));
        self
    }

    /// Builds the application from the current settings and starts the event loop.
    ///
    /// This method consumes the builder, creates the runtime, instantiates every
    /// registered window, and blocks until the application closes.
    ///
    /// # Errors
    /// Returns [`crate::system::Error`] if the application cannot be initialized
    /// (for example, if another application is already running).
    #[inline(always)]
    pub fn run(mut self) -> Result<(), crate::system::Error> {
        let app = self.builder.build()?;
        for factory in self.window_factories.drain(..) {
            factory(RuntimeManager::get());
        }
        app.start_app()
    }

    impl_terminal_builder_methods!();
    impl_ui_builder_methods!();
}
