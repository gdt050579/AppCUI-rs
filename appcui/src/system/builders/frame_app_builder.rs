use std::time::Duration;

use crate::prelude::*;

use super::impl_terminal_builder_methods;
use super::app_desktop;
use super::InternalBuilder;

pub trait FrameApp {
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

app_desktop!(AppDesktop, FrameApp);
impl<T: FrameApp> AppDesktop<T> {
    fn new(frame_app: T) -> Self {
        Self { base: Desktop::new(), frame_app }
    }
}
impl<T: FrameApp> OnPaint for AppDesktop<T> {
    fn on_paint(&self, surface: &mut Surface, _: &Theme) {
        self.frame_app.on_paint(surface);
    }
}
impl<T: FrameApp> OnKeyPressed for AppDesktop<T> {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
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
        self.frame_app.on_start();
    }
}

pub struct FrameAppBuilder<T: FrameApp + 'static> {
    builder: InternalBuilder,
    fps: u32,
    frame_app: T,
}

impl<T: FrameApp + 'static> FrameAppBuilder<T> {
    pub(crate) fn new(frame_app: T) -> Self {
        Self {
            builder: InternalBuilder::new(),
            fps: 30,
            frame_app,
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
    pub fn run(mut self) -> Result<(), crate::system::Error> {
        self.builder.desktop(AppDesktop::new(self.frame_app));
        let app = self.builder.build()?;
        app.start_app()
    }

    impl_terminal_builder_methods!();
}
