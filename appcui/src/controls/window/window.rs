use AppCUIProcMacro::*;

use super::decorator::DecoratorLayout;
use super::Decorator;
use super::DecoratorType;
use super::DecoratorsManager;
use super::DragStatus;
use super::WindowFlags;
use crate::controls::events::*;
use crate::controls::menu::Menu;
use crate::controls::menu::MenuBar;
use crate::controls::Handle;
use crate::controls::*;
use crate::graphics::*;
use crate::input::*;
use crate::system::*;
use crate::utils::Caption;
use crate::utils::Strategy;
use crate::utils::VectorIndex;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
enum MoveDirection {
    ToLeft,
    ToRight,
    ToTop,
    ToBottom,
}

#[AppCUIControl(overwrite=OnPaint+OnResize+OnKeyPressed+OnMouseEvent)]
pub struct Window {
    title: String,
    flags: WindowFlags,
    menu: Option<MenuBar>,
    decorators: DecoratorsManager,
    resize_move_mode: bool,
    maximized: bool,
    drag_status: DragStatus,
    drag_start_point: Point,
    title_max_width: u16,
    title_left_margin: i32,
    old_rect: Rect,
}

const MOVE_TO_LOWER_MARGIN: i32 = -100000;
const MOVE_TO_UPPER_MARGIN: i32 = 100000;

