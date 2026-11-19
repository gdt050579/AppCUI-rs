use appcui::prelude::*;
use std::time::Duration;

// The bar runs from 0 to TOTAL_STEPS, moving one step on every timer tick (~5 seconds in total).
const TOTAL_STEPS: u64 = 100;
const TICK: Duration = Duration::from_millis(50);

#[Window(events = ButtonEvents + TimerEvents)]
#[derive(Default)]
struct MyWin {
    progress: Handle<ProgressBar>,
    b_start: Handle<Button>,
    b_pause: Handle<Button>,
    b_resume: Handle<Button>,
    done: bool,
}

impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("'Progress Bar',a:c,w:44,h:9"),
            ..Default::default()
        };
        w.progress = w.add(progressbar!("total:100,x:1,y:1,w:40,h:2,text:'Press Start to begin'"));
        w.b_start = w.add(button!("&Start,x:1,y:4,w:12"));
        w.b_pause = w.add(button!("&Pause,x:15,y:4,w:12,enable:false"));
        w.b_resume = w.add(button!("&Resume,x:29,y:4,w:12,enable:false"));
        w
    }

    // Moves the bar to `step` and updates its caption.
    fn show_progress(&mut self, step: u64, text: &str) {
        let h = self.progress;
        if let Some(p) = self.control_mut(h) {
            p.update_progress(step.min(TOTAL_STEPS));
            p.update_text(text);
        }
    }

    // After the first run the Start button becomes "Restart".
    fn set_buttons(&mut self, can_pause: bool, can_resume: bool) {
        let (s, p, r) = (self.b_start, self.b_pause, self.b_resume);
        if let Some(b) = self.control_mut(s) {
            b.set_caption("Re&start");
        }
        if let Some(b) = self.control_mut(p) {
            b.set_enabled(can_pause);
        }
        if let Some(b) = self.control_mut(r) {
            b.set_enabled(can_resume);
        }
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        let (start, pause) = (self.b_start, self.b_pause);
        if let Some(timer) = self.timer() {
            match handle {
                // start() also resets the tick counter, so the bar restarts from 0.
                h if h == start => timer.start(TICK),
                h if h == pause => timer.pause(),
                _ => timer.resume(),
            }
        }
        EventProcessStatus::Processed
    }
}

impl TimerEvents for MyWin {
    fn on_start(&mut self) -> EventProcessStatus {
        let h = self.progress;
        if let Some(p) = self.control_mut(h) {
            p.reset(TOTAL_STEPS); // clear the previous run
        }
        self.done = false;
        self.show_progress(0, "Working ...");
        self.set_buttons(true, false);
        EventProcessStatus::Processed
    }

    fn on_update(&mut self, ticks: u64) -> EventProcessStatus {
        self.show_progress(ticks, "Working ...");
        if ticks >= TOTAL_STEPS {
            // Pause (not stop) at 100%: a stopped timer cannot be started again.
            self.done = true;
            if let Some(timer) = self.timer() {
                timer.pause();
            }
        }
        EventProcessStatus::Processed
    }

    fn on_pause(&mut self, ticks: u64) -> EventProcessStatus {
        let text = if self.done { "Done!" } else { "Paused" };
        self.show_progress(ticks, text);
        self.set_buttons(false, !self.done);
        EventProcessStatus::Processed
    }

    fn on_resume(&mut self, ticks: u64) -> EventProcessStatus {
        self.show_progress(ticks, "Working ...");
        self.set_buttons(true, false);
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().window(MyWin::new).run()
}
