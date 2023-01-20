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
#[derive(Copy,Clone)]
struct Margins {
    left: u8,
    right: u8,
    top: u8,
    bottom: u8
}
pub struct Control {
    layout: ControlLayout,
    margins: Margins,
    pub(crate) children: Vec<Rc<Control>>,
    pub(crate) parent: Option<Rc<Control>>,
    status_flags: StatusFlags,
    pub(crate) screen_clip: ClipArea,
    pub(crate) screen_origin: Point,
}

impl Control {
    fn new(layout_format: Layout) -> Self {
        Self {
            children: Vec::new(),
            layout: ControlLayout::new(layout_format.format),
            margins: Margins { left: 0, right: 0, top: 0, bottom: 0 },
            parent: None,
            status_flags: StatusFlags::None,
            screen_clip: ClipArea::default(),
            screen_origin: Point::default(),
        }
    }
    #[inline]
    pub fn get_width(&self)->u16 {
        self.layout.get_width()
    }
    #[inline]
    pub fn get_height(&self)->u16 {
        self.layout.get_height()
    }
    #[inline]
    pub fn is_visible(&self)->bool {
        self.status_flags.contains(StatusFlags::Visible)
    }
    #[inline]
    pub fn is_enabled(&self)->bool {
        self.status_flags.contains(StatusFlags::Enabled)
    }


    #[inline]
    pub (crate) fn update_control_layout_and_screen_origin(&mut self, parent_clip: &ClipArea, parent_screen_origin: Point)
    {
        self.screen_origin.x = parent_screen_origin.x + self.layout.get_x();
        self.screen_origin.y = parent_screen_origin.y + self.layout.get_y();
        self.screen_clip.set_with_size(self.screen_origin.x,self.screen_origin.y,self.layout.get_width(),self.layout.get_height());
        self.screen_clip.intersect_with(parent_clip);    
    }
    #[inline]
    pub (crate) fn get_client_clip(&self)->ClipArea {
        let mut c = ClipArea::with_size(self.screen_origin.x, self.screen_origin.y, self.layout.get_width(), self.layout.get_height());
        c.reduce_margines(self.margins.left as i32, self.margins.top as i32, self.margins.right as i32, self.margins.bottom as i32);
        c.intersect_with(&self.screen_clip);
        c
    }

}