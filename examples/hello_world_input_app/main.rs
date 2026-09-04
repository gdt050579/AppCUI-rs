use appcui::prelude::*;

struct HelloWorld;
impl InputApp for HelloWorld {
    fn on_paint(&self, surface: &mut Surface) {
        surface.write_string(0, 0, "Hello World !", charattr!("white"), false);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::input_app(HelloWorld {}).run()
}
