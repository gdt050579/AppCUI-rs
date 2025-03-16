use appcui::prelude::*;

enum Status {
    Start(u32),
    Progress(u32),
}

#[Window(events = ButtonEvents+BackgroundTaskEvents<Status,bool>)]
struct MyWin {
    p: Handle<ProgressBar>,
    b_start: Handle<Button>,
    b_pause: Handle<Button>,
    b_resume: Handle<Button>,
}

impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("'Background Task',d:c,w:50,h:15,flags:sizeable"),
            p: Handle::None,
            b_pause: Handle::None,
            b_resume: Handle::None,
            b_start: Handle::None,
        };
        w.p = w.add(progressbar!("l:1,t:1,r:1,h:2"));
        w.b_start = w.add(button!("&Start,l:1,b:0,w:10"));
        w.b_pause = w.add(button!("&Pause,l:12,b:0,w:10,enabled: false"));
        w.b_resume = w.add(button!("&Resume,l:23,b:0,w:10, enabled: false"));
        w
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        BackgroundTask::<Status, bool>::new().run(do_something, self.handle());
        EventProcessStatus::Processed
    }
}
impl BackgroundTaskEvents<Status, bool> for MyWin {
    fn on_update(&mut self, value: Status, _: &BackgroundTask<Status, bool>) -> EventProcessStatus {
        let h = self.p;
        if let Some(p) = self.control_mut(h) {
            match value {
                Status::Start(value) => p.reset(value as u64),
                Status::Progress(value) => p.update_progress(value as u64),
            }
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

fn do_something(conector: &BackgroundTaskConector<Status, bool>) {
    conector.notify(Status::Start(100));
    for i in 0..100 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        conector.notify(Status::Progress(i));
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
