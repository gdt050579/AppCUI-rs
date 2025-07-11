use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static TYPES: FlagsSignature = FlagsSignature::new(&["HiddenTabs", "OnTop", "OnBottom", "OnLeft"]);
static FLAGS: FlagsSignature = FlagsSignature::new(&["TransparentBackground", "TabsBar"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("tabs", "tabs", ParamType::List),
    NamedParameter::new("tab-width", "tabwidth", ParamType::Integer),
    NamedParameter::new("tabwidth", "tabwidth", ParamType::Integer),
    NamedParameter::new("tw", "tabwidth", ParamType::Integer),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("tab", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("Tab::with_type");
    cb.add_layout();
    cb.add_flags_parameter("flags", "tab::Flags", &FLAGS);
    cb.add_enum_parameter("type", "tab::Type", &TYPES, Some("OnTop"));
    cb.finish_control_initialization();
    if let Some(l) = cb.get_list("tabs") {
        let mut v = Vec::with_capacity(l.len() + 1);
        for item in l {
            v.push(format!("control.add_tab(\"{}\");", item.get_string()));
        }
        for line in v {
            cb.add_line(line.as_str());
        }
    }
    if let Some(tw) = cb.get_i32("tabwidth") {
        if !(3..=32).contains(&tw) {
            panic!("Tab width parameter must be a value between 3 and 32 !");
        }
        cb.add_line(format!("control.set_tab_width({tw});").as_str());
    }
    cb.add_basecontrol_operations();
    cb.into()
}
