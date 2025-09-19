use appcui::prelude::*;
use flat_string::FlatString;
use std::time::Duration;

#[Window(events = AppBarEvents+TimerEvents, commands=A)]
pub(crate) struct Win {
    cnt: i32,
    h_label: Handle<appbar::Label>,
}
impl Win {
    pub(crate) fn new() -> Self {
        let mut w = Win {
            base: window!("'Progress Bar',a:c,w:40,h:8,Flags: Sizeable"),
            h_label: Handle::None,
            cnt: 0,
        };

        w.add(label!("'One label where we simulate a download progress bar',d:f"));
        w.h_label = w.appbar().add(appbar::Label::new("", 0, appbar::Side::Right));

        if let Some(timer) = w.timer() {
            timer.start(Duration::from_secs(1));
        }

        w
    }
}
impl TimerEvents for Win {
    fn on_update(&mut self, _: u64) -> EventProcessStatus {
        const PROG_MAX: i32 = 15;
        self.cnt = (self.cnt + 1) % PROG_MAX;
        let mut f: FlatString<64> = FlatString::new();
        f.push('[');
        for _ in 0..self.cnt {
            f.push(SpecialChar::BlockCentered.into());
        }
        for _ in self.cnt..PROG_MAX - 1 {
            f.push(' ');
        }
        f.push(']');
        f.push(' ');
        let proc = self.cnt * 100 / (PROG_MAX - 1);
        if proc >= 100 {
            f.push_str("100");
        } else {
            f.push(' ');
            if proc >= 10 {
                f.push((proc / 10 + 48) as u8 as char);
            } else {
                f.push(' ');
            }
            f.push((proc % 10 + 48) as u8 as char);
        }
        f.push('%');
        f.push(' ');
        let h = self.h_label;
        self.appbar().get_mut(h).unwrap().set_caption(f.as_str());
        f.clear();
        f.push_str("Downloading ...\n");
        f.push_str("Status: ");
        if proc >= 100 {
            f.push_str("100");
        } else {
            f.push(' ');
            if proc >= 10 {
                f.push((proc / 10 + 48) as u8 as char);
            } else {
                f.push(' ');
            }
            f.push((proc % 10 + 48) as u8 as char);
        }
        f.push('%');
        self.appbar().get_mut(h).unwrap().set_tooltip(f.as_str());

        EventProcessStatus::Processed
    }
}
impl AppBarEvents for Win {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_label);
    }
}
