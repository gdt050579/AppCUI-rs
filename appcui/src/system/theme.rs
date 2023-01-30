use crate::graphics::*;

#[derive(Default)]
pub struct Desktop {
    pub character: Character
}
#[derive(Default)]
pub struct Text {
    pub normal: CharAttribute,
    pub hot_key: CharAttribute,
    pub inactive: CharAttribute,
    pub error: CharAttribute,
    pub warning: CharAttribute,
    pub hovered: CharAttribute,
    pub focused: CharAttribute,
    pub highlighted: CharAttribute,
    pub enphasized_1: CharAttribute,
    pub enphasized_2: CharAttribute,
    pub enphasized_3: CharAttribute,
}
#[derive(Default)]
pub struct Symbol {
    pub inactive: CharAttribute,
    pub hovered: CharAttribute,
    pub pressed: CharAttribute,
    pub checked: CharAttribute,
    pub unchecked: CharAttribute,
    pub unknown: CharAttribute,
    pub arrows: CharAttribute,
    pub close: CharAttribute,
    pub maximized: CharAttribute,
    pub resize: CharAttribute,
}
#[derive(Default)]
pub struct Theme {
    pub desktop: Desktop,
    pub text: Text,
    pub symbol: Symbol,
}
impl Theme {
    fn new()->Self {
        let mut t = Theme::default();
        t.set_regular_theme();
        return t;
    }
    fn set_regular_theme(&mut self) {
        self.desktop.character = Character::new(SpecialChar::Block50,Color::Gray, Color::Black, CharFlags::None);
        
        self.text.normal = CharAttribute::with_fore_color(Color::Silver);
        self.text.error = CharAttribute::with_fore_color(Color::Red);
        self.text.warning = CharAttribute::with_fore_color(Color::Olive);
        self.text.focused = CharAttribute::with_fore_color(Color::White);
        self.text.inactive = CharAttribute::with_fore_color(Color::Gray);
        self.text.hot_key = CharAttribute::with_fore_color(Color::Aqua);
        self.text.hovered = CharAttribute::with_fore_color(Color::Yellow);
        self.text.highlighted = CharAttribute::with_fore_color(Color::Yellow);
        self.text.enphasized_1 = CharAttribute::with_fore_color(Color::Aqua);
        self.text.enphasized_2 = CharAttribute::with_fore_color(Color::Green);
        self.text.enphasized_3 = CharAttribute::with_fore_color(Color::Pink);

        self.symbol.inactive = CharAttribute::with_fore_color(Color::Gray);
        self.symbol.hovered = CharAttribute::with_color(Color::Black, Color::Yellow);
        self.symbol.pressed = CharAttribute::with_color(Color::Black, Color::Silver);
        self.symbol.checked = CharAttribute::with_fore_color(Color::Green);
        self.symbol.unchecked = CharAttribute::with_fore_color(Color::Red);
        self.symbol.unknown = CharAttribute::with_fore_color(Color::Olive);
        self.symbol.arrows = CharAttribute::with_fore_color(Color::Aqua);
        self.symbol.close = CharAttribute::with_fore_color(Color::Red);
        self.symbol.maximized = CharAttribute::with_fore_color(Color::Aqua);
        self.symbol.resize = CharAttribute::with_fore_color(Color::Aqua);

    }
}