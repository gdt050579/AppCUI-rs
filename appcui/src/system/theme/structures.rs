use crate::{graphics::*, ui::common::ControlCharAttributesState};

pub enum Themes {
    Default,
    DarkGray,
}

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
}
#[derive(Default)]
pub struct AccordionTheme {
    pub text: ControlCharAttributesState,
    pub hotkey: ControlCharAttributesState,
}
#[derive(Default)]
pub struct ScrollBarTheme {
    pub arrow: ControlCharAttributesState,
    pub bar: ControlCharAttributesState,
    pub position: ControlCharAttributesState,
}

#[derive(Default)]
pub struct SearchBarTheme {
    pub normal: CharAttribute,
    pub focused: CharAttribute,
    pub count: CharAttribute,
}

#[derive(Default)]
pub struct ListCurentItemTheme {
    pub focus: CharAttribute,
    pub over_inactive: CharAttribute,
    pub over_selection: CharAttribute,
    pub normal: CharAttribute,
    pub selected: CharAttribute,
    pub icon: CharAttribute,
}

#[derive(Default)]
pub struct HeaderTheme {
    pub text: ControlCharAttributesState,
    pub hotkey: ControlCharAttributesState,
    pub symbol: ControlCharAttributesState,
}

#[derive(Default)]
pub struct ToggleButtonTheme {
    pub selected: ControlCharAttributesState,
    pub unselected: ControlCharAttributesState,
}


#[derive(Default)]
pub struct MarkdownTheme {
    pub text: CharAttribute,
    pub bold: CharAttribute,
    pub italic: CharAttribute,
    pub link: CharAttribute,
    pub code: CharAttribute,
    pub h1: CharAttribute,
    pub h2: CharAttribute,
    pub h3: CharAttribute,
    pub code_block: CharAttribute,
    pub ordered_list: CharAttribute,
    pub unordered_list: CharAttribute
}

#[derive(Default)]
pub struct Theme {
    pub accordion: AccordionTheme,
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
    pub searchbar: SearchBarTheme,
    pub editor: ControlCharAttributesState,
    pub list_current_item: ListCurentItemTheme,
    pub header: HeaderTheme,
    pub toggle_button: ToggleButtonTheme,
    pub markdown: MarkdownTheme,
}
impl Theme {
    pub fn new(theme: Themes) -> Self {
        match theme {
            Themes::Default => super::default::new(),
            Themes::DarkGray => super::dark_gray::new(),            
        }
    }
}
