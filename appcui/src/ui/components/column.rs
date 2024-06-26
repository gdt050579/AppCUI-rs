use crate::prelude::*;
use crate::utils::*;

pub struct Column {
    name: Caption,
    width: u16,
    alignment: TextAlignament,
    tooltip: String,
    x: i32,
}

impl Column {
    pub fn new(name: &str, width: u16, alignment: TextAlignament) -> Self {
        Self {
            name: Caption::new(name, ExtractHotKeyMethod::CtrlPlusKey),
            width,
            alignment,
            tooltip: String::new(),
            x: 0,
        }
    }
    pub fn set_name(&mut self, name: &str) {
        self.name.set_text(name, ExtractHotKeyMethod::CtrlPlusKey)
    }
    pub fn set_tooltip(&mut self, tooltip: &str) {
        self.tooltip.clear();
        self.tooltip.push_str(tooltip);
    }
    pub fn set_alignment(&mut self, alignment: TextAlignament) {
        self.alignment = alignment;
    }
    pub fn set_width(&mut self, width: u16) {
        self.width = width;
    }
    pub fn name(&self) -> &str {
        self.name.text()
    }
    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }
    pub fn alignment(&self) -> TextAlignament {
        self.alignment
    }
    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn on_paint(&self, surface: &mut Surface, theme: &Theme, x: i32, y: i32, width: u16, selected: bool) {
        // let mut format = TextFormat::new(x, y, theme.columns_header.text.normal, self.alignment);
        // format.width = Some(width);
        // if selected {
        //     format.background = theme.columns_header.background.selected;
        //     format.text = theme.columns_header.text.selected;
        // }
        // surface.write_text(self.name.text(), &format);
    }
    pub fn on_mouse_event(&self, event: &MouseEvent, x: i32, y: i32, width: u16) -> EventProcessStatus {
        // if event.is_event(MouseEventType::Released) {
        //     if event.x >= x && event.x < x + width as i32 {
        //         return EventProcessStatus::Processed;
        //     }
        // }
        EventProcessStatus::Ignored
    }
}