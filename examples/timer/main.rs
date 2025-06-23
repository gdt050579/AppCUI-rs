mod digits;
use appcui::prelude::*;
use std::time::Duration;

#[Window(events = ButtonEvents+ TimerEvents)]
#[derive(Default)]
struct MyWin {
    c: Handle<Canvas>,
    b_start: Handle<Button>,
    b_pause: Handle<Button>,
    b_resume: Handle<Button>,
}
impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("Timer, d:c,w:36,h:13"),
            ..Default::default()
        };
        w.c = w.add(canvas!("x:1,y:1,w:32,h:7,size:32x7,back:{' ',white,black},enable:false"));
        w.b_start = w.add(button!("&Start,x:1,y:9,w:10"));
        w.b_pause = w.add(button!("&Pause,x:12,y:9,w:10,enable: false"));
        w.b_resume = w.add(button!("&Resume,x:23,y:9,w:10,enable: false"));
        w
    }
    fn update_timer(&mut self, seconds: u64, paused: bool) {
        let theme = self.theme();
        let attr = if paused { theme.text.error } else { theme.text.normal };
        let back = Character::with_attributes(' ', theme.editor.normal);
        let attr_two_points = theme.text.inactive;
        let c = self.c;
        if let Some(c) = self.control_mut(c) {
            let s = c.drawing_surface_mut();
            s.clear(back);
            let sec = seconds % 60;
            let min = seconds / 60;
            let s_second_digit = (sec % 10) as u8;
            let s_first_digit = (sec / 10) as u8;
            let m_second_digit = (min % 10) as u8;
            let m_first_digit = ((min / 10).min(9)) as u8;
            s.write_string(3, 0, digits::digit_to_text(m_first_digit), attr, true);
            s.write_string(8, 0, digits::digit_to_text(m_second_digit), attr, true);
            s.write_string(18, 0, digits::digit_to_text(s_first_digit), attr, true);
            s.write_string(23, 0, digits::digit_to_text(s_second_digit), attr, true);
            if (seconds & 1) == 0 {
                s.draw_rect(Rect::with_size(14, 1, 3, 2), LineType::Single, attr_two_points);
                s.draw_rect(Rect::with_size(14, 4, 3, 2), LineType::Single, attr_two_points);
            }
        }
    }
    fn set_buttons_state(&mut self, pause_state: bool, resume_state: bool) {
        let h = self.b_pause;
        if let Some(pause) = self.control_mut(h) {
            pause.set_enabled(pause_state);
        }
        let h = self.b_resume;
        if let Some(resume) = self.control_mut(h) {
            resume.set_enabled(resume_state);
        }
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.b_start {
            if let Some(t) = self.timer() {
                t.start(Duration::from_secs(1));
            }
        }
        if handle == self.b_pause {
            if let Some(t) = self.timer() {
                t.pause();
            }
        }
        if handle == self.b_resume {
            if let Some(t) = self.timer() {
                t.resume();
            }
        }
        EventProcessStatus::Processed
    }
}
impl TimerEvents for MyWin {
    fn on_start(&mut self) -> EventProcessStatus {
        self.update_timer(0, false);
        self.set_buttons_state(true,false);
        EventProcessStatus::Processed
    }

    fn on_resume(&mut self, ticks: u64) -> EventProcessStatus {
        self.update_timer(ticks, false);
        self.set_buttons_state(true,false);
        EventProcessStatus::Processed
    }

    fn on_pause(&mut self, ticks: u64) -> EventProcessStatus {
        self.update_timer(ticks, true);
        self.set_buttons_state(false,true);
        EventProcessStatus::Processed
    }

    fn on_update(&mut self, ticks: u64) -> EventProcessStatus {
        self.update_timer(ticks, false);
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
