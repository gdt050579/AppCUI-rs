use crate::prelude::*;

use super::initialization_flags::*;
use super::toolbar;
use super::toolbar::*;
use super::DragStatus;
use super::Flags;
use super::Title;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
enum MoveDirectionTowards {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Copy, Clone)]
struct Distance {
    value: u32,
    handle: Handle<()>,
}

#[CustomControl(overwrite=OnPaint+OnResize+OnKeyPressed+OnMouseEvent, internal=true, window=true)]
#[derive(Default)]
pub struct Window {
    title: Title,
    flags: Flags,
    window_type: Type,
    toolbar: ToolBar,
    resize_move_mode: bool,
    maximized: bool,
    drag_status: DragStatus,
    drag_start_point: Point,
    old_rect: Rect,
    hotkey_handle: Handle<super::toolbar::HotKey>,
    tag_handle: Handle<super::toolbar::Tag>,
}

const MOVE_TO_LOWER_MARGIN: i32 = -100000;
const MOVE_TO_UPPER_MARGIN: i32 = 100000;

impl Window {
    fn point_to_point_distance(origin_rect: Rect, object_rect: Rect, dir: MoveDirectionTowards) -> u32 {
        let origin: Point;
        let object: Point;
        match dir {
            MoveDirectionTowards::Left => {
                // we need to have <object>[space]<origin>
                // we compare <TOP,LEFT>
                object = Point::new(object_rect.right(), object_rect.top());
                origin = Point::new(origin_rect.left(), origin_rect.top());
                if object.x >= origin.x {
                    return u32::MAX;
                }
            }
            MoveDirectionTowards::Right => {
                // we need to have <origin>[space]<object>
                object = Point::new(object_rect.left(), object_rect.top());
                origin = Point::new(origin_rect.right(), origin_rect.top());
                if object.x <= origin.x {
                    return u32::MAX;
                }
            }
            MoveDirectionTowards::Top => {
                // we need to have <object>[space]<origin>
                object = Point::new(object_rect.left(), object_rect.bottom());
                origin = Point::new(origin_rect.left(), origin_rect.top());
                if object.y >= origin.y {
                    return u32::MAX;
                }
            }
            MoveDirectionTowards::Bottom => {
                // we need to have <origin>[space]<object>
                object = Point::new(object_rect.left(), object_rect.top());
                origin = Point::new(origin_rect.left(), origin_rect.bottom());
                if object.y <= origin.y {
                    return u32::MAX;
                }
            }
        }
        (((object.x - origin.x) * (object.x - origin.x)) as u32) + (((object.y - origin.y) * (object.y - origin.y)) as u32)
    }

