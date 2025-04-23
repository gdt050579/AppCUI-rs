use crate::prelude::*;

#[test]
fn check_from_for_keycode() {
    assert_eq!(KeyCode::from(0),KeyCode::None);
    assert_eq!(KeyCode::from(1),KeyCode::F1);
    assert_eq!(KeyCode::from(2),KeyCode::F2);
    assert_eq!(KeyCode::from(3),KeyCode::F3);
    assert_eq!(KeyCode::from(4),KeyCode::F4);
    assert_eq!(KeyCode::from(5),KeyCode::F5);
    assert_eq!(KeyCode::from(6),KeyCode::F6);
    assert_eq!(KeyCode::from(7),KeyCode::F7);
    assert_eq!(KeyCode::from(8),KeyCode::F8);
    assert_eq!(KeyCode::from(9),KeyCode::F9);
    assert_eq!(KeyCode::from(10),KeyCode::F10);
    assert_eq!(KeyCode::from(11),KeyCode::F11);
    assert_eq!(KeyCode::from(12),KeyCode::F12);
    assert_eq!(KeyCode::from(13),KeyCode::Enter);
    assert_eq!(KeyCode::from(14),KeyCode::Escape);
    assert_eq!(KeyCode::from(15),KeyCode::Insert);
    assert_eq!(KeyCode::from(16),KeyCode::Delete);
    assert_eq!(KeyCode::from(17),KeyCode::Backspace);
    assert_eq!(KeyCode::from(18),KeyCode::Tab);
    assert_eq!(KeyCode::from(19),KeyCode::Left);
    assert_eq!(KeyCode::from(20),KeyCode::Up);
    assert_eq!(KeyCode::from(21),KeyCode::Down);
    assert_eq!(KeyCode::from(22),KeyCode::Right);
    assert_eq!(KeyCode::from(23),KeyCode::PageUp);
    assert_eq!(KeyCode::from(24),KeyCode::PageDown);
    assert_eq!(KeyCode::from(25),KeyCode::Home);
    assert_eq!(KeyCode::from(26),KeyCode::End);
    assert_eq!(KeyCode::from(27),KeyCode::Space);
    assert_eq!(KeyCode::from(28),KeyCode::A);
    assert_eq!(KeyCode::from(29),KeyCode::B);
    assert_eq!(KeyCode::from(30),KeyCode::C);
    assert_eq!(KeyCode::from(31),KeyCode::D);
    assert_eq!(KeyCode::from(32),KeyCode::E);
    assert_eq!(KeyCode::from(33),KeyCode::F);
    assert_eq!(KeyCode::from(34),KeyCode::G);
    assert_eq!(KeyCode::from(35),KeyCode::H);
    assert_eq!(KeyCode::from(36),KeyCode::I);
    assert_eq!(KeyCode::from(37),KeyCode::J);
    assert_eq!(KeyCode::from(38),KeyCode::K);
    assert_eq!(KeyCode::from(39),KeyCode::L);
    assert_eq!(KeyCode::from(40),KeyCode::M);
    assert_eq!(KeyCode::from(41),KeyCode::N);
    assert_eq!(KeyCode::from(42),KeyCode::O);
    assert_eq!(KeyCode::from(43),KeyCode::P);
    assert_eq!(KeyCode::from(44),KeyCode::Q);
    assert_eq!(KeyCode::from(45),KeyCode::R);
    assert_eq!(KeyCode::from(46),KeyCode::S);
    assert_eq!(KeyCode::from(47),KeyCode::T);
    assert_eq!(KeyCode::from(48),KeyCode::U);
    assert_eq!(KeyCode::from(49),KeyCode::V);
    assert_eq!(KeyCode::from(50),KeyCode::W);
    assert_eq!(KeyCode::from(51),KeyCode::X);
    assert_eq!(KeyCode::from(52),KeyCode::Y);
    assert_eq!(KeyCode::from(53),KeyCode::Z);
    assert_eq!(KeyCode::from(54),KeyCode::N0);
    assert_eq!(KeyCode::from(55),KeyCode::N1);
    assert_eq!(KeyCode::from(56),KeyCode::N2);
    assert_eq!(KeyCode::from(57),KeyCode::N3);
    assert_eq!(KeyCode::from(58),KeyCode::N4);
    assert_eq!(KeyCode::from(59),KeyCode::N5);
    assert_eq!(KeyCode::from(60),KeyCode::N6);
    assert_eq!(KeyCode::from(61),KeyCode::N7);
    assert_eq!(KeyCode::from(62),KeyCode::N8);
    assert_eq!(KeyCode::from(63),KeyCode::N9);
    for i in 64..255 {
        assert_eq!(KeyCode::from(i),KeyCode::None);
    }
}

