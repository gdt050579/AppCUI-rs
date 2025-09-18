use appcui::prelude::*;

#[Window(events = AppBarEvents, commands=A)]
pub(crate) struct Win {
    h_sep: Handle<appbar::Separator>,
    h_left: Handle<appbar::MenuButton>,
    h_color: Handle<appbar::MenuButton>,
    h_opt: Handle<appbar::MenuButton>,
}
impl Win {
    pub(crate) fn new() -> Self {
        let mut w = Win {
            base: window!("'Aligned menus',a:c,w:40,h:8,Flags: Sizeable"),
            h_sep: Handle::None,
            h_left: Handle::None,
            h_color: Handle::None,
            h_opt: Handle::None,
        };

        w.add(label!("'Three manus: Left, Color and Options, and a separator from the desktop specific menus to these one. Left menu is aligned to the left, while Color and Options are aligned to th right of the application bar.',d:f"));

        let m = menu!(
            "class: Win, items=[
                { Command-1, cmd: A , key: Ctrl+N },
                { Command-2, cmd: A, key: Ctrl+S },
                { Command-3, cmd: A },
                { --- },
                { E&xit, cmd: A, key: Alt+F4 },
            ]"
        );
        w.h_left = w.appbar_mut().add(appbar::MenuButton::new("&Left", m, 1, appbar::Side::Left));
        let m = menu!(
            "class: Win, items=[
                { &Red, cmd: A , selected: true},
                { &Green, cmd: A, selected: false },
                { &Blue, cmd: A, selected: false },
            ]"
        );
        w.h_color = w.appbar_mut().add(appbar::MenuButton::new("&Color", m, 2, appbar::Side::Right));
        let m = menu!(
            "class: Win, items=[
                { 'Option &1', cmd: A, checked: true },
                { 'Option &2', cmd: A, checked: false },
                { 'Option &3', cmd: A, checked: true },
                { --- },
                { 'Option &1', cmd: A, checked: false },
                { 'Option &1', cmd: A, checked: true },
            ]"
        );
        w.h_opt = w.appbar_mut().add(appbar::MenuButton::new("&Options", m, 1, appbar::Side::Right));
        w.h_sep = w.appbar_mut().add(appbar::Separator::new(1, appbar::Side::Left));
        w
    }
}
impl AppBarEvents for Win {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_sep);
        appbar.show(self.h_left);
        appbar.show(self.h_color);
        appbar.show(self.h_opt);
    }
}
