use super::Theme;
use crate::controls::events::Control;
use crate::controls::*;
use crate::graphics::{ClipArea, Point, Surface};
use crate::terminal::*;

pub struct App {
    theme: Theme,
    terminal: Box<dyn Terminal>,
    surface: Surface,
    root_control: Box<dyn Control>,
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
            root_control: Box::new(Desktop::new()),
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
    fn paint(&mut self) {
        self.root_control
            .get_mut_basic_control()
            .paint(&mut self.root_control, &mut self.surface, &self.theme);
        self.terminal.update_screen(&self.surface);
    }
}
