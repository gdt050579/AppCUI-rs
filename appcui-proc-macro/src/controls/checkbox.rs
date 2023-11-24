use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;
static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("caption", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("caption", "caption", ParamType::String),
    NamedParameter::new("text", "caption", ParamType::String),
    NamedParameter::new("checked", "checked", ParamType::Bool),
    NamedParameter::new("check", "checked", ParamType::Bool),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("checkbox", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS);
    cb.init_control("CheckBox::new");
    cb.add_strng_parameter("caption");  
    cb.add_layout();
    cb.add_bool_parameter_with_default("checked", false);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();
    cb.into()
}

// pub(crate) fn create(input: TokenStream) -> TokenStream {
//     let s = utils::token_stream_to_string("checkbox", input);
//     let mut p = parameter_parser::parse(&s).unwrap();
//     p.validate_positional_parameters(&s, POSILITIONAL_PARAMETERS).unwrap();
//     p.validate_names_parameters(&s, NAMED_PARAMETERS).unwrap();
//     p.validate_names_parameters(&s, common::CONTROL_NAMED_PARAMATERS).unwrap();
//     p.check_unkwnon_params(&s).unwrap();
//     // all good --> lets build the query
//     let mut result = String::with_capacity(512);
//     result.push_str("{\n\t");
//     result.push_str("let mut checkbox = CheckBox::new(");
//     // first add the caption
//     let caption = p.get("caption").expect("First parameter (caption) has to be provided !");
//     common::add_string(&mut result, caption.get_string());
//     // second add the layout
//     result.push_str(" , ");
//     layout::add_layout(&mut result, &p);
//     // lastly add the check status
//     result.push_str(" , ");
//     common::add_bool(&mut result, p.get_bool("checked").unwrap_or(false));
//     result.push_str(");\n\t");
//     // basic controls
//     common::add_basecontrol_operations(&mut result, "checkbox", &mut p);
//     // finally close block
//     result.push_str("checkbox\n}");
//     utils::to_token_stream(result)
// }
