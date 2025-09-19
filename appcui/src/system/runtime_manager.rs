use std::cell::RefCell;
use std::sync::mpsc::{Receiver, Sender};

use self::layout::ControlLayout;
use self::menu::events::MousePressedMenuResult;

use super::background_task::BackgroundTaskManager;
use super::runtime_manager_traits::*;
use super::timer::TimerManager;
use super::{ControlHandleManager, Handle, MenuHandleManager, Theme, ToolTip};
use crate::backend::{self, Backend};
use crate::graphics::{Point, Rect, Size, Surface};
use crate::input::{Key, KeyModifier, MouseButton, MouseEvent, MouseEventData};
use crate::prelude::*;
use crate::ui::appbar::events::{AppBarEvent, AppBarEvents};
use crate::ui::command_bar::events::GenericCommandBarEvents;
use crate::ui::command_bar::{events::CommandBarEvent, CommandBar};
use crate::ui::common::control_manager::ParentLayout;
use crate::ui::common::ControlEvent;
use crate::ui::common::ControlManager;
use crate::ui::desktop::EmptyDesktop;
use crate::ui::menu::events::{GenericMenuEvents, MenuEvent};
use crate::ui::window::events::WindowEvents;
use crate::ui::{AppBar, Menu};
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
    Control(Handle<()>),
    CommandBar,
    AppBar,
}

#[derive(Default)]
struct ExpandedControlInfo {
    handle: Handle<()>,
    min_size: Size,
    prefered_size: Size,
}

#[repr(u8)]
enum ExpandStatus {
    None,
    Pack,
    ExpandOnTop,
    ExpandOnBottom,
}

pub(crate) struct RuntimeManager {
    theme: Theme,
    backend: Box<dyn Backend>,
    surface: Surface,
    controls: *mut ControlHandleManager,
    menus: *mut MenuHandleManager,
    timers_manager: TimerManager,
    task_manager: BackgroundTaskManager,
    desktop_handle: Handle<()>,
    tooltip: ToolTip,
    commandbar: Option<CommandBar>,
    appbar: Option<AppBar>,
    recompute_layout: bool,
    repaint: bool,
    mouse_pos: Point,
    key_modifier: KeyModifier,
    desktop_os_start_called: bool,
    recompute_parent_indexes: bool,
    update_command_and_app_bars: bool,
    request_update_timer_threads: bool,
    single_window: bool,
    loop_status: LoopStatus,
    request_focus: Option<Handle<()>>,
    current_focus: Option<Handle<()>>,
    request_default_action: Option<Handle<()>>,
    expanded_control: ExpandedControlInfo,
    mouse_over_control: Handle<()>,
    focus_chain: Vec<Handle<()>>,
    events: Vec<ControlEvent>,
    commandbar_event: Option<CommandBarEvent>,
    menu_event: Option<MenuEvent>,
    appbar_event: Option<AppBarEvent>,
    mouse_locked_object: MouseLockedObject,
    opened_menu_handle: Handle<Menu>,
    modal_windows: Vec<Handle<()>>,
    to_remove_list: Vec<Handle<()>>,
    event_receiver: Receiver<SystemEvent>,
    event_sender: Sender<SystemEvent>,
    #[cfg(feature = "EVENT_RECORDER")]
    event_recorder: super::event_recorder::EventRecorder,
}

#[cfg(feature = "GLOBAL_RUNTIME")]
static mut RUNTIME_MANAGER: Option<RuntimeManager> = None;

#[cfg(not(feature = "GLOBAL_RUNTIME"))]
thread_local! {
    static RUNTIME_MANAGER: RefCell<Option<RuntimeManager>> = const { RefCell::new(None) };
}

impl RuntimeManager {
    pub(super) fn create(mut builder: crate::system::Builder) -> Result<(), super::Error> {
        #[cfg(debug_assertions)]
        if let Some(fname) = builder.log_file.as_ref() {
            crate::utils::log::init_log_file(fname, builder.log_append);
        }

        let (sender, receiver) = std::sync::mpsc::channel::<SystemEvent>();
        let backend_term = backend::new(&builder, sender.clone())?;
        let term_sz = backend_term.size();
        let surface = Surface::new(term_sz.width, term_sz.height);
        let mut manager = RuntimeManager {
            theme: builder.theme,
            backend: backend_term,
            event_receiver: receiver,
            event_sender: sender,
            surface,
            desktop_handle: Handle::new(0),
            tooltip: ToolTip::new(),
            recompute_layout: true,
            repaint: true,
            desktop_os_start_called: false,
            update_command_and_app_bars: true,
            recompute_parent_indexes: true,
            request_update_timer_threads: false,
            single_window: builder.single_window,
            mouse_pos: Point::new(-1, -1),
            key_modifier: KeyModifier::None,
            request_focus: None,
            current_focus: None,
            request_default_action: None,
            mouse_over_control: Handle::None,
            opened_menu_handle: Handle::None,
            expanded_control: ExpandedControlInfo::default(),
            focus_chain: Vec::with_capacity(16),
            events: Vec::with_capacity(16),
            modal_windows: Vec::with_capacity(16),
            to_remove_list: Vec::with_capacity(4),
            commandbar_event: None,
            menu_event: None,
            appbar_event: None,
            controls: Box::into_raw(Box::new(ControlHandleManager::new())),
            timers_manager: TimerManager::new(builder.max_timer_count),
            task_manager: BackgroundTaskManager::new(),
            menus: Box::into_raw(Box::new(MenuHandleManager::new())),
            loop_status: LoopStatus::Normal,
            mouse_locked_object: MouseLockedObject::None,
            commandbar: if builder.has_command_bar {
                Some(CommandBar::new(term_sz.width, term_sz.height))
            } else {
                None
            },
            appbar: if builder.has_app_bar { Some(AppBar::new(term_sz.width)) } else { None },
            #[cfg(feature = "EVENT_RECORDER")]
            event_recorder: super::event_recorder::EventRecorder::new(),
        };
        let mut desktop = if manager.single_window {
            // first check if a desktop was provided - if so panic
            if builder.desktop_manager.is_some() {
                panic!("When `single_window(...)` is being used to initialized an application, you can not use `.desktop(...)` command to provide a custom desktop !");
            }
            // for a single window we will use a custom (empty) desktop
            // that does nothing
            ControlManager::new(EmptyDesktop::new())
        } else if let Some(desktop) = builder.desktop_manager.take() {
            desktop
        } else {
            ControlManager::new(Desktop::new())
        };

        let controls = unsafe { &mut *manager.controls };
        desktop.base_mut().update_focus_flag(true);
        manager.desktop_handle = controls.add(desktop);
        manager.current_focus = Some(manager.desktop_handle);
        controls.get_mut(manager.desktop_handle).unwrap().base_mut().handle = manager.desktop_handle;
        // all good --> add single window if case

        #[cfg(feature = "GLOBAL_RUNTIME")]
        unsafe {
            RUNTIME_MANAGER = Some(manager);
        }

        #[cfg(not(feature = "GLOBAL_RUNTIME"))]
        RUNTIME_MANAGER.set(Some(manager));

        Ok(())
    }
    pub(crate) fn is_instantiated() -> bool {
        #[cfg(feature = "GLOBAL_RUNTIME")]
        unsafe {
            RUNTIME_MANAGER.is_some()
        }

        #[cfg(not(feature = "GLOBAL_RUNTIME"))]
        RUNTIME_MANAGER.with(|manager| manager.borrow().is_some())
    }
    pub(crate) fn get() -> &'static mut RuntimeManager {
        #[cfg(feature = "GLOBAL_RUNTIME")]
        unsafe {
            RUNTIME_MANAGER.as_mut().unwrap()
        }

