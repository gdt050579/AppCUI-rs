use super::control_builder::ControlBuilder;
use crate::bar::{parse_bar_attr, parse_custom_char};
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["Margins", "DimSegmentsOnSelection"]);

static SORTS: &[(&str, &str)] = &[
    ("None", "none"),
    ("HighestToLowest", "highesttolowest"),
    ("HighestToLowest", "highest-to-lowest"),
    ("LowestToHighest", "lowesttohighest"),
    ("LowestToHighest", "lowest-to-highest"),
    ("Alphabetical", "alphabetical"),
    ("Alphabetical", "alpha"),
    ("ReverseAlphabetical", "reversealphabetical"),
    ("ReverseAlphabetical", "reverse-alphabetical"),
];

static TOTALS: &[(&str, &str)] = &[("Auto", "auto"), ("Fixed", "fixed"), ("Fixed", "fix")];

static FILLS: &[(&str, &str)] = &[
    ("Solid", "solid"),
    ("Shade75", "shade75"),
    ("Shade50", "shade50"),
    ("Shade25", "shade25"),
    ("Braille", "braille"),
    ("Checkerboard", "checkerboard"),
    ("Grid", "grid"),
    ("GridDouble", "griddouble"),
    ("GridDouble", "grid-double"),
    ("CrossHatch", "crosshatch"),
    ("CrossHatch", "cross-hatch"),
    ("Dashed", "dashed"),
    ("DiagonalUp", "diagonalup"),
    ("DiagonalUp", "diagonal-up"),
    ("DiagonalDown", "diagonaldown"),
    ("DiagonalDown", "diagonal-down"),
    ("Notched", "notched"),
    ("Custom", "custom"),
];

static SEGMENT_POSITIONAL: &[PositionalParameter] = &[
    PositionalParameter::new("value", ParamType::String),
    PositionalParameter::new("label", ParamType::String),
];
static SEGMENT_NAMED: &[NamedParameter] = &[
    NamedParameter::new("value", "value", ParamType::String),
    NamedParameter::new("val", "value", ParamType::String),
    NamedParameter::new("v", "value", ParamType::String),
    NamedParameter::new("label", "label", ParamType::String),
    NamedParameter::new("text", "label", ParamType::String),
    NamedParameter::new("caption", "label", ParamType::String),
    NamedParameter::new("name", "label", ParamType::String),
    NamedParameter::new("fill", "fill", ParamType::String),
    NamedParameter::new("fill-type", "fill", ParamType::String),
    NamedParameter::new("filltype", "fill", ParamType::String),
    NamedParameter::new("ft", "fill", ParamType::String),
    NamedParameter::new("attr", "attr", ParamType::String),
    NamedParameter::new("attribute", "attr", ParamType::String),
    NamedParameter::new("charattr", "attr", ParamType::String),
    NamedParameter::new("color", "attr", ParamType::String),
];

static POSITIONAL_PARAMETERS: &[PositionalParameter] = &[];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("segments", "segments", ParamType::List),
    NamedParameter::new("values", "segments", ParamType::List),
    NamedParameter::new("data", "segments", ParamType::List),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("sort", "sort", ParamType::String),
    NamedParameter::new("sort-type", "sort", ParamType::String),
    NamedParameter::new("sorttype", "sort", ParamType::String),
    NamedParameter::new("order", "sort", ParamType::String),
    NamedParameter::new("total", "total", ParamType::String),
    NamedParameter::new("total-usage", "total", ParamType::String),
    NamedParameter::new("totalusage", "total", ParamType::String),
    NamedParameter::new("selected", "selected", ParamType::Integer),
    NamedParameter::new("select", "selected", ParamType::Integer),
    NamedParameter::new("selected-segment", "selected", ParamType::Integer),
    NamedParameter::new("numeric-format", "numeric-format", ParamType::String),
    NamedParameter::new("nf", "numeric-format", ParamType::String),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("usagechart", input, POSITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    cb.init_control("UsageChart::new");
    cb.add_layout();
    cb.add_flags_parameter("flags", "usagechart::Flags", &FLAGS);
    cb.finish_control_initialization();
    cb.call_method_with_string_parameter_parser("set_sort_type", "sort", parse_sort);
    cb.call_method_with_string_parameter_parser("set_total_usage", "total", parse_total);
    if cb.has_parameter("numeric-format") {
        let str_repr = String::from(cb.get_string_representation());
        let tmp = if let Some(d) = cb.get_dict("numeric-format") {
            crate::numericformat::builder::create_from_dict(&str_repr, d)
        } else if let Some(v) = cb.get_value("numeric-format") {
            let mut parsed = crate::parameter_parser::parse(v).unwrap_or_else(|e| {
                panic!("Invalid numeric-format: {v} !{e:?}");
            });
            crate::numericformat::builder::create_from_dict(v, &mut parsed)
        } else {
            panic!("Invalid numeric-format ! Expected a number format (e.g. 'dec', 'hex, prefix: 0x' or '{{dec, group: 3}}')");
        };
        cb.add("control.set_number_format(");
        cb.add(&tmp);
        cb.add(");\n");
    }
    // segments are added after sort and total, so every segment is laid out with those settings
    if cb.has_parameter("segments") {
        let param_list = cb.get_string_representation().to_string();
        let Some(list) = cb.get_list("segments") else {
            panic!("Parameter 'segments' in usagechart must be a list of segments: segments: [10, {{20, 'Label', fill: Grid}}, ...] !");
        };
        let mut items = Vec::with_capacity(list.len());
        for item in list.iter_mut() {
            if let Some(d) = item.get_dict() {
                items.push(parse_segment_from_dict(d, &param_list));
            } else if item.is_list() {
                panic!("Invalid segments format - a segment must be a number or a dictionary {{value, label: 'text', fill: Solid, attr: red}} !");
            } else {
                items.push(format!("usagechart::Segment::from({})", parse_u64(item.get_string(), "segments")));
            }
        }
        if !items.is_empty() {
            cb.add_line(&format!("control.add_segments([{}]);", items.join(", ")));
        }
    }
    if cb.has_parameter("selected") {
        let value = cb
            .get_i32("selected")
            .unwrap_or_else(|| panic!("Parameter 'selected' in usagechart should be a segment index (a non-negative integer) !"));
        if value < 0 {
            panic!("Parameter 'selected' in usagechart should be a segment index (a non-negative integer) but got {value} !");
        }
        cb.add_line(&format!("control.set_selected_segment(Some({value}));"));
    }
    cb.add_basecontrol_operations();
    cb.into()
}

