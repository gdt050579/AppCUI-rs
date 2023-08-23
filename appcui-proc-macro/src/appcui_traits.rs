use super::templates;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum AppCUITraits {
    Deref = 0,
    OnPaint = 1,
    OnKeyPressed = 2,
    OnMouseEvents = 3,
    OnDefaultAction = 4,
    OnResize = 5,
    OnFocus = 6,
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
            AppCUITraits::OnPaint => "OnPaint",
            AppCUITraits::OnKeyPressed => "OnKeyPressed",
            AppCUITraits::OnMouseEvents => "OnMouseEvents",
            AppCUITraits::OnDefaultAction => "OnDefaultAction",
            AppCUITraits::OnResize => "OnResize",
            AppCUITraits::OnFocus => "OnFocus",
        }
    }
    pub(crate) fn get_trait_type(&self) -> TraitType {
        match self {
            AppCUITraits::Deref => TraitType::Other,
            AppCUITraits::OnPaint => TraitType::RawEvent,
            AppCUITraits::OnKeyPressed => TraitType::RawEvent,
            AppCUITraits::OnMouseEvents => TraitType::RawEvent,
            AppCUITraits::OnDefaultAction => TraitType::RawEvent,
            AppCUITraits::OnResize => TraitType::RawEvent,
            AppCUITraits::OnFocus => TraitType::RawEvent,
        }
    }
    pub(crate) fn get_basefallback_implementation(&self) -> &'static str {
        match self {
            AppCUITraits::Deref => templates::DEREF_TRAIT,
            AppCUITraits::OnPaint => templates::ON_PAINT_TRAIT,
            AppCUITraits::OnKeyPressed => templates::ON_KEY_PRESSED_TRAIT,
            AppCUITraits::OnMouseEvents => templates::ON_MOUSE_EVENT_TRAIT,
            AppCUITraits::OnDefaultAction => templates::ON_DEFAULT_ACTION_TRAIT,
            AppCUITraits::OnResize => templates::ON_RESIZE_TRAIT,
            AppCUITraits::OnFocus => templates::ON_FOCUS_TRAIT,
        }
    }
    pub(crate) fn get_default_implementation(&self) -> &'static str {
        match self {
            AppCUITraits::Deref => "",
            AppCUITraits::OnPaint => "impl OnPaint for $(STRUCT_NAME) {}",
            AppCUITraits::OnKeyPressed => "impl OnKeyPressed for $(STRUCT_NAME) {}",
            AppCUITraits::OnMouseEvents => "impl OnMouseEvent for $(STRUCT_NAME) {}",
            AppCUITraits::OnDefaultAction => "impl OnDefaultAction for $(STRUCT_NAME) {}",
            AppCUITraits::OnResize => "impl OnResize for $(STRUCT_NAME) {}",
            AppCUITraits::OnFocus => "impl OnFocus for $(STRUCT_NAME) {}",
        }
    }
    pub(crate) fn new(name: &str) -> Option<AppCUITraits> {
        match name {
            "OnPaint" => Some(AppCUITraits::OnPaint),
            "OnKeyPressed" => Some(AppCUITraits::OnKeyPressed),
            "OnMouseEvents" => Some(AppCUITraits::OnMouseEvents),
            "OnDefaultAction" => Some(AppCUITraits::OnDefaultAction),
            "OnResize" => Some(AppCUITraits::OnResize),
            "OnFocus" => Some(AppCUITraits::OnFocus),
            _ => None,
        }
    }
    pub(crate) fn with_discriminant(value: u8) -> Option<AppCUITraits> {
        let result = match value {
            0 => Some(AppCUITraits::Deref),
            1 => Some(AppCUITraits::OnPaint),
            2 => Some(AppCUITraits::OnKeyPressed),
            3 => Some(AppCUITraits::OnMouseEvents),
            4 => Some(AppCUITraits::OnDefaultAction),
            5 => Some(AppCUITraits::OnDefaultAction),
            6 => Some(AppCUITraits::OnFocus),
            _ => None,
        };
        if result.is_none() {
            return None;
        }
        // double check
        if value != (result.unwrap() as u8) {
            panic!("Internal error: Conversion of discriminant {} to AppCUITraits failed !",value);
        }
        return result;
    }
}