    fn compute_closest_distance(handle_parent: Handle<()>, object_rect: Rect, dir: MoveDirectionTowards) -> Option<Distance> {
        let controls = RuntimeManager::get().get_controls_mut();
        if let Some(parent) = controls.get(handle_parent) {
            let base = parent.base();
            if !base.is_active() {
                return None;
            }
            let mut best = Distance {
                value: u32::MAX,
                handle: Handle::None,
            };
            for child in &base.children {
                if let Some(result) = Window::compute_closest_distance(*child, object_rect, dir) {
                    if result.value < best.value {
                        best = result;
                    }
                }
            }
            if best.handle.is_none() {
                if base.can_receive_input() {
                    let r = base.absolute_rect();
                    let dist = Window::point_to_point_distance(r, object_rect, dir);
                    if dist < best.value {
                        best.value = dist;
                        best.handle = handle_parent;
                        return Some(best);
                    }
                }
                return None;
            }
            Some(best)
        } else {
            None
        }
    }
    fn find_closest_control(handle: Handle<()>, dir: MoveDirectionTowards) -> Handle<()> {
        let rm = RuntimeManager::get();
        let controls = rm.get_controls_mut();
        let mut h = handle;
        let mut found = Handle::None;
        while let Some(ctrl) = controls.get_mut(h) {
            let base = ctrl.base();
            if !base.is_active() {
                break;
            }
            // found a possible candidate
            if base.can_receive_input() {
                found = h;
            }
            if !base.focused_child_index.in_range(base.children.len()) {
                break;
            }
            h = base.children[base.focused_child_index.index()];
        }
        if (found.is_none()) || (handle == found) {
            return Handle::None;
        }
        if let Some(ctrl) = controls.get_mut(found) {
            let object_rect = ctrl.base().absolute_rect();
            if let Some(result) = Window::compute_closest_distance(handle, object_rect, dir) {
                if (result.value == u32::MAX) || (result.handle.is_none()) {
                    return Handle::None;
                }
                return result.handle;
            } else {
                return Handle::None;
            }
        }
        Handle::None
    }
    pub fn with_type(title: &str, layout: Layout, flags: Flags, window_type: Type) -> Self {
        Window::with_type_and_status_flags(title, layout, flags, window_type, StatusFlags::None)
    }
    #[inline(always)]
    pub(super) fn with_type_and_status_flags(title: &str, layout: Layout, flags: Flags, window_type: Type, status_flags: StatusFlags) -> Self {
        let mut win: Window = Window {
            base: ControlBase::with_status_flags(
                layout,
                status_flags | StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | StatusFlags::WindowControl,
            ),
            title: Title::new(title),
            flags,
            window_type,
            resize_move_mode: false,
            maximized: false,
            toolbar: ToolBar::new(),
            drag_status: DragStatus::None,
            drag_start_point: Point::new(0, 0),
            old_rect: Rect::new(0, 0, 0, 0),
            hotkey_handle: Handle::None,
            tag_handle: Handle::None,
        };
        win.set_size_bounds(12, 3, u16::MAX, u16::MAX);
        win.set_margins(1, 1, 1, 1);
        if !flags.contains(Flags::NoCloseButton) {
            let g = win.toolbar.create_group(GroupPosition::TopRight);
            win.toolbar.add(g, toolbar::CloseButton::new());
        }
        if flags.contains(Flags::Sizeable) {
            let g = win.toolbar.create_group(GroupPosition::TopLeft);
            win.toolbar.add(g, toolbar::MaximizeRestoreButton::new());
            let g = win.toolbar.create_group(GroupPosition::BottomRight);
            win.toolbar.add(g, toolbar::ResizeCorner::new());
        }
        // hotkey
        let g = win.toolbar.create_group(GroupPosition::TopLeft);
        win.hotkey_handle = win.toolbar.add(g, toolbar::HotKey::new());

        // tag
        let g = win.toolbar.create_group(GroupPosition::TopLeft);
        win.tag_handle = win.toolbar.add(g, toolbar::Tag::new());

        win
    }
    #[inline(always)]
    pub fn new(title: &str, layout: Layout, flags: Flags) -> Self {
        Window::with_type(title, layout, flags, Type::Normal)
    }
    /// Adds a control to the current window. Once the control was added
    /// a handle for that control wil be returned or `Handle::None` if some
    /// error occured.
    ///
    /// # Exemple
    /// ```rust,no_run
    ///     use appcui::prelude::*;
    ///
    ///     let mut a = App::new().build().unwrap();
    ///     let mut w = Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None);
    ///     w.add(Button::new("Press me",layout!("x:1,y:1,w:10"),button::Type::Normal));
    /// ```    
    ///
    /// You can not add a Window as a child to another Window nor can you add a Desktop
    /// as a child for another Desktop. The following example will not compile as we
    /// try to add a Window as a child to another window.
    /// ```rust,compile_fail
    ///     use appcui::prelude::*;
    ///
    ///     let mut a = App::new().build().unwrap();
    ///     let mut w = Window::new("Title", layout!("a:c,w:20,h:10"), window::Flags::None);
    ///     w.add(Window::new("aaa",layout!("a:c,w:20,h:10"),window::Flags::None));
    /// ```    
    pub fn add<T>(&mut self, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static,
    {
        self.add_child(control)
    }
    /// Gets a reference to a control from its handle.
    ///
    /// Returns `None` if the handle is invalid or if the control has been deleted.
    ///
    /// # Parameters
    ///
    /// * `handle` - The handle to the control
    ///
    /// # Returns
    ///
    /// * `Some(&T)` - A reference to the control if the handle is valid
    /// * `None` - If the handle is invalid or the control has been deleted
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'My Window',a:c,w:40,h:10");
    /// let label_handle = win.add(label!("'Hello World',a:c,w:12,h:1"));
    /// 
    /// if let Some(label) = win.control(label_handle) {
    ///     // Use the label reference
    ///     assert_eq!(label.caption(), "Hello World");
    /// }
    /// ```
    pub fn control<T>(&self, handle: Handle<T>) -> Option<&T>
    where
        T: Control + 'static,
    {
        RuntimeManager::get().get_control(handle)
    }
    /// Gets a mutable reference to a control from its handle.
    ///
    /// Returns `None` if the handle is invalid or if the control has been deleted.
    ///
    /// # Parameters
    ///
    /// * `handle` - The handle to the control
    ///
    /// # Returns
    ///
    /// * `Some(&mut T)` - A mutable reference to the control if the handle is valid
    /// * `None` - If the handle is invalid or the control has been deleted
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'My Window',a:c,w:40,h:10");
    /// let label_handle = win.add(label!("'Hello World',a:c,w:12,h:1"));
    /// 
    /// if let Some(label) = win.control_mut(label_handle) {
    ///     // Use the mutable label reference
    ///     label.set_caption("New Text");
    /// }
    /// ```
    pub fn control_mut<T>(&mut self, handle: Handle<T>) -> Option<&mut T>
    where
        T: Control + 'static,
    {
        RuntimeManager::get().get_control_mut(handle)
    }
    /// Requests focus for a specific control.
    ///
    /// This method attempts to set the focus to the control specified by the handle.
    /// If the control can receive focus, it will become the active control.
    ///
    /// # Parameters
    ///
    /// * `handle` - The handle to the control that should receive focus
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'My Window',a:c,w:40,h:10");
    /// let button_handle = win.add(button!("'Click Me',a:c,w:10,h:1"));
    /// 
    /// // Set focus to the button
    /// win.request_focus_for_control(button_handle);
    /// ```
    pub fn request_focus_for_control<T>(&mut self, handle: Handle<T>)
    where
        T: Control + 'static,
    {
        RuntimeManager::get().request_focus_for_control(handle.cast());
    }

    /// Gets a mutable reference to the window's toolbar.
    ///
    /// The toolbar allows adding toolbar items like buttons, checkboxes, labels, and 
    /// single choice items to the window.
    ///
    /// # Returns
    ///
    /// A mutable reference to the window's toolbar
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'My Window',a:c,w:40,h:10");
    /// 
    /// // Create a toolbar group and add a button to it
    /// let group = win.toolbar().create_group(toolbar::GroupPosition::TopRight);
    /// let button = win.toolbar().add(group, toolbar::Button::new("Help"));
    /// ```
    pub fn toolbar(&mut self) -> &mut ToolBar {
        &mut self.toolbar
    }

    /// Sets the title of the window.
    ///
    /// # Parameters
    ///
    /// * `title` - The new title text
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'My Window',a:c,w:40,h:10");
    /// win.set_title("New Window Title");
    /// ```
    pub fn set_title(&mut self, title: &str) {
        self.title.set_text(title);
        self.update_positions(self.size());
    }
    /// Gets the current title of the window.
    ///
    /// # Returns
    ///
    /// The current window title as a string slice
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'My Window',a:c,w:40,h:10");
    /// assert_eq!(win.title(), "My Window");
    /// ```
    pub fn title(&self) -> &str {
        self.title.get_text()
    }

    /// Sets a tag for the window.
    ///
    /// A tag is an additional text displayed in the window's title bar.
    /// It can be used to show extra information about the window's state or content.
    ///
    /// # Parameters
    ///
    /// * `name` - The tag text
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'Document Editor',a:c,w:60,h:20");
    /// win.set_tag("[Modified]");
    /// ```
    pub fn set_tag(&mut self, name: &str) {
        if let Some(item) = self.toolbar.get_mut(self.tag_handle) {
            item.set_text(name);
            self.update_positions(self.size());
        }
    }
    /// Gets the current tag of the window.
    ///
    /// # Returns
    ///
    /// * `Some(&str)` - The current tag text if a tag is set
    /// * `None` - If no tag is set
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'Document Editor',a:c,w:60,h:20");
    /// win.set_tag("[Modified]");
    /// assert_eq!(win.tag(), Some("[Modified]"));
    /// ```
    pub fn tag(&self) -> Option<&str> {
        if let Some(item) = self.toolbar.get(self.tag_handle) {
            return Some(item.get_text());
        }
        None
    }
    /// Clears the window's tag.
    ///
    /// This removes any tag that was previously set with `set_tag()`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'Document Editor',a:c,w:60,h:20");
    /// win.set_tag("[Modified]");
    /// // After saving the document
    /// win.clear_tag();
    /// assert_eq!(win.tag(), Some(""));
    /// ```
    pub fn clear_tag(&mut self) {
        if let Some(item) = self.toolbar.get_mut(self.tag_handle) {
            item.set_text("");
            self.update_positions(self.size());
        }
    }
    /// Sets an automatically assigned hotkey for the window.
    ///
    /// This method finds the first available hotkey in the system and assigns it to this window.
    /// The hotkey, if available, will use the Alt modifier.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'My Window',a:c,w:40,h:10");
    /// win.set_auto_hotkey();
    /// ```
    pub fn set_auto_hotkey(&mut self) {
        let mut k = RuntimeManager::get().find_first_free_hotkey();
        if k.code != KeyCode::None {
            k.modifier = KeyModifier::Alt;
            self.set_hotkey(k);
        }
    }
    /// Sets a specific hotkey for the window.
    ///
    /// This hotkey can be used to activate the window from anywhere in the application.
    ///
    /// # Parameters
    ///
    /// * `key` - The key to use as a hotkey
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'My Window',a:c,w:40,h:10");
    /// win.set_hotkey(key!("Alt+W"));
    /// ```
    pub fn set_hotkey<T>(&mut self, key: T)
    where
        Key: From<T>,
    {
        if let Some(item) = self.toolbar.get_mut(self.hotkey_handle) {
            let k: Key = key.into();
            self.base.hotkey = k; // we need this deduplication for desktop to be able to change focus
            item.set_key(k);
            self.update_positions(self.size());
        }
    }

    /// Enters resize mode for the window.
    ///
    /// When in resize mode, the window can be resized using keyboard controls:
    /// - Arrow keys move the window
    /// - Alt+Arrow keys snap the window to edges
    ///
    /// The resize mode can be exited by pressing Enter, Escape, Space, or Tab.
    ///
    /// This method only has an effect if the window has focus and is not in single window mode.
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'Resizable Window',a:c,w:40,h:10,flags:Sizeable");
    /// win.enter_resize_mode();
    /// ```
    pub fn enter_resize_mode(&mut self) {
        if (self.has_focus()) && (!self.is_singlewindow()) {
            self.resize_move_mode = true;
            self.base.set_key_input_before_children_flag(true);
        }
    }
    pub(super) fn is_in_resize_mode(&self) -> bool {
        self.resize_move_mode
    }

    fn center_to_screen(&mut self) {
        let screen_size = RuntimeManager::get().terminal_size();
        let win_size = self.size();
        let x = (screen_size.width as i32 - win_size.width as i32) / 2;
        let y = (screen_size.height as i32 - win_size.height as i32) / 2;
        self.set_position(x, y);
    }
    fn resize_window_with(&mut self, add_to_width: i32, add_to_height: i32) {
        let size = self.size();
        let new_width = ((size.width as i32) + add_to_width).clamp(0, 0xFFFF);
        let new_height = ((size.height as i32) + add_to_height).clamp(0, 0xFFFF);
        self.set_size(new_width as u16, new_height as u16);
    }
    fn move_window_pos_to(&mut self, add_x: i32, add_y: i32, keep_in_desktop_bounderies: bool) {
        let size = self.size();
        let screen_size = RuntimeManager::get().terminal_size();
        let mut pos = self.position();
        if keep_in_desktop_bounderies {
            pos.x = (pos.x + add_x).clamp(0, screen_size.width as i32 - size.width as i32);
            pos.y = (pos.y + add_y).clamp(0, screen_size.height as i32 - size.height as i32);
        } else {
            pos.x = (pos.x + add_x).clamp(0, screen_size.width as i32 - 1);
            pos.y = (pos.y + add_y).clamp(0, screen_size.height as i32 - 1);
        }
        self.set_position(pos.x, pos.y);
    }

    fn maximize_restore(&mut self) {
        if !self.maximized {
            self.old_rect = Rect::with_point_and_size(self.position(), self.size());
            let desktop_rect = RuntimeManager::get().get_desktop_rect();
            self.set_position(desktop_rect.left(), desktop_rect.top());
            self.set_size(desktop_rect.width() as u16, desktop_rect.height() as u16);
            self.maximized = true;
        } else {
            let l = self.old_rect.left();
            let t = self.old_rect.top();
            self.set_position(l, t);
            let w = self.old_rect.width() as u16;
            let h = self.old_rect.height() as u16;
            self.set_size(w, h);
            self.maximized = false;
        }
    }

    fn get_children_start_index(index: VectorIndex, count: usize, start_from_current: bool) -> VectorIndex {
        if (start_from_current) && (index.in_range(count)) {
            index
        } else {
            VectorIndex::Invalid
        }
    }
    fn find_next_child_control(handle: Handle<()>, forward: bool, start_from_current: bool, window_level: bool) -> Option<Handle<()>> {
        let controls = RuntimeManager::get().get_controls();
        if let Some(control) = controls.get(handle) {
            let base = control.base();
            if !base.is_active() {
                return None;
            }
            let count = base.children.len();
            if count > 0 {
                let mut idx = Window::get_children_start_index(base.focused_child_index, count, start_from_current);
                if idx.in_range(count) {
                    let result = Window::find_next_child_control(base.children[idx.index()], forward, start_from_current, false);
                    if result.is_some() {
                        return result;
                    }
                }
                let strategy = if window_level {
                    Strategy::RotateFromInvalidState
                } else {
                    Strategy::RotateWithInvalidState
                };
                let mut steps = 0;
                while steps < count {
                    steps += 1;
                    if forward {
                        idx.add(1, count, strategy);
                    } else {
                        idx.sub(1, count, strategy);
                    }
                    if !idx.in_range(count) {
                        return None;
                    }
                    let child_handle = base.children[idx.index()];
                    if let Some(child) = controls.get(child_handle) {
                        let result = Window::find_next_child_control(child_handle, forward, false, false);
                        if result.is_some() {
                            return result;
                        }
                        if child.base().can_receive_input() {
                            return Some(child_handle);
                        }
                    }
                }
            }
        }
        None
    }

    fn hotkey_to_handle(controls: &ControlHandleManager, parent: Handle<()>, hotkey: Key) -> Handle<()> {
        if let Some(control) = controls.get(parent) {
            let base = control.base();
            // object has to be visible and enabled
            if base.is_visible() && base.is_enabled() {
                if base.can_receive_input() && base.hotkey() == hotkey {
                    // I hold te hotkey
                    return parent;
                }
                for child in base.children.iter() {
                    let result = Window::hotkey_to_handle(controls, *child, hotkey);
                    if !result.is_none() {
                        return result;
                    }
                }
            }
        }
        Handle::None
    }

    fn update_positions(&mut self, size: Size) {
        // recompute decorator based on the new size
        let (left, right) = self.toolbar.update_positions(size);
        // recompute title position
        self.title.set_margin(left, right);
    }

    fn on_mouse_over(&mut self, x: i32, y: i32) -> EventProcessStatus {
        if let Some(item) = self.toolbar.get_from_position(x, y) {
            let base = item.get_base();
            let cx = base.center_x();
            let y = base.get_y();
            let tooltip = base.get_tooltip();
            if tooltip.is_empty() {
                self.hide_tooltip();
            } else {
                self.show_tooltip_on_point(tooltip, cx, y);
            }
            self.toolbar.set_current_item_handle(item.handle().cast());
            return EventProcessStatus::Processed;
        }
        // if I reach this point - tool tip should not be shown and there is no win button selected
        self.hide_tooltip();
        if self.toolbar.get_current_item_handle().is_none() {
            return EventProcessStatus::Ignored;
        }
        self.toolbar.clear_current_item_handle();
        EventProcessStatus::Processed
    }

    fn on_mouse_leave(&mut self) -> EventProcessStatus {
        self.toolbar.set_current_item_pressed(false);
        if self.toolbar.get_current_item_handle().is_none() {
            return EventProcessStatus::Ignored;
        }
        self.toolbar.clear_current_item_handle();
        self.hide_tooltip();
        EventProcessStatus::Processed
    }

    fn on_mouse_pressed(&mut self, x: i32, y: i32) -> EventProcessStatus {
        self.toolbar.set_current_item_pressed(false);
        self.drag_status = DragStatus::None;
        self.resize_move_mode = false;
        self.base.set_key_input_before_children_flag(false);

        let item_handle = if let Some(item) = self.toolbar.get_from_position(x, y) {
            if let ToolBarItem::ResizeCorner(_) = item {
                self.drag_status = DragStatus::Resize;
            }
            item.handle()
        } else {
            Handle::None
        };

        if !item_handle.is_none() {
            self.toolbar.set_current_item_handle(item_handle.cast());
            self.toolbar.set_current_item_pressed(true);
            return EventProcessStatus::Processed;
        }

        self.toolbar.clear_current_item_handle();
        self.hide_tooltip();

        if !self.flags.contains(Flags::FixedPosition) {
            self.drag_status = DragStatus::Move;
            self.drag_start_point.x = x;
            self.drag_start_point.y = y;
        }
        EventProcessStatus::Processed
    }
    fn on_mouse_drag(&mut self, x: i32, y: i32) -> EventProcessStatus {
        self.resize_move_mode = false;
        self.base.set_key_input_before_children_flag(false);
        match self.drag_status {
            DragStatus::None => EventProcessStatus::Ignored,
            DragStatus::Move => {
                let left = self.screen_clip.left;
                let top = self.screen_clip.top;
                let p = self.drag_start_point;
                self.set_position(x + left - p.x, y + top - p.y);
                EventProcessStatus::Processed
            }
            DragStatus::Resize => {
                if (x > 0) && (y > 0) {
                    self.set_size((x + 1) as u16, (y + 1) as u16);
                }
                EventProcessStatus::Processed
            }
        }
    }

    /// Closes the window.
    ///
    /// This method closes the window after calling the `on_cancel` method from the `WindowEvents` trait.
    /// If `on_cancel` returns `ActionRequest::Allow`, the window will be closed.
    /// If it returns `ActionRequest::Deny`, the window will remain open.
    ///
    /// For single window applications, closing the window will exit the application.
    ///
    /// # Panics
    ///
    /// This method will panic if called on a modal window. To close a modal window,
    /// use the `exit()` or `exit_with()` methods instead.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let mut win = window!("'My Window',a:c,w:40,h:10");
    /// // Later, when you need to close the window:
    /// win.close();
    /// ```    
    pub fn close(&mut self) {
        if self.base.is_modal_window() {
            panic!("close method can not be called directly on a modal window (or the base of a modal window)");
        }
        if let Some(interface) = self.interface_mut() {
            let result = WindowEvents::on_cancel(interface);
            if result == ActionRequest::Allow {
                // logic to remove me
                if self.is_singlewindow() {
                    // close the entire app
                    RuntimeManager::get().close();
                } else {
                    // remove me from the desktop
                    RuntimeManager::get().request_remove(self.handle);
                }
            }
        }
    }
    fn on_mouse_release(&mut self) -> EventProcessStatus {
        self.toolbar.set_current_item_pressed(false);
        self.resize_move_mode = false;
        self.base.set_key_input_before_children_flag(false);

        if self.drag_status != DragStatus::None {
            self.drag_status = DragStatus::None;
        } else {
            self.on_toolbar_item_clicked(self.toolbar.get_current_item_handle());
        }
        EventProcessStatus::Processed
    }
    fn on_toolbar_item_clicked(&mut self, handle: Handle<()>) -> bool {
        if let Some(item) = self.toolbar.get_item_mut(handle) {
            match item {
                ToolBarItem::CloseButton(_) => {
                    self.close();
                    return true;
                }
                ToolBarItem::ResizeCorner(_) => {
                    self.maximize_restore();
                    return true;
                }
                ToolBarItem::Button(_) => {
                    if let Some(me) = self.interface_mut() {
                        return ToolBarEvents::on_button_clicked(me, handle.cast()) == EventProcessStatus::Processed;
                    }
                    return false;
                }
                ToolBarItem::CheckBox(checkbox) => {
                    checkbox.reverse_check();
                    let is_checked = checkbox.is_checked();
                    if let Some(me) = self.interface_mut() {
                        ToolBarEvents::on_checkbox_clicked(me, handle.cast(), is_checked);
                    }
                    return true; // regardless on what we do in the interface
                }
                ToolBarItem::SingleChoice(_) => {
                    self.toolbar.update_singlechoice_group_id(handle);
                    if let Some(me) = self.interface_mut() {
                        ToolBarEvents::on_choice_selected(me, handle.cast());
                    }
                    return true; // regardless on what we do in the interface
                }
                ToolBarItem::Label(_) => return false,
                ToolBarItem::HotKey(_) => return false,
                ToolBarItem::Tag(_) => return false,
                ToolBarItem::MaximizeRestoreButton(_) => {
                    self.maximize_restore();
                    self.toolbar.clear_current_item_handle();
                    return true;
                }
            }
        }
        false
    }
    pub(super) fn interface_mut(&mut self) -> Option<&mut dyn Control> {
        if let Some(control) = RuntimeManager::get().get_controls_mut().get_mut(self.handle.cast()) {
            return Some(control.control_mut());
        }
        None
    }
    // pub(super) fn interface(&self) -> Option<&dyn Control> {
    //     if let Some(control) = RuntimeManager::get().get_controls().get(self.handle.cast()) {
    //         return Some(control.get_control());
    //     }
    //     None
    // }

    /// Returns a generic handle to the current window.
    ///
    /// # Returns
    ///
    /// A handle to the window
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let win = window!("'My Window',a:c,w:40,h:10");
    /// let handle = win.handle();
    /// ```
    #[inline(always)]
    pub fn handle(&self) -> Handle<Window> {
        self.handle.cast()
    }

    /// Returns the background task associated with a specific handle.
    ///
    /// # Parameters
    ///
    /// * `handle` - A handle to a background task
    ///
    /// # Returns
    ///
    /// * `Some(BackgroundTask<T, R>)` - The background task if the handle is valid
    /// * `None` - If the handle is invalid
    #[inline(always)]
    pub fn background_task<T: Send + 'static, R: Send + 'static>(&self, handle: Handle<BackgroundTask<T, R>>) -> Option<BackgroundTask<T, R>> {
        BackgroundTask::from_handle(handle)
    }

    /// Executes a callback function over a specific control (if that control handle is valid)
    ///
    /// # Example
    /// ```rust,no_run
    ///    use appcui::prelude::*;
    ///    let mut app = App::new().build().unwrap();
    ///    let mut w = window!("Title,a:c,w:20,h:10");
    ///    let handle_b = w.add(button!("Button,a:C,w:10"));
    ///    Window::update_control(handle_b, |button| { button.set_caption("New text"); });
    /// ```
    pub fn update_control<T: Control + 'static>(handle: Handle<T>, run: fn(&mut T)) {
        if let Some(control) = RuntimeManager::get().get_control_mut(handle) {
            run(control);
        }
    }
}

