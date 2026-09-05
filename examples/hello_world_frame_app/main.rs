use appcui::prelude::*;

struct HelloWorld;
impl FrameApp for HelloWorld {
    fn on_paint(&self, surface: &mut Surface) {
        surface.write_string(0, 0, "Hello World !", charattr!("white"), false);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::frame_app(HelloWorld {}).fps(1).run()
}
