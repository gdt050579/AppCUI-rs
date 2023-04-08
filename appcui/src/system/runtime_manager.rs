use super::{CommandBar, ControlsVector, InitializationData, InitializationFlags, Theme, ToolTip};
use crate::controls::control_manager::ParentLayout;
use crate::controls::events::{Control, EventProcessStatus};
use crate::controls::menu::{Menu, MenuBar};
use crate::controls::ControlManager;
use crate::controls::*;
use crate::graphics::{Rect, Size, Surface};
use crate::input::{Key, KeyModifier};
use crate::terminal::*;
use crate::utils::Caption;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum LoopStatus {
    Normal,
    StopApp,
    StopCurrent,
}

pub(crate) struct RuntimeManager {
    theme: Theme,
    terminal: Box<dyn Terminal>,
    surface: Surface,
    controls: *mut ControlsVector,
    desktop_handler: Handle,
    tooltip: ToolTip,
    commandbar: Option<CommandBar>,
    menubar: Option<MenuBar>,
    recompute_layout: bool,
    repaint: bool,
    loop_status: LoopStatus,
    request_focus: Option<Handle>,
    current_focus: Option<Handle>,
    focus_chain: Vec<Handle>,
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
            request_focus: None,
            current_focus: None,
            focus_chain: Vec::with_capacity(16),
            controls: Box::into_raw(Box::new(ControlsVector::new())),
            loop_status: LoopStatus::Normal,
            commandbar: if data.flags.contains(InitializationFlags::CommandBar) {
                Some(CommandBar::new(width, height))
            } else {
                None
            },
            menubar: if data.flags.contains(InitializationFlags::Menu) {
                None
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
    pub(crate) fn get_controls(&self) -> &mut ControlsVector {
        unsafe { &mut *self.controls }
    }
    pub(crate) fn add_menu(&mut self, menu: Menu, caption: Caption) {
        if self.menubar.is_some() {
            self.menubar.as_mut().unwrap().add(menu, caption);
        }
    }
    pub(crate) fn run(&mut self) {
        // must pe self so that after a run a second call will not be possible
        self.recompute_layout = true;
        self.repaint = true;
        while self.loop_status == LoopStatus::Normal {
            if let Some(handle) = self.request_focus {
                self.update_focus(handle);
                self.request_focus = None;
            }
            if self.recompute_layout {
                self.recompute_layouts();
            }
            if self.repaint | self.recompute_layout {
                self.paint();
            }
            self.recompute_layout = false;
            self.repaint = false;
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
                    break false;
                }
                if let Some(parent) = control.get_base().parent {
                    h = parent;
                } else {
                    break true;
                }
            } else {
                break false;
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
        while let Some(handle) = self.focus_chain.pop() {
            if let Some(control) = controls.get(handle) {
                let base = control.get_base_mut();
                base.clear_mark_to_receive_focus();
                if base.has_focus() {
                    continue;
                }
                base.update_focus_flag(true);
                control.get_control_mut().on_focus();
            }
        }
        self.current_focus = Some(handle);
        self.request_focus = None;
    }

    fn recompute_layouts(&mut self) {
        let term_layout = ParentLayout::from(&self.terminal);
        self.update_control_layout(self.desktop_handler, &term_layout);
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

    fn process_key_modifier_changed_event(&mut self, new_state: KeyModifier) {
        if let Some(commandbar) = self.commandbar.as_mut() {
            commandbar.set_key_modifier(new_state);
            self.repaint = true;
        }
    }

    fn process_keypressed_event(&mut self, event: KeyPressedEvent) {
        self.process_control_keypressed_event(self.desktop_handler, event.key, event.character);
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
            let focused_child_index = base.focused_child_index as usize;
            if focused_child_index >= base.children.len() {
                return EventProcessStatus::Ignored;
            }
            let handle_child = base.children[focused_child_index];
            if self.process_control_keypressed_event(handle_child, key, character)
                == EventProcessStatus::Processed
            {
                return EventProcessStatus::Processed;
            }
            // else --> call it ourselves
            return control.get_control_mut().on_key_pressed(key, character);
        }

        return EventProcessStatus::Ignored;
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
    fn process_mousewheel_event(&mut self, _event: MouseWheelEvent) {}
    fn process_mousemove_event(&mut self, _event: MouseMoveEvent) {}
    fn process_mousebuttondown_event(&mut self, _event: MouseButtonDownEvent) {}
    fn process_mousebuttonup_event(&mut self, _event: MouseButtonUpEvent) {}
    fn process_mouse_dblclick_event(&mut self, _event: MouseDoubleClickEvent) {}
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
