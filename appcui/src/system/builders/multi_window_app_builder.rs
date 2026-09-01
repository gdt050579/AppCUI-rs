use super::impl_terminal_builder_methods;
use super::impl_ui_builder_methods;
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

    /// Builds the application using the current settings.
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
