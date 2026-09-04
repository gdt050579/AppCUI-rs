use appcui::prelude::*;


struct Typewriter {
    visible: usize,
}

impl Typewriter {
    const TEXT: &str = "Hello from AppCUI";
    fn new() -> Self {
        Self { visible: 0 }
    }
}

impl FrameApp for Typewriter {
    fn on_update(&mut self, _ticks: u64) {
        self.visible = (self.visible % Self::TEXT.len()) + 1;
    }

    fn on_key_event(&mut self, key: Key, _ch: char) {
        if key.value() == key!("Escape") && dialogs::validate("Quit", "Do you want to quit?") {
            App::close();
        }
    }

    fn on_paint(&self, surface: &mut Surface) {
        let text = &Self::TEXT[..self.visible];
        let size = surface.size();
        let x = (size.width as i32 - text.len() as i32) / 2;
        let y = size.height as i32 / 2;
        surface.write_string(x, y, text, charattr!("white"), false);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::frame_app(Typewriter::new())
        .fps(8)
        .title("Hello from AppCUI")
        .auto_close(false)
        .run()
}
