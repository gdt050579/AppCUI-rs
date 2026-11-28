use appcui::prelude::*;

fn hello_world_window() -> Window {
    let mut win = Window::new(
        "First Window",
        LayoutBuilder::new().alignment(Alignment::Center).width(30).height(9).build(),
        window::Flags::Sizeable,
    );
    win.add(Label::new(
        "Hello World !",
        LayoutBuilder::new().alignment(Alignment::Center).width(13).height(1).build(),
    ));
    win
}
fn main() -> Result<(), appcui::system::Error> {
    App::new().window(hello_world_window).run()
}
