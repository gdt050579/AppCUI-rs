use crate::prelude::*;

use super::impl_app_desktop_methods;
use super::impl_terminal_builder_methods;
use super::InternalBuilder;

pub trait InputApp {
    /// Called once, after the app is created, before the first paint.
    fn on_start(&mut self) {}

    /// Called when the terminal/surface size changes.
    fn on_resize(&mut self, _new_size: Size) {}

    /// Called when a key is pressed.
    fn on_key_event(&mut self, _key: Key, _ch: char) -> EventProcessStatus { EventProcessStatus::Ignored }

    /// Called when a mouse event occurs.
    fn on_mouse_event(&mut self, _ev: &MouseEvent) -> EventProcessStatus { EventProcessStatus::Ignored }

    /// Draw current state to the surface.
    fn on_paint(&self, surface: &mut Surface);

    /// Called once, after the loop exits, before terminal restore.
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
    fn new(input_app: T,  auto_close: bool, clear_char: Option<Character>) -> Self {
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
        if self.auto_close {
            if key.value() == key!("Escape") {
                App::close();
                return EventProcessStatus::Processed;
            }
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

    #[inline(always)]
    pub fn auto_close(mut self, value: bool) -> Self {
        self.auto_close = value;
        self
    }

    #[inline(always)]
    pub fn clear_char(mut self, ch: Option<Character>) -> Self {
        self.clear_screen_char = ch;
        self
    }

    /// Runs the application using the current settings.
    #[inline(always)]
    pub fn run(mut self) -> Result<(), crate::system::Error> {
        self.builder
            .desktop(AppDesktop::new(self.input_app, self.auto_close, self.clear_screen_char));
        let app = self.builder.build()?;
        app.start_app()
    }

    impl_terminal_builder_methods!();
}
