use appcui::prelude::*;

#[Window(events = AppBarEvents)]
pub(crate) struct Win {
    cnt: u8,
    h_minus: Handle<appbar::Button>,
    h_plus: Handle<appbar::Button>,
    h_label: Handle<appbar::Label>,
    h_sep: Handle<appbar::Separator>,
}
impl Win {
    pub(crate) fn new() -> Self {
        let mut w = Win {
            base: window!("'Navigation',a:c,w:40,h:8,Flags: Sizeable"),
            h_label: Handle::None,
            h_plus: Handle::None,
            h_minus: Handle::None,
            h_sep: Handle::None,
            cnt: 2,
        };

        w.add(label!("'Two buttons < and > and a page status informion (e.g. 1/3)',d:f"));
        w.h_sep = w.appbar().add(appbar::Separator::new(1, appbar::Side::Left));
        w.h_label = w.appbar().add(appbar::Label::new("2/5", 1, appbar::Side::Left));
        w.h_minus = w.appbar().add(appbar::Button::with_tooltip(" < ", "Previous", 1, appbar::Side::Left));
        w.h_plus = w.appbar().add(appbar::Button::with_tooltip(" > ", "Next", 1, appbar::Side::Left));

        w
    }
}
impl AppBarEvents for Win {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_sep);
        appbar.show(self.h_minus);
        appbar.show(self.h_label);
        appbar.show(self.h_plus);
    }

    fn on_button_click(&mut self, button: Handle<appbar::Button>) {
        if button == self.h_minus {
            self.cnt = (self.cnt - 1).clamp(1, 5);
        }
        if button == self.h_plus {
            self.cnt = (self.cnt + 1).clamp(1, 5);
        }
        let cnt = self.cnt;
        let h = self.h_minus;
        self.appbar().get_mut(h).unwrap().set_enabled(cnt > 1);
        let h = self.h_plus;
        self.appbar().get_mut(h).unwrap().set_enabled(cnt < 5);
        let v: [u8; 3] = [cnt + 48, b'/', b'5'];
        let h = self.h_label;
        self.appbar().get_mut(h).unwrap().set_caption(str::from_utf8(&v).unwrap());
    }
}
