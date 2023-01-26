use super::Theme;
use crate::controls::ControlWrapper;
use crate::controls::events::Control;
use crate::controls::*;
use crate::graphics::{ClipArea, Point, Surface};
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
            TerminalType::new(TerminalType::Debug).expect("Unable to create a terminal object !");
        let surface = Surface::new(term.get_width(), term.get_height());
        App {
            theme: Theme::default(),
            terminal: term,
            surface: surface,
            root_control: ControlWrapper::,
        }
    }
    pub fn run(mut self) {
        // must pe self so that after a run a second call will not be possible
        self.recompute_layouts();
        self.paint();

    }

    fn recompute_layouts(&mut self) {
        let client = ClipArea::new(
            0,
            0,
            (self.terminal.get_width() as i32) - 1,
            (self.terminal.get_height() as i32) - 1,
        );
        self.root_control.get_mut_basic_control().update_layout(
            &client,
            Point::default(),
            self.terminal.get_width() as u16,
            self.terminal.get_height() as u16,
        );
    }
    fn paint_control(control: &mut Box<dyn Control>, surface: &mut Surface, theme: &Theme) {
        control.get_basic_control().prepare_paint(surface);
        control.on_paint(surface, theme);
        // paint all children
        // should be painted in a specific order
        for c in control.get_mut_basic_control().children.iter_mut() {
            App::paint_control(c, surface, theme);
        }
    }
    fn paint(&mut self) {
        App::paint_control(&mut self.root_control, &mut self.surface, &self.theme);
        self.terminal.update_screen(&self.surface);
    }
}
