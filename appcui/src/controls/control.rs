use std::rc::Rc;
use super::layout::ControlLayout;
use super::Layout;

pub struct Control {
    layout: ControlLayout,
    children: Vec<Rc<Control>>,
}

impl Control {
    fn new(layout_format: Layout) -> Self {
        Self {
            children: Vec::new(),
            layout: ControlLayout::new(layout_format.format)
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
}