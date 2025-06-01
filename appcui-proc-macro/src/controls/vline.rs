use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["DoubleLine"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[];
static NAMED_PARAMETERS: &[NamedParameter] = &[NamedParameter::new("flags", "flags", ParamType::Flags)];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("vline", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("VLine::new");
    cb.add_layout();
    cb.add_flags_parameter("flags", "vline::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
