use appcui::controls::*;
use appcui::system::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::debug(60,20)?;
    //let mut a = App::default()?;
    let mut w = Window::new(
        "Simple window",
        Layout::new("d:c,w:40,h:10"),
        WindowFlags::Sizeable,
    );
    w.add(CheckBox::new(
        "This is a checkbox that &enables a certain property that is required by this program",
        Layout::new("x:1,y:1,w:35,h:3"),
        true,
    ));
    w.add(CheckBox::new(
        "&Second check box",
        Layout::new("x:1,y:4,w:35"),
        true,
    ));
    w.add(CheckBox::new(
        "&Third check box",
        Layout::new("x:1,y:5,w:35"),
        true,
    ));
    a.add(w);
    a.run();
    Ok(())
}
