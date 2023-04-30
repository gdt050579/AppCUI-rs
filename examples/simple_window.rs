use appcui::controls::*;
use appcui::controls::menu::{Menu};
use appcui::system::*;
use appcui::terminals::TerminalType;

fn main() -> Result<(), appcui::system::Error> {
    let script = "
        Key.Pressed(Tab)
        //Paint('all checkboxes are checked')
        CheckHash(0xECEDF66E8A6A588C)
        Key.Pressed(Enter)
        Resize(50,16)
        //Paint('Checkbox is not checked')
        CheckHash(0x9FADB347088D23A5)
        Mouse.Move(2,0)
        //Paint('After mouse move')
        CheckHash(0x12D05703BE0F9EE5)
    ";
    let mut a = App::debug(60,20,InitializationFlags::Menu,script)?;
    //let mut a = App::default()?;
    //let mut a = App::new(TerminalType::Default,None,InitializationFlags::Menu)?;
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
    let mut m_file = Menu::new();
    a.add_menu(m_file, "&File");
    a.add(w);
    a.run();
    Ok(())
}
