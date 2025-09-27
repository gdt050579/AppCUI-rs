use appcui::prelude::*;

#[Window(events=TimePickerEvents)]
struct MyWin {
    dp: Handle<TimePicker>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("Time,a:c,w:25,h:6"),
            dp: Handle::None,
        };
        win.dp = win.add(timepicker!("'12:34:56',x:1,y:1,w:19,flags: Seconds"));
        win
    }
}

impl TimePickerEvents for MyWin {
    fn on_time_changed(&mut self, _handle: Handle<TimePicker>, time: chrono::prelude::NaiveTime) -> EventProcessStatus {
        self.set_title(&format!("Time: {time}"));
        EventProcessStatus::Processed
    }
}

fn main() {
    let mut a = App::new().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
