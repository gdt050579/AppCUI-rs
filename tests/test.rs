use appcui::graphics::*;
use appcui::input::KeyCode;
use appcui::terminal::*;

#[test]
fn test_1() {
    let term = TerminalType::new(TerminalType::WindowsConsole);
    assert!(term.is_some());
    let mut term = term.unwrap();
    let mut s = Surface::new(term.get_width(), term.get_height());
    
    s.clear(Character::new(SpecialChar::CircleEmpty, Color::White, Color::DarkBlue,Attribute::None));
    s.fill_rect(5, 5, 7, 7, Character::with_char(' '));
    s.fill_rect(15, 5, 20, 7, Character::new('x',Color::Red, Color::Black, Attribute::Underline));
    s.set_origin(30, 5);
    s.fill_rect_with_size(35, 5, 40, 7, Character::new(SpecialChar::Block25,Color::DarkRed, Color::Black, Attribute::None));
    s.reset_origin();
    s.fill_horizontal_line(1, 3, 5, Character::new('|', Color::Yellow, Color::Red, Attribute::None));

    term.update_screen(&s);

}

#[test]
fn test_2() {
    let term = TerminalType::new(TerminalType::WindowsConsole);
    assert!(term.is_some());
    let mut term = term.unwrap();
    loop {
        let sys_event = term.get_system_event();
        if let SystemEvent::Key(kb) = sys_event {
            println!("Key pressed: {:?}",kb);
            if kb.code == KeyCode::Escape {
                break;
            }
        }
    }
    println!("Done");
}