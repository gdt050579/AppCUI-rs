use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ShowDescription"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("items", "items", ParamType::List),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("selected_index", "index", ParamType::Integer),
    NamedParameter::new("index", "index", ParamType::Integer),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("combobox", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("ComboBox::new");
    cb.add_layout();
    cb.add_flags_parameter("flags", "combobox::Flags", &FLAGS);
    cb.finish_control_initialization();
    if cb.has_parameter("items") {
        let mut s = String::with_capacity(256);
        if let Some(list) = cb.get_list("items") {
            for item in list.iter() {
                s.push_str("control.add(\"");
                s.push_str(item.get_string());
                s.push_str("\");\n");
            }
        } else {
            panic!("Parameter `items` in combobox must contains a list a strings: items=['String1','String2',...] !");
        }
        cb.add_line(&s);
    }
    if cb.has_parameter("index") {
        let value = cb.get_i32("index").unwrap_or(-1);
        if value >= 0 {
            cb.add_line(format!("control.set_index({});", value).as_str());
        } else {
            panic!("Invalid index (should be a positive number) for combo box selection index parameter !");
        }
    }
    cb.add_basecontrol_operations();
    cb.into()
}
