use super::control_builder::ControlBuilder;
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ScrollBars", "SearchBar"]);

static POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("background", "back", ParamType::Dict),
    NamedParameter::new("back", "back", ParamType::Dict),
    NamedParameter::new("left-scroll-margin", "lsm", ParamType::Integer),
    NamedParameter::new("lsm", "lsm", ParamType::Integer),
    NamedParameter::new("top-scroll-margin", "tsm", ParamType::Integer),
    NamedParameter::new("tsm", "tsm", ParamType::Integer),
    // line time
    NamedParameter::new("line-type", "elt", ParamType::String),
    NamedParameter::new("edge-line-type", "elt", ParamType::String),
    NamedParameter::new("elt", "elt", ParamType::String),
    // routing
    NamedParameter::new("routing", "routing", ParamType::String),
    NamedParameter::new("edge-routing", "routing", ParamType::String),
    // arrange
    NamedParameter::new("arrange", "arrange", ParamType::String),
    NamedParameter::new("arrange-nodes", "arrange", ParamType::String),
    // arrow headz
    NamedParameter::new("arrow-heads", "arrow-heads", ParamType::Bool),
    NamedParameter::new("arrows", "arrow-heads", ParamType::Bool),
    // edge highlighting
    NamedParameter::new("highlight-incoming-edges", "hie", ParamType::Bool),
    NamedParameter::new("hie", "hie", ParamType::Bool),
    NamedParameter::new("highlight-outgoing-edges", "hoe", ParamType::Bool),
    NamedParameter::new("hoe", "hoe", ParamType::Bool)
];

static LINE_TYPE: FlagsSignature = FlagsSignature::new(&[
    "Single",
    "Double",
    "SingleThick",
    "Border",
    "Ascii",
    "AsciiRound",
    "SingleRound",
    "Braille",
]);

static ROUTING: FlagsSignature = FlagsSignature::new(&[
    "Direct",
    "Orthogonal",
]);

static ARRANGE: FlagsSignature = FlagsSignature::new(&[
    "Grid",
    "Circular",
    "Hierarchical",
    "HierarchicalPacked",
    "ForceDirected",
]);

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("graphview", input, POSILITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("GraphView::new");
    cb.add_layout();
    cb.add_flags_parameter("flags", "graphview::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.add_basecontrol_operations();

    // background
    if cb.has_parameter("back") {
        let str_repr = String::from(cb.get_string_representation());
        if let Some(d) = cb.get_dict("back") {
            let s = crate::chars::builder::create_from_dict(&str_repr, d);
            cb.add_line(format!("control.set_background({s});").as_str());
        }
    }

    // edge-line-type
    if cb.has_parameter("elt") {
        cb.add("control.set_edge_line_type(");
        cb.add_enum_parameter("elt", "LineType", &LINE_TYPE, None);
        cb.add_line(");");
    }

    // edge routing
    if cb.has_parameter("routing") {
        cb.add("control.set_edge_routing(");
        cb.add_enum_parameter("routing", "graphview::EdgeRouting", &ROUTING, None);
        cb.add_line(");");
    }   

    // arrange
    if cb.has_parameter("arrange") {
        cb.add("control.arrange_nodes(");
        cb.add_enum_parameter("arrange", "graphview::ArrangeMethod", &ARRANGE, None);
        cb.add_line(");");
    }      
    // arrow heads
    if cb.has_parameter("arrow-heads") {
        let v = cb.get_bool("arrow-heads").unwrap_or(false);
        cb.add_line(format!("control.enable_arrow_heads({v});").as_str());
    }   
    // edge highlighting
    if cb.has_parameter("hie") || cb.has_parameter("hoe") {
        let hie = cb.get_bool("hie").unwrap_or(false);
        let hoe = cb.get_bool("hoe").unwrap_or(false);
        cb.add_line(format!("control.enable_edge_highlighting({hie}, {hoe});").as_str());
    }


    cb.add_scroll_margin_setup("lsm", "tsm");
    cb.into()
}
