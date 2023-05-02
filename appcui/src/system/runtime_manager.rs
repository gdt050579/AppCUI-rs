use super::{
    CommandBar, ControlHandleManager, InitializationData, InitializationFlags, Theme, ToolTip, Handle, MenuHandleManager,
};
use crate::controls::control_manager::ParentLayout;
use crate::controls::events::{Control, Event, EventProcessStatus};
use crate::controls::menu::{Menu, MenuBar, MenuHandle};
use crate::controls::ControlManager;
use crate::controls::*;
use crate::graphics::{Point, Rect, Size, Surface};
use crate::input::{Key, KeyModifier, MouseButton, MouseEvent, MouseEventData};
use crate::terminals::*;
use crate::utils::{Caption, Strategy, VectorIndex};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum LoopStatus {
    Normal,
    StopApp,
    StopCurrent,
}

#[derive(Clone, Copy)]
enum MouseLockedObject {
    None,
    Control(Handle),
    CommandBar,
    MenuBar,
}

#[derive(Copy, Clone)]
struct EmittedEvent {
    event: Event,
    sender: Handle,
}
pub(crate) struct RuntimeManager {
    theme: Theme,
    terminal: Box<dyn Terminal>,
    surface: Surface,
    controls: *mut ControlHandleManager,
    menus: *mut MenuHandleManager,
    desktop_handler: Handle,
    tooltip: ToolTip,
    commandbar: Option<CommandBar>,
    menubar: Option<MenuBar>,
    recompute_layout: bool,
    repaint: bool,
    recompute_parent_indexes: bool,
    loop_status: LoopStatus,
    request_focus: Option<Handle>,
    current_focus: Option<Handle>,
    mouse_over_control: Option<Handle>,
    focus_chain: Vec<Handle>,
    events: Vec<EmittedEvent>,
    commands: Vec<u32>,
    mouse_locked_object: MouseLockedObject,
    opened_menu: Option<MenuHandle>,
}

static mut RUNTIME_MANAGER: Option<RuntimeManager> = None;

