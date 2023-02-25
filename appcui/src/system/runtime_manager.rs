use super::Theme;
use crate::controls::control_manager::ParentLayout;
use crate::controls::events::Control;
use crate::controls::ControlManager;
use crate::controls::*;
use crate::graphics::Surface;
use crate::terminal::*;

pub(crate) struct RuntimeManager {
    theme: Theme,
    terminal: Box<dyn Terminal>,
    surface: Surface,
    root: ControlManager,
}

static mut RUNTIME_MANAGER: Option<RuntimeManager> = None;

impl RuntimeManager {
    pub(super) fn create() {
        let term =
            TerminalType::new(TerminalType::Debug).expect("Unable to create a terminal object !");
        let surface = Surface::new(term.get_width(), term.get_height());
        let manager = RuntimeManager {
            theme: Theme::new(),
            terminal: term,
            surface: surface,
            root: ControlManager::new(Desktop::new()),
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
        let v = c.get_version();
        self.root.get_base_mut().children.push(c);
        return ControlHandle::new(0, v);
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

    fn recompute_layouts(&mut self) {
        let term_layout = ParentLayout::from(&self.terminal);
        self.root.update_layout(&term_layout);
    }

    fn paint(&mut self) {
        self.root.paint(&mut self.surface, &self.theme);
        self.terminal.update_screen(&self.surface);
    }

    fn process_keypressed_event(&mut self, event: KeyPressedEvent) {
        self.root
            .process_keypressed_event(event.key, event.character);
    }
}
