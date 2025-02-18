use crate::prelude::*;

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct ProgressBar {
    items_count: u64,
    items_processed: u64,
    proc_buf: [u8; 4],
    text: String,
    percentage: u8,
}
impl ProgressBar {
    pub fn new(items_count: u64, layout: Layout) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled),
            items_count,
            items_processed: 0,
            proc_buf: [b' ', b' ', b' ', b' '],
            text: String::new(),
            percentage: 0,
        }
    }
    fn update_text(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text);
    }
    fn update_items_count(&mut self, new_count: u64) {
        self.items_count = new_count.min(self.items_processed);
        if self.items_count == 0 {
            self.proc_buf = [b'-', b'-', b'-', b'%'];
            self.percentage = 0;
        } else {
            if self.items_processed >= self.items_count {
                self.proc_buf = [b'1', b'0', b'0', b'%'];
                self.percentage = 100;
            } else {
                if self.items_processed > 0xFF_FFFF_FFFF_FFFF {
                    self.proc_buf = [b' ', b'9', b'9', b'%'];
                    self.percentage = 99;
                } else {
                    self.percentage = ((self.items_processed * 100) / self.items_count) as u8;
                    self.proc_buf[3] = b'%';
                    self.proc_buf[2] = (self.percentage % 10) + 48;
                    let proc = self.percentage / 10;
                    if proc > 0 {
                        self.proc_buf[1] = (proc % 10) + 48;
                        let proc = proc / 10;
                        if proc > 0 {
                            self.proc_buf[1] = proc + 48;
                        } else {
                            self.proc_buf[0] = 32;
                        }
                    } else {
                        self.proc_buf[1] = 32;
                        self.proc_buf[0] = 32;
                    }
                }
            }
        }
    }
}
impl OnPaint for ProgressBar {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let w = self.size().width as i32;
        let m = w * (self.percentage as i32) / 100;
        surface.fill_horizontal_line(
            0,
            0,
            m,
            Character::new(' ', theme.progressbar.text, theme.progressbar.progress, CharFlags::None),
        );
        surface.fill_horizontal_line(
            m + 1,
            0,
            w,
            Character::new(' ', theme.progressbar.text, theme.progressbar.background, CharFlags::None),
        );
        let attr = CharAttribute::with_fore_color(theme.progressbar.text);
        surface.write_string(1, 0, &self.text, attr, false);
        surface.write_ascii(w - 5, 0, &self.proc_buf, attr, false);
    }
}
