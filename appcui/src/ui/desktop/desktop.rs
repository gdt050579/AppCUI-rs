use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ArrangeWindowsMethod {
    Cascade,
    Vertical,
    Horizontal,
    Grid,
}

#[CustomControl(overwrite=OnPaint+OnKeyPressed, internal=true, desktop=true)]
pub struct Desktop {}

impl Desktop {
    /// Creates a new desktop object. This is something that the AppCUI does automatically unless a custom Desktop is provided.
    /// The desktop object is the main control of the application and it is used to manage the windows and controls.
    pub fn new() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        if RuntimeManager::is_instantiated() {
            panic!("A desktop object can only be created once (when the application is started) !");
        }

        Self {
            base: ControlBase::with_status_flags(
                Layout::new("x:0,y:0,w:100%,h:100%"),
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput | StatusFlags::DesktopControl,
            ),
        }
    }

    /// Returns the size of the terminal.
    pub fn terminal_size(&self) -> Size {
        RuntimeManager::get().get_terminal_size()
    }

    /// Returns the size of the desktop.
    /// This is the size of the terminal minus the size of the status bar and the command bar (if they are present).
    pub fn desktop_rect(&self) -> Rect {
        RuntimeManager::get().get_desktop_rect()
    }

    /// Closes the application.
    pub fn close(&mut self) {
        RuntimeManager::get().close();
    }
    fn interface_mut(&mut self) -> Option<&mut dyn Control> {
        if let Some(control) = RuntimeManager::get().get_controls_mut().get_mut(self.handle.cast()) {
            return Some(control.control_mut());
        }
        None
    }
    fn set_focus_for_child_window(&mut self, next_window: bool) {
        let mut idx = self.base.focused_child_index;
        let len = self.base.children.len();
        if next_window {
            idx.add(1, len, Strategy::RotateFromInvalidState);
        } else {
            idx.sub(1, len, Strategy::RotateFromInvalidState);
        }
        if (idx.in_range(len)) && (idx.index() != self.base.focused_child_index.index()) {
            let handle = self.base.children[idx.index()];
            if !handle.is_none() {
                RuntimeManager::get().request_focus_for_control(handle);
            }
        }
    }
    fn reposition_child(&self, handle: Handle<()>, new_poz: Rect) {
        if let Some(control) = RuntimeManager::get().get_controls_mut().get_mut(handle) {
            let base = control.base_mut();
            base.set_position(new_poz.left(), new_poz.top());
            base.set_size(new_poz.width() as u16, new_poz.height() as u16);
        }
    }
    fn arrange_cascade(&self, r: Rect) {
        for (index, child) in self.base.children.iter().enumerate() {
            self.reposition_child(*child, Rect::new(r.left() + index as i32, r.top() + index as i32, r.right(), r.bottom()));
        }
    }
    fn arrange_vertical(&self, r: Rect) {
        let count = self.children.len() as u32;
        if count == 0 {
            return;
        }
        let w = (r.width() / count) as i32;
        for (index, child) in self.base.children.iter().enumerate() {
            self.reposition_child(
                *child,
                Rect::new(
                    r.left() + (index as i32) * w,
                    r.top(),
                    if (index + 1) as u32 == count {
                        r.right()
                    } else {
                        r.left() + ((index as i32) + 1) * w - 1
                    },
                    r.bottom(),
                ),
            );
        }
    }
    fn arrange_grid(&self, r: Rect) {
        let count = self.children.len() as u32;
        if count == 0 {
            return;
        }
        let mut columns = ((count as f32).sqrt()) as i32;
        if columns * columns < count as i32 {
            columns += 1;
        }
        let mut rows = (count as i32) / columns;
        if rows * columns < count as i32 {
            rows += 1;
        }
        let mut x = r.left();
        let mut y = r.top();
        let mut column = 1;
        let mut row = 1;
        let w = (r.width() as i32) / columns;
        let h = (r.height() as i32) / rows;
        let len = self.base.children.len();
        for (index, child) in self.base.children.iter().enumerate() {
            self.reposition_child(
                *child,
                Rect::new(
                    x,
                    y,
                    if (column < columns) && (index + 1 < len) { x + w - 1 } else { r.right() },
                    if row < rows { y + h - 1 } else { r.bottom() },
                ),
            );
            x += w;
            column += 1;
            if column > columns {
                x = r.left();
                column = 1;
                y += h;
                row += 1;
            }
        }
    }
    fn arrange_horizontal(&self, r: Rect) {
        let count = self.children.len() as u32;
        if count == 0 {
            return;
        }
        let h = (r.height() / count) as i32;
        for (index, child) in self.base.children.iter().enumerate() {
            self.reposition_child(
                *child,
                Rect::new(
                    r.left(),
                    r.top() + (index as i32) * h,
                    r.right(),
                    if (index + 1) as u32 == count {
                        r.bottom()
                    } else {
                        r.top() + ((index as i32) + 1) * h - 1
                    },
                ),
            );
        }
    }
    pub fn arrange_windows(&mut self, method: ArrangeWindowsMethod) {
        let r = self.desktop_rect();
        match method {
            ArrangeWindowsMethod::Cascade => self.arrange_cascade(r),
            ArrangeWindowsMethod::Vertical => self.arrange_vertical(r),
            ArrangeWindowsMethod::Horizontal => self.arrange_horizontal(r),
            ArrangeWindowsMethod::Grid => self.arrange_grid(r),
        }
    }

    /// Adds a new window to the desktop. The window will be displayed on top of all other windows.
    pub fn add_window<T>(&mut self, window: T) -> Handle<T>
    where
        T: Control + WindowControl + NotModalWindow + 'static,
    {
        RuntimeManager::get().add_window(window)
    }
}
impl OnPaint for Desktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(theme.desktop.character);
    }
}
impl OnKeyPressed for Desktop {
    fn on_key_pressed(&mut self, key: Key, _: char) -> EventProcessStatus {
        match key.value() {
            key!("Escape") => {
                if let Some(desktop_interface) = self.interface_mut() {
                    if DesktopEvents::on_close(desktop_interface) == ActionRequest::Allow {
                        RuntimeManager::get().close();
                    }
                }
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Tab") | key!("Tab") => {
                self.set_focus_for_child_window(true);
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Shift+Tab") | key!("Shift+Tab") => {
                self.set_focus_for_child_window(false);
                return EventProcessStatus::Processed;
            }
            key!("Alt+0") => {
                // Dialogs::WindowManager::Show();
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        // check controls hot keys
        let controls = RuntimeManager::get().get_controls_mut();
        for ctrl in self.children.iter() {
            if let Some(child) = controls.get(*ctrl) {
                if child.base().hotkey() == key {
                    RuntimeManager::get().request_focus_for_control(*ctrl);
                    return EventProcessStatus::Processed;
                }
            }
        }
        EventProcessStatus::Ignored
    }
}
