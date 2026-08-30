use super::impl_internal_builder_methods;
use super::InternalBuilder;
use crate::system::RuntimeManager;
use crate::ui::common::traits::Control;
use crate::ui::common::traits::NotModalWindow;
use crate::ui::common::traits::WindowControl;

/// Builder for a multi-window AppCUI application.
type WindowFactory = Box<dyn FnOnce(&mut RuntimeManager)>;

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
    /// Enables the Application bar.
    #[inline(always)]
    pub fn app_bar(mut self) -> Self {
        self.builder.app_bar();
        self
    }

    /// Enables the command bar.
    #[inline(always)]
    pub fn command_bar(mut self) -> Self {
        self.builder.command_bar();
        self
    }

    /// Sets the theme of the application. If not specified, the default theme will be used.
    #[inline(always)]
    pub fn theme(mut self, theme: crate::system::Theme) -> Self {
        self.builder.theme(theme);
        self
    }

    /// Sets the number of timers that can be used in the application.
    #[inline(always)]
    pub fn timers_count(mut self, count: u8) -> Self {
        self.builder.timers_count(count);
        self
    }

    /// Sets the desktop manager.
    #[inline(always)]
    pub fn desktop<T>(mut self, desktop: T) -> Self
    where
        T: crate::ui::common::traits::Control + crate::ui::common::traits::DesktopControl + 'static,
    {
        self.builder.desktop(desktop);
        self
    }

    pub fn window<F, T>(mut self, factory: F) -> Self
    where
        F: FnOnce() -> T + 'static,
        T: Control + WindowControl + NotModalWindow + 'static,
    {
        self.window_factories.push(Box::new(move |rt: &mut RuntimeManager| {
            let _handle = rt.add_window(factory()); // T consumed here; handle discarded
        }));
        self
    }

    /// Builds the application using the current settings.
    #[inline(always)]
    pub fn run(mut self) -> Result<(), crate::system::Error> {
        let app = self.builder.build()?;
        for factory in self.window_factories.drain(..) {
            factory(RuntimeManager::get());
        }
        app.start_app()
    }

    impl_internal_builder_methods!();
}
