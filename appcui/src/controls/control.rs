use std::rc::Rc;
use EnumBitFlags::EnumBitFlags;
use super::layout::ControlLayout;
use super::Layout;
use crate::graphics::*;

#[EnumBitFlags(bits=8)]
enum StatusFlags {
    Visible = 0x01,
    Enabled = 0x02,
    TabStop = 0x04
}
pub struct Control {
    layout: ControlLayout,
    children: Vec<Rc<Control>>,
    parent: Option<Rc<Control>>,
    status_flags: StatusFlags,
    screen_clip: ClipArea,
}

impl Control {
    fn new(layout_format: Layout) -> Self {
        Self {
            children: Vec::new(),
            layout: ControlLayout::new(layout_format.format),
            parent: None,
            status_flags: StatusFlags::None,
            screen_clip: ClipArea::default(),
        }
    }
    #[inline]
    pub fn get_width(&self)->u16 {
        self.layout.get_width()
    }
    #[inline]
    pub fn get_height(&self)->u16 {
        self.layout.get_heght()
    }
    #[inline]
    pub fn is_visible(&self)->bool {
        self.status_flags.contains(StatusFlags::Visible)
    }
    #[inline]
    pub fn is_enabled(&self)->bool {
        self.status_flags.contains(StatusFlags::Enabled)
    }
}