        #[cfg(not(feature = "GLOBAL_RUNTIME"))]
        RUNTIME_MANAGER.with(|manager| {
            let mut binding = manager.borrow_mut();
            let mut_ref = binding.as_mut().expect("RuntimeManager is not initialized");
            unsafe { &mut *(mut_ref as *mut RuntimeManager) }
        })
    }
    pub(crate) fn terminal_size(&self) -> Size {
        self.backend.size()
    }
    pub(crate) fn get_desktop_rect(&self) -> Rect {
        let sz = self.backend.size();
        Rect::new(
            0,
            if self.appbar.is_some() { 1 } else { 0 },
            (sz.width as i32) - 1,
            if self.commandbar.is_some() {
                (sz.height as i32) - 2
            } else {
                (sz.height as i32) - 1
            },
        )
    }
    pub(crate) fn backend(&self) -> &dyn Backend {
        self.backend.as_ref()
    }
    pub(crate) fn backend_mut(&mut self) -> &mut dyn Backend {
        self.backend.as_mut()
    }

    pub(crate) fn exit_execution_loop(&mut self) {
        self.loop_status = LoopStatus::ExitCurrentLoop;
    }
    pub(crate) fn cancel_exit_from_execution_loop(&mut self) {
        self.loop_status = LoopStatus::Normal;
    }
    pub(crate) fn show_tooltip(&mut self, txt: &str, rect: &Rect) {
        self.tooltip.show(txt, rect, self.backend.size(), &self.theme);
    }
    pub(crate) fn hide_tooltip(&mut self) {
        self.tooltip.hide();
    }
    pub(crate) fn close_opened_menu(&mut self) {
        if !self.opened_menu_handle.is_none() {
            self.opened_menu_handle = Handle::None;
            self.repaint = true;
            // close menu bar (in case the opened menu was part of the menubar)
            if let Some(menubar) = self.appbar.as_mut() {
                menubar.close();
            }
        }
    }

    pub(crate) fn find_first_free_hotkey(&self) -> Key {
        let controls = unsafe { &mut *self.controls };
        if let Some(desktop) = controls.get(self.desktop_handle) {
            let base = desktop.base();
            let mut free_keys: [KeyCode; 9] = [
                KeyCode::N1,
                KeyCode::N2,
                KeyCode::N3,
                KeyCode::N4,
                KeyCode::N5,
                KeyCode::N6,
                KeyCode::N7,
                KeyCode::N8,
                KeyCode::N9,
            ];
            for child_handle in base.children.iter() {
                if let Some(child) = controls.get(*child_handle) {
                    let key = child.base().hotkey;
                    match key.code {
                        KeyCode::N1 => free_keys[0] = KeyCode::None,
                        KeyCode::N2 => free_keys[1] = KeyCode::None,
                        KeyCode::N3 => free_keys[2] = KeyCode::None,
                        KeyCode::N4 => free_keys[3] = KeyCode::None,
                        KeyCode::N5 => free_keys[4] = KeyCode::None,
                        KeyCode::N6 => free_keys[5] = KeyCode::None,
                        KeyCode::N7 => free_keys[6] = KeyCode::None,
                        KeyCode::N8 => free_keys[7] = KeyCode::None,
                        KeyCode::N9 => free_keys[8] = KeyCode::None,
                        _ => {}
                    }
                }
            }
            // search for the first free one
            for k in free_keys {
                if k != KeyCode::None {
                    return Key::new(k, KeyModifier::None);
                }
            }
        }
        Key::None
    }

    pub(crate) fn send_event(&mut self, event: ControlEvent) {
        self.events.push(event);
    }
    pub(crate) fn set_menu_event(&mut self, event: MenuEvent) {
        self.menu_event = Some(event);
    }
    pub(crate) fn set_appbar_event(&mut self, event: AppBarEvent) {
        self.appbar_event = Some(event);
    }
    pub(crate) fn close(&mut self) {
        self.loop_status = LoopStatus::StopApp;
    }
    pub(crate) fn request_focus_for_control(&mut self, handle: Handle<()>) {
        self.request_focus = Some(handle);
    }
    pub(crate) fn request_default_action_for_control(&mut self, handle: Handle<()>) {
        self.request_default_action = Some(handle);
    }
    pub(crate) fn request_expand_for_control(&mut self, handle: Handle<()>, min_size: Size, prefered_size: Size) {
        self.expanded_control.handle = handle;
        self.expanded_control.min_size = min_size;
        self.expanded_control.prefered_size = prefered_size;
        self.request_recompute_layout();
        self.request_repaint();
    }
    pub(crate) fn request_update_command_and_app_bars(&mut self) {
        self.update_command_and_app_bars = true;
        self.repaint = true;
    }
    pub(crate) fn request_update(&mut self) {
        self.update_command_and_app_bars = true;
        self.repaint = true;
        self.recompute_layout = true;
    }
    pub(crate) fn request_remove(&mut self, handle: Handle<()>) {
        if !handle.is_none() {
            self.to_remove_list.push(handle);
        }
    }
    fn set_event_processors(&mut self, control_handle: Handle<()>, event_processor: Handle<()>) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(control_handle) {
            let base = control.base_mut();
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
        if self.single_window && (!controls.desktop_mut().base().children.is_empty()) {
            // check to see how many window were added
            panic!("When `single_window(...)` is being used to initialized an application, you can only use add_window(...) method once (to add the first and single window) !");
        }
        let controls = unsafe { &mut *self.controls };
        let handle = controls.desktop_mut().base_mut().add_child(obj);
        // since it is the first time I register this window
        // I need to recursively set the event processor for all of its childern to
        // this window current handle
        self.set_event_processors(handle.cast(), handle.cast());
        // all good --> the window has been registered
        if let Some(win) = controls.get_mut(handle.cast()) {
            if self.single_window {
                let base = win.base_mut();
                base.set_singlewindow_flag();
                let top = self.appbar.is_some();
                let bottom = self.commandbar.is_some();
                base.layout = ControlLayout::from(match () {
                    _ if (!top) && (!bottom) => layout!("l:0,t:0,r:0,b:0"),
                    _ if (!top) && (bottom) => layout!("l:0,t:0,r:0,b:1"),
                    _ if (top) && (!bottom) => layout!("l:0,t:1,r:0,b:0"),
                    _ if (top) && (bottom) => layout!("l:0,t:1,r:0,b:1"),
                    _ => layout!("l:0,t:0,r:0,b:0"),
                });
            }
            // this must be called last as it will inactivate some flags on a window if in single window mode
            win.control_mut().on_registered();
        }
        self.update_desktop_window_count();
        self.recompute_parent_indexes = true;
        handle
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
            win.control_mut().on_registered();
        }
        // add to modal stack
        if !handle.is_none() {
            self.modal_windows.push(handle);
            // if the existing requested focus is for a child of the modal window - keep-it
            if !self.is_nth_child(handle, self.request_focus.unwrap_or(Handle::None), true) {
                self.request_focus_for_control(handle);
            }
            self.request_update();
        }
        handle.cast()
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
    pub(crate) fn get_system_event_sender(&self) -> std::sync::mpsc::Sender<SystemEvent> {
        self.event_sender.clone()
    }
    #[inline(always)]
    pub(crate) fn get_background_task_manager(&mut self) -> &mut BackgroundTaskManager {
        &mut self.task_manager
    }

    #[inline(always)]
    pub(crate) fn get_menus(&mut self) -> &mut MenuHandleManager {
        unsafe { &mut *self.menus }
    }
    pub(crate) fn add_menu(&mut self, menu: Menu) -> Handle<Menu> {
        self.get_menus().add(menu)
    }
    pub(crate) fn get_menu(&mut self, handle: Handle<Menu>) -> Option<&mut Menu> {
        let menus = unsafe { &mut *self.menus };
        menus.get_mut(handle)
    }
    pub(crate) fn get_appbar(&mut self) -> &mut AppBar {
        self.appbar.as_mut().expect("AppBar (application bar) was not enabled ! Have you forgot to add '.app_bar()' when you initialized the Application ? (e.g. App::new().app_bar().build())")
    }
    pub(crate) fn show_menu(&mut self, handle: Handle<Menu>, receiver_control_handle: Handle<()>, x: i32, y: i32, title_width: u32, max_size: Option<Size>) {
        let menus = unsafe { &mut *self.menus };
        let controls = unsafe { &mut *self.controls };
        if let Some(menu) = menus.get_mut(handle) {
            // 1. make sure that the receiver control is set up for the menu
            menu.set_receiver_control_handle(receiver_control_handle);
            // 2. update menu items handles (link them to the menu parent)
            menu.update_menuitems_menu_handle();
            // 3. call on_menu_opened here
            // we need to call this first because if on_menu_open changes the size of the menu
            // we can recompute its position after this
            if let Some(ctrl) = controls.get(receiver_control_handle) {
                GenericMenuEvents::on_menu_open(ctrl.control(), menu);
            }
            // 4. compute the position and show
            menu.compute_position(x, y, title_width, max_size.unwrap_or(Size::new(0, 0)), self.backend.size());
            self.opened_menu_handle = handle;
        }
    }
    pub(crate) fn activate_opened_menu_parent(&mut self) {
        let menus = unsafe { &mut *self.menus };
        if let Some(menu) = menus.get_mut(self.opened_menu_handle) {
            let parent_handle = menu.get_parent_handle();
            if menus.get(parent_handle).is_some() {
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
            DesktopEvents::on_start(desktop.control_mut());
        }
    }
    pub(crate) fn update_desktop_window_count(&mut self) {
        let controls = unsafe { &mut *self.controls };
        if let Some(desktop) = controls.get_mut(self.desktop_handle.cast()) {
            let count = desktop.base().children.len();
            DesktopEvents::on_update_window_count(desktop.control_mut(), count);
        }
    }

    pub(crate) fn tick(&mut self, single_threaded: bool) {
        // 1. Process events from command bar
        if let Some(event) = self.commandbar_event {
            self.process_commandbar_event(event);
        }

        // 2. Process events from menu
        if let Some(event) = self.menu_event {
            self.process_menu_event(event);
        }

        // 2. Process appbar from menu
        if let Some(event) = self.appbar_event {
            self.process_appbar_event(event);
        }

        // 3. Process events from controls
        if !self.events.is_empty() {
            self.process_events_queue();
        }

        // 4. if there is a control that was removed (due to the previously fired events) remove it
        if !self.to_remove_list.is_empty() {
            self.remove_deleted_controls();
            self.recompute_parent_indexes = true;
            self.update_command_and_app_bars = true;
        }

        // If we reach this point, there should not be any change in the logic of controls
        if self.recompute_parent_indexes {
            self.update_parent_indexes(self.get_root_control_handle());
            self.recompute_parent_indexes = false;
        }

        if let Some(handle) = self.request_focus {
            self.update_focus(handle);
            self.request_focus = None;
            self.request_default_action = None;
            self.repaint = true;
            self.update_command_and_app_bars = true;
        }

        if self.recompute_layout {
            self.recompute_layouts();
        }
        if self.update_command_and_app_bars {
            self.update_command_and_app_bars();
        }
        if self.repaint || self.recompute_layout {
            self.paint();
        }

        self.recompute_layout = false;
        self.repaint = false;

        // timer threads update
        if self.request_update_timer_threads {
            self.timers_manager.update_threads();
            self.request_update_timer_threads = false;
        }

        // auto save changes
        #[cfg(feature = "EVENT_RECORDER")]
        self.event_recorder.auto_update(&self.surface);

        if single_threaded {
            if let Some(sys_event) = self.backend.query_system_event() {
                self.process_system_event(sys_event);
            }
        } else {
            let event = if cfg!(target_arch = "wasm32") {
                self.event_receiver.try_recv().ok()
            } else {
                self.event_receiver.recv().ok()
            };

            if let Some(sys_event) = event {
                self.process_system_event(sys_event);
                #[cfg(feature = "EVENT_RECORDER")]
                self.event_recorder.add(&sys_event, &mut self.backend, &self.surface);
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            if self.loop_status == LoopStatus::ExitCurrentLoop {
                self.exit_loop();
            }
        }
    }

    fn exit_loop(&mut self) {
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

    pub(crate) fn run(&mut self) {
        self.recompute_layout = true;
        self.repaint = true;
        self.recompute_parent_indexes = true;
        self.commandbar_event = None;
        self.menu_event = None;
        let single_threaded = self.backend.is_single_threaded();
        // if first time an execution start
        if !self.desktop_os_start_called {
            self.process_terminal_resize_event(self.backend.size());
            self.process_desktop_on_start();
            if self.single_window && self.get_controls_mut().desktop_mut().base().children.len() != 1 {
                panic!("You can not run a single window app and not add a window to the app. Have you forget to add an '.add_window(...)' call before the .run() call ?")
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            while self.loop_status == LoopStatus::Normal {
                self.tick(single_threaded);
            }
            // loop has ended
            if self.loop_status == LoopStatus::ExitCurrentLoop {
                self.exit_loop();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            use crate::system::App;
            use std::{cell::RefCell, rc::Rc};
            use wasm_bindgen::{closure::Closure, JsCast};
            use web_sys::window;

            self.request_update();

            let window = window().expect("No global `window` exists");
            let window_clone = window.clone();

            let callback_holder = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
            let callback_holder_clone = callback_holder.clone();

            // Create the closure that represents one tick of the animation loop
            *callback_holder.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                let rt = RuntimeManager::get();
                rt.tick(single_threaded);

                if rt.loop_status == LoopStatus::Normal {
                    // Continue the animation loop
                    window_clone
                        .request_animation_frame(callback_holder_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                        .expect("Failed to request animation frame");
                } else if rt.loop_status == LoopStatus::ExitCurrentLoop {
                    rt.exit_loop();
                    App::drop_app();
                } else {
                    App::drop_app();
                }
            }) as Box<dyn FnMut()>));

            window
                .request_animation_frame(callback_holder.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .expect("Failed to request animation frame");
        }
    }
    #[inline(always)]
    fn process_system_event(&mut self, sys_event: SystemEvent) {
        match sys_event {
            SystemEvent::AppClose => self.loop_status = LoopStatus::StopApp,
            SystemEvent::KeyPressed(event) => self.process_keypressed_event(event),
            SystemEvent::KeyModifierChanged(event) => self.process_key_modifier_changed_event(event.new_state),
            SystemEvent::Resize(new_size) => {
                self.backend.on_resize(new_size);
                self.process_terminal_resize_event(new_size);
            }
            SystemEvent::MouseButtonDown(event) => self.process_mousebuttondown_event(event),
            SystemEvent::MouseButtonUp(event) => self.process_mousebuttonup_event(event),
            SystemEvent::MouseDoubleClick(event) => self.process_mouse_dblclick_event(event),
            SystemEvent::MouseMove(event) => self.process_mousemove_event(event),
            SystemEvent::MouseWheel(event) => self.process_mousewheel_event(event),
            SystemEvent::TimerTickUpdate(event) => self.process_timer_tick_update_event(event.id, event.tick.value()),
            SystemEvent::TimerStart(event) => self.process_timer_start_event(event.id, event.tick.value()),
            SystemEvent::TimerPaused(event) => self.process_timer_paused_event(event.id, event.tick.value()),
            SystemEvent::BackgroundTaskStart(h) => BackgroundTaskMethods::on_start(self, h),
            SystemEvent::BackgroundTaskEnd(h) => BackgroundTaskMethods::on_finish(self, h),
            SystemEvent::BackgroundTaskNotify(h) => BackgroundTaskMethods::on_notify(self, h),
            SystemEvent::BackgroundTaskQuery(h) => BackgroundTaskMethods::on_query(self, h),
        }
    }
    fn remove_control(&mut self, handle: Handle<()>, unlink_from_parent: bool) -> (Handle<()>, bool) {
        if handle.is_none() {
            return (Handle::None, false);
        }
        let controls = unsafe { &mut *self.controls };
        let mut parent: Handle<()> = Handle::None;
        let mut has_focus = false;
        let mut is_window_control = false;
        let mut timer_handle: Handle<Timer> = Handle::None;
        // remove the link from its parent if requested
        if unlink_from_parent {
            if let Some(control) = controls.get(handle.cast()) {
                is_window_control = control.base().is_window_control();
                parent = control.base().parent;
                has_focus = control.base().has_focus();
                if let Some(parent) = controls.get_mut(parent.cast()) {
                    let base = parent.base_mut();
                    let count = base.children.len();
                    if let Some(index) = base.children.iter().position(|&elem| elem == handle) {
                        // if the index is bigger than the focused children index --> than all god
                        // otherwise, we need to reset the index
                        // println!("Before removing index {} focus was {}", index, base.focused_child_index.index());
                        // println!("Focus = {:?}", base.children[base.focused_child_index.index()]);
                        if base.focused_child_index.is_valid() {
                            match index.cmp(&base.focused_child_index.index()) {
                                std::cmp::Ordering::Less => {
                                    // we need to decrease the index of the current focused index
                                    base.focused_child_index.sub(1, count, crate::utils::Strategy::Clamp);
                                }
                                std::cmp::Ordering::Equal => base.focused_child_index = VectorIndex::Invalid,
                                std::cmp::Ordering::Greater => { /* do nothing */ }
                            }
                        }
                        // remove from the vector
                        // println!("Remove index: {} - from vector: {:?}", index, base.children);
                        // println!("New Focus index: {}", base.focused_child_index.index());
                        base.children.remove(index);
                        // println!("After removal: {:?}", base.children);
                        // println!("Focus = {:?}", base.children[base.focused_child_index.index()]);
                    }
                }
            }
        }
        // first remove my children, then myself
        if let Some(control) = controls.get_mut(handle.cast()) {
            let base = control.base();
            for child in &base.children {
                self.remove_control(*child, false);
            }
            timer_handle = control.base().timer_handle;
        }
        controls.remove(handle);
        if timer_handle.is_none() {
            self.timers_manager.terminate_thread(timer_handle.index());
        }

        if has_focus {
            (parent, is_window_control)
        } else {
            (Handle::None, is_window_control)
        }
    }
    fn remove_deleted_controls(&mut self) {
        let mut window_removed = false;
        while let Some(handle) = self.to_remove_list.pop() {
            let (_focused_parent, is_window_control) = self.remove_control(handle, true);
            window_removed |= is_window_control;
            // what if the current handle has focus (in this case we will need to change the focus to
            // a different control). self.remove_control should return the handle to its parent only if
            // the current object has focus
            // if focused_parent != Handle::None, we are in this scenario
        }
        if window_removed {
            self.update_desktop_window_count();
        }
    }
    fn get_opened_menu(&mut self) -> Option<&mut Menu> {
        if self.opened_menu_handle.is_none() {
            return None;
        }
        let menus = unsafe { &mut *self.menus };
        menus.get_mut(self.opened_menu_handle)
    }
    #[inline(always)]
    fn get_root_control_handle(&self) -> Handle<()> {
        if self.modal_windows.is_empty() {
            self.desktop_handle
        } else {
            self.modal_windows[self.modal_windows.len() - 1]
        }
    }
    // pub(crate) fn get_parent_handle(&self, handle: Handle<()>) -> Handle<()> {
    //     let controls = unsafe { &mut *self.controls };
    //     if let Some(ctrl) = controls.get(handle) {
    //         return ctrl.get_base().parent;
    //     }
    //     return Handle::None;
    // }
    pub(crate) fn get_focused_control_for_parent(&self, parent_handle: Handle<()>) -> Option<Handle<()>> {
        let controls = unsafe { &mut *self.controls };
        if let Some(ctrl) = controls.get(parent_handle) {
            let base = ctrl.base();
            if !base.is_active() {
                return None;
            }
            if base.focused_child_index.in_range(base.children.len()) {
                let child_handle = base.children[base.focused_child_index.index()];
                let result = self.get_focused_control_for_parent(child_handle);
                if result.is_some() {
                    return result;
                }
            }
            if base.can_receive_input() {
                return Some(parent_handle);
            }
        }
        None
    }
    fn get_focused_control(&self) -> Handle<()> {
        self.get_focused_control_for_parent(self.get_root_control_handle()).unwrap()
        // let controls = unsafe { &mut *self.controls };
        // if let Some(ctrl) = controls.get(parent_handle) {
        //     // at least the parent must be active
        //     if ctrl.get_base().is_active()==false {
        //         return None;
        //     }
        //     let parent = parent_handle;
        //     //let mut parent = self.get_root_control_handle();
        //     //let mut ctrl = controls.get_mut(parent).unwrap();
        //     loop {
        //         let base = ctrl.get_base();
        //         if base.focused_child_index.in_range(base.children.len()) {
        //             let child_handle = base.children[base.focused_child_index.index()];
        //             if let Some(child) = controls.get(child_handle) {
        //                 if child.get_base().can_receive_input() {
        //                     parent = child_handle;
        //                     ctrl = child;
        //                 } else {
        //                     return parent;
        //                 }
        //             } else {
        //                 return parent;
        //             }
        //         } else {
        //             return parent;
        //         }
        //     }
        // }
        // return None;
    }

    fn process_events_queue(&mut self) {
        let controls = unsafe { &mut *self.controls };
        while let Some(evnt) = self.events.pop() {
            if let Some(receiver) = controls.get_mut(evnt.receiver) {
                let result = evnt.invoke(receiver.control_mut());
                self.repaint |= result == EventProcessStatus::Processed;
            }
        }
    }

    fn process_commandbar_event(&mut self, event: CommandBarEvent) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(event.control_receiver_handle) {
            GenericCommandBarEvents::on_event(control.control_mut(), event.command_id);
        }
        self.commandbar_event = None;
    }
    fn process_menu_event(&mut self, event: MenuEvent) {
        let controls = unsafe { &mut *self.controls };
        match event {
            MenuEvent::Command(cmd) => {
                if let Some(control) = controls.get_mut(cmd.control_receiver_handle) {
                    GenericMenuEvents::on_command(control.control_mut(), cmd.menu, cmd.item, cmd.command_id);
                    self.repaint = true;
                }
            }
            MenuEvent::CheckBoxStateChanged(cmd) => {
                if let Some(control) = controls.get_mut(cmd.control_receiver_handle) {
                    GenericMenuEvents::on_check(control.control_mut(), cmd.menu, cmd.item, cmd.command_id, cmd.checked);
                    self.repaint = true;
                }
            }
            MenuEvent::SingleChoiceSelected(cmd) => {
                if let Some(control) = controls.get_mut(cmd.control_receiver_handle) {
                    GenericMenuEvents::on_select(control.control_mut(), cmd.menu, cmd.item, cmd.command_id);
                    self.repaint = true;
                }
            }
        }
        self.menu_event = None;
    }
    fn process_appbar_event(&mut self, event: AppBarEvent) {
        let controls = unsafe { &mut *self.controls };
        match event {
            AppBarEvent::ButtonClick(ev) => {
                if let Some(control) = controls.get_mut(ev.control_receiver_handle) {
                    AppBarEvents::on_button_click(control.control_mut(), ev.button_handle);
                    self.repaint = true;
                }
            }
        }
        self.appbar_event = None;
    }
    fn update_command_and_app_bars(&mut self) {
        if self.commandbar.is_none() && self.appbar.is_none() {
            self.update_command_and_app_bars = false;
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
                control.control_mut().on_update_commandbar(cmdbar);
                h = control.base().parent;
                if h.is_none() {
                    break;
                }
            }
            cmdbar.update_positions();
        }
        // process appbar
        if let Some(appbar) = self.appbar.as_mut() {
            appbar.clear();
            // start from the focused control and call on_update_menubar for each control
            let mut h = focused_handle;
            while let Some(control) = controls.get_mut(h) {
                appbar.set_receiver_control_handle(h);
                AppBarEvents::on_update(control.control(), appbar);
                h = control.base().parent;
                if h.is_none() {
                    break;
                }
            }
            appbar.update_positions();
        }
        self.process_menu_and_cmdbar_mousemove(self.mouse_pos.x, self.mouse_pos.y);
        self.update_command_and_app_bars = false;
    }

    fn find_last_leaf(&mut self, handle: Handle<()>) -> Handle<()> {
        let controls = unsafe { &mut *self.controls };
        let mut result = Handle::None;
        let mut handle = handle;
        while let Some(c) = controls.get_mut(handle) {
            let base = c.base();
            if !base.is_active() {
                break;
            }
            // curent handle is a possible candidate for a valid child focused leaf
            // if it can receive focus, let's make it the new leaf
            if base.can_receive_input() {
                result = handle;
            }
            // since we know it is active, let move towards its focused child
            handle = base.get_focused_control();
        }
        result
    }

    fn is_nth_child(&mut self, parent: Handle<()>, child: Handle<()>, focusable: bool) -> bool {
        if child.is_none() || parent.is_none() {
            return false;
        }
        if child == parent {
            return true;
        }
        let controls = unsafe { &mut *self.controls };
        if focusable {
            // check to see if the child can receive focus
            if let Some(c) = controls.get(child) {
                if !c.base().can_receive_input() {
                    return false;
                }
            }
        }
        let mut handle = child;
        while let Some(c) = controls.get_mut(handle) {
            let base = c.base();
            if focusable && !base.is_active() {
                break;
            }
            if base.parent == parent {
                return true;
            }
            handle = base.parent;
        }
        false
    }

    fn update_focus(&mut self, handle: Handle<()>) {
        // if an expanded control exists --> pack it
        if !self.expanded_control.handle.is_none() {
            self.request_expand_for_control(Handle::None, Size::default(), Size::default());
        }
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
                if !control.base_mut().mark_to_receive_focus() {
                    break true;
                }
                h = control.base().parent;
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
                if control.base().is_marked_to_receive_focus() {
                    break;
                }
                control.base_mut().update_focus_flag(false);
                control.control_mut().on_lose_focus();
                if control.base().is_window_control() {
                    control.control_mut().on_deactivate();
                }
                h = control.base().parent;
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
                let base = control.base_mut();
                let parent_index = base.parent_index;
                let window_control = base.is_window_control();
                base.clear_mark_to_receive_focus();
                if !base.has_focus() {
                    base.update_focus_flag(true);
                    let interface = control.control_mut();
                    interface.on_focus();
                    if window_control {
                        interface.on_activate();
                    }
                }
                if parent_index.is_valid() {
                    if let Some(p_handle) = parent_handle {
                        if let Some(p) = controls.get_mut(p_handle) {
                            let base = p.base_mut();
                            base.focused_child_index = parent_index;
                        }
                    }
                }
            }
            parent_handle = Some(handle);
        }
        self.current_focus = Some(handle);
        self.request_focus = None;
        // check default actio
        if handle == self.request_default_action.unwrap_or(Handle::None) {
            if let Some(c) = controls.get_mut(handle) {
                OnDefaultAction::on_default_action(c.control_mut());
            }
        }
    }

    fn update_parent_indexes(&mut self, handle: Handle<()>) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(handle) {
            let base = control.base_mut();
            for i in 0..base.children.len() {
                let child_handle = base.children[i];
                if let Some(child) = unsafe { (*self.controls).get_mut(child_handle) } {
                    child.base_mut().parent_index = VectorIndex::with_value(i);
                    self.update_parent_indexes(child_handle);
                }
            }
        }
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
        if let Some(menubar) = self.appbar.as_mut() {
            menubar.update_width(new_size.width);
        }
        // resize the desktop as well
        let desktop = self.get_controls_mut().desktop_mut();
        let original_size = desktop.base().size();
        desktop.base_mut().set_size(new_size.width as u16, new_size.height as u16);
        desktop.control_mut().on_resize(original_size, new_size);
        self.recompute_layout = true;
    }

    // fn debug_print(&self, handle: Handle<()>, depth: i32) {
    //     println!("----------------------------- Control Tree -----------------------------");
    //     for _ in 0..depth {
    //         print!(" ");
    //     }
    //     let base = self.get_controls().get(handle).unwrap().base();
    //     if base.parent_index.is_valid() {
    //         print!("{}. ", base.parent_index.index());
    //     } else {
    //         print!("?.");
    //     }
    //     //print!("[ID:{},Index:{}],", handle.get_id(), handle.get_index());
    //     print!("  Children: {}", base.children.len());
    //     if base.focused_child_index.is_valid() {
    //         print!("  Focused-child-index:{}", base.focused_child_index.index());
    //     } else {
    //         print!("  Focused-child-index:Invalid");
    //     }
    //     print!("  Focus:{}", base.has_focus());
    //     if base.is_window_control() {
    //         print!("  [Window]");
    //     }
    //     if base.is_desktop_control() {
    //         print!("  [Desktop]");
    //     }

    //     if depth == 0 {
    //         println!(" --> [ROOT]");
    //     } else {
    //         println!();
    //     }
    //     for handle in base.children.iter() {
    //         self.debug_print(*handle, depth + 2);
    //     }
    // }

    pub(super) fn destroy() {
        // save all records to a file
        #[cfg(feature = "EVENT_RECORDER")]
        RuntimeManager::get().event_recorder.save();

        #[cfg(feature = "GLOBAL_RUNTIME")]
        unsafe {
            RUNTIME_MANAGER = None;
        }

        #[cfg(not(feature = "GLOBAL_RUNTIME"))]
        RUNTIME_MANAGER.set(None);
    }
}

