use super::impl_terminal_builder_methods;
use super::impl_ui_builder_methods;
use super::InternalBuilder;
use crate::system::RuntimeManager;
use crate::ui::common::traits::Control;
use crate::ui::common::traits::NotModalWindow;
use crate::ui::common::traits::WindowControl;

/// Builder for a multi-window AppCUI application.
type WindowFactory = Box<dyn FnOnce(&mut RuntimeManager)>;

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
    /// Runs the application using the current settings.
    #[inline(always)]
    pub fn run(self) -> Result<(), crate::system::Error> {
        let app = self.builder.build()?;
        (self.window_factory)(RuntimeManager::get());
        app.start_app()
    }

    impl_terminal_builder_methods!();
    impl_ui_builder_methods!();
}
