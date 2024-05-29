use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["AllowNoneSelection", "ShowDescription"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("class", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("class", "class", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("symbolsize", "symbolsize", ParamType::Integer),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("DropDownList", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control_with_template("DropDownList", "with_symbol", "class");
    let sz = cb.get_i32("symbolsize").unwrap_or(0);
    match sz {
        0 => cb.add("0"),
        1 => cb.add("1"),
        2 => cb.add("2"),
        3 => cb.add("3"),
        _ => {
            panic!("Symbol size parameter can have one of the following values: 0,1,2 or 3");
        }
    }
    cb.add_layout();
    cb.add_flags_parameter("flags", "dropdownlist::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}
