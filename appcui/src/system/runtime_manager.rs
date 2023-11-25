use super::{ControlHandleManager, Handle, MenuHandleManager, Theme, ToolTip};
use crate::graphics::{Point, Rect, Size, Surface};
use crate::input::{Key, KeyModifier, MouseButton, MouseEvent, MouseEventData};
use crate::prelude::*;
use crate::terminals::*;
use crate::ui::command_bar::events::CommandBarEvents;
use crate::ui::command_bar::{events::CommandBarEvent, CommandBar};
use crate::ui::common::control_manager::ParentLayout;
use crate::ui::common::{traits::*, ControlEvent};
use crate::ui::common::{ControlEventData, ControlManager, UIElement};
use crate::ui::menu::events::{MenuEvent, MenuEvents};
use crate::ui::menu::{Menu, MenuBar, MousePressedResult};
use crate::ui::window::events::WindowEvents;
use crate::utils::VectorIndex;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum LoopStatus {
    Normal,
    StopApp,
    ExitCurrentLoop,
}

#[derive(Clone, Copy)]
enum MouseLockedObject {
    None,
    Control(Handle<UIElement>),
    CommandBar,
    MenuBar,
}

pub(crate) struct RuntimeManager {
    theme: Theme,
    terminal: Box<dyn Terminal>,
    surface: Surface,
    controls: *mut ControlHandleManager,
    menus: *mut MenuHandleManager,
    desktop_handle: Handle<UIElement>,
    tooltip: ToolTip,
    commandbar: Option<CommandBar>,
    menubar: Option<MenuBar>,
    recompute_layout: bool,
    repaint: bool,
    desktop_os_start_called: bool,
    recompute_parent_indexes: bool,
    request_update_command_and_menu_bars: bool,
    loop_status: LoopStatus,
    request_focus: Option<Handle<UIElement>>,
    current_focus: Option<Handle<UIElement>>,
    mouse_over_control: Handle<UIElement>,
    focus_chain: Vec<Handle<UIElement>>,
    events: Vec<ControlEvent>,
    commandbar_event: Option<CommandBarEvent>,
    menu_event: Option<MenuEvent>,
    mouse_locked_object: MouseLockedObject,
    opened_menu_handle: Handle<Menu>,
    modal_windows: Vec<Handle<UIElement>>,
    to_remove_list: Vec<Handle<UIElement>>,
}

static mut RUNTIME_MANAGER: Option<RuntimeManager> = None;

