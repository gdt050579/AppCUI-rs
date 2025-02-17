use crate::prelude::*;

#[CustomControl(overwrite=OnPaint, internal=true)]
pub struct ProgressBar {
    items_count: u64,
    items_processed: u64,
    proc_buf: [u8; 4],
}
impl ProgressBar {
    pub fn new(items_count: u64, layout: Layout) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled),
            items_count,
            items_processed: 0,
            proc_buf: [b' ', b' ', b' ', b' '],
        }
    }
    fn update_values(&mut self) {
        if self.items_count == 0 {
            self.proc_buf = [b'-', b'-', b'-', b'%']
        } else {
            if self.items_processed >= self.items_count {
                self.proc_buf = [b'1', b'0', b'0', b'%']
            } else {
                if self.items_processed > 0xFF_FFFF_FFFF_FFFF {
                    self.proc_buf = [b' ', b'9', b'9', b'%']
                } else {
                    let proc = ((self.items_processed * 100) / self.items_count) as u8;
                    self.proc_buf[3] = b'%';
                    self.proc_buf[2] = (proc % 10) + 48;
                    let proc = proc / 10;
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
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {}
}
