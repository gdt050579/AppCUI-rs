mod digits;
use std::time::Duration;
use appcui::prelude::*;



#[Window(events = ButtonEvents+ TimerEvents)]
struct MyWin {
    h: Handle<ListBox>,
}
impl MyWin {
    fn new() -> Self {
        let mut w = Self { base: window!("Test, d:c"), h: Handle::None };
        w.h = w.add(listbox!("x:0,y:0,w:100%,h:50%,flags:Autoscroll"));
        w.add(button!("Test,d:b,w:10"));
        w
    }
    fn log(&mut self, text: &str) {
        let h = self.h;
        if let Some(lb) = self.control_mut(h){
            lb.add(text);
        }
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        self.log("Attemp to start the timer ...");
        if let Some(t) = self.timer() {
            t.start(Duration::from_secs(1));
        } else {
            self.log("Fail to start the timer ...");
        }
        EventProcessStatus::Processed
    }
}
impl TimerEvents for MyWin {
    fn on_start(&mut self) -> EventProcessStatus {
        self.log("Timer started!");
        EventProcessStatus::Processed
    }

    fn on_resume(&mut self, _tick: u64) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    fn on_pause(&mut self, _tick: u64) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }

    fn on_update(&mut self, tick: u64) -> EventProcessStatus {
        self.log(format!("Tick: {}",tick).as_str());
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
