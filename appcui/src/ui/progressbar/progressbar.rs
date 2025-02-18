use crate::prelude::*;
use super::initialization_flags::Flags;

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct ProgressBar {
    items_count: u64,
    items_processed: u64,
    proc_buf: [u8; 4],
    text: String,
    percentage: u8,
    flags: Flags,
}
impl ProgressBar {
    pub fn new(items_count: u64, layout: Layout, flags: Flags) -> Self {
        let mut me = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled),
            items_count,
            items_processed: 0,
            proc_buf: [b' ', b' ', b' ', b' '],
            text: String::new(),
            percentage: 0,
            flags,
        };
        me.set_size_bounds(4, 1, u16::MAX, 1);
        me
    }

    /// Updates the text displayed on the progress bar
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let p = ProgressBar::new(100,Layout::new("x:1,y:1,w:20"));
    /// p.update_text("Running ...");
    /// ```
    pub fn update_text(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text);
    }
    pub fn update_processed_items(&mut self, processed_items: u64) {
        self.items_processed = processed_items.min(self.items_count);
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
    /// Returns the number of items processed
    #[inline(always)]
    pub fn processed_items(&self) -> u64 {
        self.items_processed
    }

    /// Returns the items that were processed so far
    #[inline(always)]
    pub fn items_count(&self) -> u64 {
        self.items_count
    }
    /// Sets the totsal number of items that need to be processed
    /// If the number of items processed is greater than the number of items to be processed, the number of items processed will be set to the number of items to be processed
    #[inline(always)]
    pub fn set_items_count(&mut self, items_count: u64) {
        self.items_count = items_count;
        self.update_processed_items(self.items_processed);
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
            m,
            0,
            w,
            Character::new(' ', theme.progressbar.text, theme.progressbar.background, CharFlags::None),
        );
        let attr = CharAttribute::with_fore_color(theme.progressbar.text);
        surface.write_string(1, 0, &self.text, attr, false);
        if !self.flags.contains(Flags::HidePercentage) {
            surface.write_ascii(w - 5, 0, &self.proc_buf, attr, false);
        }
    }
}
