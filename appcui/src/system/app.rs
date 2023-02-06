use super::Theme;
use crate::controls::ControlWrapper;
use crate::controls::events::Control;
use crate::controls::*;
use crate::graphics::{ClipArea, Point, Surface};
use crate::input::Key;
use crate::terminal::*;

pub struct App {
    theme: Theme,
    terminal: Box<dyn Terminal>,
    surface: Surface,
    root_control: ControlWrapper,
}

impl App {
    pub fn new() -> Self {
        let term =
            TerminalType::new(TerminalType::WindowsConsole).expect("Unable to create a terminal object !");
        let surface = Surface::new(term.get_width(), term.get_height());
        App {
            theme: Theme::new(),
            terminal: term,
            surface: surface,
            root_control: ControlWrapper::new(Desktop::new()),
        }        
    }
    pub fn run(mut self) {
        // must pe self so that after a run a second call will not be possible
        self.recompute_layouts();
        self.paint();
        let sys_event = self.terminal.get_system_event();
        match sys_event {
            SystemEvent::None => {},
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
        let client = ClipArea::new(
            0,
            0,
            (self.terminal.get_width() as i32) - 1,
            (self.terminal.get_height() as i32) - 1,
        );
        self.root_control.get_manager_mut().update_layout(
            &client,
            Point::default(),
            self.terminal.get_width() as u16,
            self.terminal.get_height() as u16,
        );
    }

    fn paint(&mut self) {
        self.root_control.paint(&mut self.surface, &self.theme);
        self.terminal.update_screen(&self.surface);
    }
    fn process_keypressed_event(&mut self, event: KeyPressedEvent) {
        // check if desktop can process this event
        if self.root_control.process_keypressed_event(event.key, event.character) {
            return;
        }
    }
}
