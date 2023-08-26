use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::default()?;
    let mut win = Window::new("First Window", Layout::new("d:c,w:30,h:9"), window::Flags::None);
    win.add(Label::new("Hello World !",Layout::new("d:c,w:13")));
    app.add_window(win);
    app.run();
    Ok(())
}