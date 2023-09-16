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
    NotDesktop = 5,
    OnRegistered = 6,
    // raw events
    OnPaint = 7,
    OnKeyPressed = 8,
    OnMouseEvent = 9,
    OnDefaultAction = 10,
    OnResize = 11,
    OnFocus = 12,
    // control events
    ButtonEvents = 13,
    CheckBoxEvents = 14,
    WindowEvents = 15,
    CommandBarEvents = 16,
    MenuEvents = 17,
    DesktopEvents = 18,
    ToolBarEvents = 19,
}

#[repr(u8)]
#[derive(Copy, Clone,PartialEq,Eq)]
pub(crate) enum TraitType {
    RawEvent,
    ControlEvent,
    TerminalEvent,
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
            AppCUITrait::OnRegistered => "OnRegistered",
            // raw events
            AppCUITrait::OnPaint => "OnPaint",
            AppCUITrait::OnKeyPressed => "OnKeyPressed",
            AppCUITrait::OnMouseEvent => "OnMouseEvent",
            AppCUITrait::OnDefaultAction => "OnDefaultAction",
            AppCUITrait::OnResize => "OnResize",
            AppCUITrait::OnFocus => "OnFocus",
            // control events
            AppCUITrait::ButtonEvents => "ButtonEvents",
            AppCUITrait::CheckBoxEvents => "CheckBoxEvents",
            AppCUITrait::WindowEvents => "WindowEvents",
            AppCUITrait::CommandBarEvents => "CommandBarEvents",
            AppCUITrait::MenuEvents => "MenuEvents",
            AppCUITrait::DesktopEvents => "DesktopEvents",
            AppCUITrait::ToolBarEvents => "ToolBarEvents",            
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
            AppCUITrait::OnRegistered => TraitType::Other,
            // raw events
            AppCUITrait::OnPaint => TraitType::RawEvent,
            AppCUITrait::OnKeyPressed => TraitType::RawEvent,
            AppCUITrait::OnMouseEvent => TraitType::RawEvent,
            AppCUITrait::OnDefaultAction => TraitType::RawEvent,
            AppCUITrait::OnResize => TraitType::RawEvent,
            AppCUITrait::OnFocus => TraitType::RawEvent,
            // control events
            AppCUITrait::ButtonEvents => TraitType::ControlEvent,
            AppCUITrait::CheckBoxEvents => TraitType::ControlEvent,
            AppCUITrait::WindowEvents => TraitType::ControlEvent,
            AppCUITrait::CommandBarEvents => TraitType::ControlEvent,
            AppCUITrait::MenuEvents => TraitType::ControlEvent,
            AppCUITrait::DesktopEvents => TraitType::ControlEvent,
            AppCUITrait::ToolBarEvents => TraitType::ControlEvent,           
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
            // raw events
            AppCUITrait::OnRegistered => templates::ON_REGISTERED_TRAIT,
            AppCUITrait::OnPaint => templates::ON_PAINT_TRAIT,
            AppCUITrait::OnKeyPressed => templates::ON_KEY_PRESSED_TRAIT,
            AppCUITrait::OnMouseEvent => templates::ON_MOUSE_EVENT_TRAIT,
            AppCUITrait::OnDefaultAction => templates::ON_DEFAULT_ACTION_TRAIT,
            AppCUITrait::OnResize => templates::ON_RESIZE_TRAIT,
            AppCUITrait::OnFocus => templates::ON_FOCUS_TRAIT,
            // control events
            AppCUITrait::ButtonEvents => "",
            AppCUITrait::CheckBoxEvents => "",
            AppCUITrait::WindowEvents => "",
            AppCUITrait::CommandBarEvents => "",
            AppCUITrait::MenuEvents => "",
            AppCUITrait::DesktopEvents => "",
            AppCUITrait::ToolBarEvents => "",
        }
    }
    pub(crate) fn get_default_implementation(&self) -> &'static str {
        match self {
            AppCUITrait::Deref => "",
            // markers
            AppCUITrait::Control => "impl Control for $(STRUCT_NAME) {}",
            AppCUITrait::DesktopControl => "impl DesktopControl for $(STRUCT_NAME) {}",
            AppCUITrait::WindowControl => "impl WindowControl for $(STRUCT_NAME) {}",
            AppCUITrait::NotDesktop => "impl NotDesktop for $(STRUCT_NAME) {}",
            AppCUITrait::NotWindow => "impl NotWindow for $(STRUCT_NAME) {}",
            AppCUITrait::OnRegistered => "impl OnRegistered for $(STRUCT_NAME) {}",
            // raw events
            AppCUITrait::OnPaint => "impl OnPaint for $(STRUCT_NAME) {}",
            AppCUITrait::OnKeyPressed => "impl OnKeyPressed for $(STRUCT_NAME) {}",
            AppCUITrait::OnMouseEvent => "impl OnMouseEvent for $(STRUCT_NAME) {}",
            AppCUITrait::OnDefaultAction => "impl OnDefaultAction for $(STRUCT_NAME) {}",
            AppCUITrait::OnResize => "impl OnResize for $(STRUCT_NAME) {}",
            AppCUITrait::OnFocus => "impl OnFocus for $(STRUCT_NAME) {}",
            // control events
            AppCUITrait::ButtonEvents => "impl ButtonEvents for $(STRUCT_NAME) {}",
            AppCUITrait::CheckBoxEvents => "impl CheckBoxEvents for $(STRUCT_NAME) {}",
            AppCUITrait::WindowEvents => "impl WindowEvents for $(STRUCT_NAME) {}",
            AppCUITrait::CommandBarEvents => "impl CommandBarEvents for $(STRUCT_NAME) {}",
            AppCUITrait::MenuEvents => "impl MenuEvents for $(STRUCT_NAME) {}",
            AppCUITrait::DesktopEvents => "impl DesktopEvents for $(STRUCT_NAME) {}",
            AppCUITrait::ToolBarEvents => "impl ToolBarEvents for $(STRUCT_NAME) {}",
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
            // control events
            "ButtonEvents" | "Button" => Some(AppCUITrait::ButtonEvents),
            "CheckBoxEvents" | "CheckBox" => Some(AppCUITrait::CheckBoxEvents),
            "WindowEvents" | "Window" => Some(AppCUITrait::WindowEvents),
            "CommandBarEvents" | "CommandBar" => Some(AppCUITrait::CommandBarEvents),
            "MenuEvents" | "MenuBar" => Some(AppCUITrait::MenuEvents),
            "DesktopEvents" | "Desktop" => Some(AppCUITrait::DesktopEvents),
            "ToolBarEvents" | "ToolBar" => Some(AppCUITrait::ToolBarEvents),
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
            5 => Some(AppCUITrait::NotDesktop),
            6 => Some(AppCUITrait::OnRegistered),
            // raw events
            7 => Some(AppCUITrait::OnPaint),
            8 => Some(AppCUITrait::OnKeyPressed),
            9 => Some(AppCUITrait::OnMouseEvent),
            10 => Some(AppCUITrait::OnDefaultAction),
            11 => Some(AppCUITrait::OnResize),
            12 => Some(AppCUITrait::OnFocus),
            // control events
            13 => Some(AppCUITrait::ButtonEvents),
            14 => Some(AppCUITrait::CheckBoxEvents),
            15 => Some(AppCUITrait::WindowEvents),
            16 => Some(AppCUITrait::CommandBarEvents),
            17 => Some(AppCUITrait::MenuEvents),
            18 => Some(AppCUITrait::DesktopEvents),
            19 => Some(AppCUITrait::ToolBarEvents),
            _ => None,
        };
        if result.is_none() {
            return None;
        }
        // double check
        if value != (result.unwrap() as u8) {
            panic!(
                "Internal error: Conversion of discriminant {} to AppCUITraits failed !",
                value
            );
        }
        return result;
    }
}
