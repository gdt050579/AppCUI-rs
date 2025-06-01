use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];

static STATE_TYPE: FlagsSignature = FlagsSignature::new(&["Checked", "Unchecked", "Unknown"]);
static BOX_TYPE: FlagsSignature = FlagsSignature::new(&["Standard", "Ascii", "CheckBox", "CheckMark", "FilledBox", "YesNo", "PlusMinus"]);

static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("state", "state", ParamType::String),
    NamedParameter::new("type", "type", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("threestatebox", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("ThreeStateBox::with_type");
    cb.add_string_parameter("caption", None);
    cb.add_layout();
    cb.add_enum_parameter("state", "threestatebox::State", &STATE_TYPE, Some("Unknown"));
    cb.add_enum_parameter("type", "threestatebox::Type", &BOX_TYPE, Some("Standard"));
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
