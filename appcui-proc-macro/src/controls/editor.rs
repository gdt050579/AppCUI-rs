use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ShowLineNumber", "ReadOnly", "ScrollBars", "HighlightCursor"]);

static POSITIONAL_PARAMETERS: &[PositionalParameter] = &[PositionalParameter::new("text", ParamType::String)];

static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("text", "text", ParamType::String),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("word_wrap", "word_wrap", ParamType::Bool),
    NamedParameter::new("ww", "wrap_width", ParamType::Integer),
    NamedParameter::new("tab_align", "tab_align", ParamType::Integer),
    NamedParameter::new("ta", "tab_align", ParamType::Integer),
    NamedParameter::new("left-scroll-margin", "lsm", ParamType::Integer),
    NamedParameter::new("lsm", "lsm", ParamType::Integer),
    NamedParameter::new("top-scroll-margin", "tsm", ParamType::Integer),
    NamedParameter::new("tsm", "tsm", ParamType::Integer),    
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("editor", input, POSITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("Editor::new");
    cb.add_string_parameter("text", Some(""));
    cb.add_layout();
    cb.add_flags_parameter("flags", "editor::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_scroll_margin_setup("lsm","tsm");
    if cb.has_parameter("word_wrap") {
        cb.add("control.set_word_wrap(");
        cb.add_bool_parameter("word_wrap", Some(false));
        cb.add(");\n");
    }    
    if cb.has_parameter("tab_align") {
        cb.add("control.set_tab_align(");
        cb.add_integer_parameter("tab_align", Some(4));
        cb.add(");\n");
    }
    cb.add_basecontrol_operations();
    cb.into()
}