impl OnWindowRegistered for Window {
    fn on_registered(&mut self) {
        if self.is_singlewindow() {
            if self.flags.contains(Flags::Sizeable) {
                // a single window can not be sizeable and can not have the resiz grip and/or the maximized button
                panic!("A window used in a single window mode (via App::build().single_window()) can not be sizeable as it will always have the same size as the desktop. Remove the Sizeable flag and try again !");
            }
            self.flags |= Flags::FixedPosition;
        }
        // propagate my handle to toolbar elements
        self.toolbar.set_window_handle(self.handle);
    }
}

impl OnPaint for Window {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let color_window = if self.has_focus() {
            match self.window_type {
                Type::Normal => theme.window.normal,
                Type::Error => theme.window.error,
                Type::Warning => theme.window.warning,
                Type::Notification => theme.window.info,
            }
        } else {
            theme.window.inactive
        };
        // set some colors
        let color_title: CharAttribute;
        let color_border: CharAttribute;
        let line_type: LineType;

        // initialization
        if self.has_focus() {
            color_title = theme.text.focused;
            color_border = if (self.drag_status == DragStatus::None) && (!self.resize_move_mode) {
                theme.border.focused
            } else {
                theme.border.pressed_or_selectd
            };
            line_type = if (self.drag_status == DragStatus::None) && (!self.resize_move_mode) {
                LineType::Double
            } else {
                LineType::Single
            };
        } else {
            color_title = theme.text.normal;
            color_border = theme.border.normal;
            line_type = LineType::Single;
        }

