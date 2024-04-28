use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["TransparentBackground"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("panels", "panels", ParamType::List),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("accordion", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("Accordion::new");
    cb.add_layout();
    cb.add_flags_parameter("flags", "accordion::Flags", &FLAGS);
    cb.finish_control_initialization();
    if let Some(l) = cb.get_list("panels") {
        let mut v = Vec::with_capacity(l.len() + 1);
        for item in l {
            v.push(format!("control.add_panel(\"{}\");", item.get_string()));
        }
        for line in v {
            cb.add_line(line.as_str());
        }
    }
    cb.add_basecontrol_operations();
    cb.into()
}
