use super::templates;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum AppCUITraits {
    Deref = 0,
    // markers
    Control = 1,
    DesktopControl = 2,
    WindowControl = 3,
    NotWindow = 4,
    NotDesktop = 5,
    // raw events
    OnPaint = 6,
    OnKeyPressed = 7,
    OnMouseEvents = 8,
    OnDefaultAction = 9,
    OnResize = 10,
    OnFocus = 11,
    // control events
    ButtonEvents = 12,
    CheckBoxEvents = 13,
    WindowEvents = 14,
    CommandBarEvents = 15,
    MenuEvents = 16,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum TraitType {
    RawEvent,
    ControlEvent,
    TerminalEvent,
    Other,
}

impl AppCUITraits {
    pub(crate) fn get_name(&self) -> &'static str {
        match self {
            AppCUITraits::Deref => "Deref",
            // markers
            AppCUITraits::Control => "Control",
            AppCUITraits::DesktopControl => "DesktopControl",
            AppCUITraits::WindowControl => "WindowControl",
            AppCUITraits::NotDesktop => "NotDesktop",
            AppCUITraits::NotWindow => "NotWindow",
            // raw events
            AppCUITraits::OnPaint => "OnPaint",
            AppCUITraits::OnKeyPressed => "OnKeyPressed",
            AppCUITraits::OnMouseEvents => "OnMouseEvents",
            AppCUITraits::OnDefaultAction => "OnDefaultAction",
            AppCUITraits::OnResize => "OnResize",
            AppCUITraits::OnFocus => "OnFocus",
            // control events
            AppCUITraits::ButtonEvents => "ButtonEvents",
            AppCUITraits::CheckBoxEvents => "CheckBoxEvents",
            AppCUITraits::WindowEvents => "WindowEvents",
            AppCUITraits::CommandBarEvents => "CommandBarEvents",
            AppCUITraits::MenuEvents => "MenuEvents",
        }
    }
    pub(crate) fn get_trait_type(&self) -> TraitType {
        match self {
            AppCUITraits::Deref => TraitType::Other,
            // markers
            AppCUITraits::Control => TraitType::Other,
            AppCUITraits::DesktopControl => TraitType::Other,
            AppCUITraits::WindowControl => TraitType::Other,
            AppCUITraits::NotDesktop => TraitType::Other,
            AppCUITraits::NotWindow => TraitType::Other,
            // raw events
            AppCUITraits::OnPaint => TraitType::RawEvent,
            AppCUITraits::OnKeyPressed => TraitType::RawEvent,
            AppCUITraits::OnMouseEvents => TraitType::RawEvent,
            AppCUITraits::OnDefaultAction => TraitType::RawEvent,
            AppCUITraits::OnResize => TraitType::RawEvent,
            AppCUITraits::OnFocus => TraitType::RawEvent,
            // control events
            AppCUITraits::ButtonEvents => TraitType::ControlEvent,
            AppCUITraits::CheckBoxEvents => TraitType::ControlEvent,
            AppCUITraits::WindowEvents => TraitType::ControlEvent,
            AppCUITraits::CommandBarEvents => TraitType::ControlEvent,
            AppCUITraits::MenuEvents => TraitType::ControlEvent,
        }
    }
    pub(crate) fn get_basefallback_implementation(&self) -> &'static str {
        match self {
            AppCUITraits::Deref => templates::DEREF_TRAIT,
            // markers
            AppCUITraits::Control => "",
            AppCUITraits::DesktopControl => "",
            AppCUITraits::WindowControl => "",
            AppCUITraits::NotDesktop => "",
            AppCUITraits::NotWindow => "",
            // raw events
            AppCUITraits::OnPaint => templates::ON_PAINT_TRAIT,
            AppCUITraits::OnKeyPressed => templates::ON_KEY_PRESSED_TRAIT,
            AppCUITraits::OnMouseEvents => templates::ON_MOUSE_EVENT_TRAIT,
            AppCUITraits::OnDefaultAction => templates::ON_DEFAULT_ACTION_TRAIT,
            AppCUITraits::OnResize => templates::ON_RESIZE_TRAIT,
            AppCUITraits::OnFocus => templates::ON_FOCUS_TRAIT,
            // control events
            AppCUITraits::ButtonEvents => "",
            AppCUITraits::CheckBoxEvents => "",
            AppCUITraits::WindowEvents => "",
            AppCUITraits::CommandBarEvents => "",
            AppCUITraits::MenuEvents => "",
        }
    }
    pub(crate) fn get_default_implementation(&self) -> &'static str {
        match self {
            AppCUITraits::Deref => "",
            // markers
            AppCUITraits::Control => "impl Control for $(STRUCT_NAME) {}",
            AppCUITraits::DesktopControl => "impl DesktopControl for $(STRUCT_NAME) {}",
            AppCUITraits::WindowControl => "impl WindowControl for $(STRUCT_NAME) {}",
            AppCUITraits::NotDesktop => "impl NotDesktop for $(STRUCT_NAME) {}",
            AppCUITraits::NotWindow => "impl NotWindow for $(STRUCT_NAME) {}",
            // raw events
            AppCUITraits::OnPaint => "impl OnPaint for $(STRUCT_NAME) {}",
            AppCUITraits::OnKeyPressed => "impl OnKeyPressed for $(STRUCT_NAME) {}",
            AppCUITraits::OnMouseEvents => "impl OnMouseEvent for $(STRUCT_NAME) {}",
            AppCUITraits::OnDefaultAction => "impl OnDefaultAction for $(STRUCT_NAME) {}",
            AppCUITraits::OnResize => "impl OnResize for $(STRUCT_NAME) {}",
            AppCUITraits::OnFocus => "impl OnFocus for $(STRUCT_NAME) {}",
            // control events
            AppCUITraits::ButtonEvents => "impl ButtonEvents for $(STRUCT_NAME) {}",
            AppCUITraits::CheckBoxEvents => "impl CheckBoxEvents for $(STRUCT_NAME) {}",
            AppCUITraits::WindowEvents => "impl WindowEvents for $(STRUCT_NAME) {}",
            AppCUITraits::CommandBarEvents => "impl CommandBarEvents for $(STRUCT_NAME) {}",
            AppCUITraits::MenuEvents => "impl MenuEvents for $(STRUCT_NAME) {}",
        }
    }
    pub(crate) fn new(name: &str) -> Option<AppCUITraits> {
        match name {
            // raw events
            "OnPaint" => Some(AppCUITraits::OnPaint),
            "OnKeyPressed" => Some(AppCUITraits::OnKeyPressed),
            "OnMouseEvents" => Some(AppCUITraits::OnMouseEvents),
            "OnDefaultAction" => Some(AppCUITraits::OnDefaultAction),
            "OnResize" => Some(AppCUITraits::OnResize),
            "OnFocus" => Some(AppCUITraits::OnFocus),
            // control events
            "ButtonEvents" | "Button" => Some(AppCUITraits::ButtonEvents),
            "CheckBoxEvents" | "CheckBox" => Some(AppCUITraits::CheckBoxEvents),
            "WindowEvents" | "Window" => Some(AppCUITraits::WindowEvents),
            "CommandBarEvents" | "CommandBar" => Some(AppCUITraits::CommandBarEvents),
            "MenuEvents" | "MenuBar" => Some(AppCUITraits::MenuEvents),
            _ => None,
        }
    }
    pub(crate) fn with_discriminant(value: u8) -> Option<AppCUITraits> {
        let result = match value {
            0 => Some(AppCUITraits::Deref),
            // markers
            1 => Some(AppCUITraits::Control),
            2 => Some(AppCUITraits::DesktopControl),
            3 => Some(AppCUITraits::WindowControl),
            4 => Some(AppCUITraits::NotWindow),
            5 => Some(AppCUITraits::NotDesktop),
            // raw events
            6 => Some(AppCUITraits::OnPaint),
            7 => Some(AppCUITraits::OnKeyPressed),
            8 => Some(AppCUITraits::OnMouseEvents),
            9 => Some(AppCUITraits::OnDefaultAction),
            10 => Some(AppCUITraits::OnDefaultAction),
            11 => Some(AppCUITraits::OnFocus),
            // control events
            12 => Some(AppCUITraits::ButtonEvents),
            13 => Some(AppCUITraits::CheckBoxEvents),
            14 => Some(AppCUITraits::WindowEvents),
            15 => Some(AppCUITraits::CommandBarEvents),
            16 => Some(AppCUITraits::MenuEvents),

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
