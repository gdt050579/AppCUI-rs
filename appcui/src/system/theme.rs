use crate::graphics::*;

#[derive(Default)]
pub struct Desktop {
    pub character: Character
}
#[derive(Default)]
pub struct Theme {
    pub desktop: Desktop,
}
impl Theme {
    fn new()->Self {
        let mut t = Theme::default();
        t.set_regular_theme();
        return t;
    }
    fn set_regular_theme(&mut self) {
        self.desktop.character = Character::new(SpecialChar::Block50,Color::Gray, Color::Black, CharFlags::None);
    }
}