impl Window {
    fn point_to_point_distance(origin_rect: Rect, object_rect: Rect, dir: MoveDirection) -> u32 {
        let origin: Point;
        let object: Point;
        match dir {
            MoveDirection::ToLeft => {
                // we need to have <object>[space]<origin>
                // we compare <TOP,LEFT>
                object = Point::new(object_rect.get_right(), object_rect.get_top());
                origin = Point::new(origin_rect.get_left(), origin_rect.get_top());
                if object.x >= origin.x {
                    return u32::MAX;
                }
            }
            MoveDirection::ToRight => {
                // we need to have <origin>[space]<object>
                object = Point::new(object_rect.get_left(), object_rect.get_top());
                origin = Point::new(origin_rect.get_right(), origin_rect.get_top());
                if object.x <= origin.x {
                    return u32::MAX;
                }
            }
            MoveDirection::ToTop => {
                // we need to have <object>[space]<origin>
                object = Point::new(object_rect.get_left(), object_rect.get_bottom());
                origin = Point::new(origin_rect.get_left(), origin_rect.get_top());
                if object.y >= origin.y {
                    return u32::MAX;
                }
            }
            MoveDirection::ToBottom => {
                // we need to have <origin>[space]<object>
                object = Point::new(object_rect.get_left(), object_rect.get_top());
                origin = Point::new(origin_rect.get_left(), origin_rect.get_bottom());
                if object.y <= origin.y {
                    return u32::MAX;
                }
            }
        }
        return (((object.x - origin.x) * (object.x - origin.x)) as u32)
            + (((object.y - origin.y) * (object.y - origin.y)) as u32);
    }
    pub fn new(title: &str, layout: Layout, flags: WindowFlags) -> Self {
        let mut win = Window {
            base: ControlBase::new(
                layout,
                StatusFlags::Visible | StatusFlags::Enabled | StatusFlags::AcceptInput,
            ),
            title: String::from(title),
            flags,
            menu: None,
            resize_move_mode: false,
            maximized: false,
            decorators: DecoratorsManager::new(),
            drag_status: DragStatus::None,
            drag_start_point: Point::new(0, 0),
            title_max_width: 0,
            title_left_margin: 0,
            old_rect: Rect::new(0, 0, 0, 0),
        };
        win.set_size_bounds(12, 3, u16::MAX, u16::MAX);
        win.set_margins(1, 1, 1, 1);
        if flags.contains(WindowFlags::NoCloseButton) == false {
            win.decorators.add(Decorator::with_type(
                DecoratorType::CloseButton,
                DecoratorLayout::TopRight,
                3,
                "Close window",
            ));
        }
        if flags.contains(WindowFlags::Sizeable) {
            win.decorators.add(Decorator::with_type(
                DecoratorType::MaximizeRestoreButton,
                DecoratorLayout::TopLeft,
                3,
                "Maximize or restore the size of this window",
            ));
            win.decorators.add(Decorator::with_type(
                DecoratorType::WindowResize,
                DecoratorLayout::BottomRight,
                1,
                "Click and drag to resize this window",
            ));
        }
        // hotkey
        let mut hotkey_decorator = Decorator::with_type(
            DecoratorType::HotKeY,
            DecoratorLayout::TopRight,
            3,
            "Press Alt+xx to switch to this window",
        );
        hotkey_decorator.hide();
        win.decorators.add(hotkey_decorator);

        // tag
        let mut tag_decorator =
            Decorator::with_type(DecoratorType::Tag, DecoratorLayout::TopRight, 3, "");
        tag_decorator.hide();
        win.decorators.add(tag_decorator);

        if flags.contains(WindowFlags::Menu) {
            win.menu = Some(MenuBar::new());
            win.set_margins(1, 2, 1, 1);
        }

        win

        /*

           Members->DialogResult                    = Dialogs::Result::None;
           Members->referalItemHandle               = InvalidItemHandle;
           Members->windowItemHandle                = InvalidItemHandle;


           UpdateWindowsButtonsPoz(Members);

           if ((Flags & WindowFlags::Maximized) == WindowFlags::Maximized)
           {
               ASSERT(Maxim izeRestore(), "Fail to maximize window !");
           }

        */
    }
    pub fn add<T>(&mut self, control: T) -> ControlHandle<T>
    where
        T: Control + 'static,
    {
        return self.add_child(control);
    }
    pub fn set_title(&mut self, title: &str) {
        self.title.clear();
        self.title.push_str(title);
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn add_menu(&mut self, menu: Menu, text: &str) {
        if let Some(m) = &mut self.menu {
            m.add(menu, Caption::new(text, true));
        }
    }
    fn center_to_screen(&mut self) {
        let screen_size = RuntimeManager::get().get_terminal_size();
        let win_size = self.get_size();
        let x = (screen_size.width as i32 - win_size.width as i32) / 2;
        let y = (screen_size.height as i32 - win_size.height as i32) / 2;
        self.set_position(x, y);
    }
    fn resize_window_with(&mut self, add_to_width: i32, add_to_height: i32) {
        let size = self.get_size();
        let new_width = ((size.width as i32) + add_to_width).clamp(0, 0xFFFF);
        let new_height = ((size.height as i32) + add_to_height).clamp(0, 0xFFFF);
        self.set_size(new_width as u16, new_height as u16);
    }
    fn move_window_pos_to(&mut self, add_x: i32, add_y: i32, keep_in_desktop_bounderies: bool) {
        let size = self.get_size();
        let screen_size = RuntimeManager::get().get_terminal_size();
        let mut pos = self.get_position();
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
        if self.maximized == false {
            self.old_rect = Rect::with_point_and_size(self.get_position(), self.get_size());
            let desktop_rect = RuntimeManager::get().get_desktop_rect();
            self.set_position(desktop_rect.get_left(), desktop_rect.get_top());
            self.set_size(
                desktop_rect.get_width() as u16,
                desktop_rect.get_height() as u16,
            );
            self.maximized = true;
        } else {
            let l = self.old_rect.get_left();
            let t = self.old_rect.get_top();
            self.set_position(l, t);
            let w = self.old_rect.get_width() as u16;
            let h = self.old_rect.get_height() as u16;
            self.set_size(w, h);
            self.maximized = false;
        }
    }

    fn find_next_control(
        handle: Handle,
        forward: bool,
        start_from_current: bool,
    ) -> Option<Handle> {
        let rm = RuntimeManager::get();
        if let Some(control) = rm.get_controls().get(handle) {
            let base = control.get_base();
            // if I have any child --> check to see if I can move there
            if base.children.len() > 0 {
                let mut idx = if start_from_current {
                    base.focused_child_index
                } else {
                    VectorIndex::Invalid
                };
                let count = base.children.len();
                loop {
                    if forward {
                        idx.add(1, count, Strategy::RotateWithInvalidState);
                    } else {
                        idx.sub(1, count, Strategy::RotateWithInvalidState);
                    }
                    if idx.in_range(count) {
                        let child_handle = base.children[idx.index()];
                        if let Some(child) = rm.get_controls().get(child_handle) {
                            if child.get_base().can_receive_input() {
                                return Window::find_next_control(child_handle, forward, false);
                            }
                        }
                    } else {
                        if start_from_current == false {
                            break;
                        }
                    }
                }
                None
            } else {
                // no childrem --> see if I am a good fit
                if base.can_receive_input() {
                    return Some(handle);
                }
                return None;
            }
        } else {
            return None; // invalid handle
        }

        /*

                Control* FindNextControl(Control* parent, bool forward, bool startFromCurrentOne, bool rootLevel, bool noSteps)
        {
            if (parent == nullptr)
                return nullptr;
            CREATE_CONTROL_CONTEXT(parent, Members, nullptr);
            // daca am copii
            if (Members->ControlsCount != 0)
            {
                int start, end;
                if (startFromCurrentOne)
                    start = Members->CurrentControlIndex;
                else
                    start = -1;
                if (start < 0)
                {
                    if (forward)
                        start = 0;
                    else
                        start = Members->ControlsCount - 1;
                }
                // calculez si end
                if (forward)
                    end = Members->ControlsCount;
                else
                    end = -1;
                // sanity check
                if (((forward) && (start >= end)) || ((!forward) && (start <= end)))
                    return nullptr;
                // ma plimb intre elemente
                bool firstElement = true;
                while (true)
                {
                    Control* copil = Members->Controls[start];
                    if ((copil != nullptr) && (copil->Context != nullptr))
                    {
                        ControlContext* cMembers = (ControlContext*) copil->Context;
                        // am un element posibil ok
                        if (((cMembers->Flags & (GATTR_VISIBLE | GATTR_ENABLE)) == (GATTR_VISIBLE | GATTR_ENABLE)))
                        {
                            Control* res = FindNextControl(
                                  copil, forward, startFromCurrentOne & firstElement, false, noSteps & firstElement);
                            if (res != nullptr)
                                return res;
                        }
                    }

                    if (forward)
                        start++;
                    else
                        start--;
                    noSteps = false;
                    if (start == end)
                    {
                        if ((!rootLevel) || (startFromCurrentOne == false))
                            return nullptr;
                        // am ajuns la finalul listei si nu am gasit sau am parcurs toata lista
                        // daca nu - parcurg si restul listei
                        if (forward)
                        {
                            start = 0;
                            end   = Members->CurrentControlIndex + 1;
                        }
                        else
                        {
                            start = Members->ControlsCount - 1;
                            end   = Members->CurrentControlIndex + 1;
                        }
                        // sanity check
                        if (((forward) && (start >= end)) || ((!forward) && (start <= end)))
                            return nullptr;
                        rootLevel    = false;
                        firstElement = false;
                    }
                    firstElement = false;
                }
            }
            // daca nu am copii
            if (((Members->Flags & GATTR_TABSTOP) != 0) && (noSteps == false))
                return parent;
            return nullptr;


                 */
    }

    pub fn set_tag(&mut self, name: &str) {
        self.decorators.set_tag(name);
        self.decorators.update_positions(self.get_size());
    }
    pub fn get_tag(&self) -> Option<&str> {
        self.decorators.get_tag()
    }
    pub fn clear_tag(&mut self) {
        self.decorators.set_tag("");
        self.decorators.update_positions(self.get_size());
    }

    fn on_mouse_over(&mut self, x: i32, y: i32) -> EventProcessStatus {
        if let Some(menu) = self.menu.as_mut() {
            let result = menu.on_mouse_move(x, y);
            if result.is_processed_or_update() {
                self.hide_tooltip();
                return result;
            }
        }
        if let Some((index, decorator)) = self.decorators.get_from_position(x, y) {
            let cx = decorator.center_x();
            let y = decorator.get_y();
            let tooltip = decorator.get_tooltip();
            if tooltip.is_empty() {
                self.hide_tooltip();
            } else {
                self.show_tooltip_on_point(tooltip, cx, y);
            }
            self.decorators.set_current(VectorIndex::with_value(index));
            return EventProcessStatus::Processed;
        }
        // if I reach this point - tool tip should not be shown and there is no win button selected
        self.hide_tooltip();
        let cidx = self.decorators.get_current();
        if !cidx.is_valid() {
            return EventProcessStatus::Ignored;
        }
        self.decorators.set_current(VectorIndex::Invalid);
        return EventProcessStatus::Processed;
    }

    fn on_mouse_leave(&mut self) -> EventProcessStatus {
        let cidx = self.decorators.get_current();
        self.decorators.set_current_item_pressed(false);
        if !cidx.is_valid() {
            return EventProcessStatus::Ignored;
        }
        self.decorators.set_current(VectorIndex::Invalid);
        self.hide_tooltip();
        return EventProcessStatus::Processed;
    }

    fn on_mouse_pressed(&mut self, x: i32, y: i32) -> EventProcessStatus {
        self.decorators.set_current_item_pressed(false);
        self.drag_status = DragStatus::None;
        self.resize_move_mode = false;

        if let Some(menubar) = self.menu.as_mut() {
            return menubar.on_mouse_pressed(x, y);
        }

        if let Some(index) = self.decorators.get_index_from_position(x, y) {
            self.decorators.set_current(VectorIndex::with_value(index));
            self.decorators.set_current_item_pressed(true);
            let decorator = self.decorators.get(index).unwrap();
            if decorator.get_type() == DecoratorType::WindowResize {
                self.drag_status = DragStatus::Resize;
            }
            return EventProcessStatus::Processed;
        }
        self.decorators.set_current(VectorIndex::Invalid);
        self.hide_tooltip();

        if !self.flags.contains(WindowFlags::FixedPosition) {
            self.drag_status = DragStatus::Move;
            self.drag_start_point.x = x;
            self.drag_start_point.y = y;
        }
        return EventProcessStatus::Processed;
    }
    fn on_mouse_drag(&mut self, x: i32, y: i32) -> EventProcessStatus {
        self.resize_move_mode = false;
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

    fn on_mouse_release(&mut self) -> EventProcessStatus {
        self.decorators.set_current_item_pressed(false);
        self.resize_move_mode = false;
        if self.drag_status != DragStatus::None {
            self.drag_status = DragStatus::None;
        } else {
            let cdec = self.decorators.get_current();
            if cdec.is_valid() {
                if self.on_click_on_decorator(cdec.index()) {
                    return EventProcessStatus::Processed;
                }
            }
        }
        return EventProcessStatus::Processed;
    }
    fn on_click_on_decorator(&mut self, index: usize) -> bool {
        let dec = self.decorators.get(index).unwrap();
        let btype = dec.get_type();
        let id = dec.get_id();
        match btype {
            DecoratorType::CloseButton => {
                self.raise_event(Event::WindowClose);
                return true;
            }
            DecoratorType::MaximizeRestoreButton => {
                self.maximize_restore();
                return true;
            }
            DecoratorType::Button => {
                // RaiseEvent(Event::Command, b.ID);
                return true;
            }
            DecoratorType::SingleChoice => {
                self.decorators.check_singlechoice(index);
                // RaiseEvent(Event::Command, b.ID);
                return true;
            }
            DecoratorType::CheckBox => {
                let d = self.decorators.get_mut(index).unwrap();
                d.set_checked(!d.is_checked());
                // RaiseEvent(Event::Command, b.ID);
                return true;
            }
            _ => {}
        }
        false     
    }
}
impl OnPaint for Window {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let color_window = match () {
            _ if !self.has_focus() => theme.window.inactive,
            _ if self.flags.contains(WindowFlags::WarningWindow) => theme.window.warning,
            _ if self.flags.contains(WindowFlags::ErrorWindow) => theme.window.error,
            _ if self.flags.contains(WindowFlags::NotifyWindow) => theme.window.info,
            _ => theme.window.normal,
        };
        // set some colors
        let color_title: CharAttribute;
        let color_border: CharAttribute;
        let line_type: LineType;

        // initialization
        if self.has_focus() {
            color_title = theme.text.focused;
            color_border = match self.drag_status {
                DragStatus::None => theme.border.focused,
                _ => theme.border.pressed_or_selectd,
            };
            line_type = match self.drag_status {
                DragStatus::None => LineType::Double,
                _ => LineType::Single,
            };
        } else {
            color_title = theme.text.normal;
            color_border = theme.border.normal;
            line_type = LineType::Single;
        }

        let sz = self.get_size();
        surface.clear(Character::with_attributes(' ', color_window));
        surface.draw_rect(
            Rect::with_size(0, 0, sz.width as u16, sz.height as u16),
            line_type,
            color_border,
        );

        // paint decorators
        self.decorators
            .paint(surface, theme, self.has_focus(), self.maximized);

        // paint title
        if self.title_max_width >= 2 {
            let mut format = TextFormat::single_line(
                self.title_left_margin + ((self.title_max_width as i32) / 2),
                0,
                color_title,
                TextAlignament::Center,
            );
            format.width = Some(self.title_max_width);
            surface.write_text(self.title.as_str(), &format);
        }
        // paint the menu
        if self.menu.is_some() {
            self.menu.as_ref().unwrap().paint(surface, theme);
        }
    }
}

impl OnResize for Window {
    fn on_resize(&mut self, _: Size, new_size: Size) {
        // recompute decorator based on the new size
        let (title_pos, title_width) = self.decorators.update_positions(new_size);
        self.title_left_margin = title_pos;
        self.title_max_width = title_width;
        // recompute menu based on new size
        if let Some(menu) = &mut self.menu {
            menu.set_position(0, 0, new_size.width);
        }
    }
}

impl OnKeyPressed for Window {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        if self.resize_move_mode {
            match key.get_compact_code() {
                key!("Escape") | key!("Enter") | key!("Space") | key!("Tab") => {
                    self.resize_move_mode = false;
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
            match key.get_compact_code() {
                key!("Tab") => {
                    if let Some(my_handle) = self.handle {
                        if let Some(new_child) = Window::find_next_control(my_handle, true, true) {
                            RuntimeManager::get().request_focus_for_control(new_child);
                        }
                    }
                    return EventProcessStatus::Processed;
                }
                key!("Shift+Tab") => {
                    if let Some(my_handle) = self.handle {
                        if let Some(new_child) = Window::find_next_control(my_handle, false, true) {
                            RuntimeManager::get().request_focus_for_control(new_child);
                        }
                    }
                    return EventProcessStatus::Processed;
                }
                _ => {}
            }
        }
        EventProcessStatus::Ignored
    }
    /*
    Control* tmp;
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);

    if (Members->ResizeMoveMode)
    {
        switch (KeyCode)
        {

        }
    }
    else
    {
        switch (KeyCode)
        {
        case Key::Ctrl | Key::Alt | Key::M:
        case Key::Ctrl | Key::Alt | Key::R:
            Members->ResizeMoveMode = true;
            return true;

        case Key::Tab | Key::Shift:
            tmp = FindNextControl(this, false, true, true, true);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Tab:
            tmp = FindNextControl(this, true, true, true, true);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Left:
        case Key::Left | Key::Ctrl:
        case Key::Left | Key::Alt:
            tmp = FindClosestControl(this, MoveDirection::ToLeft);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Right:
        case Key::Right | Key::Ctrl:
        case Key::Right | Key::Alt:
            tmp = FindClosestControl(this, MoveDirection::ToRight);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Up:
        case Key::Up | Key::Ctrl:
        case Key::Up | Key::Alt:
            tmp = FindClosestControl(this, MoveDirection::ToTop);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Down:
        case Key::Down | Key::Ctrl:
        case Key::Down | Key::Alt:
            tmp = FindClosestControl(this, MoveDirection::ToBottom);
            if (tmp != nullptr)
                tmp->SetFocus();
            return true;
        case Key::Escape:
            if (!(Members->Flags && WindowFlags::NoCloseButton))
            {
                RaiseEvent(Event::WindowClose);
                return true;
            }
            return false;
        case Key::Enter:
            if (Members->Flags && WindowFlags::ProcessReturn)
            {
                RaiseEvent(Event::WindowAccept);
                return true;
            }
            return false;
        }
        // first we check menu hot keys
        if (Members->menu)
        {
            if (Members->menu->OnKeyEvent(KeyCode))
                return true;
        }
        // check cntrols hot keys
        if ((((uint32) KeyCode) & (uint32) (Key::Shift | Key::Alt | Key::Ctrl)) == ((uint32) Key::Alt))
        {
            if (ProcessHotKey(this, KeyCode))
                return true;
            auto* b = Members->ControlBar.Items;
            auto* e = b + Members->ControlBar.Count;
            while (b < e)
            {
                if (b->HotKey == KeyCode)
                {
                    if (ProcessControlBarItem((uint32) (b - Members->ControlBar.Items)))
                        return true;
                }
                b++;
            }
        }
    }
    // key was not prcessed, pass it to my parent
    return false;


     */
}

impl OnMouseEvent for Window {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        match event {
            MouseEvent::Enter => return EventProcessStatus::Ignored,
            MouseEvent::Leave => return self.on_mouse_leave(),
            MouseEvent::Over(point) => return self.on_mouse_over(point.x, point.y),
            MouseEvent::Pressed(event) => return self.on_mouse_pressed(event.x, event.y),
            MouseEvent::Released(_) => return self.on_mouse_release(),
            MouseEvent::DoubleClick(_) => EventProcessStatus::Ignored,
            MouseEvent::Drag(event) => return self.on_mouse_drag(event.x, event.y),
            MouseEvent::Wheel(_) => EventProcessStatus::Ignored,
        }
    }
}

/*

#include "ControlContext.hpp"

namespace AppCUI
{
constexpr uint8 NO_CONTROLBAR_ITEM   = 0xFF;
constexpr uint32 MAX_TAG_CHARS       = 8U;
constexpr int32 MOVE_TO_LOWER_MARGIN = -1000000;
constexpr int32 MOVE_TO_UPPER_MARGIN = 1000000;
constexpr uint32 INFINITE_DISTANCE   = 0xFFFFFFFF;

const static CharacterBuffer tempReferenceChBuf;

struct WindowControlBarLayoutData
{
    int Left, Right, Y;
    WindowBarItem* LeftGroup;
    WindowBarItem* RighGroup;
};

Control* FindNextControl(Control* parent, bool forward, bool startFromCurrentOne, bool rootLevel, bool noSteps)
{
    if (parent == nullptr)
        return nullptr;
    CREATE_CONTROL_CONTEXT(parent, Members, nullptr);
    // daca am copii
    if (Members->ControlsCount != 0)
    {
        int start, end;
        if (startFromCurrentOne)
            start = Members->CurrentControlIndex;
        else
            start = -1;
        if (start < 0)
        {
            if (forward)
                start = 0;
            else
                start = Members->ControlsCount - 1;
        }
        // calculez si end
        if (forward)
            end = Members->ControlsCount;
        else
            end = -1;
        // sanity check
        if (((forward) && (start >= end)) || ((!forward) && (start <= end)))
            return nullptr;
        // ma plimb intre elemente
        bool firstElement = true;
        while (true)
        {
            Control* copil = Members->Controls[start];
            if ((copil != nullptr) && (copil->Context != nullptr))
            {
                ControlContext* cMembers = (ControlContext*) copil->Context;
                // am un element posibil ok
                if (((cMembers->Flags & (GATTR_VISIBLE | GATTR_ENABLE)) == (GATTR_VISIBLE | GATTR_ENABLE)))
                {
                    Control* res = FindNextControl(
                          copil, forward, startFromCurrentOne & firstElement, false, noSteps & firstElement);
                    if (res != nullptr)
                        return res;
                }
            }

            if (forward)
                start++;
            else
                start--;
            noSteps = false;
            if (start == end)
            {
                if ((!rootLevel) || (startFromCurrentOne == false))
                    return nullptr;
                // am ajuns la finalul listei si nu am gasit sau am parcurs toata lista
                // daca nu - parcurg si restul listei
                if (forward)
                {
                    start = 0;
                    end   = Members->CurrentControlIndex + 1;
                }
                else
                {
                    start = Members->ControlsCount - 1;
                    end   = Members->CurrentControlIndex + 1;
                }
                // sanity check
                if (((forward) && (start >= end)) || ((!forward) && (start <= end)))
                    return nullptr;
                rootLevel    = false;
                firstElement = false;
            }
            firstElement = false;
        }
    }
    // daca nu am copii
    if (((Members->Flags & GATTR_TABSTOP) != 0) && (noSteps == false))
        return parent;
    return nullptr;
}
uint32 PointToPointDistance(const Rect& originRect, const Rect& objectRect, MoveDirection dir)
{
    // done
}
Control* FindClosestControl(Control* parent, MoveDirection dir, const Rect& origin)
{
    if (parent == nullptr)
        return nullptr;
    CREATE_CONTROL_CONTEXT(parent, Members, nullptr);
    // check my children and find the best fit
    Control* result = nullptr;
    uint32 best     = INFINITE_DISTANCE;
    for (auto idx = 0U; idx < Members->ControlsCount; idx++)
    {
        auto child = Members->Controls[idx];
        auto flags = ((ControlContext*) child->Context)->Flags;
        if ((flags & (GATTR_ENABLE | GATTR_VISIBLE)) != (GATTR_ENABLE | GATTR_VISIBLE))
            continue;
        if (child->GetChildrenCount() > 0)
        {
            // check its children
            child = FindClosestControl(child, dir, origin);
            if (child == nullptr)
                continue;
        }
        else
        {
            // if TABSTOP is not set ==> skip it (e.g. a label)
            if ((flags & GATTR_TABSTOP) == 0)
                continue;
        }

        auto r = child->GetAbsoluteRectangle();
        auto d = PointToPointDistance(origin, r, dir);
        // LOG_INFO(
        //       "%s => (%d,%d  %dx%d), D=%d",
        //       ((std::string) child->GetText()).c_str(),
        //       r.GetLeft(),
        //       r.GetTop(),
        //       r.GetWidth(),
        //       r.GetHeight(),
        //       d);
        if (d < best)
        {
            best   = d;
            result = child;
        }
    }
    return result;
}
Control* FindClosestControl(Control* parent, MoveDirection dir)
{
    if (parent == nullptr)
        return nullptr;
    CREATE_CONTROL_CONTEXT(parent, Members, nullptr);
    // first search current control
    auto child = parent;
    while (child != nullptr)
    {
        auto ctx = ((ControlContext*) (child->Context));
        if (ctx->CurrentControlIndex >= ctx->ControlsCount)
            break;
        auto prnt  = child;
        child      = ctx->Controls[ctx->CurrentControlIndex];
        auto flags = ((ControlContext*) child->Context)->Flags;
        if ((flags & (GATTR_ENABLE | GATTR_VISIBLE)) != (GATTR_ENABLE | GATTR_VISIBLE))
        {
            // current control is unreacheable --> move to parent and stop
            child = prnt;
            break;
        }
    }
    // if child is nullptr --> then we have an error (return)
    CHECK(child, nullptr, "");
    // now we have the current control --> create a center point
    Rect currenChild = child->GetAbsoluteRectangle();
    // Log info
    // LOG_INFO(
    //      "Current control (X=%d,Y=%d, Size=%dx%d)",
    //      currenChild.GetLeft(),
    //      currenChild.GetTop(),
    //      currenChild.GetWidth(),
    //      currenChild.GetHeight());

    // now we need to search the first child that is closest to childPos
    return FindClosestControl(parent, dir, currenChild);
}
bool ProcessHotKey(Control* ctrl, Input::Key KeyCode)
{
    if (ctrl == nullptr)
        return false;
    CREATE_CONTROL_CONTEXT(ctrl, Members, false);
    if (((Members->Flags & (GATTR_VISIBLE | GATTR_ENABLE)) != (GATTR_VISIBLE | GATTR_ENABLE)))
        return false;
    for (uint32 tr = 0; tr < Members->ControlsCount; tr++)
    {
        if (ProcessHotKey(Members->Controls[tr], KeyCode))
            return true;
    }
    if (ctrl->GetHotKey() == KeyCode)
    {
        ctrl->SetFocus();
        ctrl->OnHotKey();
        return true;
    }
    return false;
}

void WindowRadioButtonClicked(WindowBarItem* start, WindowBarItem* end, WindowBarItem* current)
{
    // done
}
void MoveWindowPosTo(Window* win, int addX, int addY, bool keepInDesktopBounderies)
{
    // done
}
void ResizeWindow(Window* win, int addToWidth, int addToHeight)
{
    // done
}
//=========================================================================================================================================================
ItemHandle Controls::WindowControlsBar::AddCommandItem(const ConstString& name, int ID, const ConstString& toolTip)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, InvalidItemHandle);
    CHECK(Members->ControlBar.Count < MAX_WINDOWBAR_ITEMS,
          InvalidItemHandle,
          "Max number of items in a control bar was exceeded !");
    auto* b = &Members->ControlBar.Items[Members->ControlBar.Count];
    CHECK(b->Init(WindowBarItemType::Button, this->Layout, name, toolTip),
          InvalidItemHandle,
          "Fail to initialize item !");
    b->ID = ID;
    Members->ControlBar.Count++;
    UpdateWindowsButtonsPoz(Members);
    return Members->ControlBar.Count - 1;
}
ItemHandle Controls::WindowControlsBar::AddSingleChoiceItem(
      const ConstString& name, int ID, bool checked, const ConstString& toolTip)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, InvalidItemHandle);
    CHECK(Members->ControlBar.Count < MAX_WINDOWBAR_ITEMS,
          InvalidItemHandle,
          "Max number of items in a control bar was exceeded !");
    auto* b = &Members->ControlBar.Items[Members->ControlBar.Count];
    CHECK(b->Init(WindowBarItemType::SingleChoice, this->Layout, name, toolTip),
          InvalidItemHandle,
          "Fail to initialize item !");
    b->ID = ID;
    Members->ControlBar.Count++;
    if (checked)
        WindowRadioButtonClicked(
              Members->ControlBar.Items,
              Members->ControlBar.Items + Members->ControlBar.Count,
              Members->ControlBar.Items + Members->ControlBar.Count - 1);
    UpdateWindowsButtonsPoz(Members);
    return Members->ControlBar.Count - 1;
}
ItemHandle Controls::WindowControlsBar::AddCheckItem(
      const ConstString& name, int ID, bool checked, const ConstString& toolTip)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, InvalidItemHandle);
    CHECK(Members->ControlBar.Count < MAX_WINDOWBAR_ITEMS,
          InvalidItemHandle,
          "Max number of items in a control bar was exceeded !");
    auto* b = &Members->ControlBar.Items[Members->ControlBar.Count];
    CHECK(b->Init(WindowBarItemType::CheckBox, this->Layout, name, toolTip),
          InvalidItemHandle,
          "Fail to initialize item !");
    b->ID = ID;
    Members->ControlBar.Count++;
    if (checked)
        b->SetFlag(WindowBarItemFlags::Checked);
    else
        b->RemoveFlag(WindowBarItemFlags::Checked);
    UpdateWindowsButtonsPoz(Members);
    return Members->ControlBar.Count - 1;
}
ItemHandle Controls::WindowControlsBar::AddTextItem(const ConstString& caption, const ConstString& toolTip)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, InvalidItemHandle);
    CHECK(Members->ControlBar.Count < MAX_WINDOWBAR_ITEMS,
          InvalidItemHandle,
          "Max number of items in a control bar was exceeded !");
    auto* b = &Members->ControlBar.Items[Members->ControlBar.Count];
    CHECK(b->Init(WindowBarItemType::Text, this->Layout, caption, toolTip),
          InvalidItemHandle,
          "Fail to initialize item !");

    Members->ControlBar.Count++;
    UpdateWindowsButtonsPoz(Members);
    return Members->ControlBar.Count - 1;
}
WindowBarItem* GetWindowControlsBarItem(void* Context, ItemHandle itemHandle)
{
    WindowControlContext* Members = (WindowControlContext*) Context;
    CHECK(Members, nullptr, "");
    uint32 id = (uint32) itemHandle;
    CHECK(id < Members->ControlBar.Count, nullptr, "Invalid item index (%d/%d)", id, Members->ControlBar.Count);
    auto* b = Members->ControlBar.Items + id;
    CHECK((b->Type == WindowBarItemType::Button) || (b->Type == WindowBarItemType::CheckBox) ||
                (b->Type == WindowBarItemType::SingleChoice) || (b->Type == WindowBarItemType::Text),
          nullptr,
          "");
    return b;
}
bool Controls::WindowControlsBar::SetItemText(ItemHandle itemHandle, const ConstString& caption)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");

    CHECK(b->Text.SetWithHotKey(caption, b->HotKeyOffset, b->HotKey, Key::Alt), false, "");
    b->Size = b->Text.Len();
    if (b->Type == WindowBarItemType::CheckBox)
        b->Size += 2;
    UpdateWindowsButtonsPoz((WindowControlContext*) Context);
    return true;
}
bool Controls::WindowControlsBar::SetItemTextWithHotKey(
      ItemHandle itemHandle, const ConstString& caption, uint32 hotKeyTextOffset)
{
    CHECK(SetItemText(itemHandle, caption), false, "");
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    b->HotKeyOffset = CharacterBuffer::INVALID_HOTKEY_OFFSET;
    b->HotKey       = Key::None;

    ConstStringObject txt(caption);
    char16 ch = 0;
    if (hotKeyTextOffset < txt.Length)
    {
        switch (txt.Encoding)
        {
        case StringEncoding::Ascii:
            ch = (((const char*) txt.Data)[hotKeyTextOffset]);
            break;
        case StringEncoding::Unicode16:
            ch = (((const char16*) txt.Data)[hotKeyTextOffset]);
            break;
        case StringEncoding::CharacterBuffer:
            ch = (((const Character*) txt.Data)[hotKeyTextOffset].Code);
            break;
        case StringEncoding::UTF8:
            ch = (((const uint8*) txt.Data)[hotKeyTextOffset]);
            break;
        }
        if (ch != 0)
        {
            b->HotKey = Utils::KeyUtils::CreateHotKey(ch, Key::Alt);
            if (b->HotKey != Key::None)
                b->HotKeyOffset = hotKeyTextOffset;
        }
    }
    return true;
}
bool Controls::WindowControlsBar::SetItemToolTip(ItemHandle itemHandle, const ConstString& caption)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    CHECK(b->ToolTipText.Set(caption), false, "");
    return true;
}
bool Controls::WindowControlsBar::IsItemChecked(ItemHandle itemHandle)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    return b->IsChecked();
}
bool Controls::WindowControlsBar::SetItemCheck(ItemHandle itemHandle, bool value)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    if (b->Type == WindowBarItemType::CheckBox)
    {
        if (value)
            b->SetFlag(WindowBarItemFlags::Checked);
        else
            b->RemoveFlag(WindowBarItemFlags::Checked);
        return true;
    }
    if (b->Type == WindowBarItemType::SingleChoice)
    {
        CHECK(value, false, "For radio buttom only 'true' can be used as a value");
        WindowControlContext* Members = (WindowControlContext*) Context;
        WindowRadioButtonClicked(Members->ControlBar.Items, Members->ControlBar.Items + Members->ControlBar.Count, b);
        return true;
    }
    RETURNERROR(false, "This method can only be applied on Check and Radio items");
}
bool Controls::WindowControlsBar::IsItemVisible(ItemHandle itemHandle)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    return !b->IsHidden();
}
bool Controls::WindowControlsBar::IsItemShown(ItemHandle itemHandle)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    return b->IsVisible() && (!b->IsHidden());
}
bool Controls::WindowControlsBar::SetItemVisible(ItemHandle itemHandle, bool value)
{
    auto b = GetWindowControlsBarItem(this->Context, itemHandle);
    CHECK(b, false, "");
    if ((b->Type == WindowBarItemType::CheckBox) || (b->Type == WindowBarItemType::Button) ||
        (b->Type == WindowBarItemType::SingleChoice) || (b->Type == WindowBarItemType::Text))
    {
        // change visibility
        if (value)
            b->RemoveFlag(WindowBarItemFlags::Hidden);
        else
            b->SetFlag(WindowBarItemFlags::Hidden);
        UpdateWindowsButtonsPoz((WindowControlContext*) this->Context);
        return true;
    }
    RETURNERROR(false, "This method can only be applied on Check and Radio items");
}
//=========================================================================================================================================================
bool WindowBarItem::Init(WindowBarItemType type, WindowControlsBarLayout layout, uint8 size, string_view toolTipText)
{
    this->Type         = type;
    this->Layout       = layout;
    this->Size         = size;
    this->X            = 0;
    this->Y            = 0;
    this->Flags        = WindowBarItemFlags::None;
    this->ID           = -1;
    this->HotKey       = Key::None;
    this->HotKeyOffset = CharacterBuffer::INVALID_HOTKEY_OFFSET;
    if (!toolTipText.empty())
    {
        CHECK(this->ToolTipText.Set(toolTipText), false, "");
    }
    return true;
}
bool WindowBarItem::Init(
      WindowBarItemType type, WindowControlsBarLayout layout, const ConstString& name, const ConstString& toolTip)
{
    this->Type         = type;
    this->Layout       = layout;
    this->Size         = 0;
    this->X            = 0;
    this->Y            = 0;
    this->Flags        = WindowBarItemFlags::None;
    this->ID           = -1;
    this->HotKey       = Key::None;
    this->HotKeyOffset = CharacterBuffer::INVALID_HOTKEY_OFFSET;
    // name
    ConstStringObject objName(name);
    CHECK(objName.Length > 0, false, "Expecting a valid item name (non-empty)");
    CHECK(this->Text.SetWithHotKey(name, this->HotKeyOffset, this->HotKey, Key::Alt), false, "Fail to create name !");
    this->Size = this->Text.Len();
    if (type == WindowBarItemType::CheckBox)
        this->Size += 2; // for the checkmark
    // tool tip
    ConstStringObject objToolTip(toolTip);
    if (objToolTip.Length > 0)
    {
        CHECK(this->ToolTipText.Set(toolTip), false, "");
    }
    // all good
    return true;
}
//=========================================================================================================================================================
Window::~Window()
{
    DELETE_CONTROL_CONTEXT(WindowControlContext);
}
Window::Window(const ConstString& caption, string_view layout, WindowFlags Flags)
    : Control(new WindowControlContext(), caption, layout, false)
{
    // done
}

void Window::Paint(Graphics::Renderer& renderer)
{
    // done
}
bool Window::MaximizeRestore()
{
    // done
}
bool Window::CenterScreen()
{
//
}
void Window::OnMousePressed(int x, int y, Input::MouseButton button)
{
    // done
}
bool Window::ProcessControlBarItem(uint32 index)
{
    // done
}
void Window::OnMouseReleased(int, int, Input::MouseButton)
{
    // done
}
bool Window::OnMouseDrag(int x, int y, Input::MouseButton)
{
    // done
}
bool Window::OnMouseOver(int x, int y)
{
    // done
}
bool Window::OnMouseLeave()
{
    // done
}
bool Window::OnBeforeResize(int newWidth, int newHeight)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    if ((Members->Flags & WindowFlags::Sizeable) == WindowFlags::None)
        return false;
    return (newWidth >= Members->Layout.MinWidth) && (newWidth <= Members->Layout.MaxWidth) &&
           (newHeight >= Members->Layout.MinHeight) && (newHeight <= Members->Layout.MaxHeight);
}
void Window::OnAfterResize(int, int)
{
    WindowControlContext* Members = (WindowControlContext*) this->Context;
    if (Members)
    {
        UpdateWindowsButtonsPoz(Members);
    }
}
void Window::RemoveMe()
{
    auto app = Application::GetApplication();
    if (!app)
        return;
    // check if I am part of the modal stack
    for (auto i = 0U; i < app->ModalControlsCount; i++)
        if (app->ModalControlsStack[i] == this)
            return;
    if (!app->AppDesktop)
        return;
    // all good -> I am a top level window --> remove me
    app->AppDesktop->RemoveControl(this);
    app->CheckIfAppShouldClose();
}
bool Window::OnEvent(Reference<Control>, Event eventType, int)
{
    if ((eventType == Event::WindowClose) || (eventType == Event::WindowAccept))
    {
        // check if current win is a modal dialog
        auto app = Application::GetApplication();
        if ((app->ModalControlsCount > 0) && (app->ModalControlsStack[app->ModalControlsCount - 1] == this))
        {
            if (eventType == Event::WindowClose)
                return Exit(Dialogs::Result::Cancel);
            else
                return Exit(Dialogs::Result::Ok);
        }
        else
        {
            RemoveMe();
            return true;
        }
    }
    return false;
}
bool Window::OnKeyEvent(Input::Key KeyCode, char16)
{
    // done
}
void Window::OnHotKeyChanged()
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, );
    // find hotkey win button
    WindowBarItem* btnHotKey = nullptr;
    for (uint32 tr = 0; tr < Members->ControlBar.Count; tr++)
        if (Members->ControlBar.Items[tr].Type == WindowBarItemType::HotKeY)
        {
            btnHotKey = &Members->ControlBar.Items[tr];
            break;
        }
    // sanity check (in reality the pointer should always be valid)
    if (!btnHotKey)
        return;

    if (Members->HotKey == Key::None)
    {
        btnHotKey->SetFlag(WindowBarItemFlags::Hidden);
    }
    else
    {
        btnHotKey->Size = (int) (KeyUtils::GetKeyName(Members->HotKey).size() + 2);
        btnHotKey->ToolTipText.Set("Press Alt+");
        btnHotKey->ToolTipText.Add(KeyUtils::GetKeyName(Members->HotKey));
        btnHotKey->ToolTipText.Add(" to activate this window");
        btnHotKey->RemoveFlag(WindowBarItemFlags::Hidden);
    }
    UpdateWindowsButtonsPoz(Members);
}
void Window::SetTag(const ConstString& name, const ConstString& toolTipText)
{
    // done
}
const Graphics::CharacterBuffer& Window::GetTag()
{
    // done
}

bool Window::Exit(Dialogs::Result dialogResult)
{
    //CHECK(dialogResult != Dialogs::Result::None, false, "Dialog result code must not be None !");
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    Members->DialogResult                     = dialogResult;
    Members->ResizeMoveMode                   = false;
    Application::GetApplication()->loopStatus = Internal::LoopStatus::StopCurrent;
    return true;
}
Dialogs::Result Window::Show()
{
    CHECK(GetParent() == nullptr,
          Dialogs::Result::None,
          "Unable to run modal window if it is attached to another control !");
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, Dialogs::Result::None);
    CHECK(Members->RecomputeLayout(nullptr), Dialogs::Result::None, "Fail to recompute layout !");
    this->RecomputeLayout();
    CHECK(Application::GetApplication()->ExecuteEventLoop(this, true), Dialogs::Result::None, "Modal execution failed !");

    return Members->DialogResult;
}
Dialogs::Result Window::GetDialogResult()
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, Dialogs::Result::None);
    return Members->DialogResult;
}
bool Window::IsWindowInResizeMode()
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    return (Members->dragStatus == WindowDragStatus::Resize);
}
bool Window::EnableResizeMode()
{
    CHECK(this->HasFocus(), false, "To enable resize mode a window must be focused !");
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, false);
    Members->ResizeMoveMode = true;
    return true;
}
Reference<Menu> Window::AddMenu(const ConstString& name)
{
    CREATE_TYPECONTROL_CONTEXT(WindowControlContext, Members, nullptr);
    CHECK(Members->menu, nullptr, "Application was not initialized with Menu option set up !");
    ItemHandle itm         = Members->menu->AddMenu(name);
    Controls::Menu* result = Members->menu->GetMenu(itm);
    CHECK(result, nullptr, "Fail to create menu !");
    return Reference<Menu>(result);
}
WindowControlsBar Window::GetControlBar(WindowControlsBarLayout layout)
{
    if ((this->Context) && (layout != WindowControlsBarLayout::None))
        return WindowControlsBar(this->Context, layout);
    else
        return WindowControlsBar(nullptr, WindowControlsBarLayout::None);
}
} // namespace AppCUI


 */
