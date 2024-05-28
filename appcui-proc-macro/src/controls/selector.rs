use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["AllowNoneVariant"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("enum", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("enum", "enum", ParamType::String),
    NamedParameter::new("class", "enum", ParamType::String),
    NamedParameter::new("value", "value", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("Selector", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control_with_template("Selector", "new", "enum");
    if cb.has_parameter("value") {
        cb.add("Some(");
        cb.add_param_value("enum");
        cb.add("::");
        cb.add_param_value("value");
        cb.add(")");
    } else {
        cb.add("None");
    }
    cb.add_layout();
    cb.add_flags_parameter("flags", "selector::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
