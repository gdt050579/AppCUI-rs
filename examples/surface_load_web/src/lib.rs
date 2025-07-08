#![cfg(target_arch = "wasm32")]

use appcui::prelude::*;
use wasm_bindgen::prelude::*;

// WASM does not support the `std::fs` module, so we need to use a build script to generate the code.
include!(concat!(env!("OUT_DIR"), "/slides.rs"));

struct PresentationData {
    slides:        Vec<Surface>,
    current_slide: usize,
}

impl PresentationData {
    fn new() -> Self {
        let mut slides = Vec::new();

        let slide_contents = get_slides();

        for slide_content in slide_contents.iter() {
            match Surface::from_buffer(slide_content) {
                Ok(mut srf) => {
                    // Self::contrast(&mut srf);
                    slides.push(srf)
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to load slide '{slide_content:?}': {e}").into());
                }
            }
        }

        Self { slides, current_slide: 0 }
    }

    fn contrast(surface: &mut Surface) {
        // Apply contrast to the surface
        for y in 0..surface.size().height {
            for x in 0..surface.size().width {
                let mut ch = *surface.char(x as i32, y as i32).unwrap();
                // ch.foreground = ch.foreground.inverse_color();
                ch.foreground = match ch.foreground {
                    Color::Black => Color::White,
                    Color::DarkBlue => Color::DarkBlue,
                    Color::DarkGreen => Color::DarkGreen,
                    Color::Teal => Color::Teal,
                    Color::DarkRed => Color::DarkRed,
                    Color::Magenta => Color::Magenta,
                    Color::Olive => Color::Black,
                    Color::Silver => Color::Gray,
                    Color::Gray => Color::Black,
                    Color::Blue => Color::DarkBlue,
                    Color::Green => Color::DarkGreen,
                    Color::Aqua => Color::Teal,
                    Color::Red => Color::DarkRed,
                    Color::Pink => Color::Magenta,
                    Color::Yellow => Color::Black,
                    Color::White => Color::Black,
                    Color::Transparent => Color::Transparent,

                    #[cfg(target_feature = "TRUE_COLORS")]
                    Color::RGB(r, g, b) => Color::RGB(255 - r, 255 - g, 255 - b),
                };
                ch.background = Color::White;
                surface.write_char(x as i32, y as i32, ch);
            }
        }
    }

    fn next_slide(&mut self) -> bool {
        if self.current_slide + 1 < self.slides.len() {
            self.current_slide += 1;
            true
        } else {
            false
        }
    }
    fn prev_slide(&mut self) -> bool {
        if self.current_slide > 0 {
            self.current_slide -= 1;
            true
        } else {
            false
        }
    }
    fn current_slide_content(&self) -> Option<&Surface> {
        self.slides.get(self.current_slide)
    }
    fn slide_info(&self) -> String {
        format!("{}/{}", self.current_slide + 1, self.slides.len())
    }
}

#[CustomControl(overwrite = OnPaint + OnKeyPressed)]
struct PresentationControl {
    data: PresentationData,
}

impl PresentationControl {
    pub fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::new(layout, true),
            data: PresentationData::new(),
        }
    }
}

impl OnPaint for PresentationControl {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        surface.clear(Character::new(' ', Color::Black, Color::Black, CharFlags::None));

        let Some(content) = self.data.current_slide_content() else {
            web_sys::console::error_1(&"No content available for the current slide".into());
            return;
        };

        surface.draw_surface(0, 0, content);
        let info = self.data.slide_info();
        let sz = surface.size();
        surface.write_string(
            (sz.width as i32) - (info.len() as i32) - 2,
            (sz.height as i32) - 2,
            &info,
            CharAttribute::with_color(Color::White, Color::Black),
            false,
        );

        surface.write_string(
            2,
            (sz.height as i32) - 2,
            "◄ ► Navigate | ESC Exit",
            CharAttribute::with_color(Color::Gray, Color::Black),
            false,
        );
    }
}

impl OnKeyPressed for PresentationControl {
    fn on_key_pressed(&mut self, key: Key, _ch: char) -> EventProcessStatus {
        let processed = match key.code {
            KeyCode::Right | KeyCode::PageDown | KeyCode::Space => self.data.next_slide(),
            KeyCode::Left | KeyCode::PageUp | KeyCode::Backspace => self.data.prev_slide(),
            KeyCode::Home => {
                self.data.current_slide = 0;
                true
            }
            KeyCode::End => {
                self.data.current_slide = self.data.slides.len() - 1;
                true
            }
            KeyCode::Escape => return EventProcessStatus::Ignored,
            _ => return EventProcessStatus::Ignored,
        };
        if processed {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

#[Window(events = WindowEvents)]
struct PresentationWindow {
    presentation: Handle<PresentationControl>,
}

impl PresentationWindow {
    fn new() -> Self {
        let mut win = Window::new(
            "Web Terminal Presentation",
            Layout::new("d:c,w:100%,h:100%"),
            window::Flags::NoCloseButton,
        );
        let ctl = win.add(PresentationControl::new(Layout::new("d:c,w:100%,h:100%")));
        PresentationWindow {
            base:         win,
            presentation: ctl,
        }
    }
}

impl WindowEvents for PresentationWindow {
    fn on_cancel(&mut self) -> ActionRequest {
        ActionRequest::Allow
    }
}

#[wasm_bindgen]
pub fn wasm_main() {
    let mut theme = Theme::new(Themes::Default);
    theme.window.normal = CharAttribute::with_color(Color::Black, Color::Black);
    theme.border.focused = CharAttribute::with_color(Color::Black, Color::Black);
    theme.text.focused = CharAttribute::with_color(Color::Black, Color::Black);

    let mut app = App::new().single_window().theme(theme).build().unwrap();
    app.add_window(PresentationWindow::new());
    app.run();
}
