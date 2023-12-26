use super::control_manager::ParentLayout;
use crate::graphics::*;
use crate::input::*;
use crate::prelude::colorpicker::events::ColorPickerEvents;
use crate::system::{Handle, RuntimeManager};
use crate::ui::{
    button::events::ButtonEvents, checkbox::events::CheckBoxEvents, command_bar::events::CommandBarEvents, common::traits::*, common::*,
    desktop::events::DesktopEvents, layout::*, menu::events::MenuEvents, window::events::ToolBarEvents, window::events::WindowEvents,
};
use crate::utils::VectorIndex;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 16)]
pub enum StatusFlags {
    Visible = 0x0001,
    Enabled = 0x0002,
    AcceptInput = 0x0004,
    Focused = 0x0008,
    MarkedForFocus = 0x0010,
    MouseOver = 0x0020,
    WindowControl = 0x0040,
    DesktopControl = 0x0080,
    KeyInputBeforeChildren = 0x0100,
    Expanded = 0x0200,
}
#[derive(Copy, Clone)]
pub(crate) struct Margins {
    pub(crate) left: u8,
    pub(crate) right: u8,
    pub(crate) top: u8,
    pub(crate) bottom: u8,
}

#[repr(C)]
pub struct ControlBase {
    layout: ControlLayout,
    pub(crate) margins: Margins,
    pub(crate) handle: Handle<UIElement>,
    pub(crate) parent: Handle<UIElement>,
    pub(crate) event_processor: Handle<UIElement>,
    pub(crate) children: Vec<Handle<UIElement>>,
    pub(crate) focused_child_index: VectorIndex,
    pub(crate) parent_index: VectorIndex,
    status_flags: StatusFlags,
    pub(crate) screen_clip: ClipArea,
    pub(crate) screen_origin: Point,
    pub(crate) hotkey: Key,
}

impl ControlBase {
    pub fn new(layout: Layout, status_flags: StatusFlags) -> Self {
        Self {
            parent: Handle::None,
            handle: Handle::None,
            event_processor: Handle::None,
            children: Vec::new(),
            focused_child_index: VectorIndex::Invalid,
            parent_index: VectorIndex::Invalid,
            layout: ControlLayout::new(layout.format),
            margins: Margins {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            },
            status_flags: status_flags,
            screen_clip: ClipArea::default(),
            screen_origin: Point::default(),
            hotkey: Key::default(),
        }
    }

    /// Returns the size of a control
    #[inline(always)]
    pub fn get_size(&self) -> Size {
        Size {
            width: self.layout.get_width() as u32,
            height: self.layout.get_height() as u32,
        }
    }

    /// Returns the client size of a control. In most cases it is the same as the size returned the method `.get_size()`. However, if the control has margins (for example in case of a Window) this size will be smaller.
    #[inline(always)]
    pub fn get_client_size(&self) -> Size {
        let horizontal_margins = (self.margins.left as u32) + (self.margins.right as u32);
        let vertical_margins = (self.margins.top as u32) + (self.margins.bottom as u32);
        let width = self.layout.get_width() as u32;
        let height = self.layout.get_height() as u32;
        Size {
            width: if horizontal_margins > width { 0 } else { width - horizontal_margins },
            height: if vertical_margins > height { 0 } else { height - vertical_margins },
        }
    }

    /// Sets the new size for a control (to a specified size given by parameters `width` and `height`). Keep in mind that this method will change the existing layout to an a layout based on top-left corner (given by controls `x` and `y` coordonates) and the new provided size. Any dock or alignament properties will be removed.
    /// This method has no effect on a Desktop control.
    #[inline(always)]
    pub fn set_size(&mut self, width: u16, height: u16) {
        if self.status_flags.contains(StatusFlags::DesktopControl) {
            return;
        }
        self.layout.layout_resize(width, height);
        RuntimeManager::get().request_recompute_layout();
    }

    /// Returns the relatove position (x,y) of the current control to its parent.
    #[inline(always)]
    pub fn get_position(&self) -> Point {
        Point {
            x: self.layout.get_x() as i32,
            y: self.layout.get_y() as i32,
        }
    }

