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
    pub(crate) fn new(name: &str)->Option<AppCUITraits> {
        match name {
            "OnPaint" => Some(AppCUITraits::OnPaint),
            "OnKeyPressed" => Some(AppCUITraits::OnKeyPressed),
            "OnMouseEvents" => Some(AppCUITraits::OnMouseEvents),
            "OnDefaultAction" => Some(AppCUITraits::OnDefaultAction),
            "OnResize" => Some(AppCUITraits::OnResize),
            "OnFocus" => Some(AppCUITraits::OnFocus), 
            _ => None        
        }
    }
}
