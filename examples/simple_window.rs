use appcui::controls::menu::Menu;
use appcui::controls::*;
use appcui::input::{Key, KeyCode, KeyModifier};
use appcui::system::*;
use appcui::terminals::TerminalType;

fn main() -> Result<(), appcui::system::Error> {
    let script = "
        Key.Pressed(Tab)
        Paint('all checkboxes are checked')
        //CheckHash(0xECEDF66E8A6A588C)
        Key.Pressed(Enter)
        Resize(50,16)
        //Paint('Checkbox is not checked')
        //CheckHash(0x9FADB347088D23A5)
        Mouse.Move(16,0)
        //Paint('After mouse move')
        //CheckHash(0x68FB99AFFF8F9CF5)
        Mouse.Click(16,0,left)
        Paint('After mouse click')
        Mouse.Move(16,2);
        Paint('After mouse move')
        Mouse.Click(16,2,left)
        Paint('Show sub-menu')
    ";
    let mut a = App::debug(60, 20, InitializationFlags::Menu, script)?;
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
    m_file.add_command("&New", Key::new(KeyCode::N, KeyModifier::Ctrl), 100);
    m_file.add_command("&Open", Key::new(KeyCode::O, KeyModifier::Ctrl), 101);
    m_file.add_command("&Save", Key::new(KeyCode::S, KeyModifier::Ctrl), 102);
    m_file.add_command("Save &as ...", Key::default(), 103);
    m_file.add_separator();
    m_file.add_checkbox("Option &1", Key::default(), 100, true);
    m_file.add_checkbox(
        "Option &2",
        Key::new(KeyCode::F10, KeyModifier::None),
        101,
        false,
    );
    m_file.add_separator();
    m_file.add_command("E&xit", Key::new(KeyCode::F4, KeyModifier::Alt), 100);
    a.add_menu(m_file, "&File");

    let mut m_opt = Menu::new();
    m_opt.add_radiobox("Radio &1", Key::default(), 1000, false);
    m_opt.add_radiobox("Radio &2", Key::default(), 1000, false);
    m_opt.add_radiobox("Radio &3", Key::default(), 1000, true);
    m_opt.add_radiobox("Radio &4", Key::default(), 1000, false);
    m_opt.add_radiobox("Radio &5", Key::default(), 1000, false);
    a.add_menu(m_opt, "&Radio");

    let mut m_sm = Menu::new();
    let mut m_colors = Menu::new();
    m_colors.add_command("Red", Key::default(), 103);
    m_colors.add_command("Green", Key::default(), 103);
    m_colors.add_command("Blue", Key::default(), 103);
    m_colors.add_command("White", Key::default(), 103);
    m_colors.add_command("Teak", Key::default(), 103);
    m_sm.add_submenu("&Colors", m_colors);
    let mut m_size = Menu::new();
    m_size.add_radiobox("Km", Key::new(KeyCode::F1, KeyModifier::None), 123, false);
    m_size.add_radiobox("Cm", Key::new(KeyCode::F2, KeyModifier::None), 123, false);
    m_size.add_radiobox("Mm", Key::new(KeyCode::F3, KeyModifier::None), 123, true);
    m_sm.add_submenu("&Size in ...", m_size);
    a.add_menu(m_sm, "&Submenus");

    a.add(w);
    a.run();
    Ok(())
}
