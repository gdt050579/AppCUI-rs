use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().app_bar().build()?;

    let mut w = Window::with_type("Test", layout!("x:1,y:2,w:40,h:10"), window::Flags::Sizeable, Some(window::Type::Panel), window::Background::Normal);
    w.set_tag("win");
    w.set_hotkey(key!("Alt+B"));
    app.add_window(w);

    let mut w = Window::with_type("Test", layout!("x:50,y:2,w:40,h:10"), window::Flags::Sizeable, Some(window::Type::Round), window::Background::Normal);
    w.set_tag("win");
    w.set_hotkey(key!("Alt+2"));
    app.add_window(w);

    let mut w = Window::with_type("Test", layout!("x:1,y:15,w:40,h:10"), window::Flags::Sizeable, Some(window::Type::Normal), window::Background::Normal);
    w.set_tag("win");
    w.set_hotkey(key!("Alt+3"));
    app.add_window(w);

    let mut w = Window::with_type("Test", layout!("x:50,y:15,w:40,h:10"), window::Flags::Sizeable, Some(window::Type::Normal), window::Background::Normal);
    w.set_tag("win");
    w.set_hotkey(key!("Alt+4"));
    app.add_window(w);



    app.run();
    Ok(())
}
