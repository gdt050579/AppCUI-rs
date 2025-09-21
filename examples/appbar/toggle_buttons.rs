use appcui::prelude::*;

#[Window(events = AppBarEvents)]
pub(crate) struct Win {
    lb: Handle<Label>,
    h_bold: Handle<appbar::ToggleButton>,
    h_italic: Handle<appbar::ToggleButton>,
    h_underline: Handle<appbar::ToggleButton>,
    h_sep: Handle<appbar::Separator>,
}
impl Win {
    pub(crate) fn new() -> Self {
        let mut w = Win {
            base: window!("'Toggle Buttons',a:c,w:40,h:8,Flags: Sizeable"),
            lb: Handle::None,
            h_bold: Handle::None,
            h_italic: Handle::None,
            h_underline: Handle::None,
            h_sep: Handle::None,
        };

        w.lb = w.add(label!("'Toggle buttons can change their state from selected or not',d:f"));
        w.h_sep = w.appbar().add(appbar::Separator::new(1, appbar::Side::Left));
        w.h_bold = w
            .appbar()
            .add(appbar::ToggleButton::with_tooltip(" B ", "Bold", false, 1, appbar::Side::Left));
        w.h_italic = w
            .appbar()
            .add(appbar::ToggleButton::with_tooltip(" I ", "Italc", false, 1, appbar::Side::Left));
        w.h_underline = w
            .appbar()
            .add(appbar::ToggleButton::with_tooltip(" U ", "Underline", false, 1, appbar::Side::Left));

        w
    }
}
impl AppBarEvents for Win {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_sep);
        appbar.show(self.h_bold);
        appbar.show(self.h_italic);
        appbar.show(self.h_underline);
    }

    fn on_togglebutton_state_changed(&mut self, togglebutton: Handle<appbar::ToggleButton>, selected: bool) {
        let s = match () {
            _ if togglebutton == self.h_bold => "Bold is :",
            _ if togglebutton == self.h_italic => "Italic is :",
            _ if togglebutton == self.h_underline => "Underline is :",
            _ => ""
        };
        let txt = format!("{s} {selected}");
        let h = self.lb;
        if let Some(lb) = self.control_mut(h) {
            lb.set_caption(&txt);
        }
    }
}
