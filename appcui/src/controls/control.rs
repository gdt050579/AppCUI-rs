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


    fn recompute_control_layout(&mut self, parent_clip: &ClipArea, parent_screen_origin: Point)
{
    // check if the parent is available
    if self.parent.is_none() {
        return; // nothing to compute --> I'm the desktop
    }
    let p = self.parent.as_ref().unwrap();
    self.screen_origin.x = parent_screen_origin.x + self.layout.get_x();
    self.screen_origin.y = parent_screen_origin.y + self.layout.get_y();
    self.screen_clip.set_with_size(self.screen_origin.x,self.screen_origin.y,self.layout.get_width(),self.layout.get_heght());
    self.screen_clip.intersect_with(parent_clip);    
    // compute the clip
    Members->ScreenClip.Set(
          parentClip, Members->Layout.X, Members->Layout.Y, Members->Layout.Width, Members->Layout.Height);
    // compute the expanded clip if neccesary
    if (ctrl == app->ExpandedControl)
    {
        if ((Members->Flags & GATTR_EXPANDED) == 0)
        {
            Members->ExpandedViewClip = Members->ScreenClip;
            ctrl->OnExpandView(Members->ExpandedViewClip);
            Members->Flags |= GATTR_EXPANDED;
        }
    }
    // calculez clip-ul client
    Graphics::Clip client;
    client.Set(
          parentClip,
          Members->Layout.X + Members->Margins.Left,
          Members->Layout.Y + Members->Margins.Top,
          Members->Layout.Width - (Members->Margins.Right + Members->Margins.Left),
          Members->Layout.Height - (Members->Margins.Bottom + Members->Margins.Top));
    // calculez pentru fiecare copil
    for (uint32 tr = 0; tr < Members->ControlsCount; tr++)
        ComputeControlLayout(client, Members->Controls[tr]);
}

}