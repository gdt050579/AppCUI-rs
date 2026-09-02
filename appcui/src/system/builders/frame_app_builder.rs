use std::time::Duration;
use crate::prelude::*;

use super::impl_terminal_builder_methods;
use super::impl_app_desktop_methods;
use super::InternalBuilder;

pub trait FrameApp {
    /// Called once, after the app is created, before the first paint.
    fn on_start(&mut self) { }

    /// Called when the terminal/surface size changes.
    fn on_resize(&mut self, _new_size: Size) {}

    /// Called each tick at the configured rate (default 60/sec), with the number of ticks since the beginning of the application.
    fn on_update(&mut self, _ticks: u64) {}

    /// Called when a key is pressed.
    fn on_key_event(&mut self, _key: Key, _ch: char) {}

    /// Called when a mouse event occurs.
    fn on_mouse_event(&mut self, _ev: &MouseEvent) {}

    /// Draw current state to the surface.
    fn on_paint(&self, surface: &mut Surface);

    /// Called once, after the loop exits, before terminal restore.
    fn on_end(&mut self) {}
}

#[repr(C)]
struct AppDesktop<T> where T: FrameApp {
    base: Desktop,
    frame_app: T,
    fps: u32,
    auto_close: bool,
    clear_char: Option<Character>
}
impl_app_desktop_methods!(AppDesktop, FrameApp);
impl<T: FrameApp> AppDesktop<T> {
    fn new(frame_app: T, fps: u32, auto_close: bool, clear_char: Option<Character>) -> Self {
        Self { base: Desktop::new(), frame_app, fps,auto_close, clear_char }
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
        if self.auto_close {
            if key.value() == key!("Escape") {
                App::close();
                return EventProcessStatus::Processed;
            }
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
}

impl<T: FrameApp> TimerEvents for AppDesktop<T> {
    fn on_update(&mut self, ticks: u64) -> EventProcessStatus {
        self.frame_app.on_update(ticks);
        EventProcessStatus::Processed
    }
}
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
    /// The frame rate is the number of frames per second that the application will run at.
    /// The default frame rate is 30 FPS.
    /// The maximum frame rate is 120 FPS.
    /// The minimum frame rate is 1 FPS.
    #[inline(always)]
    pub fn fps(mut self, fps: u32) -> Self {
        self.fps = fps.clamp(1, 120);
        self
    }

    #[inline(always)]
    pub fn auto_close(mut self, value: bool) -> Self {
        self.auto_close = value;
        self
    }

    #[inline(always)]
    pub fn background_char(mut self, ch: Option<Character>) -> Self {
        self.clear_screen_char = ch;
        self
    }

    /// Runs the application using the current settings.
    #[inline(always)]
    pub fn run(mut self) -> Result<(), crate::system::Error> {
        self.builder.desktop(AppDesktop::new(self.frame_app, self.fps, self.auto_close, self.clear_screen_char));
        let app = self.builder.build()?;
        app.start_app()
    }

    impl_terminal_builder_methods!();
}
