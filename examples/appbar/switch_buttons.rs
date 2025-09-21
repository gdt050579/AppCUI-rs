use appcui::prelude::*;

#[Window(events = AppBarEvents)]
pub(crate) struct Win {
    lb: Handle<Label>,
    h_save: Handle<appbar::Button>,
    h_auto_save: Handle<appbar::SwitchButton>,
    h_sep: Handle<appbar::Separator>,
}
impl Win {
    pub(crate) fn new() -> Self {
        let mut w = Win {
            base: window!("'Switch Buttons',a:c,w:40,h:8,Flags: Sizeable"),
            lb: Handle::None,
            h_save: Handle::None,
            h_auto_save: Handle::None,
            h_sep: Handle::None,
        };

        w.lb = w.add(label!("'A save button with a pictogram and an auto-save switch button',d:f"));
        w.h_sep = w.appbar().add(appbar::Separator::new(1, appbar::Side::Left));
        w.h_save = w.appbar().add(appbar::Button::with_tooltip("💾  ", "Save", 1, appbar::Side::Left));
        w.h_auto_save = w.appbar().add(appbar::SwitchButton::with_tooltip(
            "on ",
            "off",
            appbar::SwitchButtonSymbol::CheckBox,
            "Enable/Disable autosave",
            false,
            1,
            appbar::Side::Left,
        ));
        w
    }
}
impl AppBarEvents for Win {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_sep);
        appbar.show(self.h_save);
        appbar.show(self.h_auto_save);
    }
}
