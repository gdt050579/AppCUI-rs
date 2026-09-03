use crate::prelude::*;

use super::impl_app_desktop_methods;
use super::impl_terminal_builder_methods;
use super::InternalBuilder;

/// An input-driven application that paints the surface and handles events directly.
///
/// Implement this trait when you want custom drawing and keyboard or mouse
/// handling without window chrome. Pass an instance to
/// [`App::input_app`](crate::system::App::input_app) to obtain an
/// [`InputAppBuilder`].
///
/// Unlike [`FrameApp`](crate::system::FrameApp), there is no periodic update
/// tick. The surface is repainted when an event returns
/// [`EventProcessStatus::Processed`] (or when the runtime otherwise requests a
/// redraw).
///
/// Only [`on_paint`](Self::on_paint) is required. The other methods have
/// default implementations.
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
pub trait InputApp {
    /// Called once after the application is created, before the first paint.
    ///
    /// Use this to initialize state. The default implementation does nothing.
    fn on_start(&mut self) {}

    /// Called when the terminal or surface size changes.
    ///
    /// # Parameters
    /// * `new_size` - The new surface size in character cells.
    fn on_resize(&mut self, _new_size: Size) {}

    /// Called when a key is pressed.
    ///
    /// If [`InputAppBuilder::auto_close`] is enabled (the default), `Escape`
    /// closes the application before this method is invoked.
    ///
    /// Return [`EventProcessStatus::Processed`] to request a repaint, or
    /// [`EventProcessStatus::Ignored`] if the key was not handled.
    ///
    /// # Parameters
    /// * `key` - The pressed key, including modifiers.
    /// * `ch` - The character produced by the key, if any (otherwise `'\0'`).
    fn on_key_event(&mut self, _key: Key, _ch: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    /// Called when a mouse event occurs (move, press, release, drag, or wheel).
    ///
    /// Return [`EventProcessStatus::Processed`] to request a repaint, or
    /// [`EventProcessStatus::Ignored`] if the event was not handled.
    ///
    /// # Parameters
    /// * `ev` - The mouse event to handle.
    fn on_mouse_event(&mut self, _ev: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    /// Draws the current state onto the surface.
    ///
    /// Called after the optional clear performed by
    /// [`InputAppBuilder::clear_char`].
    ///
    /// # Parameters
    /// * `surface` - The surface to paint. Clip and origin are already reset.
    fn on_paint(&self, surface: &mut Surface);

    /// Called when the application is about to close, before the terminal is restored.
    ///
    /// Return [`ActionRequest::Allow`] to close, or [`ActionRequest::Deny`] to
    /// keep the application running. The default implementation allows the close.
    fn on_close(&mut self) -> ActionRequest {
        ActionRequest::Allow
    }
}

#[repr(C)]
struct AppDesktop<T>
where
    T: InputApp,
{
    base: Desktop,
    input_app: T,
    auto_close: bool,
    clear_char: Option<Character>,
}
impl_app_desktop_methods!(AppDesktop, InputApp);
impl<T: InputApp> AppDesktop<T> {
    fn new(input_app: T, auto_close: bool, clear_char: Option<Character>) -> Self {
        Self {
            base: Desktop::new(),
            input_app,
            auto_close,
            clear_char,
        }
    }
}
impl<T: InputApp> OnPaint for AppDesktop<T> {
    fn on_paint(&self, surface: &mut Surface, _: &Theme) {
        if let Some(ch) = self.clear_char {
            surface.reset(ch);
        } else {
            surface.reset_clip_and_origin();
        }
        self.input_app.on_paint(surface);
    }
}
impl<T: InputApp> OnKeyPressed for AppDesktop<T> {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        if self.auto_close && key.value() == key!("Escape") {
            App::close();
            return EventProcessStatus::Processed;
        }
        self.input_app.on_key_event(key, character)
    }
}
impl<T: InputApp> OnMouseEvent for AppDesktop<T> {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        self.input_app.on_mouse_event(event)
    }
}
impl<T: InputApp> OnResize for AppDesktop<T> {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.input_app.on_resize(new_size);
    }
}
impl<T: InputApp> DesktopEvents for AppDesktop<T> {
    fn on_start(&mut self) {
        self.input_app.on_start();
    }

    fn on_close(&mut self) -> ActionRequest {
        if self.input_app.on_close() == ActionRequest::Allow {
            self.timer().unwrap().stop();
            ActionRequest::Allow
        } else {
            ActionRequest::Deny
        }
    }
}

impl<T: InputApp> TimerEvents for AppDesktop<T> {}

/// Builder for an input-driven AppCUI application.
///
/// Obtain this builder with [`App::input_app`](crate::system::App::input_app).
/// Configure close behavior, the clear character, and the terminal, then call
/// [`run`](Self::run) to start the event loop.
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
///     App::input_app(HelloWorld {})
///         .title("Hello")
///         .run()
/// }
/// ```
pub struct InputAppBuilder<T: InputApp + 'static> {
    builder: InternalBuilder,
    auto_close: bool,
    clear_screen_char: Option<Character>,
    input_app: T,
}

impl<T: InputApp + 'static> InputAppBuilder<T> {
    pub(crate) fn new(input_app: T) -> Self {
        Self {
            builder: InternalBuilder::new(),
            auto_close: true,
            clear_screen_char: Some(char!("' ',white,black")),
            input_app,
        }
    }

    /// Enables or disables closing the application with the `Escape` key.
    ///
    /// When enabled (the default), pressing `Escape` calls [`App::close`].
    /// When disabled, `Escape` is forwarded to [`InputApp::on_key_event`].
    ///
    /// # Parameters
    /// * `value` - `true` to close on `Escape`, `false` to handle it yourself.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// struct HelloWorld;
    /// impl InputApp for HelloWorld {
    ///     fn on_paint(&self, _surface: &mut Surface) {}
    /// }
    ///
    /// App::input_app(HelloWorld {}).auto_close(false);
    /// ```
    #[inline(always)]
    pub fn auto_close(mut self, value: bool) -> Self {
        self.auto_close = value;
        self
    }

    /// Sets the character used to clear the surface before each paint.
    ///
    /// The default is a space with a white foreground and black background.
    /// Pass `None` to skip the fill and only reset clip and origin.
    ///
    /// # Parameters
    /// * `ch` - The clear character, or `None` to leave the previous frame in place.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// struct HelloWorld;
    /// impl InputApp for HelloWorld {
    ///     fn on_paint(&self, _surface: &mut Surface) {}
    /// }
    ///
    /// App::input_app(HelloWorld {}).clear_char(Some(char!("' ',white,black")));
    /// ```
    #[inline(always)]
    pub fn clear_char(mut self, ch: Option<Character>) -> Self {
        self.clear_screen_char = ch;
        self
    }

    /// Builds the application from the current settings and starts the event loop.
    ///
    /// This method consumes the builder, creates the runtime, and blocks until
    /// the application closes. [`InputApp::on_paint`] is called whenever the
    /// runtime requests a redraw.
    ///
    /// # Errors
    /// Returns [`crate::system::Error`] if the application cannot be initialized
    /// (for example, if another application is already running).
    #[inline(always)]
    pub fn run(mut self) -> Result<(), crate::system::Error> {
        self.builder
            .desktop(AppDesktop::new(self.input_app, self.auto_close, self.clear_screen_char));
        let app = self.builder.build()?;
        app.start_app()
    }

    impl_terminal_builder_methods!();
}
