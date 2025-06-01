use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static TYPES: FlagsSignature = FlagsSignature::new(&["Label", "Button", "CheckBox", "SingleChoice"]);
static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("checked", "checked", ParamType::Bool),
    NamedParameter::new("check", "checked", ParamType::Bool),
    NamedParameter::new("visible", "visible", ParamType::Bool),
    NamedParameter::new("tooltip", "tooltip", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("toolbaritem", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, false);
    if let Some(item_type) = cb.get_enum_value("type", &TYPES) {
        let init_type = format!("toolbar::{}::new", item_type);
        let is_checkbox = item_type == "CheckBox";
        cb.init_control(init_type.as_str());
        cb.add_string_parameter("caption", None);
        if is_checkbox {
            cb.add_bool_parameter("checked", Some(false));
        }
        cb.finish_control_initialization();
        cb.add_toolbaritem_operations();
        cb.into()
    } else {
        panic!("Parameter 'type' is mandatory (possible value for parameter type: 'Label', 'Button', 'Checkbox', 'SingleChoice')");
    }
}