impl RuntimeManager {
    pub(super) fn create(data: InitializationData) -> Result<(), super::Error> {
        let term = TerminalType::new(&data)?;
        let width = term.get_width();
        let height = term.get_height();
        let surface = Surface::new(width, height);
        let mut manager = RuntimeManager {
            theme: Theme::new(),
            terminal: term,
            surface: surface,
            desktop_handler: Handle::new(0),
            tooltip: ToolTip::new(),
            recompute_layout: true,
            repaint: true,
            recompute_parent_indexes: true,
            request_focus: None,
            current_focus: None,
            mouse_over_control: None,
            opened_menu: None,
            focus_chain: Vec::with_capacity(16),
            events: Vec::with_capacity(16),
            commands: Vec::with_capacity(8),
            controls: Box::into_raw(Box::new(ControlHandleManager::new())),
            menus: Box::into_raw(Box::new(MenuHandleManager::new())),
            loop_status: LoopStatus::Normal,
            mouse_locked_object: MouseLockedObject::None,
            commandbar: if data.flags.contains(InitializationFlags::CommandBar) {
                Some(CommandBar::new(width, height))
            } else {
                None
            },
            menubar: if data.flags.contains(InitializationFlags::Menu) {
                Some(MenuBar::new(width))
            } else {
                None
            },
        };
        let mut desktop = ControlManager::new(Desktop::new());
        let controls = unsafe { &mut *manager.controls };
        desktop.get_base_mut().update_focus_flag(true);
        manager.desktop_handler = controls.add(desktop);
        manager.current_focus = Some(manager.desktop_handler);
        controls
            .get(manager.desktop_handler)
            .unwrap()
            .get_base_mut()
            .handle = Some(manager.desktop_handler);
        unsafe {
            RUNTIME_MANAGER = Some(manager);
        }
        Ok(())
    }
    pub(crate) fn get() -> &'static mut RuntimeManager {
        unsafe { RUNTIME_MANAGER.as_mut().unwrap() }
    }
    pub(crate) fn get_terminal_size(&self) -> Size {
        Size {
            width: self.terminal.get_width(),
            height: self.terminal.get_height(),
        }
    }
    pub(crate) fn get_desktop_rect(&self) -> Rect {
        Rect::new(
            0,
            if self.menubar.is_some() { 1 } else { 0 },
            (self.terminal.get_width() as i32) - 1,
            if self.commandbar.is_some() {
                (self.terminal.get_height() as i32) - 2
            } else {
                (self.terminal.get_height() as i32) - 1
            },
        )
    }
    pub(crate) fn request_repaint(&mut self) {
        self.repaint = true;
    }
    pub(crate) fn request_recompute_layout(&mut self) {
        self.recompute_layout = true;
    }
    pub(crate) fn show_tooltip(&mut self, txt: &str, rect: &Rect) {
        self.tooltip.show(
            txt,
            &rect,
            self.terminal.get_width(),
            self.terminal.get_height(),
            &self.theme,
        );
    }
    pub(crate) fn hide_tooltip(&mut self) {
        self.tooltip.hide();
    }
    pub(crate) fn send_event(&mut self, event: Event, sender: Handle) {
        self.events.push(EmittedEvent { event, sender });
    }
    pub(crate) fn send_command(&mut self, command: u32) {
        self.commands.push(command);
    }
    pub(crate) fn close(&mut self) {
        self.loop_status = LoopStatus::StopApp;
    }
    pub(crate) fn request_focus_for_control(&mut self, handle: Handle) {
        self.request_focus = Some(handle);
    }
    pub(crate) fn add<T>(&mut self, obj: T) -> ControlHandle<T>
    where
        T: Control + 'static,
    {
        let controls = unsafe { &mut *self.controls };
        controls.get_desktop().get_base_mut().add_child(obj)
    }
    #[inline(always)]
    pub(crate) fn get_controls(&self) -> &mut ControlHandleManager {
        unsafe { &mut *self.controls }
    }
    #[inline(always)]
    pub(crate) fn get_menus(&self) -> &mut MenuHandleManager {
        unsafe { &mut *self.menus }
    }
    pub(crate) fn add_menu(&mut self, menu: Menu, caption: Caption)->Option<MenuHandle> {
        if let Some(menubar) = self.menubar.as_mut() {
            return Some(menubar.add(menu, caption));
        }
        None
    }
    pub (crate) fn get_menu(&mut self, handle: MenuHandle) -> Option<&mut Menu> {
        let menus = unsafe { &mut *self.menus };
        menus.get_mut(handle)
    }
    pub (crate) fn show_menu(&mut self, handle: MenuHandle, x: i32, y: i32, max_size: Size) {
        let menus = unsafe { &mut *self.menus };
        if let Some(menu) = menus.get_mut(handle) {
            let term_size = Size::new(self.terminal.get_width(),self.terminal.get_height());
            menu.compute_position(x, y, max_size, term_size);
            self.opened_menu = Some(handle);
        }
    }
    pub(crate) fn run(&mut self) {
        // must pe self so that after a run a second call will not be possible
        self.recompute_layout = true;
        self.repaint = true;
        while self.loop_status == LoopStatus::Normal {
            if !self.commands.is_empty() {
                self.process_commands_queue();
            }
            if !self.events.is_empty() {
                self.process_events_queue();
            }
            if self.recompute_parent_indexes {
                self.update_parent_indexes(self.desktop_handler);
                self.recompute_parent_indexes = false;
            }
            if let Some(handle) = self.request_focus {
                self.update_focus(handle);
                self.request_focus = None;
                self.repaint = true;
            }
            if self.recompute_layout {
                self.recompute_layouts();
            }
            if self.repaint | self.recompute_layout {
                self.paint();
            }
            self.recompute_layout = false;
            self.repaint = false;
            //self.debug_print(self.desktop_handler, 0);
            //return;
            let sys_event = self.terminal.get_system_event();
            match sys_event {
                SystemEvent::None => {}
                SystemEvent::AppClose => self.loop_status = LoopStatus::StopApp,
                SystemEvent::KeyPressed(event) => self.process_keypressed_event(event),
                SystemEvent::KeyModifierChanged(event) => {
                    self.process_key_modifier_changed_event(event.new_state)
                }
                SystemEvent::Resize(new_size) => self.process_terminal_resize_event(new_size),
                SystemEvent::MouseButtonDown(event) => self.process_mousebuttondown_event(event),
                SystemEvent::MouseButtonUp(event) => self.process_mousebuttonup_event(event),
                SystemEvent::MouseDoubleClick(event) => self.process_mouse_dblclick_event(event),
                SystemEvent::MouseMove(event) => self.process_mousemove_event(event),
                SystemEvent::MouseWheel(event) => self.process_mousewheel_event(event),
            }
        }
    }
    fn get_focused_control(&self) -> Handle {
        let controls = unsafe { &mut *self.controls };
        let mut parent = self.desktop_handler;
        let mut ctrl = controls.get(parent).unwrap();
        
        loop {
            let base = ctrl.get_base();
            if base.focused_child_index.in_range(base.children.len()) {
                let child_handle = base.children[base.focused_child_index.index()];
                if let Some(child) = controls.get(child_handle) {
                    if child.get_base().can_receive_input() {
                        parent = child_handle;
                        ctrl = child;
                    } else {
                        return parent;
                    }
                } else {
                    return parent;
                }
            } else {
                return parent;
            }
        }
    }
    fn process_one_event(&mut self, evnt: EmittedEvent) {
        let mut h = evnt.sender;
        let controls = unsafe { &mut *self.controls };
        while let Some(control) = controls.get(h) {
            let result = control.get_control_mut().on_event(evnt.event, evnt.sender);
            match result {
                EventProcessStatus::Processed => {
                    return;
                }
                EventProcessStatus::Ignored => {}
                EventProcessStatus::Update => {
                    self.repaint = true;
                }
                EventProcessStatus::Cancel => {
                    return;
                }
            }
            if let Some(parent) = control.get_base().parent {
                h = parent;
            } else {
                break;
            }
        }
    }
    fn process_events_queue(&mut self) {
        while let Some(evnt) = self.events.pop() {
            self.process_one_event(evnt);
        }
    }
    fn process_one_command(&mut self, handle: Handle, command: u32) {
        let mut h = handle;
        let controls = unsafe { &mut *self.controls };
        while let Some(control) = controls.get(h) {
            let result = control.get_control_mut().on_command(command);
            match result {
                EventProcessStatus::Processed => {
                    return;
                }
                EventProcessStatus::Ignored => {}
                EventProcessStatus::Update => {
                    self.repaint = true;
                }
                EventProcessStatus::Cancel => {
                    return;
                }
            }
            if let Some(parent) = control.get_base().parent {
                h = parent;
            } else {
                break;
            }
        }
    }
    fn process_commands_queue(&mut self) {
        let focused_handle = self.get_focused_control();
        while let Some(cmd) = self.commands.pop() {
            self.process_one_command(focused_handle, cmd);
        }
    }

    fn update_focus(&mut self, handle: Handle) {
        // 1. mark all controls from the path as preparing to received focus
        // we will use focuse_chain as a temporary value to hold the chain
        self.focus_chain.clear();
        let controls = unsafe { &mut *self.controls };
        let mut h = handle;
        let invalid_chain_for_focus = loop {
            if let Some(control) = controls.get(h) {
                self.focus_chain.push(h);
                if !control.get_base_mut().mark_to_receive_focus() {
                    break true;
                }
                if let Some(parent) = control.get_base().parent {
                    h = parent;
                } else {
                    break false; // all good, we reached the desktop
                }
            } else {
                break true;
            }
        };
        if invalid_chain_for_focus {
            // clear all marks
            controls.clean_marked_for_focus();
            return;
        }

        // 2. if there is already an object with focus --> call on_focus_lost
        if let Some(focused) = self.current_focus {
            let mut h = focused;
            while let Some(control) = controls.get(h) {
                if control.get_base().is_marked_to_receive_focus() {
                    break;
                }
                control.get_base_mut().update_focus_flag(false);
                control.get_control_mut().on_lose_focus();
                if let Some(parent) = control.get_base().parent {
                    h = parent;
                } else {
                    break;
                }
            }
        }

        // 3. now lets call on_focus (in the reverse order --> from parent to child)
        let mut parent_handle = None;
        while let Some(handle) = self.focus_chain.pop() {
            //println!("Pop handle: {},{}",handle.get_id(),handle.get_index());
            let child = controls.get(handle);
            if let Some(control) = child {
                let base = control.get_base_mut();
                let parent_index = base.parent_index;
                base.clear_mark_to_receive_focus();
                if !base.has_focus() {
                    base.update_focus_flag(true);
                    control.get_control_mut().on_focus();
                }
                if parent_index.is_valid() {
                    if let Some(p_handle) = parent_handle {
                        if let Some(p) = controls.get(p_handle) {
                            let base = p.get_base_mut();
                            base.focused_child_index = parent_index;
                        }
                    }
                }
            }
            parent_handle = Some(handle);
        }
        self.current_focus = Some(handle);
        self.request_focus = None;
    }

    fn recompute_layouts(&mut self) {
        let term_layout = ParentLayout::from(&self.terminal);
        self.update_control_layout(self.desktop_handler, &term_layout);
    }

    fn update_parent_indexes(&mut self, handle: Handle) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get(handle) {
            let base = control.get_base_mut();
            for i in 0..base.children.len() {
                let child_handle = base.children[i];
                if let Some(child) = unsafe { (&mut *self.controls).get(child_handle) } {
                    child.get_base_mut().parent_index = VectorIndex::with_value(i);
                    self.update_parent_indexes(child_handle);
                }
            }
        }
    }

    pub(crate) fn update_control_layout(&mut self, handle: Handle, parent_layout: &ParentLayout) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get(handle) {
            let base = control.get_base_mut();
            let old_size = base.get_size();
            base.update_control_layout_and_screen_origin(parent_layout);
            let new_size = base.get_size();
            // process the same thing for its children
            let my_layout = ParentLayout::from(base);
            // if size has been changed --> call on_resize
            if new_size != old_size {
                control.get_control_mut().on_resize(old_size, new_size);
            }
            for child_handle in &control.get_base().children {
                self.update_control_layout(*child_handle, &my_layout)
            }
        }
    }
    fn paint(&mut self) {
        self.paint_control(self.desktop_handler);
        self.surface.reset();
        if self.commandbar.is_some() {
            self.commandbar
                .as_ref()
                .unwrap()
                .paint(&mut self.surface, &self.theme);
        }
        if self.menubar.is_some() {
            self.menubar
                .as_ref()
                .unwrap()
                .paint(&mut self.surface, &self.theme);
        }
        if self.tooltip.is_visible() {
            self.tooltip.paint(&mut self.surface, &self.theme);
        }
        if let Some(opened_menu_handle) = self.opened_menu {
            self.surface.reset();
            self.paint_menu(opened_menu_handle, true);
        }
        self.terminal.update_screen(&self.surface);
    }
    fn paint_control(&mut self, handle: Handle) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get(handle) {
            if control.get_base().prepare_paint(&mut self.surface) {
                // paint is possible
                control
                    .get_control()
                    .on_paint(&mut self.surface, &self.theme);
                for child_handle in &control.get_base().children {
                    self.paint_control(*child_handle);
                }
            }
        }
    }
    fn paint_menu(&mut self, handle: MenuHandle, activ: bool) {
        let menus = unsafe { &mut *self.menus };
        if let Some(menu) = menus.get(handle) {
            if let Some(parent_menu_handle) = menu.get_parent_handle() {
                self.paint_menu(parent_menu_handle, false);
            }
            menu.paint(&mut self.surface, &self.theme, activ);
        }
    }

    fn process_key_modifier_changed_event(&mut self, new_state: KeyModifier) {
        if let Some(commandbar) = self.commandbar.as_mut() {
            commandbar.set_key_modifier(new_state);
            self.repaint = true;
        }
    }

    fn process_keypressed_event(&mut self, event: KeyPressedEvent) {
        match self.process_control_keypressed_event(
            self.desktop_handler,
            event.key,
            event.character,
        ) {
            EventProcessStatus::Processed | EventProcessStatus::Update => self.repaint = true,
            _ => {}
        }
    }
    pub(crate) fn process_control_keypressed_event(
        &mut self,
        handle: Handle,
        key: Key,
        character: char,
    ) -> EventProcessStatus {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get(handle) {
            let base = control.get_base_mut();
            if base.can_receive_input() == false {
                return EventProcessStatus::Ignored;
            }
            if base.focused_child_index.in_range(base.children.len()) {
                let handle_child = base.children[base.focused_child_index.index()];
                if self.process_control_keypressed_event(handle_child, key, character)
                    == EventProcessStatus::Processed
                {
                    return EventProcessStatus::Processed;
                }
            }
            // else --> call it ourselves
            return control.get_control_mut().on_key_pressed(key, character);
        }

        return EventProcessStatus::Ignored;
    }

    fn coordinates_to_control(&mut self, handle: Handle, x: i32, y: i32) -> Option<Handle> {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get(handle) {
            let base = control.get_base_mut();
            if base.can_receive_input() == false {
                return None;
            }
            if !base.screen_clip.contains(x, y) {
                return None;
            }
            let count = base.children.len();
            if count > 0 {
                let mut idx = if base.focused_child_index.in_range(count) {
                    base.focused_child_index.index()
                } else {
                    0
                };
                for _ in 0..count {
                    let handle_child = base.children[idx];
                    if let Some(handle) = self.coordinates_to_control(handle_child, x, y) {
                        return Some(handle);
                    }
                    idx = (idx + 1) % count;
                }
            }

            return Some(handle);
        }
        None
    }

    fn process_menu_and_cmdbar_mousemove(&mut self, x: i32, y: i32) -> bool {
        let mut processed = false;
        // Process event in the following order:
        // first the context menu and its owner, then the menu bar and then cmdbar
        if let Some(open_menu_handle) = self.opened_menu {
            let menus = self.get_menus();
            if let Some(menu) = menus.get_mut(open_menu_handle) {
                if menu.on_mouse_move(x, y).is_processed_or_update() {
                    self.repaint = true;
                    return true;
                }
            }
        }

        /*
        if (this->VisibleMenu)
        {
            auto* mnuC = ((MenuContext*) (this->VisibleMenu->Context));
            processed =
                  mnuC->OnMouseMove(x - mnuC->ScreenClip.ScreenPosition.X, y - mnuC->ScreenClip.ScreenPosition.Y, repaint);
            if ((!processed) && (mnuC->Owner))
                processed = mnuC->Owner->OnMouseMove(x, y, repaint);
        }
        */

        if let Some(menubar) = self.menubar.as_mut() {
            processed = match menubar.on_mouse_move(x, y) {
                EventProcessStatus::Processed => {
                    self.repaint = true;
                    true
                }
                EventProcessStatus::Ignored => false,
                EventProcessStatus::Update => {
                    self.repaint = true;
                    true
                }
                EventProcessStatus::Cancel => false,
            }
        }
        if !processed {
            if let Some(cmdbar) = self.commandbar.as_mut() {
                processed = cmdbar.on_mouse_move(&MouseMoveEvent {
                    x,
                    y,
                    button: MouseButton::Left,
                });
                self.repaint |= processed;
            }
        }
        if processed {
            let controls = unsafe { &mut *self.controls };
            if let Some(c_handle) = self.mouse_over_control {
                if let Some(control) = controls.get(c_handle) {
                    let response = control.get_control_mut().on_mouse_event(&MouseEvent::Leave);
                    self.repaint |= response.is_processed_or_update();
                    control.get_base_mut().update_mouse_over_flag(false);
                }
                self.mouse_over_control = None;
            }
        }
        return processed;
    }

    fn process_terminal_resize_event(&mut self, new_size: Size) {
        // sanity checks
        if (new_size.width == 0) || (new_size.height == 0) {
            return;
        }
        if (new_size.width == self.surface.get_width())
            && (new_size.height == self.surface.get_height())
        {
            return;
        }
        self.surface.resize(new_size);
        self.terminal.on_resize(new_size);
        if let Some(commandbar) = self.commandbar.as_mut() {
            commandbar.set_desktop_size(new_size);
        }
        if let Some(menubar) = self.menubar.as_mut() {
            menubar.set_position(0, 0, new_size.width);
        }
        self.recompute_layout = true;
    }
    fn process_mousewheel_event(&mut self, event: MouseWheelEvent) {
        // if (this->VisibleMenu)
        // {
        //     auto* mcx = reinterpret_cast<MenuContext*>(this->VisibleMenu->Context);
        //     if (mcx->OnMouseWheel(x, y, direction))
        //         RepaintStatus |= REPAINT_STATUS_DRAW;
        //     return;
        // }
        match self.mouse_locked_object {
            MouseLockedObject::None => {}
            _ => return,
        }
        if let Some(handle) = self.coordinates_to_control(self.desktop_handler, event.x, event.y) {
            let controls = unsafe { &mut *self.controls };
            if let Some(control) = controls.get(handle) {
                match control
                    .get_control_mut()
                    .on_mouse_event(&MouseEvent::Wheel(event.direction))
                {
                    EventProcessStatus::Processed | EventProcessStatus::Update => {
                        self.repaint = true
                    }
                    _ => {}
                }
            }
        }
    }
    fn process_mousedrag(&mut self, handle: Handle, event: MouseMoveEvent) {
        self.hide_tooltip();
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get(handle) {
            let base = control.get_base_mut();
            let scr_x = base.screen_clip.left;
            let scr_y = base.screen_clip.top;
            let response =
                control
                    .get_control_mut()
                    .on_mouse_event(&MouseEvent::Drag(MouseEventData {
                        x: event.x - scr_x,
                        y: event.y - scr_y,
                        button: event.button,
                    }));
            let do_update = response.is_processed_or_update();
            self.repaint |= do_update;
            self.recompute_layout |= do_update;
        }
    }
    fn process_mousemove(&mut self, event: MouseMoveEvent) {
        if self.process_menu_and_cmdbar_mousemove(event.x, event.y) {
            return;
        }
        let controls = unsafe { &mut *self.controls };
        let handle = self.coordinates_to_control(self.desktop_handler, event.x, event.y);
        if handle != self.mouse_over_control {
            self.hide_tooltip();
            if let Some(c_handle) = self.mouse_over_control {
                if let Some(control) = controls.get(c_handle) {
                    let response = control.get_control_mut().on_mouse_event(&MouseEvent::Leave);
                    self.repaint |= response.is_processed_or_update();
                    control.get_base_mut().update_mouse_over_flag(false);
                }
            }
            self.mouse_over_control = handle;
            if let Some(c_handle) = self.mouse_over_control {
                if let Some(control) = controls.get(c_handle) {
                    let base = control.get_base_mut();
                    base.update_mouse_over_flag(true);
                    let scr_x = base.screen_clip.left;
                    let scr_y = base.screen_clip.top;
                    let response = control.get_control_mut().on_mouse_event(&MouseEvent::Enter);
                    self.repaint |= response.is_processed_or_update();
                    let response =
                        control
                            .get_control_mut()
                            .on_mouse_event(&MouseEvent::Over(Point::new(
                                event.x - scr_x,
                                event.y - scr_y,
                            )));
                    self.repaint |= response.is_processed_or_update();
                }
            }
        } else {
            if let Some(handle) = self.mouse_over_control {
                if let Some(control) = controls.get(handle) {
                    let base = control.get_base();
                    let scr_x = base.screen_clip.left;
                    let scr_y = base.screen_clip.top;
                    let response =
                        control
                            .get_control_mut()
                            .on_mouse_event(&MouseEvent::Over(Point::new(
                                event.x - scr_x,
                                event.y - scr_y,
                            )));
                    self.repaint |= response.is_processed_or_update();
                }
            }
        }
    }
    fn process_mousemove_event(&mut self, event: MouseMoveEvent) {
        match self.mouse_locked_object {
            MouseLockedObject::None => self.process_mousemove(event),
            MouseLockedObject::Control(handle) => self.process_mousedrag(handle, event),
            MouseLockedObject::CommandBar => {}
            MouseLockedObject::MenuBar => todo!(),
        }
    }
    fn process_mousebuttondown_event(&mut self, event: MouseButtonDownEvent) {
        // Hide ToolTip
        self.hide_tooltip();
        // check contextual menu
        /*
            if (this->VisibleMenu)
            {
                ProcessMenuMouseClick(this->VisibleMenu, x, y);
                return;
            }
        */
        // check main menu
        if let Some(menu) = self.menubar.as_mut() {
            if menu.on_mouse_pressed(event.x, event.y) == EventProcessStatus::Processed {
                self.repaint = true;
                self.mouse_locked_object = MouseLockedObject::MenuBar;
                return;
            }
        }
        // check command bar
        if let Some(commandbar) = self.commandbar.as_mut() {
            if commandbar.on_mouse_down(&event) {
                self.repaint = true;
                self.mouse_locked_object = MouseLockedObject::CommandBar;
                return;
            }
        }
        // check for a control
        if let Some(handle) = self.coordinates_to_control(self.desktop_handler, event.x, event.y) {
            let controls = unsafe { &mut *self.controls };
            if let Some(control) = controls.get(handle) {
                self.update_focus(handle);
                let base = control.get_base();
                let scr_x = base.screen_clip.left;
                let scr_y = base.screen_clip.top;
                control
                    .get_control_mut()
                    .on_mouse_event(&MouseEvent::Pressed(MouseEventData {
                        x: event.x - scr_x,
                        y: event.y - scr_y,
                        button: event.button,
                    }));
                self.mouse_locked_object = MouseLockedObject::Control(handle);
                self.repaint = true;
                return;
            }
        }
        self.mouse_locked_object = MouseLockedObject::None;
        /*
        void ApplicationImpl::OnMouseDown(int x, int y, Input::MouseButton button)
        {

            // check controls
            if (ModalControlsCount == 0)
                MouseLockedControl = CoordinatesToControl(this->AppDesktop, x, y);
            else
                MouseLockedControl = CoordinatesToControl(ModalControlsStack[ModalControlsCount - 1], x, y);

            if (MouseLockedControl != nullptr)
            {
                // done
            }
        }


        */
    }
    fn process_mousebuttonup_event(&mut self, event: MouseButtonUpEvent) {
        // check contextual menus
        if let Some(opened_menu_handle) = self.opened_menu {
            let menus = unsafe { &mut * self.menus };
            if let Some(menu) = menus.get_mut(opened_menu_handle) {
                if menu.on_mouse_pressed(event.x, event.y).is_processed_or_update() {
                    self.repaint = true;
                }
            }
        }
        /*if (this->VisibleMenu)
        {
            ProcessMenuMouseReleased(this->VisibleMenu, x, y);
        }*/
        match self.mouse_locked_object {
            MouseLockedObject::None => {}
            MouseLockedObject::Control(handle) => {
                let controls = unsafe { &mut *self.controls };
                if let Some(control) = controls.get(handle) {
                    let base = control.get_base();
                    let scr_x = base.screen_clip.left;
                    let scr_y = base.screen_clip.top;
                    control
                        .get_control_mut()
                        .on_mouse_event(&MouseEvent::Released(MouseEventData {
                            x: event.x - scr_x,
                            y: event.y - scr_y,
                            button: event.button,
                        }));
                    self.repaint = true;
                }
            }
            MouseLockedObject::CommandBar => {
                if let Some(cmdbar) = self.commandbar.as_mut() {
                    if let Some(command) = cmdbar.on_mouse_up() {
                        self.send_command(command);
                    }
                    self.repaint = true;
                }
            }
            MouseLockedObject::MenuBar => {
                if let Some(menubar) = self.menubar.as_mut() {
                    menubar.on_mouse_pressed(event.x, event.y);
                }
                self.repaint = true;
            },
        }
        self.mouse_locked_object = MouseLockedObject::None;
    }
    fn process_mouse_dblclick_event(&mut self, _event: MouseDoubleClickEvent) {}

    fn debug_print(&self, handle: Handle, depth: i32) {
        for _ in 0..depth {
            print!(" ");
        }
        let base = self.get_controls().get(handle).unwrap().get_base();
        if base.parent_index.is_valid() {
            print!("{}. ", base.parent_index.index());
        } else {
            print!("?.");
        }
        print!("[ID:{},Index:{}],", handle.get_id(), handle.get_index());
        print!("  Children: {}", base.children.len());
        if base.focused_child_index.is_valid() {
            print!("  Idx:{}", base.focused_child_index.index());
        } else {
            print!("  Idx:Invalid");
        }
        print!("  Focus:{}", base.has_focus());
        println!("");
        for handle in base.children.iter() {
            self.debug_print(*handle, depth + 2);
        }
    }
}

