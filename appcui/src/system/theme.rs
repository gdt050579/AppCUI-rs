use crate::{graphics::*, ui::common::ControlCharAttributesState};

#[derive(Default)]
pub struct DesktopTheme {
    pub character: Character,
}
#[derive(Default)]
pub struct TextTheme {
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
pub struct ToolTipTheme {
    pub text: CharAttribute,
    pub arrow: CharAttribute,
}
#[derive(Default)]
pub struct SymbolTheme {
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
pub struct MenuTheme {
    pub text: ControlCharAttributesState,
    pub hotkey: ControlCharAttributesState,
    pub shortcut: ControlCharAttributesState,
    pub symbol: ControlCharAttributesState,
}
#[derive(Default)]
pub struct WindowTheme {
    pub normal: CharAttribute,
    pub inactive: CharAttribute,
    pub error: CharAttribute,
    pub warning: CharAttribute,
    pub info: CharAttribute,
}
#[derive(Default)]
pub struct ButtonTheme {
    pub text: ControlCharAttributesState,
    pub hotkey: ControlCharAttributesState,
    pub shadow: CharAttribute,
}
#[derive(Default)]
pub struct TabTheme {
    pub text: ControlCharAttributesState,
    pub hotkey: ControlCharAttributesState,
    pub list: ControlCharAttributesState,
    pub listhotkey: ControlCharAttributesState,
}
#[derive(Default)]
pub struct ScrollBarTheme {
    pub arrow: ControlCharAttributesState,
    pub bar: ControlCharAttributesState,
    pub position: ControlCharAttributesState,
}
#[derive(Default)]
pub struct Theme {
    pub desktop: DesktopTheme,
    pub text: TextTheme,
    pub symbol: SymbolTheme,
    pub tooltip: ToolTipTheme,
    pub menu: MenuTheme,
    pub parent_menu: MenuTheme,
    pub window: WindowTheme,
    pub border: ControlCharAttributesState,
    pub lines: ControlCharAttributesState,
    pub button: ButtonTheme,
    pub tab: TabTheme,
    pub scrollbar: ScrollBarTheme,
    pub editor: ControlCharAttributesState
}
impl Theme {
    pub(crate) fn new() -> Self {
        let mut t = Theme::default();
        t.set_regular_theme();
        t
    }
    fn set_regular_theme(&mut self) {
        self.desktop.character = Character::new(SpecialChar::Block50, Color::Gray, Color::Black, CharFlags::None);

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

        self.window.inactive = CharAttribute::with_color(Color::Black, Color::Black);
        self.window.normal = CharAttribute::with_color(Color::Black, Color::DarkBlue);
        self.window.error = CharAttribute::with_color(Color::Black, Color::DarkRed);
        self.window.warning = CharAttribute::with_color(Color::Black, Color::Olive);
        self.window.info = CharAttribute::with_color(Color::Black, Color::DarkGreen);

        self.border = ControlCharAttributesState {
            normal: CharAttribute::with_fore_color(Color::Silver),
            focused: CharAttribute::with_fore_color(Color::White),
            hovered: CharAttribute::with_fore_color(Color::Yellow),
            inactive: CharAttribute::with_fore_color(Color::Gray),
            pressed_or_selectd: CharAttribute::with_color(Color::Yellow, Color::Magenta),
        };

        self.lines = ControlCharAttributesState {
            normal: CharAttribute::with_fore_color(Color::DarkGreen),
            focused: CharAttribute::with_fore_color(Color::DarkGreen),
            hovered: CharAttribute::with_fore_color(Color::Gray),
            inactive: CharAttribute::with_fore_color(Color::Gray),
            pressed_or_selectd: CharAttribute::with_color(Color::Yellow, Color::Magenta),
        };

        self.button.text = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::Black, Color::Gray),
            focused: CharAttribute::with_color(Color::Black, Color::White),
            hovered: CharAttribute::with_color(Color::Black, Color::Yellow),
            inactive: CharAttribute::with_color(Color::Gray, Color::Black),
            pressed_or_selectd: CharAttribute::with_color(Color::Black, Color::Olive),
        };
        self.button.hotkey = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::DarkRed, Color::Gray),
            focused: CharAttribute::with_color(Color::Magenta, Color::White),
            hovered: CharAttribute::with_color(Color::Magenta, Color::Yellow),
            inactive: CharAttribute::with_color(Color::Gray, Color::Black),
            pressed_or_selectd: CharAttribute::with_color(Color::DarkRed, Color::Olive),
        };
        self.button.shadow = CharAttribute::with_fore_color(Color::Black);

        self.tab.text = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::Black, Color::Gray),
            focused: CharAttribute::with_color(Color::White, Color::Gray),
            hovered: CharAttribute::with_color(Color::Black, Color::Silver),
            inactive: CharAttribute::with_color(Color::Gray, Color::Transparent),
            pressed_or_selectd: CharAttribute::with_color(Color::White, Color::Blue),
        };
        self.tab.hotkey = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::Yellow, Color::Gray),
            focused: CharAttribute::with_color(Color::DarkRed, Color::Gray),
            hovered: CharAttribute::with_color(Color::DarkRed, Color::Silver),
            inactive: CharAttribute::with_color(Color::Gray, Color::Transparent),
            pressed_or_selectd: CharAttribute::with_color(Color::Yellow, Color::Blue),
        };
        self.tab.list = self.tab.text;
        self.tab.list.pressed_or_selectd = CharAttribute::with_color(Color::Black, Color::White);
        self.tab.listhotkey = self.tab.hotkey;
        self.tab.listhotkey.pressed_or_selectd = CharAttribute::with_color(Color::DarkRed, Color::White);

        self.scrollbar.arrow = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::White, Color::DarkBlue),
            focused: CharAttribute::with_color(Color::White, Color::Teal),
            hovered: CharAttribute::with_color(Color::Yellow, Color::DarkBlue),
            inactive: CharAttribute::with_color(Color::Gray, Color::Transparent),
            pressed_or_selectd: CharAttribute::with_color(Color::White, Color::Teal),
        };
        self.scrollbar.bar = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::White, Color::DarkBlue),
            focused: CharAttribute::with_color(Color::White, Color::Teal),
            hovered: CharAttribute::with_color(Color::Yellow, Color::DarkBlue),
            inactive: CharAttribute::with_color(Color::Gray, Color::Transparent),
            pressed_or_selectd: CharAttribute::with_color(Color::White, Color::Teal),
        };
        self.scrollbar.position = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::Silver, Color::DarkBlue),
            focused: CharAttribute::with_color(Color::Green, Color::Teal),
            hovered: CharAttribute::with_color(Color::Yellow, Color::DarkBlue),
            inactive: CharAttribute::with_color(Color::Gray, Color::Transparent),
            pressed_or_selectd: CharAttribute::with_color(Color::Green, Color::Teal),
        };

        self.editor = ControlCharAttributesState {
            normal: CharAttribute::with_color(Color::Silver, Color::Black),
            focused: CharAttribute::with_color(Color::White, Color::Black),
            hovered: CharAttribute::with_color(Color::Yellow, Color::Black),
            inactive: CharAttribute::with_color(Color::Gray, Color::Transparent),
            pressed_or_selectd: CharAttribute::with_color(Color::White, Color::Black),
        }
    }
}
//         inline void Set(focused, normal, inactive, hovered, pressedOrSelected)