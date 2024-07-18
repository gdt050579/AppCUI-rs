use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ScrollBars", "SearchBar", "CheckBoxes", "Groups", "SmallIcon", "LargeIcon"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("type", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("class", "type", ParamType::String),
    NamedParameter::new("columns", "columns", ParamType::List),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("left-scroll-margin", "lsm", ParamType::Integer),
    NamedParameter::new("lsm", "lsm", ParamType::Integer),
    NamedParameter::new("top-scroll-margin", "tsm", ParamType::Integer),
    NamedParameter::new("tsm", "tsm", ParamType::Integer),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("listvew", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control_with_template("ListView", "new", "type");
    cb.add_layout();
    cb.add_flags_parameter("flags", "listview::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_scroll_margin_setup("lsm", "tsm");
    if cb.has_parameter("columns") {
        let mut s = String::with_capacity(256);
        let mut temp_s = String::with_capacity(64);
        if let Some(list) = cb.get_list("columns") {
            for item in list.iter_mut() {
                temp_s.clear();
                temp_s.push_str(item.get_string());
                if let Some(d) = item.get_dict() {
                    let res = crate::column::builder::create_from_dict(&temp_s, d);
                    s.push_str("control.add_column(");
                    s.push_str(&res);
                    s.push_str(");\n");
                } else {
                    panic!("A column must be descipted between brackets: {{ and }}. For example: `{{Name,10,Left}}` !");
                }
            }
        } else {
            panic!("Parameter `columns` in listview must contains a list a columns: columns=[{{...}},{{...}},{{...}}] !");
        }
        cb.add_line(&s);
    }
    cb.add_basecontrol_operations();
    cb.into()
}