impl RuntimeManager {
    pub(super) fn create(mut builder: crate::system::Builder) -> Result<(), super::Error> {
        let term = TerminalType::new(&builder)?;
        let term_sz = term.get_size();
        let surface = Surface::new(term_sz.width, term_sz.height);
        let mut manager = RuntimeManager {
            theme: Theme::new(),
            terminal: term,
            surface: surface,
            desktop_handle: Handle::new(0),
            tooltip: ToolTip::new(),
            recompute_layout: true,
            repaint: true,
            desktop_os_start_called: false,
            request_update_command_and_menu_bars: true,
            recompute_parent_indexes: true,
            request_focus: None,
            current_focus: None,
            mouse_over_control: Handle::None,
            opened_menu_handle: Handle::None,
            focus_chain: Vec::with_capacity(16),
            events: Vec::with_capacity(16),
            modal_windows: Vec::with_capacity(16),
            to_remove_list: Vec::with_capacity(4),
            commandbar_event: None,
            menu_event: None,
            controls: Box::into_raw(Box::new(ControlHandleManager::new())),
            menus: Box::into_raw(Box::new(MenuHandleManager::new())),
            loop_status: LoopStatus::Normal,
            mouse_locked_object: MouseLockedObject::None,
            commandbar: if builder.has_command_bar {
                Some(CommandBar::new(term_sz.width, term_sz.height))
            } else {
                None
            },
            menubar: if builder.has_menu { Some(MenuBar::new(term_sz.width)) } else { None },
        };
        let mut desktop = if let Some(desktop) = builder.desktop_manager.take() {
            desktop
        } else {
            ControlManager::new(Desktop::new())
        };
        let controls = unsafe { &mut *manager.controls };
        desktop.get_base_mut().update_focus_flag(true);
        manager.desktop_handle = controls.add(desktop);
        manager.current_focus = Some(manager.desktop_handle);
        controls.get_mut(manager.desktop_handle).unwrap().get_base_mut().handle = manager.desktop_handle;
        unsafe {
            RUNTIME_MANAGER = Some(manager);
        }
        Ok(())
    }
    pub(crate) fn get() -> &'static mut RuntimeManager {
        unsafe { RUNTIME_MANAGER.as_mut().unwrap() }
    }
    pub(crate) fn get_terminal_size(&self) -> Size {
        self.terminal.get_size()
    }
    pub(crate) fn get_desktop_rect(&self) -> Rect {
        let sz = self.terminal.get_size();
        Rect::new(
            0,
            if self.menubar.is_some() { 1 } else { 0 },
            (sz.width as i32) - 1,
            if self.commandbar.is_some() {
                (sz.height as i32) - 2
            } else {
                (sz.height as i32) - 1
            },
        )
    }
    pub(crate) fn request_repaint(&mut self) {
        self.repaint = true;
    }
    pub(crate) fn request_recompute_layout(&mut self) {
        self.recompute_layout = true;
    }
    pub(crate) fn exit_execution_loop(&mut self) {
        self.loop_status = LoopStatus::ExitCurrentLoop;
    }
    pub(crate) fn cancel_exit_from_execution_loop(&mut self) {
        self.loop_status = LoopStatus::Normal;
    }
    pub(crate) fn show_tooltip(&mut self, txt: &str, rect: &Rect) {
        self.tooltip.show(txt, &rect, self.terminal.get_size(), &self.theme);
    }
    pub(crate) fn hide_tooltip(&mut self) {
        self.tooltip.hide();
    }
    pub(crate) fn close_opened_menu(&mut self) {
        if !self.opened_menu_handle.is_none() {
            self.opened_menu_handle = Handle::None;
            self.repaint = true;
            if let Some(menubar) = self.menubar.as_mut() {
                menubar.close();
            }
        }
    }
    pub(crate) fn send_event(&mut self, event: ControlEvent) {
        self.events.push(event);
    }
    pub(crate) fn set_menu_event(&mut self, event: MenuEvent) {
        self.menu_event = Some(event);
    }
    pub(crate) fn close(&mut self) {
        self.loop_status = LoopStatus::StopApp;
    }
    pub(crate) fn request_focus_for_control(&mut self, handle: Handle<UIElement>) {
        self.request_focus = Some(handle);
    }
    pub(crate) fn request_update(&mut self) {
        self.request_update_command_and_menu_bars = true;
        self.repaint = true;
        self.recompute_layout = true;
    }
    pub(crate) fn request_remove(&mut self, handle: Handle<UIElement>) {
        if !handle.is_none() {
            self.to_remove_list.push(handle);
        }
    }
    fn set_event_processors(&mut self, control_handle: Handle<UIElement>, event_processor: Handle<UIElement>) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(control_handle) {
            let base = control.get_base_mut();
            base.event_processor = event_processor;
            for child in &base.children {
                self.set_event_processors(*child, event_processor);
            }
        }
    }
    pub(crate) fn add_window<T>(&mut self, obj: T) -> Handle<T>
    where
        T: Control + WindowControl + 'static,
    {
        let controls = unsafe { &mut *self.controls };
        let handle = controls.get_desktop().get_base_mut().add_child(obj);
        // since it is the first time I register this window
        // I need to recursively set the event processor for all of its childern to
        // this window current handle
        self.set_event_processors(handle.cast(), handle.cast());
        // all good --> the window has been registered
        if let Some(win) = controls.get_mut(handle.cast()) {
            win.get_control_mut().on_registered();
        }
        return handle;
    }
    pub(crate) fn add_modal_window<T, U>(&mut self, obj: T) -> Handle<T>
    where
        T: Control + WindowControl + ModalWindowMethods<U> + 'static,
    {
        let controls = unsafe { &mut *self.controls };
        let handle = controls.add(ControlManager::new(obj));
        // since it is the first time I register this window
        // I need to recursively set the event processor for all of its childern to
        // this window current handle
        self.set_event_processors(handle.cast(), handle.cast());
        // all good --> the window has been registered
        if let Some(win) = controls.get_mut(handle.cast()) {
            win.get_control_mut().on_registered();
        }
        // add to modal stack
        if !handle.is_none() {
            self.modal_windows.push(handle);
            self.request_focus_for_control(handle);
            self.request_update();
        }
        return handle.cast();
    }
    pub(crate) fn get_control_mut<T>(&mut self, handle: Handle<T>) -> Option<&mut T>
    where
        T: Control + 'static,
    {
        let controls = unsafe { &mut *self.controls };
        if let Some(cm) = controls.get_mut(handle.cast()) {
            return Some(cm.get_mut::<T>());
        }
        None
    }
    pub(crate) fn get_control<T>(&self, handle: Handle<T>) -> Option<&T>
    where
        T: Control + 'static,
    {
        let controls = unsafe { &mut *self.controls };
        if let Some(cm) = controls.get_mut(handle.cast()) {
            return Some(cm.get::<T>());
        }
        None
    }
    #[inline(always)]
    pub(crate) fn get_controls_mut(&mut self) -> &mut ControlHandleManager {
        unsafe { &mut *self.controls }
    }
    #[inline(always)]
    pub(crate) fn get_controls(&self) -> &ControlHandleManager {
        unsafe { &*self.controls }
    }
    #[inline(always)]
    pub(crate) fn get_menus(&self) -> &mut MenuHandleManager {
        unsafe { &mut *self.menus }
    }
    pub(crate) fn add_menu(&mut self, menu: Menu) -> Handle<Menu> {
        self.get_menus().add(menu)
    }
    pub(crate) fn get_menu(&mut self, handle: Handle<Menu>) -> Option<&mut Menu> {
        let menus = unsafe { &mut *self.menus };
        menus.get_mut(handle)
    }
    pub(crate) fn show_menu(&mut self, handle: Handle<Menu>, receiver_control_handle: Handle<UIElement>, x: i32, y: i32, max_size: Size) {
        let menus = unsafe { &mut *self.menus };
        if let Some(menu) = menus.get_mut(handle) {
            menu.compute_position(x, y, max_size, self.terminal.get_size());
            menu.set_receiver_control_handle(receiver_control_handle);
            self.opened_menu_handle = handle;
        }
    }
    pub(crate) fn activate_opened_menu_parent(&mut self) {
        let menus = unsafe { &mut *self.menus };
        if let Some(menu) = menus.get_mut(self.opened_menu_handle) {
            let parent_handle = menu.get_parent_handle();
            if let Some(_) = menus.get(parent_handle) {
                self.opened_menu_handle = parent_handle;
                return;
            }
        }
        self.close_opened_menu();
    }
    pub(crate) fn process_desktop_on_start(&mut self) {
        self.desktop_os_start_called = true;
        let controls = unsafe { &mut *self.controls };
        if let Some(desktop) = controls.get_mut(self.desktop_handle.cast()) {
            DesktopEvents::on_start(desktop.get_control_mut());
        }
    }
    pub(crate) fn run(&mut self) {
        self.recompute_layout = true;
        self.repaint = true;
        self.recompute_parent_indexes = true;
        self.commandbar_event = None;
        self.menu_event = None;
        // if first time an execution start
        if !self.desktop_os_start_called {
            self.process_terminal_resize_event(self.terminal.get_size());
            self.process_desktop_on_start();
        }
        while self.loop_status == LoopStatus::Normal {
            // 1. Process events from command bar
            if let Some(event) = self.commandbar_event {
                self.process_commandbar_event(event);
            }
            // 2. Process events from menu
            if let Some(event) = self.menu_event {
                self.process_menu_event(event);
            }
            // 3. Process events from controls
            if !self.events.is_empty() {
                self.process_events_queue();
            }
            // 4. if there is a control that was removed (due to the previously fired events) remove it
            if !self.to_remove_list.is_empty() {
                self.remove_deleted_controls();
            }

            // If we reach this point, there should not be any change in the logic of controls
            if self.recompute_parent_indexes {
                self.update_parent_indexes(self.get_root_control_handle());
                self.recompute_parent_indexes = false;
            }
            if let Some(handle) = self.request_focus {
                self.update_focus(handle);
                self.request_focus = None;
                self.repaint = true;
                self.request_update_command_and_menu_bars = true;
            }
            if self.recompute_layout {
                self.recompute_layouts();
            }
            if self.request_update_command_and_menu_bars {
                self.update_command_and_menu_bars();
            }
            if self.repaint || self.recompute_layout {
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
                SystemEvent::KeyModifierChanged(event) => self.process_key_modifier_changed_event(event.new_state),
                SystemEvent::Resize(new_size) => self.process_terminal_resize_event(new_size),
                SystemEvent::MouseButtonDown(event) => self.process_mousebuttondown_event(event),
                SystemEvent::MouseButtonUp(event) => self.process_mousebuttonup_event(event),
                SystemEvent::MouseDoubleClick(event) => self.process_mouse_dblclick_event(event),
                SystemEvent::MouseMove(event) => self.process_mousemove_event(event),
                SystemEvent::MouseWheel(event) => self.process_mousewheel_event(event),
            }
        }
        // loop has ended
        if self.loop_status == LoopStatus::ExitCurrentLoop {
            // if we are in a modal loop --> we just need to change loop_statup
            // and delete the window and its children (mark them for deleteion)
            if let Some(modal_handle) = self.modal_windows.pop() {
                self.request_remove(modal_handle);
            }
            // also we need to change focus to the previous window in the loop or desktop
            if let Some(previous_modal_handle) = self.modal_windows.last() {
                self.request_focus_for_control(*previous_modal_handle);
            } else {
                self.request_focus_for_control(self.desktop_handle);
            }
            self.loop_status = LoopStatus::Normal;
            self.request_update();
        }
    }
    fn remove_control(&mut self, handle: Handle<UIElement>, unlink_from_parent: bool) {
        if handle.is_none() {
            return;
        }
        let controls = unsafe { &mut *self.controls };
        // remove the link from its parent if requested
        if unlink_from_parent {
            if let Some(control) = controls.get(handle.cast()) {
                let parent = control.get_base().parent;
                if let Some(parent) = controls.get_mut(parent.cast()) {
                    let base = parent.get_base_mut();
                    if let Some(index) = base.children.iter().position(|&elem| elem == handle) {
                        // if the index is bigger than the focused children index --> than all god
                        // otherwise, we need to reset the index
                        if index <= base.focused_child_index.index() {
                            base.focused_child_index = VectorIndex::Invalid;
                        }
                    }
                }
            }
        }
        // first remove my children, then myself
        if let Some(control) = controls.get_mut(handle.cast()) {
            let base = control.get_base();
            for child in &base.children {
                self.remove_control(*child, false);
            }
        }
        controls.remove(handle);
    }
    fn remove_deleted_controls(&mut self) {
        while let Some(handle) = self.to_remove_list.pop() {
            self.remove_control(handle, true);
        }
    }
    fn get_opened_menu(&mut self) -> Option<&mut Menu> {
        if self.opened_menu_handle.is_none() {
            return None;
        }
        let menus = unsafe { &mut *self.menus };
        return menus.get_mut(self.opened_menu_handle);
    }
    #[inline(always)]
    fn get_root_control_handle(&self) -> Handle<UIElement> {
        if self.modal_windows.is_empty() {
            self.desktop_handle
        } else {
            self.modal_windows[self.modal_windows.len() - 1]
        }
    }
    fn get_focused_control(&self) -> Handle<UIElement> {
        let controls = unsafe { &mut *self.controls };
        let mut parent = self.get_root_control_handle();
        let mut ctrl = controls.get_mut(parent).unwrap();

        loop {
            let base = ctrl.get_base();
            if base.focused_child_index.in_range(base.children.len()) {
                let child_handle = base.children[base.focused_child_index.index()];
                if let Some(child) = controls.get_mut(child_handle) {
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

    fn process_events_queue(&mut self) {
        let controls = unsafe { &mut *self.controls };
        while let Some(evnt) = self.events.pop() {
            if let Some(receiver) = controls.get_mut(evnt.receiver) {
                let result = evnt.invoke(receiver.get_control_mut());
                self.repaint |= result == EventProcessStatus::Processed;
            }
        }
    }

    fn process_commandbar_event(&mut self, event: CommandBarEvent) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(event.control_receiver_handle) {
            CommandBarEvents::on_event(control.get_control_mut(), event.command_id);
        }
        self.commandbar_event = None;
    }
    fn process_menu_event(&mut self, event: MenuEvent) {
        todo!("call the MenuEvents with the current event -> to be discussed !");
        // let controls = unsafe { &mut *self.controls };
        // if let Some(control) = controls.get(event.get_control_receiver_handle()) {
        //     MenuEvents::on_event(control.get_control_mut(), event);
        // }
        // self.menu_event = None;
    }
    fn update_command_and_menu_bars(&mut self) {
        if self.commandbar.is_none() && self.menubar.is_none() {
            self.request_update_command_and_menu_bars = false;
            return;
        }
        let focused_handle = self.get_focused_control();
        let controls = unsafe { &mut *self.controls };
        // process cmdbar
        if let Some(cmdbar) = self.commandbar.as_mut() {
            cmdbar.clear();
            // start from the focused control and call on_update_commandbar for each control
            let mut h = focused_handle;
            while let Some(control) = controls.get_mut(h) {
                cmdbar.set_receiver_control_handle(h);
                control.get_control_mut().on_update_commandbar(cmdbar);
                h = control.get_base().parent;
                if h.is_none() {
                    break;
                }
            }
            cmdbar.update_positions();
        }
        // process menubar
        if let Some(menubar) = self.menubar.as_mut() {
            menubar.clear();
            // start from the focused control and call on_update_menubar for each control
            let mut h = focused_handle;
            while let Some(control) = controls.get_mut(h) {
                menubar.set_receiver_control_handle(h);
                control.get_control_mut().on_update_menubar(menubar);
                h = control.get_base().parent;
                if h.is_none() {
                    break;
                }
            }
            menubar.update_positions();
        }
        self.request_update_command_and_menu_bars = false;
    }

    fn find_last_leaf(&mut self, handle: Handle<UIElement>) -> Handle<UIElement> {
        let controls = unsafe { &mut *self.controls };
        let mut result = Handle::None;
        let mut handle = handle;
        while let Some(c) = controls.get_mut(handle) {
            let base = c.get_base();
            if base.can_receive_input() == false {
                break;
            }
            // curent handle is a possible candidate for a valid child focused leaf
            result = handle;
            handle = base.get_focused_control();
        }
        result
    }
    fn update_focus(&mut self, handle: Handle<UIElement>) {
        // even if we request a specific control, we will select its deepest inner child
        // to receive a focus (this way a set focus over a window will preserve the focus
        // we already have for one of its childern)
        let handle = self.find_last_leaf(handle);
        // 1. mark all controls from the path as preparing to received focus
        // we will use focuse_chain as a temporary value to hold the chain
        self.focus_chain.clear();
        let controls = unsafe { &mut *self.controls };
        let mut h = handle;
        let invalid_chain_for_focus = loop {
            if let Some(control) = controls.get_mut(h) {
                self.focus_chain.push(h);
                if !control.get_base_mut().mark_to_receive_focus() {
                    break true;
                }
                h = control.get_base().parent;
                if h.is_none() {
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
            while let Some(control) = controls.get_mut(h) {
                if control.get_base().is_marked_to_receive_focus() {
                    break;
                }
                control.get_base_mut().update_focus_flag(false);
                control.get_control_mut().on_lose_focus();
                if control.get_base().is_window_control() {
                    control.get_control_mut().on_deactivate();
                }
                h = control.get_base().parent;
                if h.is_none() {
                    break;
                }
            }
        }

        // 3. now lets call on_focus (in the reverse order --> from parent to child)
        let mut parent_handle = None;
        while let Some(handle) = self.focus_chain.pop() {
            //println!("Pop handle: {},{}",handle.get_id(),handle.get_index());
            let child = controls.get_mut(handle);
            if let Some(control) = child {
                let base = control.get_base_mut();
                let parent_index = base.parent_index;
                let window_control = base.is_window_control();
                base.clear_mark_to_receive_focus();
                if !base.has_focus() {
                    base.update_focus_flag(true);
                    let interface = control.get_control_mut();
                    interface.on_focus();
                    if window_control {
                        interface.on_activate();
                    }
                }
                if parent_index.is_valid() {
                    if let Some(p_handle) = parent_handle {
                        if let Some(p) = controls.get_mut(p_handle) {
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
        self.update_control_layout(self.desktop_handle, &term_layout);
        let count = self.modal_windows.len();
        for index in 0..count {
            let handle = self.modal_windows[index];
            self.update_control_layout(handle, &term_layout);
        }
    }

    fn update_parent_indexes(&mut self, handle: Handle<UIElement>) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(handle) {
            let base = control.get_base_mut();
            for i in 0..base.children.len() {
                let child_handle = base.children[i];
                if let Some(child) = unsafe { (&mut *self.controls).get_mut(child_handle) } {
                    child.get_base_mut().parent_index = VectorIndex::with_value(i);
                    self.update_parent_indexes(child_handle);
                }
            }
        }
    }

    pub(crate) fn update_control_layout(&mut self, handle: Handle<UIElement>, parent_layout: &ParentLayout) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(handle) {
            let base = control.get_base_mut();
            let window_control = base.is_window_control();
            let old_size = base.get_size();
            let old_pos = base.get_position();
            base.update_control_layout_and_screen_origin(parent_layout);
            let new_size = base.get_size();
            let new_pos = base.get_position();
            // process the same thing for its children
            let my_layout = ParentLayout::from(base);
            // if size has been changed --> call on_resize
            if new_size != old_size {
                control.get_control_mut().on_resize(old_size, new_size);
            }
            // just for window
            if window_control && ((new_size != old_size) || (old_pos != new_pos)) {
                // call the window specific event
                WindowEvents::on_layout_changed(
                    control.get_control_mut(),
                    Rect::with_point_and_size(old_pos, old_size),
                    Rect::with_point_and_size(new_pos, new_size),
                );
            }
            for child_handle in &control.get_base().children {
                self.update_control_layout(*child_handle, &my_layout)
            }
        }
    }
    fn paint(&mut self) {
        // reset the surface clip and hide the cursor
        self.surface.hide_cursor();
        self.surface.reset();
        self.paint_control(self.desktop_handle);
        if !self.modal_windows.is_empty() {
            let count = self.modal_windows.len();
            for index in 0..count {
                self.surface.reset();
                if index + 1 == count {
                    self.surface.clear(Character::with_color(Color::Gray, Color::Black));
                }
                self.paint_control(self.modal_windows[index]);
            }
        }
        self.surface.reset();
        if self.commandbar.is_some() {
            self.commandbar.as_ref().unwrap().paint(&mut self.surface, &self.theme);
        }
        if self.menubar.is_some() {
            self.menubar.as_ref().unwrap().paint(&mut self.surface, &self.theme);
        }
        if self.tooltip.is_visible() {
            self.tooltip.paint(&mut self.surface, &self.theme);
        }
        if !self.opened_menu_handle.is_none() {
            self.surface.reset();
            self.paint_menu(self.opened_menu_handle, true);
        }
        self.terminal.update_screen(&self.surface);
    }
    fn paint_control(&mut self, handle: Handle<UIElement>) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(handle) {
            if control.get_base().prepare_paint(&mut self.surface) {
                // paint is possible
                control.get_control().on_paint(&mut self.surface, &self.theme);
                for child_handle in &control.get_base().children {
                    self.paint_control(*child_handle);
                }
            }
        }
    }
    fn paint_menu(&mut self, handle: Handle<Menu>, activ: bool) {
        if handle.is_none() {
            return;
        }
        let menus = unsafe { &mut *self.menus };
        if let Some(menu) = menus.get(handle) {
            self.paint_menu(menu.get_parent_handle(), false);
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
        // check controls first
        if self.process_control_keypressed_event(self.get_root_control_handle(), event.key, event.character) == EventProcessStatus::Processed {
            self.repaint = true;
            return;
        };
        // check for a menu on_key_event
        if let Some(menu) = self.get_opened_menu() {
            if menu.on_key_pressed(event.key) == EventProcessStatus::Processed {
                self.repaint = true;
                return;
            }
        }
        // check the menubar
        if let Some(menubar) = self.menubar.as_mut() {
            if menubar.on_key_event(event.key) == EventProcessStatus::Processed {
                self.repaint = true;
                return;
            }
        }
        // check cmdbar
        if let Some(cmdbar) = self.commandbar.as_mut() {
            self.commandbar_event = cmdbar.get_event(event.key);
            self.repaint |= self.commandbar_event.is_some();
        }
    }
    pub(crate) fn process_control_keypressed_event(&mut self, handle: Handle<UIElement>, key: Key, character: char) -> EventProcessStatus {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(handle) {
            let base = control.get_base_mut();
            if base.can_receive_input() == false {
                return EventProcessStatus::Ignored;
            }
            if base.focused_child_index.in_range(base.children.len()) {
                let handle_child = base.children[base.focused_child_index.index()];
                if self.process_control_keypressed_event(handle_child, key, character) == EventProcessStatus::Processed {
                    return EventProcessStatus::Processed;
                }
            }
            // else --> call it ourselves
            return control.get_control_mut().on_key_pressed(key, character);
        }

        return EventProcessStatus::Ignored;
    }

    fn coordinates_to_control(&mut self, handle: Handle<UIElement>, x: i32, y: i32) -> Handle<UIElement> {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(handle) {
            let base = control.get_base_mut();
            if base.is_activ() == false {
                return Handle::None;
            }
            if !base.screen_clip.contains(x, y) {
                return Handle::None;
            }
            let count = base.children.len();
            if count > 0 {
                let mut idx = if base.focused_child_index.in_range(count) {
                    base.focused_child_index.index()
                } else {
                    0
                };
                for _ in 0..count {
                    let handle_child = self.coordinates_to_control(base.children[idx], x, y);
                    if !handle_child.is_none() {
                        return handle_child;
                    }
                    idx = (idx + 1) % count;
                }
            }
            if base.can_receive_input() {
                return handle;
            } else {
                return Handle::None;
            }
        }
        Handle::None
    }

    fn process_menu_and_cmdbar_mousemove(&mut self, x: i32, y: i32) -> bool {
        let mut processed = false;
        // Process event in the following order:
        // first the context menu and its owner, then the menu bar and then cmdbar
        if let Some(menu) = self.get_opened_menu() {
            if menu.on_mouse_move(x, y) == EventProcessStatus::Processed {
                self.repaint = true;
                return true;
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
            processed = menubar.on_mouse_move(x, y) == EventProcessStatus::Processed;
            self.repaint |= processed;
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
            if !self.mouse_over_control.is_none() {
                if let Some(control) = controls.get_mut(self.mouse_over_control) {
                    let response = control.get_control_mut().on_mouse_event(&MouseEvent::Leave);
                    self.repaint |= response == EventProcessStatus::Processed;
                    control.get_base_mut().update_mouse_over_flag(false);
                }
                self.mouse_over_control = Handle::None;
            }
        }
        return processed;
    }

    fn process_menu_mouse_click(&mut self, handle: Handle<Menu>, x: i32, y: i32) {
        let mut result = MousePressedResult::None;
        let mut parent_handle = Handle::None;
        let menus = unsafe { &mut *self.menus };
        if let Some(menu) = menus.get_mut(handle) {
            parent_handle = menu.get_parent_handle();
            if handle == self.opened_menu_handle {
                result = menu.on_mouse_pressed(x, y);
            } else {
                result = if menu.is_on_menu(x, y) {
                    MousePressedResult::Activate
                } else {
                    MousePressedResult::CheckParent
                };
            }
        }
        match result {
            MousePressedResult::None => {}
            MousePressedResult::Repaint => self.repaint = true,
            MousePressedResult::CheckParent => {
                if !parent_handle.is_none() {
                    self.process_menu_mouse_click(parent_handle, x, y);
                } else {
                    self.close_opened_menu();
                }
            }
            MousePressedResult::Activate => {
                self.repaint = true;
                self.opened_menu_handle = handle;
                if let Some(menu) = menus.get_mut(handle) {
                    // trigger an on_mouse_move to force selection
                    menu.on_mouse_move(x, y);
                }
            }
        }

        /*
        void ApplicationImpl::ProcessMenuMouseClick(Controls::Menu* mnu, int x, int y)
        {

            switch (result)
            {
            case MousePressedResult::None:
                break;
            case MousePressedResult::Repaint:
                RepaintStatus |= REPAINT_STATUS_DRAW;
                break;
            case MousePressedResult::CheckParent:
                if (mcx->Parent)
                    ProcessMenuMouseClick(mcx->Parent, x, y);
                else
                    this->CloseContextualMenu();
                break;
            case MousePressedResult::Activate:
                RepaintStatus |= REPAINT_STATUS_DRAW;
                ShowContextualMenu(mnu);
                break;
            }
        }


        */
    }

    fn process_terminal_resize_event(&mut self, new_size: Size) {
        // sanity checks
        if (new_size.width == 0) || (new_size.height == 0) {
            return;
        }
        if new_size == self.surface.size {
            return;
        }
        self.surface.resize(new_size);
        if let Some(commandbar) = self.commandbar.as_mut() {
            commandbar.set_desktop_size(new_size);
        }
        if let Some(menubar) = self.menubar.as_mut() {
            menubar.set_position(0, 0, new_size.width);
        }
        // resize the desktop as well
        let desktop = self.get_controls_mut().get_desktop();
        let original_size = desktop.get_base().get_size();
        desktop.get_base_mut().set_size(new_size.width as u16, new_size.height as u16);
        desktop.get_control_mut().on_resize(original_size, new_size);
        self.recompute_layout = true;
    }
    fn process_mousewheel_event(&mut self, event: MouseWheelEvent) {
        if let Some(menu) = self.get_opened_menu() {
            self.repaint |= menu.on_mouse_wheel(event.direction) == EventProcessStatus::Processed;
            return;
        }
        match self.mouse_locked_object {
            MouseLockedObject::None => {}
            _ => return,
        }
        let handle = self.coordinates_to_control(self.get_root_control_handle(), event.x, event.y);
        if !handle.is_none() {
            let controls = unsafe { &mut *self.controls };
            if let Some(control) = controls.get_mut(handle) {
                self.repaint |= control.get_control_mut().on_mouse_event(&MouseEvent::Wheel(event.direction)) == EventProcessStatus::Processed;
            }
        }
    }
    fn process_mousedrag(&mut self, handle: Handle<UIElement>, event: MouseMoveEvent) {
        self.hide_tooltip();
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(handle) {
            let base = control.get_base_mut();
            let scr_x = base.screen_clip.left;
            let scr_y = base.screen_clip.top;
            let response = control.get_control_mut().on_mouse_event(&MouseEvent::Drag(MouseEventData {
                x: event.x - scr_x,
                y: event.y - scr_y,
                button: event.button,
            }));
            let do_update = response == EventProcessStatus::Processed;
            self.repaint |= do_update;
            self.recompute_layout |= do_update;
        }
    }
    fn process_mousemove(&mut self, event: MouseMoveEvent) {
        if self.process_menu_and_cmdbar_mousemove(event.x, event.y) {
            return;
        }
        let controls = unsafe { &mut *self.controls };
        let handle = self.coordinates_to_control(self.get_root_control_handle(), event.x, event.y);
        if handle != self.mouse_over_control {
            self.hide_tooltip();
            if !self.mouse_over_control.is_none() {
                if let Some(control) = controls.get_mut(self.mouse_over_control) {
                    let response = control.get_control_mut().on_mouse_event(&MouseEvent::Leave);
                    self.repaint |= response == EventProcessStatus::Processed;
                    control.get_base_mut().update_mouse_over_flag(false);
                }
            }
            self.mouse_over_control = handle;
            if !self.mouse_over_control.is_none() {
                if let Some(control) = controls.get_mut(self.mouse_over_control) {
                    let base = control.get_base_mut();
                    base.update_mouse_over_flag(true);
                    let scr_x = base.screen_clip.left;
                    let scr_y = base.screen_clip.top;
                    let response = control.get_control_mut().on_mouse_event(&MouseEvent::Enter);
                    self.repaint |= response == EventProcessStatus::Processed;
                    let response = control
                        .get_control_mut()
                        .on_mouse_event(&MouseEvent::Over(Point::new(event.x - scr_x, event.y - scr_y)));
                    self.repaint |= response == EventProcessStatus::Processed;
                }
            }
        } else {
            if !self.mouse_over_control.is_none() {
                if let Some(control) = controls.get_mut(handle) {
                    let base = control.get_base();
                    let scr_x = base.screen_clip.left;
                    let scr_y = base.screen_clip.top;
                    let response = control
                        .get_control_mut()
                        .on_mouse_event(&MouseEvent::Over(Point::new(event.x - scr_x, event.y - scr_y)));
                    self.repaint |= response == EventProcessStatus::Processed;
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
        if !self.opened_menu_handle.is_none() {
            self.process_menu_mouse_click(self.opened_menu_handle, event.x, event.y);
            return;
        }
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
        let handle = self.coordinates_to_control(self.get_root_control_handle(), event.x, event.y);
        if !handle.is_none() {
            let controls = unsafe { &mut *self.controls };
            if let Some(control) = controls.get_mut(handle) {
                self.update_focus(handle);
                let base = control.get_base();
                let scr_x = base.screen_clip.left;
                let scr_y = base.screen_clip.top;
                control.get_control_mut().on_mouse_event(&MouseEvent::Pressed(MouseEventData {
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
        if let Some(menu) = self.get_opened_menu() {
            self.repaint |= menu.on_mouse_released(event.x, event.y) == EventProcessStatus::Processed;
        }
        match self.mouse_locked_object {
            MouseLockedObject::None => {}
            MouseLockedObject::Control(handle) => {
                let controls = unsafe { &mut *self.controls };
                if let Some(control) = controls.get_mut(handle) {
                    let base = control.get_base();
                    let scr_x = base.screen_clip.left;
                    let scr_y = base.screen_clip.top;
                    control.get_control_mut().on_mouse_event(&MouseEvent::Released(MouseEventData {
                        x: event.x - scr_x,
                        y: event.y - scr_y,
                        button: event.button,
                    }));
                    self.repaint = true;
                }
            }
            MouseLockedObject::CommandBar => {
                if let Some(cmdbar) = self.commandbar.as_mut() {
                    self.commandbar_event = cmdbar.on_mouse_up();
                    self.repaint |= self.commandbar_event.is_some();
                }
            }
            MouseLockedObject::MenuBar => {
                if let Some(menubar) = self.menubar.as_mut() {
                    menubar.on_mouse_pressed(event.x, event.y);
                }
                self.repaint = true;
            }
        }
        self.mouse_locked_object = MouseLockedObject::None;
    }
    fn process_mouse_dblclick_event(&mut self, _event: MouseDoubleClickEvent) {}

    fn debug_print(&self, handle: Handle<UIElement>, depth: i32) {
        for _ in 0..depth {
            print!(" ");
        }
        let base = self.get_controls().get(handle).unwrap().get_base();
        if base.parent_index.is_valid() {
            print!("{}. ", base.parent_index.index());
        } else {
            print!("?.");
        }
        //print!("[ID:{},Index:{}],", handle.get_id(), handle.get_index());
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

    pub(super) fn destroy() {
        unsafe {
            RUNTIME_MANAGER = None;
        }
    }
}

impl Drop for RuntimeManager {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.controls));
            drop(Box::from_raw(self.menus));
        }
    }
}

/*
void ApplicationImpl::ProcessMenuMouseReleased(Controls::Menu* mnu, int x, int y)
{
    auto* mcx   = reinterpret_cast<MenuContext*>(mnu->Context);
    bool result = mcx->OnMouseReleased(x - mcx->ScreenClip.ScreenPosition.X, y - mcx->ScreenClip.ScreenPosition.Y);
    if (result)
        RepaintStatus |= REPAINT_STATUS_DRAW;
}

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
