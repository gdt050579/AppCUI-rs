use appcui::prelude::*;

enum Status {
    Start(u32),
    Progress(u32),
    ReachHaltShouldContinue,    
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Response {
    Yes,
    No,
}

#[Window(events = ButtonEvents+BackgroundTaskEvents<Status,Response>)]
struct MyWin {
    p: Handle<ProgressBar>,
    b_start: Handle<Button>,
    b_pause: Handle<Button>,
    b_resume: Handle<Button>,
    bt: Handle<BackgroundTask<Status, Response>>,
}

impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("'Background Task',d:c,w:50,h:8,flags:sizeable"),
            p: Handle::None,
            b_pause: Handle::None,
            b_resume: Handle::None,
            b_start: Handle::None,
            bt: Handle::None,
        };
        w.p = w.add(progressbar!("l:1,t:1,r:1,h:2"));
        w.b_start = w.add(button!("&Start,l:1,b:0,w:10"));
        w.b_pause = w.add(button!("&Pause,l:12,b:0,w:10,enabled: false"));
        w.b_resume = w.add(button!("&Resume,l:23,b:0,w:10, enabled: false"));
        w
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.b_start {
            Window::update_control(self.b_start, |b| b.set_enabled(false));
            Window::update_control(self.b_pause, |b| b.set_enabled(true));
            Window::update_control(self.p, |p| p.update_text(""));
            self.bt = BackgroundTask::<Status, Response>::run(do_something, self.handle());
            EventProcessStatus::Processed
        } else if handle == self.b_pause {
            Window::update_control(self.b_pause, |b| b.set_enabled(false));
            Window::update_control(self.b_resume, |b| b.set_enabled(true));
            Window::update_control(self.p, |p| { p.pause(); p.update_text("Paused"); });
            if let Some(bt) = self.background_task(self.bt) {
                bt.pause();
            }
            EventProcessStatus::Processed
        } else if handle == self.b_resume {
            Window::update_control(self.b_pause, |b| b.set_enabled(true));
            Window::update_control(self.b_resume, |b| b.set_enabled(false));
            Window::update_control(self.p, |p| { p.resume(); p.update_text(""); });
            if let Some(bt) = self.background_task(self.bt) {
                bt.resume();
            }
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
impl BackgroundTaskEvents<Status, Response> for MyWin {
    fn on_update(&mut self, value: Status, _: &BackgroundTask<Status, Response>) -> EventProcessStatus {
        let h = self.p;
        if let Some(p) = self.control_mut(h) {
            match value {
                Status::Start(value) => p.reset(value as u64),
                Status::Progress(value) => p.update_progress(value as u64),
                Status::ReachHaltShouldContinue => {}
            }
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
    fn on_query(&mut self, value: Status, _: &BackgroundTask<Status, Response>) -> Response {
        match value {
            Status::ReachHaltShouldContinue => {
                if dialogs::proceed("Question", "Reached 50% progress. Do you want to continue?") {
                    Response::Yes
                } else {
                    Response::No
                }
            }
            _ => Response::No,
        }
    }
    fn on_finish(&mut self, _: &BackgroundTask<Status, Response>) -> EventProcessStatus {
        Window::update_control(self.b_start, |b| b.set_enabled(true));
        Window::update_control(self.b_pause, |b| b.set_enabled(false));
        Window::update_control(self.b_resume, |b| b.set_enabled(false));  
        Window::update_control(self.p, |p| p.update_text("Finished"));      
        EventProcessStatus::Processed
    }
}

fn do_something(conector: &BackgroundTaskConector<Status, Response>) {
    conector.notify(Status::Start(100));
    for i in 0..100 {
        if conector.should_stop() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        conector.notify(Status::Progress(i));
        if i == 50 {
            if let Some(response) = conector.query(Status::ReachHaltShouldContinue) {
                match response {
                    Response::Yes => {}
                    Response::No => return,
                }
            }
        }
    }
    conector.notify(Status::Progress(100));
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