impl Drop for RuntimeManager {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.controls);
        }
    }
}

/*
bool ApplicationImpl::ExecuteEventLoop(Control* ctrl, bool resetState)
{
    CHECK(app->Inited, false, "Application has not been corectly initialized !");

    Internal::SystemEvent evnt;
    this->RepaintStatus      = REPAINT_STATUS_ALL;
    this->MouseLockedControl = nullptr;
    this->mouseLockedObject  = MouseLockedObject::None;

    if (resetState)
        this->loopStatus = LoopStatus::Normal;
    // hide current hovered control when new dialog is opened.
    if (this->MouseOverControl)
    {
        ((ControlContext*) (MouseOverControl->Context))->MouseIsOver = false;
        this->MouseOverControl                                       = nullptr;
    }

    PackControl(true);
    if (ctrl != nullptr)
    {
        CHECK(ModalControlsCount < MAX_MODAL_CONTROLS_STACK, false, "Too many modal calls !");
        ModalControlsStack[ModalControlsCount] = ctrl;
        ModalControlsCount++;
    }
    // update command bar
    UpdateCommandBar();

    while (loopStatus == LoopStatus::Normal)
    {
        if (!toDelete.empty())
        {
            for (auto c : toDelete)
            {
                // delete any potential references
                if (this->MouseLockedControl == c)
                    this->MouseLockedControl = nullptr;
                if (this->MouseOverControl == c)
                    this->MouseOverControl = nullptr;
                if (this->ExpandedControl == c)
                    this->ExpandedControl = nullptr;
                delete c;
            }
            toDelete.clear();
        }
        if (this->cmdBarUpdate)
        {
            UpdateCommandBar();
            RepaintStatus |= REPAINT_STATUS_DRAW;
        }
        if (RepaintStatus != REPAINT_STATUS_NONE)
        {
            if ((RepaintStatus & REPAINT_STATUS_COMPUTE_POSITION) != 0)
                ComputePositions();
            if ((RepaintStatus & REPAINT_STATUS_DRAW) != 0)
            {
                RepaintStatus = REPAINT_STATUS_NONE;
                this->Paint();
                // pentru cazul in care OnFocus sau OnLoseFocus schimba repaint status
                if ((RepaintStatus & REPAINT_STATUS_COMPUTE_POSITION) != 0)
                    ComputePositions();
                if (this->cmdBarUpdate)
                    UpdateCommandBar();
                if ((RepaintStatus & REPAINT_STATUS_DRAW) != 0)
                    this->Paint();
                this->terminal->Update();
            }
            RepaintStatus = REPAINT_STATUS_NONE;
        }
        this->terminal->GetSystemEvent(evnt);
        if (evnt.updateFrames)
        {
            if (ProcessUpdateFrameEvent(this->AppDesktop))
                this->RepaintStatus |= REPAINT_STATUS_DRAW;
            for (uint32 tr = 0; tr < ModalControlsCount; tr++)
                if (ProcessUpdateFrameEvent(this->ModalControlsStack[tr]))
                    this->RepaintStatus |= REPAINT_STATUS_DRAW;
        }
        switch (evnt.eventType)
        {
        case SystemEventType::AppClosed:
            loopStatus = LoopStatus::StopApp;
            break;
        case SystemEventType::AppResized:
            if (((evnt.newWidth != this->terminal->screenCanvas.GetWidth()) ||
                 (evnt.newHeight != this->terminal->screenCanvas.GetHeight())) &&
                (evnt.newWidth > 0) && (evnt.newHeight > 0))
            {
                LOG_INFO("New size for app: %dx%d", evnt.newWidth, evnt.newHeight);
                this->terminal->screenCanvas.Resize(evnt.newWidth, evnt.newHeight);
                this->AppDesktop->Resize(evnt.newWidth, evnt.newHeight);
                if (this->cmdBar)
                    this->cmdBar->SetDesktopSize(evnt.newWidth, evnt.newHeight);
                if (this->menu)
                    this->menu->SetWidth(evnt.newWidth);
                this->RepaintStatus = REPAINT_STATUS_ALL;
            }
            break;
        case SystemEventType::MouseDown:
            OnMouseDown(evnt.mouseX, evnt.mouseY, evnt.mouseButton);
            break;
        case SystemEventType::MouseUp:
            OnMouseUp(evnt.mouseX, evnt.mouseY, evnt.mouseButton);
            break;
        case SystemEventType::MouseMove:
            OnMouseMove(evnt.mouseX, evnt.mouseY, evnt.mouseButton);
            break;
        case SystemEventType::MouseWheel:
            OnMouseWheel(evnt.mouseX, evnt.mouseY, evnt.mouseWheel);
            break;
        case SystemEventType::KeyPressed:
            ProcessKeyPress(evnt.keyCode, evnt.unicodeCharacter);
            break;
        case SystemEventType::ShiftStateChanged:
            ProcessShiftState(evnt.keyCode);
            break;
        case SystemEventType::RequestRedraw:
            this->RepaintStatus = REPAINT_STATUS_ALL;
            break;
        default:
            break;
        }
    }
    if (ctrl != nullptr)
    {
        if (ModalControlsCount > 0)
            ModalControlsCount--;
        UpdateCommandBar();
        if (this->MouseOverControl)
        {
            ((ControlContext*) (MouseOverControl->Context))->MouseIsOver = false;
            this->MouseOverControl                                       = nullptr;
        }
        this->MouseLockedControl = nullptr;
        this->mouseLockedObject  = MouseLockedObject::None;
        RepaintStatus            = REPAINT_STATUS_ALL;
    }
    // check if current loop should be stop
    if (loopStatus == LoopStatus::StopCurrent)
    {
        loopStatus    = LoopStatus::Normal;
        RepaintStatus = REPAINT_STATUS_ALL;
        // check if desktop now has no children windows
        CheckIfAppShouldClose();
    }
    // pack extended control
    PackControl(true);
    return true;
}


 */
