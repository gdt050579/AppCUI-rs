use appcui::prelude::*;

#[Window(events = AppBarEvents, commands=A)]
pub(crate) struct Win {
    h_file: Handle<appbar::MenuButton>,
    h_edit: Handle<appbar::MenuButton>,
    h_help: Handle<appbar::MenuButton>,
}
impl Win {
    pub(crate) fn new() -> Self {
        let mut w = Win {
            base: window!("'Simple menus',a:c,w:40,h:8,Flags: Sizeable"),
            h_file: Handle::None,
            h_edit: Handle::None,
            h_help: Handle::None,
        };

        w.add(label!(
            "'Three manus: File, Edit and Help, arrange from left o right after the menus associated with the desktop.',d:f"
        ));

        let m = menu!(
            "class: Win, items=[
                { &New, cmd: A , key: Ctrl+N },
                { &Save, cmd: A, key: Ctrl+S },
                { 'Save &as...', cmd: A },
                { &Open, cmd: A, key: Ctrl+O },
                { --- },
                { E&xit, cmd: A, key: Alt+F4 },
            ]"
        );
        w.h_file = w.appbar().add(appbar::MenuButton::new("&File", m, 1, appbar::Side::Left));
        let m = menu!(
            "class: Win, items=[
                { &Copy, cmd: A , key: Ctrl+C },
                { C&ut, cmd: A, key: Ctrl+X },
                { &Paste, cmd: A, key: Ctrl+V },
                { --- },
                { 'Sub menu one', items = [
                      { &Time, cmd: A, Key: F1 },
                      { &Date, cmd: A, Key: F2 },
                      { &Conver, items = [
                           { 'From milliseconds', cmd: A, key: Ctrl+1 },
                           { 'From seconds', cmd: A, key: Ctrl+1 },
                        ] 
                       }
                   ] 
                }
            ]"
        );
        w.h_edit = w.appbar().add(appbar::MenuButton::new("&Edit", m, 1, appbar::Side::Left));
        let m = menu!(
            "class: Win, items=[
                { &About, cmd: A },
                { Welcome, cmd: A },
            ]"
        );
        w.h_help = w.appbar().add(appbar::MenuButton::new("&Help", m, 1, appbar::Side::Left));

        w
    }
}
impl AppBarEvents for Win {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_file);
        appbar.show(self.h_edit);
        appbar.show(self.h_help);
    }
}