#[test]
fn check_create_hotkey() {
    assert_eq!(Key::create_hotkey('a', KeyModifier::None).code,KeyCode::A);
    assert_eq!(Key::create_hotkey('A', KeyModifier::None).code,KeyCode::A);
    assert_eq!(Key::create_hotkey('b', KeyModifier::None).code,KeyCode::B);
    assert_eq!(Key::create_hotkey('B', KeyModifier::None).code,KeyCode::B);
    assert_eq!(Key::create_hotkey('c', KeyModifier::None).code,KeyCode::C);
    assert_eq!(Key::create_hotkey('C', KeyModifier::None).code,KeyCode::C);
    assert_eq!(Key::create_hotkey('d', KeyModifier::None).code,KeyCode::D);
    assert_eq!(Key::create_hotkey('D', KeyModifier::None).code,KeyCode::D);
    assert_eq!(Key::create_hotkey('e', KeyModifier::None).code,KeyCode::E);
    assert_eq!(Key::create_hotkey('E', KeyModifier::None).code,KeyCode::E);
    assert_eq!(Key::create_hotkey('f', KeyModifier::None).code,KeyCode::F);
    assert_eq!(Key::create_hotkey('F', KeyModifier::None).code,KeyCode::F);
    assert_eq!(Key::create_hotkey('g', KeyModifier::None).code,KeyCode::G);
    assert_eq!(Key::create_hotkey('G', KeyModifier::None).code,KeyCode::G);
    assert_eq!(Key::create_hotkey('h', KeyModifier::None).code,KeyCode::H);
    assert_eq!(Key::create_hotkey('H', KeyModifier::None).code,KeyCode::H);
    assert_eq!(Key::create_hotkey('i', KeyModifier::None).code,KeyCode::I);
    assert_eq!(Key::create_hotkey('I', KeyModifier::None).code,KeyCode::I);
    assert_eq!(Key::create_hotkey('j', KeyModifier::None).code,KeyCode::J);
    assert_eq!(Key::create_hotkey('J', KeyModifier::None).code,KeyCode::J);
    assert_eq!(Key::create_hotkey('k', KeyModifier::None).code,KeyCode::K);
    assert_eq!(Key::create_hotkey('K', KeyModifier::None).code,KeyCode::K);
    assert_eq!(Key::create_hotkey('l', KeyModifier::None).code,KeyCode::L);
    assert_eq!(Key::create_hotkey('L', KeyModifier::None).code,KeyCode::L);
    assert_eq!(Key::create_hotkey('m', KeyModifier::None).code,KeyCode::M);
    assert_eq!(Key::create_hotkey('M', KeyModifier::None).code,KeyCode::M);
    assert_eq!(Key::create_hotkey('n', KeyModifier::None).code,KeyCode::N);
    assert_eq!(Key::create_hotkey('N', KeyModifier::None).code,KeyCode::N);
    assert_eq!(Key::create_hotkey('o', KeyModifier::None).code,KeyCode::O);
    assert_eq!(Key::create_hotkey('O', KeyModifier::None).code,KeyCode::O);
    assert_eq!(Key::create_hotkey('p', KeyModifier::None).code,KeyCode::P);
    assert_eq!(Key::create_hotkey('P', KeyModifier::None).code,KeyCode::P);
    assert_eq!(Key::create_hotkey('q', KeyModifier::None).code,KeyCode::Q);
    assert_eq!(Key::create_hotkey('Q', KeyModifier::None).code,KeyCode::Q);
    assert_eq!(Key::create_hotkey('r', KeyModifier::None).code,KeyCode::R);
    assert_eq!(Key::create_hotkey('R', KeyModifier::None).code,KeyCode::R);
    assert_eq!(Key::create_hotkey('s', KeyModifier::None).code,KeyCode::S);
    assert_eq!(Key::create_hotkey('S', KeyModifier::None).code,KeyCode::S);
    assert_eq!(Key::create_hotkey('t', KeyModifier::None).code,KeyCode::T);
    assert_eq!(Key::create_hotkey('T', KeyModifier::None).code,KeyCode::T);
    assert_eq!(Key::create_hotkey('u', KeyModifier::None).code,KeyCode::U);
    assert_eq!(Key::create_hotkey('U', KeyModifier::None).code,KeyCode::U);
    assert_eq!(Key::create_hotkey('v', KeyModifier::None).code,KeyCode::V);
    assert_eq!(Key::create_hotkey('V', KeyModifier::None).code,KeyCode::V);
    assert_eq!(Key::create_hotkey('w', KeyModifier::None).code,KeyCode::W);
    assert_eq!(Key::create_hotkey('W', KeyModifier::None).code,KeyCode::W);
    assert_eq!(Key::create_hotkey('x', KeyModifier::None).code,KeyCode::X);
    assert_eq!(Key::create_hotkey('X', KeyModifier::None).code,KeyCode::X);
    assert_eq!(Key::create_hotkey('y', KeyModifier::None).code,KeyCode::Y);
    assert_eq!(Key::create_hotkey('Y', KeyModifier::None).code,KeyCode::Y);
    assert_eq!(Key::create_hotkey('z', KeyModifier::None).code,KeyCode::Z);
    assert_eq!(Key::create_hotkey('Z', KeyModifier::None).code,KeyCode::Z);

    assert_eq!(Key::create_hotkey('0', KeyModifier::None).code,KeyCode::N0);
    assert_eq!(Key::create_hotkey('1', KeyModifier::None).code,KeyCode::N1);
    assert_eq!(Key::create_hotkey('2', KeyModifier::None).code,KeyCode::N2);
    assert_eq!(Key::create_hotkey('3', KeyModifier::None).code,KeyCode::N3);
    assert_eq!(Key::create_hotkey('4', KeyModifier::None).code,KeyCode::N4);
    assert_eq!(Key::create_hotkey('5', KeyModifier::None).code,KeyCode::N5);
    assert_eq!(Key::create_hotkey('6', KeyModifier::None).code,KeyCode::N6);
    assert_eq!(Key::create_hotkey('7', KeyModifier::None).code,KeyCode::N7);
    assert_eq!(Key::create_hotkey('8', KeyModifier::None).code,KeyCode::N8);
    assert_eq!(Key::create_hotkey('9', KeyModifier::None).code,KeyCode::N9);

    assert_eq!(Key::create_hotkey('_', KeyModifier::None).code,KeyCode::None);
}


