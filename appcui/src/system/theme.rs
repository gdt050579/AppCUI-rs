use crate::{controls::ControlCharAttributesState, graphics::*};

#[derive(Default)]
pub struct Desktop {
    pub character: Character,
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
pub struct ToolTip {
    pub text: CharAttribute,
    pub arrow: CharAttribute,
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
pub struct Menu {
    pub text: ControlCharAttributesState,
    pub hotkey: ControlCharAttributesState,
    pub shortcut: ControlCharAttributesState,
    pub symbol: ControlCharAttributesState,
}

#[derive(Default)]
pub struct Theme {
    pub desktop: Desktop,
    pub text: Text,
    pub symbol: Symbol,
    pub tooltip: ToolTip,
    pub menu: Menu,
    pub parent_menu: Menu,
}
impl Theme {
    pub(crate) fn new() -> Self {
        let mut t = Theme::default();
        t.set_regular_theme();
        return t;
    }
    fn set_regular_theme(&mut self) {
        self.desktop.character = Character::new(
            SpecialChar::Block50,
            Color::Gray,
            Color::Black,
            CharFlags::None,
        );

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

        self.tooltip.text = CharAttribute::with_color(Color::Black, Color::Aqua);
        self.tooltip.arrow = CharAttribute::with_color(Color::Green, Color::Black);

        self.menu.text = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::Black, Color::White),
            focused: CharAttribute::with_color(Color::Black, Color::White),
            hovered: CharAttribute::with_color(Color::Black, Color::Silver),
            inactive: CharAttribute::with_color(Color::Gray, Color::White),
            pressed_or_selectd: CharAttribute::with_color(Color::Yellow, Color::Magenta),
        };
        self.menu.hotkey = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::DarkRed, Color::White),
            focused: CharAttribute::with_color(Color::DarkRed, Color::White),
            hovered: CharAttribute::with_color(Color::DarkRed, Color::Silver),
            inactive: CharAttribute::with_color(Color::Gray, Color::White),
            pressed_or_selectd: CharAttribute::with_color(Color::White, Color::Magenta),
        };
        self.menu.symbol = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::DarkGreen, Color::White),
            focused: CharAttribute::with_color(Color::DarkGreen, Color::White),
            hovered: CharAttribute::with_color(Color::Magenta, Color::Silver),
            inactive: CharAttribute::with_color(Color::Gray, Color::White),
            pressed_or_selectd: CharAttribute::with_color(Color::White, Color::Magenta),
        };
        self.menu.shortcut = self.menu.hotkey;

        self.parent_menu.text = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::Black, Color::Silver),
            focused: CharAttribute::with_color(Color::Black, Color::Silver),
            hovered: CharAttribute::with_color(Color::Black, Color::Gray),
            inactive: CharAttribute::with_color(Color::Gray, Color::Silver),
            pressed_or_selectd: CharAttribute::with_color(Color::Yellow, Color::Gray),
        };
        self.parent_menu.hotkey = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::DarkRed, Color::Silver),
            focused: CharAttribute::with_color(Color::DarkRed, Color::Silver),
            hovered: CharAttribute::with_color(Color::DarkRed, Color::Gray),
            inactive: CharAttribute::with_color(Color::Gray, Color::Silver),
            pressed_or_selectd: CharAttribute::with_color(Color::White, Color::Gray),
        };    
        self.parent_menu.symbol = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::DarkGreen, Color::Silver),
            focused: CharAttribute::with_color(Color::DarkGreen, Color::Silver),
            hovered: CharAttribute::with_color(Color::Magenta, Color::Gray),
            inactive: CharAttribute::with_color(Color::Gray, Color::Silver),
            pressed_or_selectd: CharAttribute::with_color(Color::White, Color::Gray),
        };     
        self.parent_menu.shortcut = self.parent_menu.hotkey;
    }
}