fn find(key: &str, names: &[(&'static str, &'static str)], repr: &str) -> &'static str {
    match crate::utils::find_string_in_array(names, repr.trim()) {
        Some(name) => name,
        None => panic!("Invalid {key}: {repr} - expected one of: {}", crate::utils::join_strings(names)),
    }
}

fn parse_u64(repr: &str, key: &str) -> String {
    match repr.trim().parse::<u64>() {
        Ok(v) => format!("{v}u64"),
        Err(_) => panic!("Invalid {key} - expecting a positive integer (u64) but got {repr} !"),
    }
}

fn parse_sort(repr: &str) -> String {
    format!("usagechart::SortType::{}", find("sort", SORTS, repr))
}

fn parse_total(repr: &str) -> String {
    let repr = repr.trim();
    if let Ok(v) = repr.parse::<u64>() {
        return format!("usagechart::TotalUsage::Fixed({v}u64)");
    }
    let Some(fncall) = crate::fncall::FnCall::new(repr) else {
        panic!("Invalid total: {repr} - expected Auto, Fixed(n) or a positive integer (u64) !");
    };
    let name = find("total", TOTALS, fncall.name());
    match name {
        "Auto" => {
            assert!(fncall.params_count() == 0, "Invalid total: {repr} - Auto takes no parameters !");
            String::from("usagechart::TotalUsage::Auto")
        }
        _ => {
            assert!(
                fncall.params_count() == 1,
                "Invalid total: {repr} - expected Fixed(n), where n is a positive integer (u64) !"
            );
            format!("usagechart::TotalUsage::Fixed({})", parse_u64(fncall.param(0).unwrap(), "total"))
        }
    }
}

fn parse_fill(repr: &str) -> String {
    let repr = repr.trim();
    if let Some(fncall) = crate::fncall::FnCall::new(repr) {
        if let Some(name) = fncall.match_name(FILLS) {
            if name == "Custom" {
                assert!(
                    fncall.params_count() == 1,
                    "Invalid fill - expecting Custom(character), for example Custom('#') or Custom(code: 0x2588) !"
                );
                return format!(
                    "usagechart::SegmentFillType::Custom({})",
                    parse_custom_char(fncall.param(0).unwrap(), "fill")
                );
            }
            assert!(fncall.params_count() == 0, "Invalid fill - {name} takes no parameters !");
            return format!("usagechart::SegmentFillType::{name}");
        }
    }
    // a single character (for example `fill: '#'`) is the same as Custom('#')
    if repr.chars().count() == 1 {
        return format!("usagechart::SegmentFillType::Custom({})", parse_custom_char(repr, "fill"));
    }
    panic!(
        "Invalid fill: {repr} - expected one of: {}, Custom(character) or a single character",
        crate::utils::join_strings(FILLS)
    );
}

fn parse_segment_from_dict(dict: &mut NamedParamsMap, param_list: &str) -> String {
    dict.validate_positional_parameters(param_list, SEGMENT_POSITIONAL).unwrap();
    dict.validate_named_parameters(param_list, SEGMENT_NAMED).unwrap();
    dict.check_unkwnon_params(param_list, SEGMENT_POSITIONAL, SEGMENT_NAMED, None).unwrap();
    let value = match dict.get("value") {
        Some(v) => parse_u64(v.get_string(), "value"),
        None => panic!("Invalid segments format - missing 'value' ! Expected {{value, label: 'text', fill: Solid, attr: red}}"),
    };
    let mut code = format!("usagechart::SegmentBuilder::new().value({value})");
    if let Some(v) = dict.get("label") {
        let label = v.get_string();
        code.push_str(&format!(".label({label:?})"));
    }
    if let Some(v) = dict.get("fill") {
        let fill = parse_fill(v.get_string());
        code.push_str(&format!(".fill({fill})"));
    }
    let attr = match dict.get_mut("attr") {
        Some(v) if v.is_dict() => {
            let inner = v.get_dict().expect("attr was checked to be a dictionary");
            Some(crate::chars::builder::create_attr_from_dict(param_list, inner))
        }
        Some(v) => Some(parse_bar_attr(v.get_string(), "attr")),
        None => None,
    };
    if let Some(attr) = attr {
        code.push_str(&format!(".attr({attr})"));
    }
    code.push_str(".build()");
    code
}
