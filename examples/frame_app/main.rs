use appcui::prelude::*;
use chrono::Local;

struct Clock;

impl FrameApp for Clock {
    fn on_paint(&self, surface: &mut Surface) {
        let size = surface.size();
        let txt = Local::now().format("%H:%M:%S").to_string();
        surface.write_string((size.width / 2 - 4) as i32, (size.height / 2) as i32, &txt, charattr!("white"), false);
    }
    fn on_key_event(&mut self, key: Key, _: char) {
        if key.code == KeyCode::Escape {
            App::close();
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::frame_app(Clock {}).fps(1).run()
}
