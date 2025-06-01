use self::control_event_wrapper::CustomEventData;
use super::control_manager::ParentLayout;
use crate::graphics::*;
use crate::input::*;
use crate::prelude::colorpicker::events::ColorPickerEvents;
use crate::system::Theme;
use crate::system::ThemeMethods;
use crate::system::Timer;
use crate::system::TimerMethods;
use crate::system::{Handle, LayoutMethods, RuntimeManager};
use crate::ui::{
    button::events::ButtonEvents, checkbox::events::CheckBoxEvents, command_bar::events::GenericCommandBarEvents, common::traits::*, common::*,
    desktop::events::DesktopEvents, layout::*, menu::events::GenericMenuEvents, menu::Menu, menu::MenuItem, window::events::ToolBarEvents,
    window::events::WindowEvents,
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
    ModalWindow = 0x0080,
    DesktopControl = 0x0100,
    KeyInputBeforeChildren = 0x0200,
    Expanded = 0x0400,
    IncreaseRightMarginOnFocus = 0x0800,
    IncreaseBottomMarginOnFocus = 0x1000,
    SingleWindow = 0x2000,
}
#[derive(Copy, Clone, Default)]
pub(crate) struct Margins {
    pub(crate) left: u8,
    pub(crate) right: u8,
    pub(crate) top: u8,
    pub(crate) bottom: u8,
}

#[repr(C)]
#[derive(Default)]
pub struct ControlBase {
    pub(crate) layout: ControlLayout,
    pub(crate) margins: Margins,
    pub(crate) handle: Handle<()>,
    pub(crate) parent: Handle<()>,
    pub(crate) timer_handle: Handle<Timer>,
    pub(crate) event_processor: Handle<()>,
    pub(crate) children: Vec<Handle<()>>,
    pub(crate) focused_child_index: VectorIndex,
    pub(crate) parent_index: VectorIndex,
    status_flags: StatusFlags,
    pub(crate) screen_clip: ClipArea,
    pub(crate) screen_origin: Point,
    pub(crate) hotkey: Key,
    pub(crate) left_components_margin: u8,
    pub(crate) top_components_margin: u8,
}

impl ControlBase {
    /// Creates a new control with the specified layout. The argument `accept_input` specifies if the control can receive input or not.
    pub fn new(layout: Layout, accept_input: bool) -> Self {
        ControlBase::with_status_flags(
            layout,
            if accept_input {
                StatusFlags::AcceptInput | StatusFlags::Enabled | StatusFlags::Visible
            } else {
                StatusFlags::Enabled | StatusFlags::Visible
            },
        )
    }
    /// Creates a new control with the specified layout that has support for focused overlay.
    /// When such a control is created if it has focus it will increase its bottom and right margins by one character.
    /// This provides aditional space for the focused control to be drawn (usually a scrollbar).
    pub fn with_focus_overlay(layout: Layout) -> Self {
        ControlBase::with_status_flags(
            layout,
            StatusFlags::AcceptInput
                | StatusFlags::Enabled
                | StatusFlags::Visible
                | StatusFlags::IncreaseBottomMarginOnFocus
                | StatusFlags::IncreaseRightMarginOnFocus,
        )
    }
    pub(crate) fn with_status_flags(layout: Layout, status_flags: StatusFlags) -> Self {
        Self {
            parent: Handle::None,
            handle: Handle::None,
            timer_handle: Handle::None,
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
            status_flags,
            screen_clip: ClipArea::default(),
            screen_origin: Point::default(),
            hotkey: Key::default(),
            left_components_margin: 0,
            top_components_margin: 0,
        }
    }

    /// Returns the size of a control
    #[inline(always)]
    pub fn size(&self) -> Size {
        Size {
            width: self.layout.get_width() as u32,
            height: self.layout.get_height() as u32,
        }
    }

    /// Returns the client size of a control. In most cases it is the same as the size returned the method `.get_size()`. However, if the control has margins (for example in case of a Window) this size will be smaller.
    #[inline(always)]
    pub fn client_size(&self) -> Size {
        let horizontal_margins = (self.margins.left as u32) + (self.margins.right as u32);
        let vertical_margins = (self.margins.top as u32) + (self.margins.bottom as u32);
        let width = self.layout.get_width() as u32;
        let height = self.layout.get_height() as u32;
        Size {
            width: width.saturating_sub(horizontal_margins),
            height: height.saturating_sub(vertical_margins),
        }
    }

    /// Sets the new size for a control (to a specified size given by parameters `width` and `height`). Keep in mind that this method will change the existing layout to an a layout based on top-left corner (given by controls `x` and `y` coordonates) and the new provided size. Any dock or alignament properties will be removed.
    /// This method has no effect on a Desktop control.
    #[inline(always)]
    pub fn set_size(&mut self, width: u16, height: u16) {
        if self.status_flags.contains(StatusFlags::DesktopControl | StatusFlags::SingleWindow) {
            return;
        }
        self.layout.layout_resize(width, height);
        RuntimeManager::get().request_recompute_layout();
    }

