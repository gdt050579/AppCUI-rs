use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().app_bar().build()?;
    let mut w = Window::with_type("Test", layout!("a:c,w:40,h:10"), window::Flags::Sizeable, Some(window::Type::Panel), window::Background::Normal);
    w.set_tag("win");
    w.set_hotkey(key!("Alt+1"));
    app.add_window(w);
    app.run();
    Ok(())
}