    /// Sets the new position for a control (to a specified coordonate given by parameters `x` and `y`). Keep in mind that this method will change the existing layout to an a layout based on top-left corner (given by coordonates `x` and `y`) and the controls current width and height. Any dock or alignament properties will be removed.
    /// This method has no effect on a Desktop control.
    pub fn set_position(&mut self, x: i32, y: i32) {
        if self.status_flags.contains(StatusFlags::DesktopControl) {
            return;
        }
        self.layout.layout_set_position(x, y);
        RuntimeManager::get().request_recompute_layout();
    }

    #[inline(always)]
    pub fn set_enabled(&mut self, enabled: bool) {
        if enabled {
            self.status_flags.set(StatusFlags::Enabled);
        } else {
            // desktop and window controls can not be disabled
            if self.status_flags.contains_one(StatusFlags::WindowControl | StatusFlags::DesktopControl) {
                return;
            }
            self.status_flags.remove(StatusFlags::Enabled);
        }
    }

    /// Can be used to make a control visible or not. This method has no effect on the Desktop control that will always be visible.
    /// # Examples
    /// ```rust,no_run
    /// use appcui::prelude::*;
    /// let mut button = button!("'Click me',x:1,y:1,w:15");
    /// button.set_visible(false); // this will hide the button
    /// ```
    #[inline(always)]
    pub fn set_visible(&mut self, visible: bool) {
        if visible {
            self.status_flags.set(StatusFlags::Visible);
        } else {
            // desktop controls can not be hidden
            if self.status_flags.contains_one(StatusFlags::DesktopControl) {
                return;
            }
            self.status_flags.remove(StatusFlags::Visible);
        }
    }

    #[inline(always)]
    pub(crate) fn update_focus_flag(&mut self, has_focus: bool) {
        if has_focus {
            self.status_flags |= StatusFlags::Focused;
        } else {
            self.status_flags.remove(StatusFlags::Focused);
        }
    }
    #[inline(always)]
    pub(crate) fn mark_to_receive_focus(&mut self) -> bool {
        //if self.can_receive_input() {
        if self.is_active() {
            self.status_flags |= StatusFlags::MarkedForFocus;
            return true;
        }
        return false;
    }
    pub(crate) fn clear_mark_to_receive_focus(&mut self) {
        self.status_flags.remove(StatusFlags::MarkedForFocus);
    }
    #[inline(always)]
    pub(crate) fn is_marked_to_receive_focus(&self) -> bool {
        self.status_flags.contains(StatusFlags::MarkedForFocus)
    }
    #[inline(always)]
    pub(crate) fn get_focused_control(&self) -> Handle<UIElement> {
        if self.focused_child_index.in_range(self.children.len()) {
            return self.children[self.focused_child_index.index()];
        }
        Handle::None
    }

    #[inline(always)]
    pub(crate) fn is_window_control(&self) -> bool {
        self.status_flags.contains(StatusFlags::WindowControl)
    }
    #[inline(always)]
    pub(crate) fn is_desktop_control(&self) -> bool {
        self.status_flags.contains(StatusFlags::DesktopControl)
    }

    #[inline(always)]
    pub(crate) fn should_receive_keyinput_before_children(&self) -> bool {
        self.status_flags.contains(StatusFlags::KeyInputBeforeChildren)
    }
    #[inline(always)]
    pub(crate) fn set_key_input_before_children_flag(&mut self, value: bool) {
        if value {
            self.status_flags.set(StatusFlags::KeyInputBeforeChildren);
        } else {
            self.status_flags.remove(StatusFlags::KeyInputBeforeChildren);
        }
    }

    #[inline(always)]
    pub(crate) fn is_expanded(&self) -> bool {
        self.status_flags.contains(StatusFlags::Expanded)
    }
    pub(crate) fn expand(&self) {
        if self.has_focus() && self.children.is_empty() && (!self.is_expanded()) {
            RuntimeManager::get().request_expand_for_control(self.handle);
        }
    }
    pub(crate) fn pack(&self) {

    }

