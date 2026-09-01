use std::time::Duration;

use crate::prelude::*;

use super::impl_terminal_builder_methods;
use super::InternalBuilder;

trait FrameApp {
    // Called once, after the app is created, before the first paint.
    fn on_start(&mut self) { }

    // Called when the terminal/surface size changes.
    fn on_resize(&mut self, new_size: Size) {}

    // Called each tick at the configured rate (default 60/sec), with measured delta.
    fn on_update(&mut self, delta: Duration);

    // Input handlers: mutate state, no return value.
    fn on_key_event(&mut self, key: Key, ch: char);
    fn on_mouse_event(&mut self, ev: &MouseEvent);

    // Draw current state to the surface.
    fn on_paint(&self, surface: &mut Surface);

    // Called once, after the loop exits, before terminal restore.
    fn on_end(&mut self) {}
}

#[Desktop(internal = true)]
struct DesktopWrapper<T> where T: FrameApp {
    frame_app: T,
}
impl<T: FrameApp> DesktopWrapper<T> {
    fn new(frame_app: T) -> Self {
        Self { base: Desktop::new(), frame_app }
    }
}
pub struct FrameAppBuilder {
    builder: InternalBuilder,
    fps: u32,
}

impl FrameAppBuilder {
    pub(crate) fn new() -> Self {
        Self {
            builder: InternalBuilder::new(),
            fps: 30,
        }
    }
    /// Sets the frame rate of the application.
    /// The frame rate is the number of frames per second that the application will run at.
    /// The default frame rate is 30 FPS.
    /// The maximum frame rate is 120 FPS.
    /// The minimum frame rate is 1 FPS.
    #[inline(always)]
    pub(crate) fn fps(mut self, fps: u32) -> Self {
        self.fps = fps.clamp(1, 120);
        self
    }
    /// Runs the application using the current settings.
    #[inline(always)]
    pub fn run(self) -> Result<(), crate::system::Error> {
        let app = self.builder.build()?;
        app.start_app()
    }

    impl_terminal_builder_methods!();
}