#[test]
fn check_key_from16() {
    for i in 64..255 {
        assert_eq!(Key::from(i as u16),Key::None);
    }
}

#[test]
fn check_key_from_char() {
    assert_eq!(Key::from('a'), Key::new(KeyCode::A, KeyModifier::None));    
    assert_eq!(Key::from('b'), Key::new(KeyCode::B, KeyModifier::None));
    assert_eq!(Key::from('c'), Key::new(KeyCode::C, KeyModifier::None));
    assert_eq!(Key::from('d'), Key::new(KeyCode::D, KeyModifier::None));
    assert_eq!(Key::from('e'), Key::new(KeyCode::E, KeyModifier::None));
    assert_eq!(Key::from('f'), Key::new(KeyCode::F, KeyModifier::None));
    assert_eq!(Key::from('g'), Key::new(KeyCode::G, KeyModifier::None));
    assert_eq!(Key::from('h'), Key::new(KeyCode::H, KeyModifier::None));
    assert_eq!(Key::from('i'), Key::new(KeyCode::I, KeyModifier::None));
    assert_eq!(Key::from('j'), Key::new(KeyCode::J, KeyModifier::None));
    assert_eq!(Key::from('k'), Key::new(KeyCode::K, KeyModifier::None));
    assert_eq!(Key::from('l'), Key::new(KeyCode::L, KeyModifier::None));
    assert_eq!(Key::from('m'), Key::new(KeyCode::M, KeyModifier::None));
    assert_eq!(Key::from('n'), Key::new(KeyCode::N, KeyModifier::None));
    assert_eq!(Key::from('o'), Key::new(KeyCode::O, KeyModifier::None));
    assert_eq!(Key::from('p'), Key::new(KeyCode::P, KeyModifier::None));
    assert_eq!(Key::from('q'), Key::new(KeyCode::Q, KeyModifier::None));
    assert_eq!(Key::from('r'), Key::new(KeyCode::R, KeyModifier::None));
    assert_eq!(Key::from('s'), Key::new(KeyCode::S, KeyModifier::None));
    assert_eq!(Key::from('t'), Key::new(KeyCode::T, KeyModifier::None));
    assert_eq!(Key::from('u'), Key::new(KeyCode::U, KeyModifier::None));
    assert_eq!(Key::from('v'), Key::new(KeyCode::V, KeyModifier::None));
    assert_eq!(Key::from('w'), Key::new(KeyCode::W, KeyModifier::None));
    assert_eq!(Key::from('x'), Key::new(KeyCode::X, KeyModifier::None));
    assert_eq!(Key::from('y'), Key::new(KeyCode::Y, KeyModifier::None));
    assert_eq!(Key::from('z'), Key::new(KeyCode::Z, KeyModifier::None));

    assert_eq!(Key::from('A'), Key::new(KeyCode::A, KeyModifier::Shift));    
    assert_eq!(Key::from('B'), Key::new(KeyCode::B, KeyModifier::Shift));
    assert_eq!(Key::from('C'), Key::new(KeyCode::C, KeyModifier::Shift));
    assert_eq!(Key::from('D'), Key::new(KeyCode::D, KeyModifier::Shift));
    assert_eq!(Key::from('E'), Key::new(KeyCode::E, KeyModifier::Shift));
    assert_eq!(Key::from('F'), Key::new(KeyCode::F, KeyModifier::Shift));
    assert_eq!(Key::from('G'), Key::new(KeyCode::G, KeyModifier::Shift));
    assert_eq!(Key::from('H'), Key::new(KeyCode::H, KeyModifier::Shift));
    assert_eq!(Key::from('I'), Key::new(KeyCode::I, KeyModifier::Shift));
    assert_eq!(Key::from('J'), Key::new(KeyCode::J, KeyModifier::Shift));
    assert_eq!(Key::from('K'), Key::new(KeyCode::K, KeyModifier::Shift));
    assert_eq!(Key::from('L'), Key::new(KeyCode::L, KeyModifier::Shift));
    assert_eq!(Key::from('M'), Key::new(KeyCode::M, KeyModifier::Shift));
    assert_eq!(Key::from('N'), Key::new(KeyCode::N, KeyModifier::Shift));
    assert_eq!(Key::from('O'), Key::new(KeyCode::O, KeyModifier::Shift));
    assert_eq!(Key::from('P'), Key::new(KeyCode::P, KeyModifier::Shift));
    assert_eq!(Key::from('Q'), Key::new(KeyCode::Q, KeyModifier::Shift));
    assert_eq!(Key::from('R'), Key::new(KeyCode::R, KeyModifier::Shift));
    assert_eq!(Key::from('S'), Key::new(KeyCode::S, KeyModifier::Shift));
    assert_eq!(Key::from('T'), Key::new(KeyCode::T, KeyModifier::Shift));
    assert_eq!(Key::from('U'), Key::new(KeyCode::U, KeyModifier::Shift));
    assert_eq!(Key::from('V'), Key::new(KeyCode::V, KeyModifier::Shift));
    assert_eq!(Key::from('W'), Key::new(KeyCode::W, KeyModifier::Shift));
    assert_eq!(Key::from('X'), Key::new(KeyCode::X, KeyModifier::Shift));
    assert_eq!(Key::from('Y'), Key::new(KeyCode::Y, KeyModifier::Shift));
    assert_eq!(Key::from('Z'), Key::new(KeyCode::Z, KeyModifier::Shift));

    assert_eq!(Key::from('0'), Key::new(KeyCode::N0, KeyModifier::None));
    assert_eq!(Key::from('1'), Key::new(KeyCode::N1, KeyModifier::None));
    assert_eq!(Key::from('2'), Key::new(KeyCode::N2, KeyModifier::None));
    assert_eq!(Key::from('3'), Key::new(KeyCode::N3, KeyModifier::None));
    assert_eq!(Key::from('4'), Key::new(KeyCode::N4, KeyModifier::None));
    assert_eq!(Key::from('5'), Key::new(KeyCode::N5, KeyModifier::None));
    assert_eq!(Key::from('6'), Key::new(KeyCode::N6, KeyModifier::None));
    assert_eq!(Key::from('7'), Key::new(KeyCode::N7, KeyModifier::None));
    assert_eq!(Key::from('8'), Key::new(KeyCode::N8, KeyModifier::None));
    assert_eq!(Key::from('9'), Key::new(KeyCode::N9, KeyModifier::None));

    assert_eq!(Key::from(')'), Key::new(KeyCode::N0, KeyModifier::Shift));
    assert_eq!(Key::from('!'), Key::new(KeyCode::N1, KeyModifier::Shift));
    assert_eq!(Key::from('@'), Key::new(KeyCode::N2, KeyModifier::Shift));
    assert_eq!(Key::from('#'), Key::new(KeyCode::N3, KeyModifier::Shift));
    assert_eq!(Key::from('$'), Key::new(KeyCode::N4, KeyModifier::Shift));
    assert_eq!(Key::from('%'), Key::new(KeyCode::N5, KeyModifier::Shift));
    assert_eq!(Key::from('^'), Key::new(KeyCode::N6, KeyModifier::Shift));
    assert_eq!(Key::from('&'), Key::new(KeyCode::N7, KeyModifier::Shift));
    assert_eq!(Key::from('*'), Key::new(KeyCode::N8, KeyModifier::Shift));
    assert_eq!(Key::from('('), Key::new(KeyCode::N9, KeyModifier::Shift));

    assert_eq!(Key::from(' '), Key::new(KeyCode::Space, KeyModifier::None));
    assert_eq!(Key::from('\n'), Key::new(KeyCode::Enter, KeyModifier::None));
    assert_eq!(Key::from('\t'), Key::new(KeyCode::Tab, KeyModifier::None));
}

#[test]
fn check_keymodifier_from() {
    assert_eq!(KeyModifier::from(0), KeyModifier::None);
    assert_eq!(KeyModifier::from(1), KeyModifier::Alt);
    assert_eq!(KeyModifier::from(2), KeyModifier::Ctrl);
    assert_eq!(KeyModifier::from(3), KeyModifier::Alt | KeyModifier::Ctrl);
    assert_eq!(KeyModifier::from(4), KeyModifier::Shift);
    assert_eq!(KeyModifier::from(5), KeyModifier::Shift | KeyModifier::Alt);
    assert_eq!(KeyModifier::from(6), KeyModifier::Ctrl | KeyModifier::Shift);
    assert_eq!(KeyModifier::from(7), KeyModifier::Shift | KeyModifier::Ctrl | KeyModifier::Alt);
    assert_eq!(KeyModifier::from(8), KeyModifier::None);
}

#[test]
fn check_key_display() {
    let key = Key::new(KeyCode::A, KeyModifier::Ctrl|KeyModifier::Shift);
    assert_eq!(key.to_string(), "Ctrl+Shift+A");
}