    /// A control can use this method to request focus
    pub fn request_focus(&mut self) -> bool {
        if self.has_focus() || !self.can_receive_input() {
            return false;
        }
        // we need to check if current child can receive focus
        // if yes, we should request focus for it
        // if no, just request focus for the current control

        if !self.handle.is_none() {
            RuntimeManager::get().request_focus_for_control(self.handle);
            return true;
        }
        return false;
    }

    pub(crate) fn add_child<T>(&mut self, control: T) -> Handle<T>
    where
        T: Control + 'static,
    {
        let mut c = ControlManager::new(control);
        // if I am already registered, I will set the parent of my child
        let base = c.get_base_mut();
        let focusable = base.can_receive_input();

        base.parent = self.handle;
        // I will use the same event_processor as my parent
        // if my parent is not register , the event_processor handle will be None
        // and the first time the root window is registered all its childern will
        // have the same event processor
        base.event_processor = self.event_processor;
        let rm = RuntimeManager::get();
        let handle = rm.get_controls_mut().add(c);
        self.children.push(handle);
        if focusable {
            rm.request_focus_for_control(handle);
            let children_count = self.children.len();
            // since we have already pushed one handle, we know that children count > 0
            self.focused_child_index.set(children_count - 1, children_count, false);
        }
        return handle.cast();
    }

