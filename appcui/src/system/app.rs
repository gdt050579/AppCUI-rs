use super::Theme;
use crate::graphics::Surface;
use crate::terminal::*;
pub struct App
{
    theme: Theme,
    terminal: Box<dyn Terminal>,
    surface: Surface,
}
impl App {
    pub fn new()->Self {
        let term = TerminalType::new(TerminalType::WindowsConsole).expect("Unable to create a terminal object !");
        let surface = Surface::new(term.get_width(),term.get_height());
        App {
            theme: Theme::default(),
            terminal: term,
            surface: surface
        }
    }
    pub fn run(self) {
        // must pe self so that after a run a second call will not be possible
    }



}