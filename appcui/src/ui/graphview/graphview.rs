use crate::prelude::*;
use super::initialization_flags::Flags;

use self::components::ScrollBars;

#[CustomControl(overwrite=OnPaint+OnKeyPressed+OnMouseEvent+OnResize, internal=true)]
pub struct GraphView {
    background: Option<Character>,
    flags: Flags,
    drag_point: Option<Point>,

    scrollbars: ScrollBars
}
impl GraphView {

    pub fn new(layout: Layout, flags: Flags) -> Self {
        Self {
            base: ControlBase::with_status_flags(
                layout,
                (StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput)
                    | if flags == Flags::ScrollBars {
                        StatusFlags::IncreaseBottomMarginOnFocus | StatusFlags::IncreaseRightMarginOnFocus
                    } else {
                        StatusFlags::None
                    },
            ),
            flags,
            background: None,
            drag_point: None,
            scrollbars: ScrollBars::new(flags == Flags::ScrollBars)
        }
    }

    /// Sets the background of the GraphView to the specified character.
    /// # Example
    /// ```rust, no_run
    /// use appcui::prelude::*;
    /// let mut GraphView = GraphView::new(Size::new(100, 100), layout!("x:1,y:1,w:30,h:10"), GraphView::Flags::ScrollBars);
    /// GraphView.set_background(Character::new('*', Color::White, Color::Black, CharFlags::None));
    /// ```
    pub fn set_background(&mut self, backgroud_char: Character) {
        self.background = Some(backgroud_char);
    }

    /// Clears the background character of the GraphView. It esentially resets it to transparent foreground and backgroud colors
    pub fn clear_background(&mut self) {
        self.background = None;
    }
}
impl OnResize for GraphView {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        // let paint_sz = self.surface.size();
        // self.scrollbars.resize(paint_sz.width as u64, paint_sz.height as u64,&self.base);
        // self.move_scroll_to(self.x, self.y);
    }
}
impl OnPaint for GraphView {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if (self.has_focus()) && (self.flags == Flags::ScrollBars) {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0,0,1,1);
        }
        if let Some(back) = self.background {
            surface.clear(back);
        }
    }
}
impl OnKeyPressed for GraphView {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}
impl OnMouseEvent for GraphView {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