    /// Returns `true` if the current control is visible or `false` otherwise
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        self.status_flags.contains(StatusFlags::Visible)
    }

    /// Returns `true` if the current control is enabled or `false` otherwise
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.status_flags.contains(StatusFlags::Enabled)
    }

    /// Returns `true` if the current control is active (enabled and visible at the same time) or `false` otherwise
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        self.status_flags.contains(StatusFlags::Enabled | StatusFlags::Visible)
    }

    /// Returns `true` if the current control can receive focus or `false` otherwise. If the control is not visible or it is disable this function will return `false`.
    #[inline(always)]
    pub fn can_receive_input(&self) -> bool {
        // all 3 flags must be present for an object to be able to receive input (key or mouse)
        self.status_flags
            .contains(StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput)
    }

    /// Returns `true` if the current control has the focus or `false` otherwise
    #[inline(always)]
    pub fn has_focus(&self) -> bool {
        self.status_flags.contains(StatusFlags::Focused)
    }

    /// Returns `true` if the mouse cursor is over the current control or `false` otherwise
    #[inline]
    pub fn is_mouse_over(&self) -> bool {
        self.status_flags.contains(StatusFlags::MouseOver)
    }
    #[inline(always)]
    pub(crate) fn update_mouse_over_flag(&mut self, mouse_is_over: bool) {
        if mouse_is_over {
            self.status_flags |= StatusFlags::MouseOver;
        } else {
            self.status_flags.remove(StatusFlags::MouseOver);
        }
    }

    /// Sets the bounds (minim and maxim sized allowed for a control). If the size of a control is outside its bounds, its size will be adjusted automatically. This method has no effect on a Desktop control.
    #[inline]
    pub fn set_size_bounds(&mut self, min_width: u16, min_height: u16, max_width: u16, max_height: u16) {
        if self.status_flags.contains(StatusFlags::DesktopControl) {
            return;
        }
        self.layout.set_size_bounds(min_width, min_height, max_width, max_height);
        RuntimeManager::get().request_recompute_layout();
    }

    #[inline]
    pub(crate) fn set_margins(&mut self, left: u8, top: u8, right: u8, bottom: u8) {
        self.margins.left = left;
        self.margins.top = top;
        self.margins.bottom = bottom;
        self.margins.right = right;
    }

    /// Sets the hot-key associated with a control. Use `Key::None` to clear an existing hotkey
    #[inline]
    pub fn set_hotkey<T>(&mut self, hotkey: T)
    where
        Key: From<T>,
    {
        self.hotkey = Key::from(hotkey)
    }

    /// Returns the hotkey associated to a control or Key::None otherwise.
    #[inline]
    pub fn get_hotkey(&self) -> Key {
        self.hotkey
    }
    #[inline(always)]
    pub(crate) fn is_coord_in_control(&self, x: i32, y: i32) -> bool {
        (x >= 0) && (y >= 0) && (x < (self.layout.get_width() as i32)) && (y < (self.layout.get_height() as i32))
    }

    #[inline]
    pub(crate) fn update_control_layout_and_screen_origin(&mut self, parent_layout: &ParentLayout) {
        self.layout.update(parent_layout.client_width, parent_layout.client_height);
        self.screen_origin.x = parent_layout.origin.x + self.layout.get_x();
        self.screen_origin.y = parent_layout.origin.y + self.layout.get_y();
        self.screen_clip.set_with_size(
            self.screen_origin.x,
            self.screen_origin.y,
            self.layout.get_width(),
            self.layout.get_height(),
        );
        self.screen_clip.intersect_with(&parent_layout.clip);
    }
    #[inline]
    pub(crate) fn get_client_clip(&self) -> ClipArea {
        let mut c = ClipArea::with_size(
            self.screen_origin.x,
            self.screen_origin.y,
            self.layout.get_width(),
            self.layout.get_height(),
        );
        c.reduce_margins(
            self.margins.left as i32,
            self.margins.top as i32,
            self.margins.right as i32,
            self.margins.bottom as i32,
        );
        c.intersect_with(&self.screen_clip);
        c
    }

    #[inline(always)]
    pub(crate) fn get_absolute_rect(&self) -> Rect {
        Rect::with_point_and_size(self.screen_origin, self.layout.get_size())
    }
    pub(crate) fn prepare_paint(&self, surface: &mut Surface) -> bool {
        if (self.is_visible() == false) || (self.screen_clip.is_visible() == false) {
            return false; // nothing to draw
        }
        // paint myself
        surface.set_base_clip(
            self.screen_clip.left,
            self.screen_clip.top,
            self.screen_clip.right,
            self.screen_clip.bottom,
        );
        surface.set_base_origin(self.screen_origin.x, self.screen_origin.y);
        surface.reset_clip();
        surface.reset_origin();
        return true;
    }
    pub(crate) fn raise_event(&self, event: ControlEvent) {
        if !self.handle.is_none() {
            RuntimeManager::get().send_event(event);
        }
    }
    pub fn request_update(&self) {
        if !self.handle.is_none() {
            RuntimeManager::get().request_update();
        }
    }
    pub(crate) fn show_tooltip_on_point(&self, txt: &str, x: i32, y: i32) {
        if self.is_visible() && self.screen_clip.is_visible() {
            let r = Rect::with_size(self.screen_clip.left + x, self.screen_clip.top + y, 1, 1);
            RuntimeManager::get().show_tooltip(txt, &r);
        } else {
            RuntimeManager::get().hide_tooltip();
        }
    }
    pub(crate) fn show_tooltip(&self, txt: &str) {
        if self.is_visible() && self.screen_clip.is_visible() {
            let r = Rect::new(
                self.screen_clip.left,
                self.screen_clip.top,
                self.screen_clip.right,
                self.screen_clip.bottom,
            );
            RuntimeManager::get().show_tooltip(txt, &r);
        } else {
            RuntimeManager::get().hide_tooltip();
        }
    }
    pub(crate) fn hide_tooltip(&self) {
        RuntimeManager::get().hide_tooltip();
    }
}
// default implementations
impl OnPaint for ControlBase {}
impl OnKeyPressed for ControlBase {}
impl OnMouseEvent for ControlBase {}
impl OnDefaultAction for ControlBase {}
impl OnResize for ControlBase {}
impl OnFocus for ControlBase {}

// default implementation for control events
impl CommandBarEvents for ControlBase {}
impl MenuEvents for ControlBase {}
impl ButtonEvents for ControlBase {}
impl ColorPickerEvents for ControlBase {}
impl CheckBoxEvents for ControlBase {}
impl WindowEvents for ControlBase {}
impl ToolBarEvents for ControlBase {}
impl DesktopEvents for ControlBase {}
