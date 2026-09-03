use super::impl_terminal_builder_methods;
use super::impl_ui_builder_methods;
use super::InternalBuilder;
use crate::system::RuntimeManager;
use crate::ui::common::traits::Control;
use crate::ui::common::traits::NotModalWindow;
use crate::ui::common::traits::WindowControl;

type WindowFactory = Box<dyn FnOnce(&mut RuntimeManager)>;

/// Builder for a single-window AppCUI application.
///
/// Obtain this builder with [`App::single_window`](crate::system::App::single_window).
/// In this mode the application hosts exactly one non-modal window (typically
/// docked to fill the desktop). Configure the terminal and desktop UI, then call
/// [`run`](Self::run) to start the event loop.
///
/// The factory passed to [`App::single_window`](crate::system::App::single_window)
/// can be a closure (`|| window!("title:'Demo',d:f")`) or a function / constructor
/// with no arguments (`MyWin::new`).
///
/// # Examples
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
pub struct SingleWindowAppBuilder {
    builder: InternalBuilder,
    window_factory: WindowFactory,
}

impl SingleWindowAppBuilder {
    pub(crate) fn new<F, T>(factory: F) -> Self
    where
        F: FnOnce() -> T + 'static,
        T: Control + WindowControl + NotModalWindow + 'static,
    {
        let mut b = InternalBuilder::new();
        b.single_window();
        Self {
            builder: b,
            window_factory: Box::new(move |rt: &mut RuntimeManager| {
                let _ = rt.add_window(factory()); // T consumed here; handle discarded
            }),
        }
    }
    /// Builds the application from the current settings and starts the event loop.
    ///
    /// This method consumes the builder, creates the runtime, instantiates the
    /// single window, and blocks until the application closes.
    ///
    /// # Errors
    /// Returns [`crate::system::Error`] if the application cannot be initialized
    /// (for example, if another application is already running).
    #[inline(always)]
    pub fn run(self) -> Result<(), crate::system::Error> {
        let app = self.builder.build()?;
        (self.window_factory)(RuntimeManager::get());
        app.start_app()
    }

    impl_terminal_builder_methods!();
    impl_ui_builder_methods!();
}
