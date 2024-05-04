use super::templates;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
    // control events
    ButtonEvents = 17,
    CheckBoxEvents = 18,
    WindowEvents = 19,
    CommandBarEvents = 20,
    MenuEvents = 21,
    DesktopEvents = 22,
    ToolBarEvents = 23,
    ColorPickerEvents = 24,
    ThreeStateBoxEvents = 25,
    RadioBoxEvents = 26,
    PasswordEvents = 27,
    KeySelectorEvents = 28,
    TextFieldEvents = 29,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum TraitType {
    RawEvent,
    ControlEvent,
    Other,
}

impl AppCUITrait {
    pub(crate) fn get_name(&self) -> &'static str {
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
            
        }
    }
    pub(crate) fn get_trait_type(&self) -> TraitType {
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
        }
    }
    pub(crate) fn get_basefallback_implementation(&self) -> &'static str {
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
        }
    }
    pub(crate) fn get_default_implementation(&self) -> &'static str {
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
        }
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
            // control events
            17 => Some(AppCUITrait::ButtonEvents),
            18 => Some(AppCUITrait::CheckBoxEvents),
            19 => Some(AppCUITrait::WindowEvents),
            20 => Some(AppCUITrait::CommandBarEvents),
            21 => Some(AppCUITrait::MenuEvents),
            22 => Some(AppCUITrait::DesktopEvents),
            23 => Some(AppCUITrait::ToolBarEvents),
            24 => Some(AppCUITrait::ColorPickerEvents),
            25 => Some(AppCUITrait::ThreeStateBoxEvents),
            26 => Some(AppCUITrait::RadioBoxEvents),
            27 => Some(AppCUITrait::PasswordEvents),
            28 => Some(AppCUITrait::KeySelectorEvents),
            29 => Some(AppCUITrait::TextFieldEvents),
            _ => None,
        };
        result?;
        // double check
        if value != (result.unwrap() as u8) {
            panic!("Internal error: Conversion of discriminant {} to AppCUITraits failed !", value);
        }
        result
    }
}