        let sz = self.size();
        surface.clear(Character::with_attributes(' ', color_window));
        surface.draw_rect(Rect::with_size(0, 0, sz.width as u16, sz.height as u16), line_type, color_border);

        // paint toolbar
        self.toolbar.paint(surface, theme, self.has_focus(), self.maximized);

        // paint title
        self.title.paint(surface, color_title);
    }
}

impl OnResize for Window {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        self.update_positions(new_size);
    }
}

impl OnKeyPressed for Window {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if self.resize_move_mode {
            match key.value() {
                key!("Escape") | key!("Enter") | key!("Space") | key!("Tab") => {
                    self.resize_move_mode = false;
                    self.base.set_key_input_before_children_flag(false);
                    return EventProcessStatus::Processed;
                }
                key!("Up") => {
                    self.move_window_pos_to(0, -1, false);
                    return EventProcessStatus::Processed;
                }
                key!("Down") => {
                    self.move_window_pos_to(0, 1, false);
                    return EventProcessStatus::Processed;
                }
                key!("Left") => {
                    self.move_window_pos_to(-1, 0, false);
                    return EventProcessStatus::Processed;
                }
                key!("Right") => {
                    self.move_window_pos_to(1, 0, false);
                    return EventProcessStatus::Processed;
                }
                key!("Alt+Up") => {
                    self.move_window_pos_to(0, MOVE_TO_LOWER_MARGIN, true);
                    return EventProcessStatus::Processed;
                }
                key!("Alt+Down") => {
                    self.move_window_pos_to(0, MOVE_TO_UPPER_MARGIN, true);
                    return EventProcessStatus::Processed;
                }
                key!("Alt+Left") => {
                    self.move_window_pos_to(MOVE_TO_LOWER_MARGIN, 0, true);
                    return EventProcessStatus::Processed;
                }
                key!("Alt+Right") => {
                    self.move_window_pos_to(MOVE_TO_UPPER_MARGIN, 0, true);
                    return EventProcessStatus::Processed;
                }
                key!("C") => {
                    self.center_to_screen();
                    return EventProcessStatus::Processed;
                }
                key!("M") | key!("R") => {
                    self.maximize_restore();
                    return EventProcessStatus::Processed;
                }
                key!("Ctrl+Up") => {
                    self.resize_window_with(0, -1);
                    return EventProcessStatus::Processed;
                }
                key!("Ctrl+Down") => {
                    self.resize_window_with(0, 1);
                    return EventProcessStatus::Processed;
                }
                key!("Ctrl+Left") => {
                    self.resize_window_with(-1, 0);
                    return EventProcessStatus::Processed;
                }
                key!("Ctrl+Right") => {
                    self.resize_window_with(1, 0);
                    return EventProcessStatus::Processed;
                }

                _ => return EventProcessStatus::Ignored,
            }
        } else {
            match key.value() {
                key!("Tab") => {
                    if let Some(new_child) = Window::find_next_child_control(self.handle, true, true, true) {
                        RuntimeManager::get().request_focus_for_control(new_child);
                    }
                    return EventProcessStatus::Processed;
                }
                key!("Shift+Tab") => {
                    if let Some(new_child) = Window::find_next_child_control(self.handle, false, true, true) {
                        RuntimeManager::get().request_focus_for_control(new_child);
                    }
                    return EventProcessStatus::Processed;
                }
                key!("Ctrl+Alt+M") | key!("Ctrl+Alt+R") => {
                    self.resize_move_mode = true;
                    self.base.set_key_input_before_children_flag(false);
                    return EventProcessStatus::Processed;
                }
                key!("Escape") => {
                    self.close();
                    return EventProcessStatus::Processed;
                }
                key!("Left") | key!("Ctrl+Left") | key!("Alt+Left") => {
                    let res = Window::find_closest_control(self.handle, MoveDirectionTowards::Right);
                    if !res.is_none() {
                        RuntimeManager::get().request_focus_for_control(res);
                    }
                    return EventProcessStatus::Processed;
                }
                key!("Right") | key!("Ctrl+Right") | key!("Alt+Right") => {
                    let res = Window::find_closest_control(self.handle, MoveDirectionTowards::Left);
                    if !res.is_none() {
                        RuntimeManager::get().request_focus_for_control(res);
                    }
                    return EventProcessStatus::Processed;
                }
                key!("Up") | key!("Ctrl+Up") | key!("Alt+Up") => {
                    let res = Window::find_closest_control(self.handle, MoveDirectionTowards::Bottom);
                    if !res.is_none() {
                        RuntimeManager::get().request_focus_for_control(res);
                    }
                    return EventProcessStatus::Processed;
                }
                key!("Down") | key!("Ctrl+Down") | key!("Alt+Down") => {
                    let res = Window::find_closest_control(self.handle, MoveDirectionTowards::Top);
                    if !res.is_none() {
                        RuntimeManager::get().request_focus_for_control(res);
                    }
                    return EventProcessStatus::Processed;
                }
                _ => {}
            }
            if (key.modifier & (KeyModifier::Alt | KeyModifier::Ctrl | KeyModifier::Shift)) == KeyModifier::Alt {
                // hotkey --> check
                if let Some(handle) = self.toolbar.hotkey_to_item(key) {
                    self.on_toolbar_item_clicked(handle);
                    return EventProcessStatus::Processed;
                }
            }
            // lets check if a key was associated with a control as a hotkey
            let rm = RuntimeManager::get();
            let control_handle = Window::hotkey_to_handle(rm.get_controls(), self.handle, key);
            if !control_handle.is_none() {
                // request focus for that control
                rm.request_focus_for_control(control_handle);
                // request the default action for that control
                rm.request_default_action_for_control(control_handle);
                // call the default method
                // if let Some(control) = rm.get_controls_mut().get_mut(control_handle) {
                //     OnDefaultAction::on_default_action(control.control_mut());
                // }
                return EventProcessStatus::Processed;
            }
        }
        EventProcessStatus::Ignored
    }
}

impl OnMouseEvent for Window {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => EventProcessStatus::Ignored,
            MouseEvent::Leave => self.on_mouse_leave(),
            MouseEvent::Over(point) => self.on_mouse_over(point.x, point.y),
            MouseEvent::Pressed(event) => self.on_mouse_pressed(event.x, event.y),
            MouseEvent::Released(_) => self.on_mouse_release(),
            MouseEvent::DoubleClick(_) => EventProcessStatus::Ignored,
            MouseEvent::Drag(event) => self.on_mouse_drag(event.x, event.y),
            MouseEvent::Wheel(_) => EventProcessStatus::Ignored,
        }
    }
}
