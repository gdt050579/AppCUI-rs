use crate::prelude::*;
use std::time::Duration;

use super::impl_app_desktop_methods;
use super::impl_terminal_builder_methods;
use super::InternalBuilder;

/// A frame-based application that paints the full surface on every tick.
///
/// Implement this trait for games, animations, or visualizations that update
/// on a fixed frame rate. Pass an instance to
/// [`App::frame_app`](crate::system::App::frame_app) to obtain a
/// [`FrameAppBuilder`].
///
/// Only [`on_paint`](Self::on_paint) is required. The other methods have
/// empty default implementations.
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
///     App::frame_app(HelloWorld {}).fps(30).run()
/// }
/// ```
pub trait FrameApp {
    /// Called once after the application is created, before the first paint.
    ///
    /// Use this to initialize state, load resources, or seed random values.
    /// The default implementation does nothing.
    fn on_start(&mut self) {}

    /// Called when the terminal or surface size changes.
    ///
    /// # Parameters
    /// * `new_size` - The new surface size in character cells.
    fn on_resize(&mut self, _new_size: Size) {}

    /// Called once per frame at the rate configured with [`FrameAppBuilder::fps`].
    ///
    /// The default frame rate is 30 FPS (not 60). Use `ticks` to drive
    /// animations or periodic logic.
    ///
    /// # Parameters
    /// * `ticks` - Number of timer ticks since the application started.
    fn on_update(&mut self, _ticks: u64) {}

    /// Called when a key is pressed.
    ///
    /// If [`FrameAppBuilder::auto_close`] is enabled (the default), `Escape`
    /// closes the application before this method is invoked.
    ///
    /// # Parameters
    /// * `key` - The pressed key, including modifiers.
    /// * `ch` - The character produced by the key, if any (otherwise `'\0'`).
    fn on_key_event(&mut self, _key: Key, _ch: char) {}

    /// Called when a mouse event occurs (move, press, release, drag, or wheel).
    ///
    /// # Parameters
    /// * `ev` - The mouse event to handle.
    fn on_mouse_event(&mut self, _ev: &MouseEvent) {}

    /// Draws the current frame onto the surface.
    ///
    /// Called after [`on_update`](Self::on_update) (and after the optional
    /// clear performed by [`FrameAppBuilder::clear_char`]).
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
    T: FrameApp,
{
    base: Desktop,
    frame_app: T,
    fps: u32,
    auto_close: bool,
    clear_char: Option<Character>,
}
impl_app_desktop_methods!(AppDesktop, FrameApp);
impl<T: FrameApp> AppDesktop<T> {
    fn new(frame_app: T, fps: u32, auto_close: bool, clear_char: Option<Character>) -> Self {
        Self {
            base: Desktop::new(),
            frame_app,
            fps,
            auto_close,
            clear_char,
        }
    }
}
impl<T: FrameApp> OnPaint for AppDesktop<T> {
    fn on_paint(&self, surface: &mut Surface, _: &Theme) {
        if let Some(ch) = self.clear_char {
            surface.reset(ch);
        } else {
            surface.reset_clip_and_origin();
        }
        self.frame_app.on_paint(surface);
    }
}
impl<T: FrameApp> OnKeyPressed for AppDesktop<T> {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        if self.auto_close && key.value() == key!("Escape") {
            App::close();
            return EventProcessStatus::Processed;
        }
        self.frame_app.on_key_event(key, character);
        EventProcessStatus::Ignored
    }
}
impl<T: FrameApp> OnMouseEvent for AppDesktop<T> {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        self.frame_app.on_mouse_event(event);
        EventProcessStatus::Ignored
    }
}
impl<T: FrameApp> OnResize for AppDesktop<T> {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.frame_app.on_resize(new_size);
    }
}
impl<T: FrameApp> DesktopEvents for AppDesktop<T> {
    fn on_start(&mut self) {
        let milis = (1000u32 / self.fps) as u64;
        self.timer().unwrap().start(Duration::from_millis(milis));
        self.frame_app.on_start();
    }

    fn on_close(&mut self) -> ActionRequest {
        if self.frame_app.on_close() == ActionRequest::Allow {
            self.timer().unwrap().stop();
            ActionRequest::Allow
        } else {
            ActionRequest::Deny
        }
    }
}

impl<T: FrameApp> TimerEvents for AppDesktop<T> {
    fn on_update(&mut self, ticks: u64) -> EventProcessStatus {
        self.frame_app.on_update(ticks);
        EventProcessStatus::Processed
    }
}
/// Builder for a frame-based AppCUI application.
///
/// Obtain this builder with [`App::frame_app`](crate::system::App::frame_app).
/// Configure the frame rate, close behavior, and terminal, then call
/// [`run`](Self::run) to start the update/paint loop.
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
///     App::frame_app(HelloWorld {})
///         .fps(30)
///         .title("Hello")
///         .run()
/// }
/// ```
pub struct FrameAppBuilder<T: FrameApp + 'static> {
    builder: InternalBuilder,
    fps: u32,
    auto_close: bool,
    clear_screen_char: Option<Character>,
    frame_app: T,
}

impl<T: FrameApp + 'static> FrameAppBuilder<T> {
    pub(crate) fn new(frame_app: T) -> Self {
        Self {
            builder: InternalBuilder::new(),
            fps: 30,
            auto_close: true,
            clear_screen_char: Some(char!("' ',white,black")),
            frame_app,
        }
    }
    /// Sets the frame rate of the application.
    ///
    /// The value is clamped to the range `1..=120`. The default is 30 FPS.
    ///
    /// # Parameters
    /// * `fps` - Requested frames per second.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// struct HelloWorld;
    /// impl FrameApp for HelloWorld {
    ///     fn on_paint(&self, _surface: &mut Surface) {}
    /// }
    ///
    /// App::frame_app(HelloWorld {}).fps(60);
    /// ```
    #[inline(always)]
    pub fn fps(mut self, fps: u32) -> Self {
        self.fps = fps.clamp(1, 120);
        self
    }

    /// Enables or disables closing the application with the `Escape` key.
    ///
    /// When enabled (the default), pressing `Escape` calls [`App::close`].
    /// When disabled, `Escape` is forwarded to [`FrameApp::on_key_event`].
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
    /// impl FrameApp for HelloWorld {
    ///     fn on_paint(&self, _surface: &mut Surface) {}
    /// }
    ///
    /// App::frame_app(HelloWorld {}).auto_close(false);
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
    /// impl FrameApp for HelloWorld {
    ///     fn on_paint(&self, _surface: &mut Surface) {}
    /// }
    ///
    /// App::frame_app(HelloWorld {}).clear_char(Some(char!("' ',white,black")));
    /// ```
    #[inline(always)]
    pub fn clear_char(mut self, ch: Option<Character>) -> Self {
        self.clear_screen_char = ch;
        self
    }

    /// Builds the application from the current settings and starts the event loop.
    ///
    /// This method consumes the builder, creates the runtime, and blocks until
    /// the application closes. [`FrameApp::on_update`] and [`FrameApp::on_paint`]
    /// are called at the configured frame rate.
    ///
    /// # Errors
    /// Returns [`crate::system::Error`] if the application cannot be initialized
    /// (for example, if another application is already running).
    #[inline(always)]
    pub fn run(mut self) -> Result<(), crate::system::Error> {
        self.builder
            .desktop(AppDesktop::new(self.frame_app, self.fps, self.auto_close, self.clear_screen_char));
        let app = self.builder.build()?;
        app.start_app()
    }

    impl_terminal_builder_methods!();
}
