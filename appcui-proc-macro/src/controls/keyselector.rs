use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static mut FLAGS: FlagsSignature = FlagsSignature::new(&["AcceptEnter", "AcceptTab", "AcceptEscape", "ReadOnly"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("key", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("key", "key", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("keyselector", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("KeySelector::new");
    cb.add_key_parameter("key", Some("Key::None"));
    cb.add_layout();
    cb.add_flags_parameter("flags", "keyselector::Flags", unsafe { &mut FLAGS });
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
