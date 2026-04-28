use ropey::RopeSlice;

use crate::prelude::*;
use super::Document;

struct PaintData {
    x: i32,
    y: i32,
    max_x: i32,
    left_margin: i32,
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent, internal=true)]
pub struct Editor {
    document: Document,
    tab_align: i32,
    start_line: u32,
}

impl Editor {
    pub fn new(text: &str, layout: Layout) -> Self {
        Self {
            base: ControlBase::with_status_flags(layout, StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput),
            document: Document::new(text),
            tab_align: 4,
            start_line: 0,
        }
    }
    fn paint_line(&self, surface: &mut Surface, p: &PaintData, line: &RopeSlice) {
        let mut virtual_x = 0i32;
        for (i, ch) in line.chars().enumerate() {
            let x = virtual_x - p.left_margin;
            if x >= p.max_x {
                break;
            }
            if x>=0 {
                // actual paint
            }
            // paint character
            if ch == '\t' {
                virtual_x += self.tab_align - (virtual_x % self.tab_align);
            } else {
                virtual_x += 1;
            }
        }
    }    
}

impl OnPaint for Editor {
    fn on_paint(&self, surface: &mut Surface, _theme: &Theme) {
        let mut p = PaintData {
            x: 0,
            y: 0,
            max_x: surface.size().width as i32,
            left_margin: 0,
        };
        let mut it = self.document.lines_starting_from(self.start_line as usize);
        while let Some(line) = it.next() {
            self.paint_line(surface, &p, &line);
            p.y += 1;
            if p.y >= surface.size().height as i32 {
                break;
            }
        }
    }

}

impl OnKeyPressed for Editor {
    fn on_key_pressed(&mut self, _key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

impl OnMouseEvent for Editor {
    fn on_mouse_event(&mut self, _event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
