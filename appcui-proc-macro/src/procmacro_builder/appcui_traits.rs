use super::templates;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub(crate) enum AppCUITrait {
    Deref = 0,
    // markers
    Control = 1,
    DesktopControl = 2,
    WindowControl = 3,
    NotWindow = 4,
    NotModalWindow = 5,
    NotDesktop = 6,
    OnWindowRegistered = 7,
    ModalWindowMethods = 8,
    // raw events
    OnPaint = 9,
    OnKeyPressed = 10,
    OnMouseEvent = 11,
    OnDefaultAction = 12,
    OnResize = 13,
    OnFocus = 14,
    OnExpand = 15,
    OnSiblingSelected = 16,
    OnThemeChanged = 17,
    // control events
    ButtonEvents = 18,
    CheckBoxEvents = 19,
    WindowEvents = 20,
    CommandBarEvents = 21,
    MenuEvents = 22,
    DesktopEvents = 23,
    ToolBarEvents = 24,
    ColorPickerEvents = 25,
    ThreeStateBoxEvents = 26,
    RadioBoxEvents = 27,
    PasswordEvents = 28,
    KeySelectorEvents = 29,
    TextFieldEvents = 30,
    CustomEvents = 31,
    GenericSelectorEvents = 32,
    ComboBoxEvents = 33,
    GenericDropDownListEvents = 34,
    GenericNumericSelectorEvents = 35,
    DatePickerEvents = 36,
    ListBoxEvents = 37,
    GenericListViewEvents = 38,
    ToggleButtonEvents = 39,
    PathFinderEvents = 40,
    TimerEvents = 41,
    GenericTreeViewEvents = 42,
    MarkdownEvents = 43,
    GenericBackgroundTaskEvents = 44,
    AccordionEvents = 45,
    TabEvents = 46,
    CharPickerEvents = 47,
    GenericGraphViewEvents = 48,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum TraitType {
    RawEvent,
    ControlEvent,
    Other,
}

impl AppCUITrait {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            AppCUITrait::Deref => "Deref",
            // markers
            AppCUITrait::Control => "Control",
            AppCUITrait::DesktopControl => "DesktopControl",
            AppCUITrait::WindowControl => "WindowControl",
            AppCUITrait::NotDesktop => "NotDesktop",
            AppCUITrait::NotWindow => "NotWindow",
            AppCUITrait::NotModalWindow => "NotModalWindw",
            AppCUITrait::OnWindowRegistered => "OnWindowRegistered",
            AppCUITrait::ModalWindowMethods => "ModalWindowMethods",
            // raw events
            AppCUITrait::OnPaint => "OnPaint",
            AppCUITrait::OnKeyPressed => "OnKeyPressed",
            AppCUITrait::OnMouseEvent => "OnMouseEvent",
            AppCUITrait::OnDefaultAction => "OnDefaultAction",
            AppCUITrait::OnResize => "OnResize",
            AppCUITrait::OnFocus => "OnFocus",
            AppCUITrait::OnExpand => "OnExpand",
            AppCUITrait::OnSiblingSelected => "OnSiblingSelected",
            AppCUITrait::OnThemeChanged => "OnThemeChanged",
            // control events
            AppCUITrait::ButtonEvents => "ButtonEvents",
            AppCUITrait::CheckBoxEvents => "CheckBoxEvents",
            AppCUITrait::WindowEvents => "WindowEvents",
            AppCUITrait::CommandBarEvents => "CommandBarEvents",
            AppCUITrait::MenuEvents => "MenuEvents",
            AppCUITrait::DesktopEvents => "DesktopEvents",
            AppCUITrait::ToolBarEvents => "ToolBarEvents",
            AppCUITrait::ColorPickerEvents => "ColorPickerEvents",
            AppCUITrait::ThreeStateBoxEvents => "ThreeStateBoxEvents",
            AppCUITrait::RadioBoxEvents => "RadioBoxEvents",
            AppCUITrait::PasswordEvents => "PasswordEvents",
            AppCUITrait::KeySelectorEvents => "KeySelectorEvents",
            AppCUITrait::TextFieldEvents => "TextFieldEvents",
            AppCUITrait::CustomEvents => "CustomEvents",
            AppCUITrait::GenericSelectorEvents => "SelectorEvents", // important to be without Generic
            AppCUITrait::ComboBoxEvents => "ComboBoxEvents",
            AppCUITrait::GenericDropDownListEvents => "DropDownListEvents", // important to be without Generic
            AppCUITrait::GenericNumericSelectorEvents => "NumericSelectorEvents", // important to be without Generic
            AppCUITrait::DatePickerEvents => "DatePickerEvents",
            AppCUITrait::ListBoxEvents => "ListBoxEvents",
            AppCUITrait::GenericListViewEvents => "ListViewEvents", // important to be without Generic
            AppCUITrait::ToggleButtonEvents => "ToggleButtonEvents",
            AppCUITrait::PathFinderEvents => "PathFinderEvents",
            AppCUITrait::TimerEvents => "TimerEvents",
            AppCUITrait::GenericTreeViewEvents => "TreeViewEvents", // important to be without Generic
            AppCUITrait::MarkdownEvents => "MarkdownEvents",
            AppCUITrait::GenericBackgroundTaskEvents => "BackgroundTaskEvents", // important to be without Generic
            AppCUITrait::AccordionEvents => "AccordionEvents",
            AppCUITrait::TabEvents => "TabEvents",
            AppCUITrait::CharPickerEvents => "CharPickerEvents",
            AppCUITrait::GenericGraphViewEvents => "GraphViewEvents", // important to be without Generic

        }
    }
    pub(crate) fn trait_type(&self) -> TraitType {
        match self {
            AppCUITrait::Deref => TraitType::Other,
            // markers
            AppCUITrait::Control => TraitType::Other,
            AppCUITrait::DesktopControl => TraitType::Other,
            AppCUITrait::WindowControl => TraitType::Other,
            AppCUITrait::NotDesktop => TraitType::Other,
            AppCUITrait::NotWindow => TraitType::Other,
            AppCUITrait::NotModalWindow => TraitType::Other,
            AppCUITrait::OnWindowRegistered => TraitType::Other,
            AppCUITrait::ModalWindowMethods => TraitType::Other,
            // raw events
            AppCUITrait::OnPaint => TraitType::RawEvent,
            AppCUITrait::OnKeyPressed => TraitType::RawEvent,
            AppCUITrait::OnMouseEvent => TraitType::RawEvent,
            AppCUITrait::OnDefaultAction => TraitType::RawEvent,
            AppCUITrait::OnResize => TraitType::RawEvent,
            AppCUITrait::OnFocus => TraitType::RawEvent,
            AppCUITrait::OnExpand => TraitType::RawEvent,
            AppCUITrait::OnSiblingSelected => TraitType::RawEvent,
            AppCUITrait::OnThemeChanged => TraitType::RawEvent,
            // control events
            AppCUITrait::ButtonEvents => TraitType::ControlEvent,
            AppCUITrait::CheckBoxEvents => TraitType::ControlEvent,
            AppCUITrait::WindowEvents => TraitType::ControlEvent,
            AppCUITrait::CommandBarEvents => TraitType::ControlEvent,
            AppCUITrait::MenuEvents => TraitType::ControlEvent,
            AppCUITrait::DesktopEvents => TraitType::ControlEvent,
            AppCUITrait::ToolBarEvents => TraitType::ControlEvent,
            AppCUITrait::ColorPickerEvents => TraitType::ControlEvent,
            AppCUITrait::ThreeStateBoxEvents => TraitType::ControlEvent,
            AppCUITrait::RadioBoxEvents => TraitType::ControlEvent,
            AppCUITrait::PasswordEvents => TraitType::ControlEvent,
            AppCUITrait::KeySelectorEvents => TraitType::ControlEvent,
            AppCUITrait::TextFieldEvents => TraitType::ControlEvent,
            AppCUITrait::CustomEvents => TraitType::ControlEvent,
            AppCUITrait::GenericSelectorEvents => TraitType::ControlEvent,
            AppCUITrait::ComboBoxEvents => TraitType::ControlEvent,
            AppCUITrait::GenericDropDownListEvents => TraitType::ControlEvent,
            AppCUITrait::GenericNumericSelectorEvents => TraitType::ControlEvent,
            AppCUITrait::DatePickerEvents => TraitType::ControlEvent,
            AppCUITrait::ListBoxEvents => TraitType::ControlEvent,
            AppCUITrait::GenericListViewEvents => TraitType::ControlEvent,
            AppCUITrait::ToggleButtonEvents => TraitType::ControlEvent,
            AppCUITrait::PathFinderEvents => TraitType::ControlEvent,
            AppCUITrait::TimerEvents => TraitType::ControlEvent,
            AppCUITrait::GenericTreeViewEvents => TraitType::ControlEvent,
            AppCUITrait::MarkdownEvents => TraitType::ControlEvent,
            AppCUITrait::GenericBackgroundTaskEvents => TraitType::ControlEvent,
            AppCUITrait::AccordionEvents => TraitType::ControlEvent,
            AppCUITrait::TabEvents => TraitType::ControlEvent,
            AppCUITrait::CharPickerEvents => TraitType::ControlEvent,
            AppCUITrait::GenericGraphViewEvents => TraitType::ControlEvent,
        }
    }
    pub(crate) fn basefallback_implementation(&self) -> &'static str {
        match self {
            AppCUITrait::Deref => templates::DEREF_TRAIT,
            // markers
            AppCUITrait::Control => "",
            AppCUITrait::DesktopControl => "",
            AppCUITrait::WindowControl => "",
            AppCUITrait::NotDesktop => "",
            AppCUITrait::NotWindow => "",
            AppCUITrait::NotModalWindow => "",
            AppCUITrait::ModalWindowMethods => templates::MODAL_WINDOW_METHODS,
            // raw events
            AppCUITrait::OnWindowRegistered => templates::ON_WINDOW_REGISTERED_TRAIT,
            AppCUITrait::OnPaint => templates::ON_PAINT_TRAIT,
            AppCUITrait::OnKeyPressed => templates::ON_KEY_PRESSED_TRAIT,
            AppCUITrait::OnMouseEvent => templates::ON_MOUSE_EVENT_TRAIT,
            AppCUITrait::OnDefaultAction => templates::ON_DEFAULT_ACTION_TRAIT,
            AppCUITrait::OnResize => templates::ON_RESIZE_TRAIT,
            AppCUITrait::OnFocus => templates::ON_FOCUS_TRAIT,
            AppCUITrait::OnExpand => templates::ON_EXPAND_TRAIT,
            AppCUITrait::OnSiblingSelected => templates::ON_SIBLING_SELECTED,
            AppCUITrait::OnThemeChanged => templates::ON_THEME_CHANGED_TRAIT,
            // control events
            AppCUITrait::ButtonEvents => "",
            AppCUITrait::CheckBoxEvents => "",
            AppCUITrait::WindowEvents => "",
            AppCUITrait::CommandBarEvents => "",
            AppCUITrait::MenuEvents => "",
            AppCUITrait::DesktopEvents => "",
            AppCUITrait::ToolBarEvents => "",
            AppCUITrait::ColorPickerEvents => "",
            AppCUITrait::ThreeStateBoxEvents => "",
            AppCUITrait::RadioBoxEvents => "",
            AppCUITrait::PasswordEvents => "",
            AppCUITrait::KeySelectorEvents => "",
            AppCUITrait::TextFieldEvents => "",
            AppCUITrait::CustomEvents => "",
            AppCUITrait::GenericSelectorEvents => "",
            AppCUITrait::ComboBoxEvents => "",
            AppCUITrait::GenericDropDownListEvents => "",
            AppCUITrait::GenericNumericSelectorEvents => "",
            AppCUITrait::DatePickerEvents => "",
            AppCUITrait::ListBoxEvents => "",
            AppCUITrait::GenericListViewEvents => "",
            AppCUITrait::ToggleButtonEvents => "",
            AppCUITrait::PathFinderEvents => "",
            AppCUITrait::TimerEvents => "",
            AppCUITrait::GenericTreeViewEvents => "",
            AppCUITrait::MarkdownEvents => "",
            AppCUITrait::GenericBackgroundTaskEvents => "",
            AppCUITrait::AccordionEvents => "",
            AppCUITrait::TabEvents => "",
            AppCUITrait::CharPickerEvents => "",
            AppCUITrait::GenericGraphViewEvents => "",
        }
    }
    pub(crate) fn default_implementation(&self) -> &'static str {
        match self {
            AppCUITrait::Deref => "",
            // markers
            AppCUITrait::Control => "impl$(TEMPLATE_TYPE) Control for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::DesktopControl => "impl$(TEMPLATE_TYPE) DesktopControl for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::WindowControl => "impl$(TEMPLATE_TYPE) WindowControl for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::NotDesktop => "impl$(TEMPLATE_TYPE) NotDesktop for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::NotWindow => "impl$(TEMPLATE_TYPE) NotWindow for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::NotModalWindow => "impl$(TEMPLATE_TYPE) NotModalWindow for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::OnWindowRegistered => "impl$(TEMPLATE_TYPE) OnWindowRegistered for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::ModalWindowMethods => "",
            // raw events
            AppCUITrait::OnPaint => "impl$(TEMPLATE_TYPE) OnPaint for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::OnKeyPressed => "impl$(TEMPLATE_TYPE) OnKeyPressed for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::OnMouseEvent => "impl$(TEMPLATE_TYPE) OnMouseEvent for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::OnDefaultAction => "impl$(TEMPLATE_TYPE) OnDefaultAction for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::OnResize => "impl$(TEMPLATE_TYPE) OnResize for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::OnFocus => "impl$(TEMPLATE_TYPE) OnFocus for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::OnExpand => "impl$(TEMPLATE_TYPE) OnExpand for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::OnSiblingSelected => "impl$(TEMPLATE_TYPE) OnSiblingSelected for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::OnThemeChanged => "impl$(TEMPLATE_TYPE) OnThemeChanged for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            // control events
            AppCUITrait::ButtonEvents => "impl$(TEMPLATE_TYPE) ButtonEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::CheckBoxEvents => "impl$(TEMPLATE_TYPE) CheckBoxEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::WindowEvents => "impl$(TEMPLATE_TYPE) WindowEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::CommandBarEvents => "impl$(TEMPLATE_TYPE) GenericCommandBarEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::MenuEvents => "impl$(TEMPLATE_TYPE) GenericMenuEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::DesktopEvents => "impl$(TEMPLATE_TYPE) DesktopEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::ToolBarEvents => "impl$(TEMPLATE_TYPE) ToolBarEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::ColorPickerEvents => "impl$(TEMPLATE_TYPE) ColorPickerEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::ThreeStateBoxEvents => "impl$(TEMPLATE_TYPE) ThreeStateBoxEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::RadioBoxEvents => "impl$(TEMPLATE_TYPE) RadioBoxEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::PasswordEvents => "impl$(TEMPLATE_TYPE) PasswordEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::KeySelectorEvents => "impl$(TEMPLATE_TYPE) KeySelectorEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::TextFieldEvents => "impl$(TEMPLATE_TYPE) TextFieldEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::CustomEvents => "impl$(TEMPLATE_TYPE) CustomEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::GenericSelectorEvents => "impl$(TEMPLATE_TYPE) GenericSelectorEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::ComboBoxEvents => "impl$(TEMPLATE_TYPE) ComboBoxEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::GenericDropDownListEvents => "impl$(TEMPLATE_TYPE) GenericDropDownListEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::GenericNumericSelectorEvents => "impl$(TEMPLATE_TYPE) GenericNumericSelectorEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::DatePickerEvents => "impl$(TEMPLATE_TYPE) DatePickerEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::ListBoxEvents => "impl$(TEMPLATE_TYPE) ListBoxEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::GenericListViewEvents => "impl$(TEMPLATE_TYPE) GenericListViewEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::ToggleButtonEvents => "impl$(TEMPLATE_TYPE) ToggleButtonEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::PathFinderEvents => "impl$(TEMPLATE_TYPE) PathFinderEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::TimerEvents => "impl$(TEMPLATE_TYPE) TimerEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::GenericTreeViewEvents => "impl$(TEMPLATE_TYPE) GenericTreeViewEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::MarkdownEvents => "impl$(TEMPLATE_TYPE) MarkdownEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::GenericBackgroundTaskEvents => "impl$(TEMPLATE_TYPE) GenericBackgroundTaskEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::AccordionEvents => "impl$(TEMPLATE_TYPE) AccordionEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::TabEvents => "impl$(TEMPLATE_TYPE) TabEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::CharPickerEvents => "impl$(TEMPLATE_TYPE) CharPickerEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
            AppCUITrait::GenericGraphViewEvents => "impl$(TEMPLATE_TYPE) GenericGraphViewEvents for $(STRUCT_NAME)$(TEMPLATE_DEF) {}",
        }
    }
    pub(crate) fn is_generic(&self) -> bool {
        matches!(
            self,
            AppCUITrait::GenericSelectorEvents
                | AppCUITrait::GenericDropDownListEvents
                | AppCUITrait::GenericNumericSelectorEvents
                | AppCUITrait::GenericListViewEvents
                | AppCUITrait::GenericTreeViewEvents
                | AppCUITrait::GenericBackgroundTaskEvents
                | AppCUITrait::GenericGraphViewEvents
        )
    }
    pub(crate) fn new(name: &str) -> Option<AppCUITrait> {
        match name {
            // raw events
            "OnPaint" => Some(AppCUITrait::OnPaint),
            "OnKeyPressed" => Some(AppCUITrait::OnKeyPressed),
            "OnMouseEvent" => Some(AppCUITrait::OnMouseEvent),
            "OnDefaultAction" => Some(AppCUITrait::OnDefaultAction),
            "OnResize" => Some(AppCUITrait::OnResize),
            "OnFocus" => Some(AppCUITrait::OnFocus),
            "OnExpand" => Some(AppCUITrait::OnExpand),
            "OnSiblingSelected" => Some(AppCUITrait::OnSiblingSelected),
            "OnThemeChanged" => Some(AppCUITrait::OnThemeChanged),
            // control events
            "ButtonEvents" | "Button" => Some(AppCUITrait::ButtonEvents),
            "CheckBoxEvents" | "CheckBox" => Some(AppCUITrait::CheckBoxEvents),
            "RadioBoxEvents" | "RadioBox" => Some(AppCUITrait::RadioBoxEvents),
            "PasswordEvents" | "Password" => Some(AppCUITrait::PasswordEvents),
            "WindowEvents" | "Window" => Some(AppCUITrait::WindowEvents),
            "CommandBarEvents" | "CommandBar" => Some(AppCUITrait::CommandBarEvents),
            "MenuEvents" | "MenuBar" => Some(AppCUITrait::MenuEvents),
            "DesktopEvents" | "Desktop" => Some(AppCUITrait::DesktopEvents),
            "ToolBarEvents" | "ToolBar" => Some(AppCUITrait::ToolBarEvents),
            "ColorPickerEvents" | "ColorPicker" => Some(AppCUITrait::ColorPickerEvents),
            "ThreeStateBoxEvents" | "ThreeStateBox" => Some(AppCUITrait::ThreeStateBoxEvents),
            "KeySelectorEvents" | "KeySelector" => Some(AppCUITrait::KeySelectorEvents),
            "TextFieldEvents" | "TextField" => Some(AppCUITrait::TextFieldEvents),
            // nothing for the custom events -> they are enabled through a different field
            "SelectorEvents" | "Selector" => Some(AppCUITrait::GenericSelectorEvents),
            "ComboBoxEvents" | "ComboBox" => Some(AppCUITrait::ComboBoxEvents),
            "DropDownListEvents" | "DropDownList" => Some(AppCUITrait::GenericDropDownListEvents),
            "NumericSelectorEvents" | "NumericSelector" => Some(AppCUITrait::GenericNumericSelectorEvents),
            "DatePickerEvents" | "DatePicker" => Some(AppCUITrait::DatePickerEvents),
            "ListBoxEvents" | "ListBox" => Some(AppCUITrait::ListBoxEvents),
            "ListViewEvents" | "ListView" => Some(AppCUITrait::GenericListViewEvents),
            "ToggleButtonEvents" | "ToggleButton" => Some(AppCUITrait::ToggleButtonEvents),
            "PathFinderEvents" | "PathFinder" => Some(AppCUITrait::PathFinderEvents),
            "TimerEvents" | "Timer" => Some(AppCUITrait::TimerEvents),
            "TreeViewEvents" | "TreeView" => Some(AppCUITrait::GenericTreeViewEvents),
            "MarkdownEvents" | "Markdown" => Some(AppCUITrait::MarkdownEvents),
            "BackgroundTaskEvents" | "BackgroundTask" => Some(AppCUITrait::GenericBackgroundTaskEvents),
            "AccordionEvents" | "Accordion" => Some(AppCUITrait::AccordionEvents),
            "TabEvents" | "Tab" => Some(AppCUITrait::TabEvents),
            "CharPickerEvents" | "CharPicker" => Some(AppCUITrait::CharPickerEvents),
            "GraphViewEvents" | "GraphView" => Some(AppCUITrait::GenericGraphViewEvents),
            _ => None,
        }
    }
    pub(crate) fn with_discriminant(value: u8) -> Option<AppCUITrait> {
        let result = match value {
            0 => Some(AppCUITrait::Deref),
            // markers
            1 => Some(AppCUITrait::Control),
            2 => Some(AppCUITrait::DesktopControl),
            3 => Some(AppCUITrait::WindowControl),
            4 => Some(AppCUITrait::NotWindow),
            5 => Some(AppCUITrait::NotModalWindow),
            6 => Some(AppCUITrait::NotDesktop),
            7 => Some(AppCUITrait::OnWindowRegistered),
            8 => Some(AppCUITrait::ModalWindowMethods),
            // raw events
            9 => Some(AppCUITrait::OnPaint),
            10 => Some(AppCUITrait::OnKeyPressed),
            11 => Some(AppCUITrait::OnMouseEvent),
            12 => Some(AppCUITrait::OnDefaultAction),
            13 => Some(AppCUITrait::OnResize),
            14 => Some(AppCUITrait::OnFocus),
            15 => Some(AppCUITrait::OnExpand),
            16 => Some(AppCUITrait::OnSiblingSelected),
            17 => Some(AppCUITrait::OnThemeChanged),
            // control events
            18 => Some(AppCUITrait::ButtonEvents),
            19 => Some(AppCUITrait::CheckBoxEvents),
            20 => Some(AppCUITrait::WindowEvents),
            21 => Some(AppCUITrait::CommandBarEvents),
            22 => Some(AppCUITrait::MenuEvents),
            23 => Some(AppCUITrait::DesktopEvents),
            24 => Some(AppCUITrait::ToolBarEvents),
            25 => Some(AppCUITrait::ColorPickerEvents),
            26 => Some(AppCUITrait::ThreeStateBoxEvents),
            27 => Some(AppCUITrait::RadioBoxEvents),
            28 => Some(AppCUITrait::PasswordEvents),
            29 => Some(AppCUITrait::KeySelectorEvents),
            30 => Some(AppCUITrait::TextFieldEvents),
            31 => Some(AppCUITrait::CustomEvents),
            32 => Some(AppCUITrait::GenericSelectorEvents),
            33 => Some(AppCUITrait::ComboBoxEvents),
            34 => Some(AppCUITrait::GenericDropDownListEvents),
            35 => Some(AppCUITrait::GenericNumericSelectorEvents),
            36 => Some(AppCUITrait::DatePickerEvents),
            37 => Some(AppCUITrait::ListBoxEvents),
            38 => Some(AppCUITrait::GenericListViewEvents),
            39 => Some(AppCUITrait::ToggleButtonEvents),
            40 => Some(AppCUITrait::PathFinderEvents),
            41 => Some(AppCUITrait::TimerEvents),
            42 => Some(AppCUITrait::GenericTreeViewEvents),
            43 => Some(AppCUITrait::MarkdownEvents),
            44 => Some(AppCUITrait::GenericBackgroundTaskEvents),
            45 => Some(AppCUITrait::AccordionEvents),
            46 => Some(AppCUITrait::TabEvents),
            47 => Some(AppCUITrait::CharPickerEvents),
            48 => Some(AppCUITrait::GenericGraphViewEvents),
            _ => None,
        };
        result?;
        // double check
        if value != (result.unwrap() as u8) {
            panic!("Internal error: Conversion of discriminant {value} to AppCUITraits failed !");
        }
        result
    }
}
