use crate::prelude::*;

#[test]
fn check_on_paint() {
    let script = "
        Paint.Enable(false)
        Paint('tests')   
        CheckHash(0x7309311AC45F730F)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("test,d:c,w:40,h:10");
    w.add(KeySelector::new(
        Key::new(KeyCode::F1, KeyModifier::Ctrl),
        Layout::new("x:1,y:1,w:35,h:1"),
        keyselector::Flags::None,
    ));
    w.add(KeySelector::new(
        Key::new(KeyCode::Insert, KeyModifier::Ctrl|KeyModifier::Alt),
        Layout::new("x:1,y:3,w:35,h:1"),
        keyselector::Flags::None,
    ));
    let mut ks = KeySelector::new(
        Key::new(KeyCode::Escape, KeyModifier::Ctrl|KeyModifier::Shift),
        Layout::new("x:1,y:5,w:35,h:1"),
        keyselector::Flags::AcceptEscape,
    );
    ks.set_enabled(false);
    w.add(ks);
    a.add_window(w);
    a.run();
}
