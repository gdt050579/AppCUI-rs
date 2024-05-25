use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ScrollBars"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("size", ParamType::Size)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("size", "size", ParamType::Size),
    NamedParameter::new("sz", "size", ParamType::Size),
    NamedParameter::new("surface", "size", ParamType::Size),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("background", "back", ParamType::Dict),
    NamedParameter::new("back", "back", ParamType::Dict),
    NamedParameter::new("left-scroll-margin", "lsm", ParamType::Integer),
    NamedParameter::new("lsm", "lsm", ParamType::Integer),
    NamedParameter::new("top-scroll-margin", "tsm", ParamType::Integer),
    NamedParameter::new("tsm", "tsm", ParamType::Integer),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("canvas", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("Canvas::new");
    cb.add_size_parameter("size", None);
    cb.add_layout();
    cb.add_flags_parameter("flags", "canvas::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();

    let has_back_param = cb.has_parameter("back");
    if has_back_param {
        let str_repr = String::from(cb.get_string_representation());
        if let Some(d) = cb.get_dict("back") {
            let s = crate::chars::builder::create_from_dict(&str_repr, d);
            cb.add_line(format!("control.set_background({});", s).as_str());
        }
    }
    let lsm = cb.get_i32("lsm").unwrap_or(0);
    let tsm = cb.get_i32("tsm").unwrap_or(0);
    if (lsm != 0) || (tsm != 0) {
        if lsm < 0 {
            panic!("Left scroll margin can not be a negative number");
        }
        if tsm < 0 {
            panic!("Top scroll margin can not be a negative number");
        }
        cb.add_line(format!("control.set_components_toolbar_margins({},{});", lsm, tsm).as_str());
    }
    cb.into()
}
