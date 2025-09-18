use std::time::Duration;
use flat_string::FlatString;
use appcui::prelude::*;

#[Window(events = AppBarEvents+TimerEvents, commands=A)]
pub(crate) struct Win {
    cnt: i32,
    tmp: String,
    h_label: Handle<appbar::Label>,
}
impl Win {
    pub(crate) fn new() -> Self {
        let mut w = Win {
            base: window!("'Progress Bar',a:c,w:40,h:8,Flags: Sizeable"),
            h_label: Handle::None,
            cnt: 0,
            tmp: String::with_capacity(16),
        };

        w.add(label!(
            "'Three manus: File, Edit and Help, with File enabled, and Edit menu disabled (both on the left) and Help menu on the right',d:f"
        ));
        w.h_label = w.appbar().add(appbar::Label::new("", 0, appbar::Side::Right));

        if let Some(timer) = w.timer() {
            timer.start(Duration::from_secs(1));
        }

        w
    }
}
impl TimerEvents for Win {
    fn on_update(&mut self, _: u64) -> EventProcessStatus {
        self.cnt = (self.cnt + 1) / 10;
        let mut f: FlatString<20> = FlatString::new();
        f.push('[');
        for _ in 0..self.cnt {
            f.push(SpecialChar::BlockCentered.into());
        }
        for _ in self.cnt..10 {
            f.push(' ');
        }
        f.push(']');
        let h = self.h_label;
        self.appbar().get_mut(h).unwrap().set_caption(f.as_str());
        EventProcessStatus::Ignored
    }
}
impl AppBarEvents for Win {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.h_label);
    }
}