impl LayoutMethods for RuntimeManager {
    fn recompute_layouts(&mut self) {
        let term_layout = ParentLayout::from(&self.backend);
        self.update_control_layout(self.desktop_handle, &term_layout);
        let count = self.modal_windows.len();
        for index in 0..count {
            let handle = self.modal_windows[index];
            self.update_control_layout(handle, &term_layout);
        }
    }
    fn update_control_layout(&mut self, handle: Handle<()>, parent_layout: &ParentLayout) {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(handle) {
            let base = control.base_mut();
            let window_control = base.is_window_control();
            let old_size = base.size();
            let old_pos = base.position();
            let expanded = base.is_expanded();
            let mut expand_status = ExpandStatus::None;
            base.update_control_layout_and_screen_origin(parent_layout);
            if expanded {
                if handle != self.expanded_control.handle {
                    // need to pack myself as there is another expamded control
                    base.set_expand_flag(false);
                    expand_status = ExpandStatus::Pack;
                }
            } else if handle == self.expanded_control.handle {
                // need to compute my expended size
                // also I need to set my internal flags to expanded
                let termsize = self.terminal_size();
                if let Some(dir) = base.update_expanded_layout(self.expanded_control.prefered_size, self.expanded_control.min_size, termsize) {
                    base.set_expand_flag(true);
                    expand_status = match dir {
                        ExpandedDirection::OnTop => ExpandStatus::ExpandOnTop,
                        ExpandedDirection::OnBottom => ExpandStatus::ExpandOnBottom,
                    };
                } else {
                    self.expanded_control.handle = Handle::None; // clear expanded handle
                }
            }
            let new_size = base.size();
            let new_pos = base.position();
            let interface = control.control_mut();
            // expand events
            match expand_status {
                ExpandStatus::None => {}
                ExpandStatus::Pack => interface.on_pack(),
                ExpandStatus::ExpandOnTop => interface.on_expand(ExpandedDirection::OnTop),
                ExpandStatus::ExpandOnBottom => interface.on_expand(ExpandedDirection::OnBottom),
            }
            // if size has been changed --> call on_resize
            if new_size != old_size {
                interface.on_resize(old_size, new_size);
            }
            // just for window
            if window_control && ((new_size != old_size) || (old_pos != new_pos)) {
                // call the window specific event
                WindowEvents::on_layout_changed(
                    interface,
                    Rect::with_point_and_size(old_pos, old_size),
                    Rect::with_point_and_size(new_pos, new_size),
                );
            }
            // process the same thing for its children
            let base = control.base();
            if !base.children.is_empty() {
                let my_layout = ParentLayout::from(base);
                for child_handle in &base.children {
                    self.update_control_layout(*child_handle, &my_layout)
                }
            }
        }
    }
    fn request_recompute_layout(&mut self) {
        self.recompute_layout = true;
    }
}
impl PaintMethods for RuntimeManager {
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
        if !self.expanded_control.handle.is_none() {
            self.paint_control(self.expanded_control.handle);
        }
        self.surface.reset();
        if self.commandbar.is_some() {
            self.commandbar.as_ref().unwrap().paint(&mut self.surface, &self.theme);
        }
        if self.appbar.is_some() {
            self.appbar.as_ref().unwrap().paint(&mut self.surface, &self.theme);
        }
        if self.tooltip.is_visible() {
            self.tooltip.paint(&mut self.surface, &self.theme);
        }
        if !self.opened_menu_handle.is_none() {
            self.surface.reset();
            self.paint_menu(self.opened_menu_handle, true);
        }
        self.backend.update_screen(&self.surface);
    }
    fn paint_control(&mut self, handle: Handle<()>) {
        let controls = unsafe { &mut *self.controls };
        if let Some(element) = controls.get_mut(handle) {
            let base = element.base();
            if base.prepare_paint(&mut self.surface) {
                // paint is possible
                element.control().on_paint(&mut self.surface, &self.theme);
                let children_count = base.children.len();
                if base.focused_child_index.in_range(children_count) {
                    // draw from the next visible element until
                    let focus_index = base.focused_child_index.index();
                    for i in 0..children_count {
                        let index = (focus_index + 1 + i) % children_count;
                        self.paint_control(base.children[index]);
                    }
                } else {
                    for child_handle in base.children.iter() {
                        self.paint_control(*child_handle);
                    }
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
    fn request_repaint(&mut self) {
        self.repaint = true;
    }
}
impl KeyboardMethods for RuntimeManager {
    fn process_key_modifier_changed_event(&mut self, new_state: KeyModifier) {
        self.key_modifier = new_state;
        if let Some(commandbar) = self.commandbar.as_mut() {
            commandbar.set_key_modifier(new_state);
            self.repaint = true;
        }
    }

    fn process_keypressed_event(&mut self, event: KeyPressedEvent) {
        // 1. check for a menu on_key_event
        if let Some(menu) = self.get_opened_menu() {
            // 1.1. check current menu open opened key process
            if menu.on_key_pressed(event.key) == EventProcessStatus::Processed {
                self.repaint = true;
                return;
            }
            // 1.2. if menubar is opened (e.g. the current menu is part of the menu bar )
            if let Some(menubar) = self.appbar.as_mut() {
                if menubar.is_opened() && menubar.on_key_event(event.key, true) == EventProcessStatus::Processed {
                    self.repaint = true;
                    return;
                }
            }
        }
        // 2. check controls
        if self.process_control_keypressed_event(self.get_root_control_handle(), event.key, event.character) == EventProcessStatus::Processed {
            self.repaint = true;
            return;
        };
        // 3. check cmdbar
        if let Some(cmdbar) = self.commandbar.as_mut() {
            self.commandbar_event = cmdbar.get_event(event.key);
            if self.commandbar_event.is_some() {
                self.repaint = true;
                return;
            }
        }
        // 4. check the menubar
        if let Some(menubar) = self.appbar.as_mut() {
            if menubar.on_key_event(event.key, false) == EventProcessStatus::Processed {
                self.repaint = true;
                //return;
            }
        }
    }
    fn process_control_keypressed_event(&mut self, handle: Handle<()>, key: Key, character: char) -> EventProcessStatus {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(handle) {
            let base = control.base();
            if !base.is_active() {
                return EventProcessStatus::Ignored;
            }
            if base.should_receive_keyinput_before_children() {
                if base.can_receive_input() && control.control_mut().on_key_pressed(key, character) == EventProcessStatus::Processed {
                    return EventProcessStatus::Processed;
                }
                let base = control.base();
                if base.focused_child_index.in_range(base.children.len()) {
                    let handle_child = base.children[base.focused_child_index.index()];
                    return self.process_control_keypressed_event(handle_child, key, character);
                }
            } else {
                if base.focused_child_index.in_range(base.children.len()) {
                    let handle_child = base.children[base.focused_child_index.index()];
                    if self.process_control_keypressed_event(handle_child, key, character) == EventProcessStatus::Processed {
                        return EventProcessStatus::Processed;
                    }
                }
                // else --> call it ourselves
                if base.can_receive_input() {
                    return control.control_mut().on_key_pressed(key, character);
                }
            }
        }

        EventProcessStatus::Ignored
    }
}
impl MouseMethods for RuntimeManager {
    fn coordinates_to_child_control(&mut self, handle: Handle<()>, x: i32, y: i32, ignore_expanded: bool) -> Handle<()> {
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(handle) {
            let base = control.base_mut();
            if !base.is_active() {
                return Handle::None;
            }

            if let Some(v) = base.should_increase_margins_on_focus() {
                if ignore_expanded {
                    if !base.screen_clip.contains(x, y) {
                        return Handle::None;
                    }
                } else {
                    // if the control has focus, then check if the margins were not extended to include a
                    // scrollbar or a different component
                    if !base.screen_clip.is_visible() {
                        return Handle::None;
                    }
                    if base.can_receive_input() {
                        let inc_right_margin = (v & 1) != 0;
                        let inc_bottom_margin = (v & 2) != 0;
                        if (inc_right_margin)
                            && (x == base.screen_clip.right + 1)
                            && (y >= base.screen_clip.top + base.top_components_margin as i32)
                            && (y <= base.screen_clip.bottom)
                        {
                            // located on the external right margin
                            return handle;
                        }
                        if (inc_bottom_margin)
                            && (y == base.screen_clip.bottom + 1)
                            && (x >= base.screen_clip.left + base.left_components_margin as i32)
                            && (x < base.screen_clip.right)
                        {
                            // located on the external bottom margin
                            return handle;
                        }
                    }
                    if !base.screen_clip.contains(x, y) {
                        return Handle::None;
                    }
                }
            } else if !base.screen_clip.contains(x, y) {
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
                    let result = self.coordinates_to_child_control(base.children[idx], x, y, ignore_expanded);
                    if !result.is_none() {
                        return result;
                    }
                    idx = (idx + count - 1) % count;
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
    fn coordinates_to_control(&mut self, x: i32, y: i32, ignore_expanded: bool) -> Handle<()> {
        // if an expanded control exists --> check it first
        if !self.expanded_control.handle.is_none() {
            let handle = self.coordinates_to_child_control(self.expanded_control.handle, x, y, ignore_expanded);
            if !handle.is_none() {
                return handle;
            }
        }
        // check the focused one
        if self.current_focus.is_some() {
            let handle = self.current_focus.unwrap();
            let handle = self.coordinates_to_child_control(handle, x, y, ignore_expanded);
            if !handle.is_none() {
                return handle;
            }
        }
        // check from root
        self.coordinates_to_child_control(self.get_root_control_handle(), x, y, ignore_expanded)
    }

    fn process_menu_and_cmdbar_mousemove(&mut self, x: i32, y: i32) -> bool {
        let mut processed = false;
        // Process event in the following order:
        // first the context menu and its owner, then the menu bar and then cmdbar
        if let Some(menu) = self.get_opened_menu() {
            match menu.on_mouse_move(x, y) {
                menu::events::MouseMoveMenuResult::ProcessedAndRepaint => {
                    self.repaint = true;
                    return true;
                }
                menu::events::MouseMoveMenuResult::RepaintAndPass => self.repaint = true,
                menu::events::MouseMoveMenuResult::ProcessWithoutRepaint => return true, // process it but no repaint needed.
                menu::events::MouseMoveMenuResult::Ignored => {}
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

        if let Some(menubar) = self.appbar.as_mut() {
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
                    let response = control.control_mut().on_mouse_event(&MouseEvent::Leave);
                    self.repaint |= response == EventProcessStatus::Processed;
                    control.base_mut().update_mouse_over_flag(false);
                }
                self.mouse_over_control = Handle::None;
            }
        }
        processed
    }

    fn process_menu_mouse_click(&mut self, handle: Handle<Menu>, x: i32, y: i32) -> bool {
        // returns true if the mouse was processed by a menu or the menu parent or false if (x,y) are outside any menu
        let mut result = MousePressedMenuResult::None;
        let mut parent_handle = Handle::None;
        let mut processed = false;
        let menus = unsafe { &mut *self.menus };
        if let Some(menu) = menus.get_mut(handle) {
            parent_handle = menu.get_parent_handle();
            if handle == self.opened_menu_handle {
                result = menu.on_mouse_pressed(x, y);
            } else {
                result = if menu.is_on_menu(x, y) {
                    MousePressedMenuResult::Activate
                } else {
                    MousePressedMenuResult::CheckParent
                };
            }
        }
        match result {
            MousePressedMenuResult::None => {}
            MousePressedMenuResult::Repaint => {
                self.repaint = true;
                processed = true;
            }
            MousePressedMenuResult::CheckParent => {
                if !parent_handle.is_none() {
                    processed = self.process_menu_mouse_click(parent_handle, x, y);
                } else {
                    self.close_opened_menu();
                    processed = false;
                }
            }
            MousePressedMenuResult::Activate => {
                self.repaint = true;
                self.opened_menu_handle = handle;
                if let Some(menu) = menus.get_mut(handle) {
                    // trigger an on_mouse_move to force selection
                    menu.on_mouse_move(x, y);
                }
                processed = true;
            }
        }
        processed
    }

    fn process_mousewheel_event(&mut self, event: MouseWheelEvent) {
        // update mouse position
        self.mouse_pos.x = event.x;
        self.mouse_pos.y = event.y;

        if let Some(menu) = self.get_opened_menu() {
            self.repaint |= menu.on_mouse_wheel(event.direction) == EventProcessStatus::Processed;
            return;
        }
        match self.mouse_locked_object {
            MouseLockedObject::None => {}
            _ => return,
        }
        let handle = self.coordinates_to_control(event.x, event.y, false);
        if !handle.is_none() {
            let controls = unsafe { &mut *self.controls };
            if let Some(control) = controls.get_mut(handle) {
                self.repaint |= control.control_mut().on_mouse_event(&MouseEvent::Wheel(event.direction)) == EventProcessStatus::Processed;
            }
        }
    }
    fn process_mousedrag(&mut self, handle: Handle<()>, event: MouseMoveEvent) {
        self.hide_tooltip();
        let controls = unsafe { &mut *self.controls };
        if let Some(control) = controls.get_mut(handle) {
            let base = control.base_mut();
            let scr_x = base.screen_clip.left;
            let scr_y = base.screen_clip.top;
            let response = control.control_mut().on_mouse_event(&MouseEvent::Drag(MouseEventData {
                x: event.x - scr_x,
                y: event.y - scr_y,
                button: event.button,
                modifier: self.key_modifier,
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
        let handle = self.coordinates_to_control(event.x, event.y, false);
        if handle != self.mouse_over_control {
            self.hide_tooltip();
            if !self.mouse_over_control.is_none() {
                if let Some(control) = controls.get_mut(self.mouse_over_control) {
                    let response = control.control_mut().on_mouse_event(&MouseEvent::Leave);
                    self.repaint |= response == EventProcessStatus::Processed;
                    control.base_mut().update_mouse_over_flag(false);
                }
            }
            self.mouse_over_control = handle;
            if !self.mouse_over_control.is_none() {
                if let Some(control) = controls.get_mut(self.mouse_over_control) {
                    let base = control.base_mut();
                    base.update_mouse_over_flag(true);
                    let scr_x = base.screen_clip.left;
                    let scr_y = base.screen_clip.top;
                    let response = control.control_mut().on_mouse_event(&MouseEvent::Enter);
                    self.repaint |= response == EventProcessStatus::Processed;
                    let response = control
                        .control_mut()
                        .on_mouse_event(&MouseEvent::Over(Point::new(event.x - scr_x, event.y - scr_y)));
                    self.repaint |= response == EventProcessStatus::Processed;
                }
            }
        } else if !self.mouse_over_control.is_none() {
            if let Some(control) = controls.get_mut(handle) {
                let base = control.base();
                let scr_x = base.screen_clip.left;
                let scr_y = base.screen_clip.top;
                let response = control
                    .control_mut()
                    .on_mouse_event(&MouseEvent::Over(Point::new(event.x - scr_x, event.y - scr_y)));
                self.repaint |= response == EventProcessStatus::Processed;
            }
        }
    }
    fn process_mousemove_event(&mut self, event: MouseMoveEvent) {
        // update mouse position
        self.mouse_pos.x = event.x;
        self.mouse_pos.y = event.y;

        match self.mouse_locked_object {
            MouseLockedObject::None => self.process_mousemove(event),
            MouseLockedObject::Control(handle) => self.process_mousedrag(handle, event),
            MouseLockedObject::CommandBar => {}
            MouseLockedObject::AppBar => {}
        }
    }
    fn process_mousebuttondown_event(&mut self, event: MouseButtonDownEvent) {
        // update mouse position
        self.mouse_pos.x = event.x;
        self.mouse_pos.y = event.y;
        // Hide ToolTip
        self.hide_tooltip();
        // check contextual menu
        if !self.opened_menu_handle.is_none() {
            if self.process_menu_mouse_click(self.opened_menu_handle, event.x, event.y) {
                return;
            }
        }
        // check appbar
        if let Some(appbar) = self.appbar.as_mut() {
            if appbar.on_mouse_pressed(event.x, event.y) == EventProcessStatus::Processed {
                self.repaint = true;
                self.mouse_locked_object = MouseLockedObject::AppBar;
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
        let handle = self.coordinates_to_control(event.x, event.y, false);
        if !handle.is_none() {
            let controls = unsafe { &mut *self.controls };
            if let Some(control) = controls.get_mut(handle) {
                // only update focus if the control does not already have focus or if it has at least one child
                // this avoids packing controls while they have focus and are expanded
                if !control.base().has_focus() || !control.base().children.is_empty() {
                    self.update_focus(handle);
                }
                let base = control.base();
                let scr_x = base.screen_clip.left;
                let scr_y = base.screen_clip.top;
                //let has_margins = base.should_increase_margins_on_focus().is_some();

                let _ = control.control_mut().on_mouse_event(&MouseEvent::Pressed(MouseEventData {
                    x: event.x - scr_x,
                    y: event.y - scr_y,
                    button: event.button,
                    modifier: self.key_modifier,
                }));
                //if response == EventProcessStatus::Processed {
                self.mouse_locked_object = MouseLockedObject::Control(handle);
                self.repaint = true;
                self.update_command_and_app_bars = true;
                return;
                //}
            }
        }
        self.mouse_locked_object = MouseLockedObject::None;
    }
    fn process_mousebuttonup_event(&mut self, event: MouseButtonUpEvent) {
        // update mouse position
        self.mouse_pos.x = event.x;
        self.mouse_pos.y = event.y;

        // check contextual menus
        if let Some(menu) = self.get_opened_menu() {
            self.repaint |= menu.on_mouse_released(event.x, event.y) == EventProcessStatus::Processed;
        }
        match self.mouse_locked_object {
            MouseLockedObject::None => {}
            MouseLockedObject::Control(handle) => {
                let controls = unsafe { &mut *self.controls };
                if let Some(control) = controls.get_mut(handle) {
                    let base = control.base();
                    let scr_x = base.screen_clip.left;
                    let scr_y = base.screen_clip.top;
                    control.control_mut().on_mouse_event(&MouseEvent::Released(MouseEventData {
                        x: event.x - scr_x,
                        y: event.y - scr_y,
                        button: event.button,
                        modifier: self.key_modifier,
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
            MouseLockedObject::AppBar => {
                if let Some(appbar) = self.appbar.as_mut() {
                    appbar.on_mouse_released(event.x, event.y);
                }
                self.repaint = true;
            }
        }
        self.mouse_locked_object = MouseLockedObject::None;
    }
    fn process_mouse_dblclick_event(&mut self, event: MouseDoubleClickEvent) {
        // update mouse position
        self.mouse_pos.x = event.x;
        self.mouse_pos.y = event.y;

        // Hide ToolTip
        self.hide_tooltip();
        // check for a control
        let handle = self.coordinates_to_control(event.x, event.y, false);
        if !handle.is_none() {
            let controls = unsafe { &mut *self.controls };
            if let Some(control) = controls.get_mut(handle) {
                self.update_focus(handle);
                let base = control.base();
                let scr_x = base.screen_clip.left;
                let scr_y = base.screen_clip.top;
                //let has_margins = base.should_increase_margins_on_focus().is_some();

                let response = control.control_mut().on_mouse_event(&MouseEvent::DoubleClick(MouseEventData {
                    x: event.x - scr_x,
                    y: event.y - scr_y,
                    button: event.button,
                    modifier: self.key_modifier,
                }));
                if response == EventProcessStatus::Processed {
                    self.mouse_locked_object = MouseLockedObject::None;
                    self.repaint = true;
                    self.update_command_and_app_bars = true;
                }
            }
        }
    }
}
impl ThemeMethods for RuntimeManager {
    #[inline(always)]
    fn theme(&self) -> &Theme {
        &self.theme
    }
    #[inline(always)]
    fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
        self.update_theme();
        self.repaint = true;
        self.recompute_layout = true;
    }

    fn update_theme(&mut self) {
        // notify desktop and its children
        self.update_theme_for_control(self.desktop_handle);
        // notify modal windows (if any)
        let count = self.modal_windows.len();
        for idx in 0..count {
            self.update_theme_for_control(self.modal_windows[idx]);
        }
    }

    fn update_theme_for_control(&mut self, handle: Handle<()>) {
        let controls = unsafe { &mut *self.controls };
        if let Some(element) = controls.get_mut(handle) {
            OnThemeChanged::on_theme_changed(element.control_mut(), &self.theme);
            let base = element.base();
            for child_handle in base.children.iter() {
                self.update_theme_for_control(*child_handle);
            }
        }
    }
}
impl TimerMethods for RuntimeManager {
    #[inline(always)]
    fn get_timer_manager(&mut self) -> &mut TimerManager {
        &mut self.timers_manager
    }
    #[inline(always)]
    fn request_timer_threads_update(&mut self) {
        self.request_update_timer_threads = true;
    }
    #[inline(always)]
    fn timer_id_to_control(&mut self, id: u8) -> Option<&mut ControlManager> {
        let h = self.timers_manager.control_handle(id);
        if h.is_none() {
            None
        } else {
            let controls = unsafe { &mut *self.controls };
            controls.get_mut(h.cast())
        }
    }

    fn process_timer_tick_update_event(&mut self, id: u8, tick: u64) {
        if let Some(cm) = self.timer_id_to_control(id) {
            if TimerEvents::on_update(cm.control_mut(), tick) == EventProcessStatus::Processed {
                self.repaint = true;
            }
        } else {
            // invalid control (should terminate the timer)
            self.timers_manager.terminate_thread(id as usize);
        }
    }

    fn process_timer_paused_event(&mut self, id: u8, tick: u64) {
        if let Some(timer) = self.timers_manager.index_mut(id) {
            timer.set_pause_state();
        } else {
            // if timer is invalid -> ignore the event
            return;
        };
        if let Some(cm) = self.timer_id_to_control(id) {
            if TimerEvents::on_pause(cm.control_mut(), tick) == EventProcessStatus::Processed {
                self.repaint = true;
            }
        } else {
            // invalid control (should terminate the timer)
            self.timers_manager.terminate_thread(id as usize);
        }
    }

    fn process_timer_start_event(&mut self, id: u8, tick: u64) {
        if let Some(timer) = self.timers_manager.index_mut(id) {
            timer.set_running_state();
        } else {
            // if timer is invalid -> ignore the event
            return;
        };
        if let Some(cm) = self.timer_id_to_control(id) {
            let result = if tick == 0 {
                TimerEvents::on_start(cm.control_mut())
            } else {
                TimerEvents::on_resume(cm.control_mut(), tick)
            };
            if result == EventProcessStatus::Processed {
                self.repaint = true;
            }
        } else {
            // invalid control (should terminate the timer)
            self.timers_manager.terminate_thread(id as usize);
        }
    }
}

impl BackgroundTaskMethods for RuntimeManager {
    #[inline(always)]
    fn background_task_handle_to_control(&mut self, backgroundtask_handle: Handle<()>) -> Option<&mut ControlManager> {
        //log!("RM","background_task_handle_to_control({:?})",backgroundtask_handle);
        if let Some(h) = self.task_manager.receiver_control_handle(backgroundtask_handle.index()) {
            let controls = unsafe { &mut *self.controls };
            controls.get_mut(h.cast())
        } else {
            None
        }
    }

    fn on_start(&mut self, backgroundtask_handle: Handle<()>) {
        if let Some(c) = self.background_task_handle_to_control(backgroundtask_handle) {
            if GenericBackgroundTaskEvents::on_start(c.control_mut(), backgroundtask_handle) == EventProcessStatus::Processed {
                self.repaint = true;
            }
        }
    }

    fn on_notify(&mut self, backgroundtask_handle: Handle<()>) {
        if let Some(c) = self.background_task_handle_to_control(backgroundtask_handle) {
            if GenericBackgroundTaskEvents::on_update(c.control_mut(), backgroundtask_handle) == EventProcessStatus::Processed {
                self.repaint = true;
            }
        }
    }

    fn on_finish(&mut self, backgroundtask_handle: Handle<()>) {
        if let Some(c) = self.background_task_handle_to_control(backgroundtask_handle) {
            if GenericBackgroundTaskEvents::on_finish(c.control_mut(), backgroundtask_handle) == EventProcessStatus::Processed {
                self.repaint = true;
            }
        }
        self.task_manager.remove_task(backgroundtask_handle);
    }

    fn on_query(&mut self, backgroundtask_handle: Handle<()>) {
        if let Some(c) = self.background_task_handle_to_control(backgroundtask_handle) {
            if GenericBackgroundTaskEvents::on_query(c.control_mut(), backgroundtask_handle) == EventProcessStatus::Processed {
                self.repaint = true;
            }
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
