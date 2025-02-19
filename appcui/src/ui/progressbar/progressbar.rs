use super::initialization_flags::Flags;
use crate::prelude::*;
use std::time::{Duration, Instant};

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct ProgressBar {
    items_count: u64,
    items_processed: u64,
    proc_buf: [u8; 4],
    eta: [u8; 8], // hh:mm:ss
    text: String,
    percentage: u8,
    flags: Flags,
    start: Instant,
    extra_duration: Duration,
    paused: bool,
}
impl ProgressBar {
    pub fn new(items_count: u64, layout: Layout, flags: Flags) -> Self {
        let mut me = Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled),
            items_count,
            items_processed: 0,
            proc_buf: [b' ', b' ', b' ', b' '],
            eta: [b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' '],
            text: String::new(),
            percentage: 0,
            flags,
            start: Instant::now(),
            extra_duration: Duration::default(),
            paused: false,
        };
        me.set_size_bounds(4, 1, u16::MAX, 2);
        me
    }

    /// Updates the text displayed on the progress bar
    ///
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// let mut p = ProgressBar::new(100,Layout::new("x:1,y:1,w:20"));
    /// p.update_text("Running ...");
    /// ```
    pub fn update_text(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text);
    }

    /// Updates the progress bar with the number of items processed so far. The progress bar will
    /// automatically calculate the percentage and ETA and will update the display.
    /// 
    /// # Example
    /// ```rust
    /// use appcui::prelude::*;
    /// 
    /// let mut p = ProgressBar::new(100,Layout::new("x:1,y:1,w:20"));
    /// p.update_progress(85);
    /// ```
    pub fn update_progress(&mut self, processed_items: u64) {
        self.items_processed = processed_items.min(self.items_count);
        if self.items_count == 0 {
            self.proc_buf = [b'-', b'-', b'-', b'%'];
            self.eta = [b'-', b'-', b':', b'-', b'-', b':', b'-', b'-'];
            self.percentage = 0;
        } else {
            if self.items_processed >= self.items_count {
                self.proc_buf = [b'1', b'0', b'0', b'%'];
                self.eta = [b'0', b'0', b':', b'0', b'0', b':', b'0', b'0'];
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

    /// Resumes the progress bar. This will restart the timer and will continue the progress bar
    /// from the last position. Calling this method multiple times have no effect if the progress
    /// bar is already running.
    #[inline(always)]
    pub fn resume(&mut self) {
        if self.paused {
            self.start = Instant::now();
            self.paused = false;
        }
    }

    /// Pauses the progress bar. This will stop the timer and will pause the progress bar.
    /// Calling this method multiple times have no effect if the progress bar is already paused.
    #[inline(always)]
    pub fn pause(&mut self) {
        if !self.paused {
            self.extra_duration += self.start.elapsed();
            self.paused = true;
        }
    }

    /// Resets the progress bar internal state (items processed, timer, etc). This is usefull if you
    /// want to restart the progress bar from the beginning or if you want to reuse the same progress
    /// bar for multiple operations.
    #[inline(always)]
    pub fn reset(&mut self, items_count: u64) {
        self.items_count = items_count;
        self.extra_duration = Duration::default();
        self.start = Instant::now();
        self.paused = false;
        self.update_progress(0);
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
