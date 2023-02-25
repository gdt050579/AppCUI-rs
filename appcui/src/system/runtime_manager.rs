use super::Theme;
use crate::controls::events::{Control, EventProcessStatus};
use crate::controls::ControlManager;
use crate::controls::*;
use crate::graphics::{ClipArea, Point, Surface};
use crate::input::Key;
use crate::terminal::*;

pub(crate) struct RuntimeManager {
    theme: Theme,
    terminal: Box<dyn Terminal>,
    surface: Surface,
    controls: Vec<Option<ControlManager>>,
}

static mut RUNTIME_MANAGER: Option<RuntimeManager> = None;

impl RuntimeManager {
    pub(super) fn create() {
        let term =
            TerminalType::new(TerminalType::Debug).expect("Unable to create a terminal object !");
        let surface = Surface::new(term.get_width(), term.get_height());
        let mut manager = RuntimeManager {
            theme: Theme::new(),
            terminal: term,
            surface: surface,
            controls: Vec::with_capacity(128),
        };
        unsafe {
            RUNTIME_MANAGER = Some(manager);
        }
    }
    pub(crate) fn get() -> &'static mut RuntimeManager {
        unsafe { RUNTIME_MANAGER.as_mut().unwrap() }
    }
    pub(crate) fn add<T>(&mut self, obj: T) -> ControlHandle<T>
    where
        T: Control + 'static,
    {
        let c = ControlManager::new(obj);
        return ControlHandle::new(0, c.get_version());
    }
    pub(crate) fn run(&mut self) {
        // must pe self so that after a run a second call will not be possible
        self.recompute_layouts();
        self.paint();
        let sys_event = self.terminal.get_system_event();
        match sys_event {
            SystemEvent::None => {}
            SystemEvent::AppClose => todo!(),
            SystemEvent::KeyPressed(event) => self.process_keypressed_event(event),
            SystemEvent::KeyModifierChanged(_) => todo!(),
            SystemEvent::Resize(_) => todo!(),
            SystemEvent::MouseButtonDown(_) => todo!(),
            SystemEvent::MouseButtonUp(_) => todo!(),
            SystemEvent::MouseDoubleClick(_) => todo!(),
            SystemEvent::MouseMove(_) => todo!(),
            SystemEvent::MouseWheel(_) => todo!(),
        }
    }
    fn get_desktop(&mut self) -> &mut ControlManager {
        return self.controls[0].as_mut().unwrap();
    }
    fn get_control_mut(&mut self, index: usize) -> Option<&mut ControlManager> {
        if index >= self.controls.len() {
            return None;
        }
        let c = &mut self.controls[index];
        if c.is_none() {
            return None;
        }
        return c.as_mut();
    }
    fn recompute_control_layouts(
        &mut self,
        parent_clip: &ClipArea,
        parent_origin: Point,
        parent_width: u16,
        parent_height: u16,
        index: usize,
    ) {
        if let Some(c) = self.get_control_mut(index) {
            let base = c.get_base_mut();
            base.update_control_layout_and_screen_origin(
                parent_clip,
                parent_origin,
                parent_width,
                parent_height,
            );
            // process the same thing for its children
            let base = c.get_base();
            let client_clip = base.get_client_clip();
            let w = base.get_width();
            let h = base.get_height();
            let p = base.screen_origin;
            for c_idx in &base.children {
                self.recompute_control_layouts(&client_clip, p, w, h, *c_idx as usize);
            }
        }
    }
    fn recompute_layouts(&mut self) {
        let client = ClipArea::new(
            0,
            0,
            (self.terminal.get_width() as i32) - 1,
            (self.terminal.get_height() as i32) - 1,
        );
        self.recompute_control_layouts(
            &client,
            Point::default(),
            self.terminal.get_width() as u16,
            self.terminal.get_height() as u16,
            0,
        );
    }

    fn paint_control(&mut self, index: usize) {
        if let Some(c) = self.get_control_mut(index) {
            c.get_base().prepare_paint(&mut self.surface);
            c.get_control().on_paint(&mut self.surface, &self.theme);
            for c_idx in c.get_base().children {
                self.paint_control(c_idx as usize);
            }
        }
    }
    fn paint(&mut self) {
        self.paint_control(0); // desktop
        self.terminal.update_screen(&self.surface);
    }
    fn process_control_keypressed_event(
        &mut self,
        key: Key,
        character: char,
        index: usize,
    ) -> EventProcessStatus {
        let mut run_through_interface = false;
        if let Some(c) = self.get_control_mut(index) {
            let base = c.get_base_mut();
            if base.can_receive_input() == false {
                return EventProcessStatus::Ignored;
            }
            let focused_child_index = base.focused_child_index as usize;
            if focused_child_index >= base.children.len() {
                return EventProcessStatus::Ignored;
            }
            let child_index = base.children[focused_child_index] as usize;
            if self.process_control_keypressed_event(key, character, child_index)
                == EventProcessStatus::Processed
            {
                return EventProcessStatus::Processed;
            }
            // else --> call it ourselves
            run_through_interface = true;
        }
        if run_through_interface {
            self.get_control_mut(index)
                .unwrap()
                .get_control_mut()
                .on_key_pressed(key, character);
        }
        return EventProcessStatus::Ignored;
    }
    fn process_keypressed_event(&mut self, event: KeyPressedEvent) {
        self.process_control_keypressed_event(event.key, event.character, 0);
    }
}
