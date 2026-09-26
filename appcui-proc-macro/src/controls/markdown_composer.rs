use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ShowMarkers", "Emoticons", "ReadOnly"]);
static LIST_FLAGS: FlagsSignature = FlagsSignature::new(&["RemoveTrigger"]);

static POSITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("content", ParamType::String)];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("content", "content", ParamType::String),
    NamedParameter::new("text", "content", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("emoji", "emoji", ParamType::String),
    NamedParameter::new("lists", "lists", ParamType::List),
];

static LIST_POSITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("trigger", ParamType::String)];
static LIST_NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("trigger", "trigger", ParamType::String),
    NamedParameter::new("items", "items", ParamType::List),
    NamedParameter::new("values", "values", ParamType::List),
    NamedParameter::new("flags", "flags", ParamType::Flags),
];

static VALUE_POSITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("name", ParamType::String),
    PositionalParameter::new("value", ParamType::String),
];
static VALUE_NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("name", "name", ParamType::String),
    NamedParameter::new("value", "value", ParamType::String),
];

fn trigger_literal(text: &str) -> String {
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(ch), None) => format!("{ch:?}"),
        _ => panic!("A trigger must be exactly one character, but found '{text}' !"),
    }
}

fn list_flags(dict: &mut NamedParamsMap) -> String {
    let mut result = String::new();
    if let Some(list) = dict.get_list("flags") {
        for name in list.iter() {
            let Some(flag) = LIST_FLAGS.get(name.get_string()) else {
                panic!("Unknown list flag: {} ! Available flags are: {}", name.get_string(), LIST_FLAGS.list());
            };
            if !result.is_empty() {
                result.push_str(" | ");
            }
            result.push_str("markdown_composer::ListFlags::");
            result.push_str(flag);
        }
    }
    if result.is_empty() {
        result.push_str("markdown_composer::ListFlags::None");
    }
    result
}

fn list_code(dict: &mut NamedParamsMap) -> String {
    let trigger = match dict.get("trigger") {
        Some(value) => trigger_literal(value.get_string()),
        None => panic!("A list needs a trigger character, for example: {{'@', items: ['Ana', 'Dan']}} !"),
    };
    let flags = list_flags(dict);

    if let Some(items) = dict.get_list("items") {
        let items: Vec<String> = items.iter().map(|item| format!("{:?}", item.get_string())).collect();
        return format!("control.add_list({trigger}, &[{}], {flags});\n", items.join(", "));
    }

    if let Some(values) = dict.get_list("values") {
        let mut pairs = Vec::with_capacity(values.len());
        for value in values.iter_mut() {
            let text = value.get_string().to_string();
            let Some(pair) = value.get_dict() else {
                panic!("Each value must be written as {{name, value}}, for example: values: [{{bug, '🐛'}}, {{todo, '📝'}}] !");
            };
            pair.validate_positional_parameters(&text, VALUE_POSITIONAL_PARAMETERS).unwrap();
            pair.validate_named_parameters(&text, VALUE_NAMED_PARAMETERS).unwrap();
            let (Some(name), Some(inserted)) = (pair.get("name"), pair.get("value")) else {
                panic!("Each value needs a name and a value, for example: {{bug, '🐛'}} !");
            };
            pairs.push(format!("({:?}, {:?})", name.get_string(), inserted.get_string()));
        }
        return format!("control.add_list_with_values({trigger}, &[{}], {flags});\n", pairs.join(", "));
    }

    panic!("A list needs `items: [...]` or `values: [{{name, value}}, ...]` !");
}

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("markdown_composer", input, POSITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("MarkdownComposer::from");
    cb.add_string_parameter("content", Some(""));
    cb.add_layout();
    cb.add_flags_parameter("flags", "markdown_composer::Flags", &FLAGS);
    cb.finish_control_initialization();

    if cb.has_parameter("emoji") {
        let trigger = trigger_literal(cb.get_value("emoji").unwrap_or_default());
        cb.add_line(&format!("control.add_emoji_list({trigger});"));
    }

    if cb.has_parameter("lists") {
        let mut code = String::with_capacity(256);
        let Some(lists) = cb.get_list("lists") else {
            panic!("Parameter `lists` must contain a list of lists: lists: [{{'@', items: ['Ana', 'Dan']}}, ...] !");
        };
        for list in lists.iter_mut() {
            let text = list.get_string().to_string();
            let Some(dict) = list.get_dict() else {
                panic!("Each list must be written between {{ and }}, for example: {{'@', items: ['Ana', 'Dan']}} !");
            };
            dict.validate_positional_parameters(&text, LIST_POSITIONAL_PARAMETERS).unwrap();
            dict.validate_named_parameters(&text, LIST_NAMED_PARAMETERS).unwrap();
            code.push_str(&list_code(dict));
        }
        cb.add_line(&code);
    }

    cb.add_basecontrol_operations();
    cb.into()
}