    /// Returns the relatove position (x,y) of the current control to its parent.
    #[inline(always)]
    pub fn position(&self) -> Point {
        Point {
            x: self.layout.get_x(),
            y: self.layout.get_y(),
        }
    }

    /// Sets the new position for a control (to a specified coordonate given by parameters `x` and `y`). Keep in mind that this method will change the existing layout to an a layout based on top-left corner (given by coordonates `x` and `y`) and the controls current width and height. Any dock or alignament properties will be removed.
    /// This method has no effect on a Desktop control.
    pub fn set_position(&mut self, x: i32, y: i32) {
        if self.status_flags.contains(StatusFlags::DesktopControl | StatusFlags::SingleWindow) {
            return;
        }
        self.layout.layout_set_position(x, y);
        RuntimeManager::get().request_recompute_layout();
    }

    /// Sets the enabled state of a control. This method has no effect on a Desktop or a Window control that will always be enabled.
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
            if self.status_flags.contains_one(StatusFlags::DesktopControl | StatusFlags::SingleWindow) {
                return;
            }
            self.status_flags.remove(StatusFlags::Visible);
        }
    }

    #[inline(always)]
    pub fn set_components_toolbar_margins(&mut self, left: u8, top: u8) {
        self.left_components_margin = left;
        self.top_components_margin = top;
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
        false
    }
    pub(crate) fn clear_mark_to_receive_focus(&mut self) {
        self.status_flags.remove(StatusFlags::MarkedForFocus);
    }
    #[inline(always)]
    pub(crate) fn is_marked_to_receive_focus(&self) -> bool {
        self.status_flags.contains(StatusFlags::MarkedForFocus)
    }
    #[inline(always)]
    pub(crate) fn get_focused_control(&self) -> Handle<()> {
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
    pub(crate) fn is_modal_window(&self) -> bool {
        self.status_flags.contains(StatusFlags::ModalWindow)
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
    pub(crate) fn set_singlewindow_flag(&mut self) {
        self.status_flags |= StatusFlags::SingleWindow;
    }
    #[inline(always)]
    pub(crate) fn is_singlewindow(&self) -> bool {
        self.status_flags.contains(StatusFlags::SingleWindow)
    }

    #[inline(always)]
    pub(crate) fn is_expanded(&self) -> bool {
        self.status_flags.contains(StatusFlags::Expanded)
    }
    pub(crate) fn set_expand_flag(&mut self, value: bool) {
        if value {
            self.status_flags.set(StatusFlags::Expanded)
        } else {
            self.status_flags.remove(StatusFlags::Expanded)
        }
    }
    pub(crate) fn expand(&self, min_size: Size, prefered_size: Size) {
        if self.has_focus() && self.children.is_empty() && (!self.is_expanded()) {
            RuntimeManager::get().request_expand_for_control(self.handle, min_size, prefered_size);
        }
    }
    pub(crate) fn pack(&self) {
        if self.has_focus() && self.is_expanded() {
            RuntimeManager::get().request_expand_for_control(Handle::None, Size::default(), Size::default());
        }
    }
    #[inline(always)]
    pub fn expanded_size(&self) -> Size {
        if self.is_expanded() {
            Size {
                width: (self.screen_clip.right + 1 - self.screen_clip.left) as u32,
                height: (self.screen_clip.bottom + 1 - self.screen_clip.top) as u32,
            }
        } else {
            self.size()
        }
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
        false
    }

    pub(crate) fn add_child<T>(&mut self, control: T) -> Handle<T>
    where
        T: Control + 'static,
    {
        let mut c = ControlManager::new(control);
        // if I am already registered, I will set the parent of my child
        let base = c.base_mut();
        let focusable = base.can_receive_input();
        let timer_handle = base.timer_handle;

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
        if !timer_handle.is_none() {
            if !handle.is_none() {
                rm.get_timer_manager().update_control_handle(timer_handle, handle.cast());
            }
            rm.request_timer_threads_update();
        }
        handle.cast()
    }

    pub fn timer(&mut self) -> Option<&mut Timer> {
        let tm = RuntimeManager::get().get_timer_manager();
        if self.timer_handle.is_none() {
            let timer_handle = tm.allocate_for(self.handle.cast());
            if timer_handle.is_none() {
                // no empty slots available to allocate a new timer
                return None;
            }
            self.timer_handle = timer_handle;
        }
        tm.get_mut(self.timer_handle)
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

    #[inline(always)]
    pub(crate) fn should_increase_margins_on_focus(&self) -> Option<u8> {
        if !self.has_focus() {
            return None;
        }
        let res = self.status_flags & (StatusFlags::IncreaseBottomMarginOnFocus | StatusFlags::IncreaseRightMarginOnFocus);
        if res.is_empty() {
            return None;
        }
        let mut v = 0;
        if self.status_flags.contains_one(StatusFlags::IncreaseRightMarginOnFocus) {
            v |= 1;
        }
        if self.status_flags.contains_one(StatusFlags::IncreaseBottomMarginOnFocus) {
            v |= 2;
        }
        Some(v)
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
    pub fn hotkey(&self) -> Key {
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
    pub(crate) fn update_expanded_layout(&mut self, prefered_size: Size, min_size: Size, terminal_size: Size) -> Option<ExpandedDirection> {
        // prefer on bottom, but if not then on top
        // leave one row on top and bottom if possible
        let space_on_bottom = (terminal_size.height as i32) - (2 + self.screen_origin.y);
        let space_on_top = self.screen_origin.y - 1;
        let min_height = min_size.height.max(1) as i32;
        if (min_height <= space_on_bottom) && (space_on_bottom > 0) {
            // on button
            let height = prefered_size.height.clamp(min_size.height, space_on_bottom as u32) as u16;
            let width = (prefered_size.width.max(min_size.width) as u16).max(1);
            self.screen_clip.set_with_size(self.screen_origin.x, self.screen_origin.y, width, height);
            return Some(ExpandedDirection::OnBottom);
        }
        if (min_height <= space_on_top) && (space_on_top > 0) {
            // on top
            let height = prefered_size.height.clamp(min_size.height, space_on_top as u32);
            let width = (prefered_size.width.max(min_size.width) as u16).max(1);
            self.screen_clip
                .set_with_size(self.screen_origin.x, self.screen_origin.y - ((height - 1) as i32), width, height as u16);
            self.screen_origin.y -= (height - 1) as i32;

            return Some(ExpandedDirection::OnTop);
        }
        // no expansion possible
        None
    }

    #[inline]
    pub(crate) fn client_clip(&self) -> ClipArea {
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
    pub(crate) fn absolute_rect(&self) -> Rect {
        Rect::with_point_and_size(self.screen_origin, self.layout.get_size())
    }
    pub(crate) fn prepare_paint(&self, surface: &mut Surface) -> bool {
        if !self.is_visible() || !self.screen_clip.is_visible() {
            return false; // nothing to draw
        }
        // paint myself
        if self.has_focus() {
            let add_right = if self.status_flags.contains_one(StatusFlags::IncreaseRightMarginOnFocus) {
                1
            } else {
                0
            };
            let add_bottom = if self.status_flags.contains_one(StatusFlags::IncreaseBottomMarginOnFocus) {
                1
            } else {
                0
            };
            surface.set_base_clip(
                self.screen_clip.left,
                self.screen_clip.top,
                self.screen_clip.right + add_right,
                self.screen_clip.bottom + add_bottom,
            );
        } else {
            surface.set_base_clip(
                self.screen_clip.left,
                self.screen_clip.top,
                self.screen_clip.right,
                self.screen_clip.bottom,
            );
        }
        surface.set_base_origin(self.screen_origin.x, self.screen_origin.y);
        surface.reset_clip();
        surface.reset_origin();
        true
    }
    pub(crate) fn raise_event(&self, event: ControlEvent) {
        if !self.handle.is_none() {
            RuntimeManager::get().send_event(event);
        }
    }
    pub fn raise_custom_event(&self, class_hash: u64, event_id: u32) {
        self.raise_event(ControlEvent {
            emitter: self.handle,
            receiver: self.event_processor,
            data: ControlEventData::Custom(CustomEventData { class_hash, event_id }),
        });
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

    pub(crate) fn notify_children_of_selection(&self, requester: Handle<()>) {
        let controls = RuntimeManager::get().get_controls_mut();
        for h_child in &self.children {
            if let Some(c) = controls.get_mut(*h_child) {
                OnSiblingSelected::on_sibling_selected(c.control_mut(), requester);
            }
        }
    }

    pub fn register_menu(&mut self, menu: Menu) -> Handle<Menu> {
        RuntimeManager::get().add_menu(menu)
    }
    pub fn show_menu(&self, handle: Handle<Menu>, x: i32, y: i32, max_size: Option<Size>) {
        let r = self.absolute_rect();
        RuntimeManager::get().show_menu(handle, self.handle, r.left() + x, r.top() + y, max_size);
    }

    #[allow(private_bounds)]
    pub fn menuitem<T>(&self, menu_handle: Handle<Menu>, menuitem_handle: Handle<T>) -> Option<&T>
    where
        T: MenuItem,
    {
        if let Some(menu) = RuntimeManager::get().get_menu(menu_handle) {
            return menu.get(menuitem_handle);
        }
        None
    }

    #[allow(private_bounds)]
    pub fn menuitem_mut<T>(&mut self, menu_handle: Handle<Menu>, menuitem_handle: Handle<T>) -> Option<&mut T>
    where
        T: MenuItem,
    {
        if let Some(menu) = RuntimeManager::get().get_menu(menu_handle) {
            return menu.get_mut(menuitem_handle);
        }
        None
    }

    #[inline(always)]
    pub fn theme(&self) -> &Theme {
        RuntimeManager::get().theme()
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
impl GenericCommandBarEvents for ControlBase {}
impl GenericMenuEvents for ControlBase {}
impl ButtonEvents for ControlBase {}
impl ColorPickerEvents for ControlBase {}
impl CheckBoxEvents for ControlBase {}
impl WindowEvents for ControlBase {}
impl ToolBarEvents for ControlBase {}
impl DesktopEvents for ControlBase {}
