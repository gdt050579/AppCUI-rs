use super::control_builder::ControlBuilder;
use crate::bar::{parse_bar_attr, parse_custom_char, validate_bar_number};
use crate::parameter_parser::*;
use proc_macro::*;

static FLAGS: FlagsSignature = FlagsSignature::new(&["ScrollBars", "Margins", "DimCellsOnSelection"]);

static TOOLTIPS: &[(&str, &str)] = &[
    ("Value", "value"),
    ("Label", "label"),
    ("Percentage", "percentage"),
    ("Percentage", "percent"),
    ("LabelAndValue", "labelandvalue"),
    ("LabelAndValue", "label-and-value"),
    ("LabelAndValue", "label+value"),
    ("LabelAndPercentage", "labelandpercentage"),
    ("LabelAndPercentage", "label-and-percentage"),
    ("LabelAndPercentage", "label+percentage"),
];

static FILLS: &[(&str, &str)] = &[
    ("Shades", "shades"),
    ("Dots", "dots"),
    ("ASCII", "ascii"),
    ("Circles", "circles"),
    ("Squares", "squares"),
    ("Diamonds", "diamonds"),
    ("Crosses", "crosses"),
    ("Stars", "stars"),
    ("Asterisks", "asterisks"),
    ("Flowers", "flowers"),
    ("Pluses", "pluses"),
    ("Custom", "custom"),
];

static CELL_POSITIONAL: &[PositionalParameter] = &[
    PositionalParameter::new("value", ParamType::String),
    PositionalParameter::new("label", ParamType::String),
];
static CELL_NAMED: &[NamedParameter] = &[
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

static SELECTED_POSITIONAL: &[PositionalParameter] = &[
    PositionalParameter::new("row", ParamType::Integer),
    PositionalParameter::new("column", ParamType::Integer),
];
static SELECTED_NAMED: &[NamedParameter] = &[
    NamedParameter::new("row", "row", ParamType::Integer),
    NamedParameter::new("r", "row", ParamType::Integer),
    NamedParameter::new("column", "column", ParamType::Integer),
    NamedParameter::new("col", "column", ParamType::Integer),
    NamedParameter::new("c", "column", ParamType::Integer),
];

static POSITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("type", ParamType::String),
    PositionalParameter::new("size", ParamType::Size),
];
static NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("type", "type", ParamType::String),
    NamedParameter::new("class", "type", ParamType::String),
    NamedParameter::new("size", "size", ParamType::Size),
    NamedParameter::new("grid", "size", ParamType::Size),
    NamedParameter::new("sz", "size", ParamType::Size),
    NamedParameter::new("heatmap-size", "size", ParamType::Size),
    NamedParameter::new("columns", "columns", ParamType::Integer),
    NamedParameter::new("cols", "columns", ParamType::Integer),
    NamedParameter::new("rows", "rows", ParamType::Integer),
    NamedParameter::new("values", "values", ParamType::List),
    NamedParameter::new("data", "values", ParamType::List),
    NamedParameter::new("cells", "values", ParamType::List),
    NamedParameter::new("flags", "flags", ParamType::Flags),
    NamedParameter::new("cell-size", "cell-size", ParamType::Size),
    NamedParameter::new("cellsize", "cell-size", ParamType::Size),
    NamedParameter::new("cs", "cell-size", ParamType::Size),
    NamedParameter::new("tooltip", "tooltip", ParamType::String),
    NamedParameter::new("tooltip-format", "tooltip", ParamType::String),
    NamedParameter::new("tf", "tooltip", ParamType::String),
    NamedParameter::new("fill", "fill", ParamType::String),
    NamedParameter::new("default-fill", "fill", ParamType::String),
    NamedParameter::new("attr", "attr", ParamType::String),
    NamedParameter::new("default-attr", "attr", ParamType::String),
    NamedParameter::new("color", "attr", ParamType::String),
    NamedParameter::new("selected", "selected", ParamType::Dict),
];

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let mut cb = ControlBuilder::new("heatmap", input, POSITIONAL_PARAMETERS, NAMED_PARAMETERS, true);
    let (columns, rows) = grid_size(&mut cb);
    cb.init_control_with_template("HeatMap", "new", "type");
    cb.add_layout();
    if cb.has_parameter("size") {
        cb.add_size_parameter("size", None);
    } else {
        cb.add(&format!(", Size::new({columns},{rows})"));
    }
    cb.add_flags_parameter("flags", "heatmap::Flags", &FLAGS);
    cb.finish_control_initialization();
    if cb.has_parameter("cell-size") {
        let repr = cb.get_value("cell-size").unwrap().to_string();
        let (width, height) = parse_size_pair(&repr, "cell-size");
        cb.add_line(&format!("control.set_cell_size(Size::new({width},{height}));"));
    }
    cb.call_method_with_string_parameter_parser("set_tooltip_format", "tooltip", |repr| {
        format!("heatmap::TooltipFormat::{}", find("tooltip", TOOLTIPS, repr))
    });
    cb.call_method_with_string_parameter_parser("set_default_fill", "fill", parse_fill);
    if cb.has_parameter("attr") {
        let repr = cb.get_string_representation().to_string();
        let attr = if let Some(dict) = cb.get_dict("attr") {
            format!(
                "heatmap::CellAttribute::Fixed({})",
                crate::chars::builder::create_attr_from_dict(&repr, dict)
            )
        } else {
            parse_cell_attribute(cb.get_value("attr").unwrap())
        };
        cb.add_line(&format!("control.set_default_attr({attr});"));
    }
    if cb.has_parameter("values") {
        let param_list = cb.get_string_representation().to_string();
        let Some(list) = cb.get_list("values") else {
            panic!("Parameter 'values' in heatmap must be a list of cells or a list of rows: values: [[1, 2], [{{3, label: Peak}}, none]] !");
        };
        let code = parse_values(list, columns, rows, &param_list);
        if !code.is_empty() {
            cb.add_line(&code);
        }
    }
    if cb.has_parameter("selected") {
        let param_list = cb.get_string_representation().to_string();
        let Some(dict) = cb.get_dict("selected") else {
            panic!("Parameter 'selected' in heatmap must be a cell position: selected: {{row, column}} !");
        };
        let (row, column) = parse_selected(dict, &param_list);
        if row >= rows || column >= columns {
            panic!("Parameter 'selected' in heatmap is outside the grid ({columns} columns, {rows} rows) but got row {row}, column {column} !");
        }
        cb.add_line(&format!("control.set_selected_cell(Some(({row}, {column})));"));
    }
    cb.add_basecontrol_operations();
    cb.into()
}

fn grid_size(cb: &mut ControlBuilder) -> (u32, u32) {
    let has_size = cb.has_parameter("size");
    let has_columns = cb.has_parameter("columns");
    let has_rows = cb.has_parameter("rows");
    if has_size && (has_columns || has_rows) {
        panic!("heatmap accepts either size (for example 24x14) or columns and rows, not both");
    }
    if has_size {
        let repr = cb.get_value("size").unwrap().to_string();
        return parse_size_pair(&repr, "size");
    }
    if has_columns != has_rows {
        panic!("heatmap needs both columns and rows (for example columns: 24, rows: 14), or a size such as 24x14");
    }
    if has_columns {
        return (positive_dimension(cb, "columns"), positive_dimension(cb, "rows"));
    }
    panic!("heatmap requires a grid size: size: 24x14, or columns: 24, rows: 14");
}

fn positive_dimension(cb: &mut ControlBuilder, name: &str) -> u32 {
    let Some(value) = cb.get_i32(name) else {
        panic!("Parameter '{name}' in heatmap should be a whole number from 1 to 32000");
    };
    if !(1..=32_000).contains(&value) {
        panic!("Parameter '{name}' in heatmap should be a whole number from 1 to 32000 but got {value}");
    }
    value as u32
}

fn parse_size_pair(repr: &str, key: &str) -> (u32, u32) {
    let repr = repr.trim();
    let Some(sep) = repr.find(['x', 'X', ',']) else {
        panic!("Invalid {key}: {repr} - expected width x height, for example 24x14");
    };
    let width = parse_dimension(repr[..sep].trim(), key);
    let height = parse_dimension(repr[sep + 1..].trim(), key);
    (width, height)
}

fn parse_dimension(repr: &str, key: &str) -> u32 {
    match repr.parse::<u32>() {
        Ok(value) if (1..=32_000).contains(&value) => value,
        _ => panic!("Invalid {key}: {repr} - expected a size from 1 to 32000, for example 24x14"),
    }
}

fn find(key: &str, names: &[(&'static str, &'static str)], repr: &str) -> &'static str {
    match crate::utils::find_string_in_array(names, repr.trim()) {
        Some(name) => name,
        None => panic!("Invalid {key}: {repr} - expected one of: {}", crate::utils::join_strings(names)),
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
                return format!("heatmap::CellFill::Custom({})", parse_custom_char(fncall.param(0).unwrap(), "fill"));
            }
            assert!(fncall.params_count() == 0, "Invalid fill - {name} takes no parameters !");
            return format!("heatmap::CellFill::{name}");
        }
    }
    // a single character (for example `fill: '#'`) is the same as Custom('#')
    if repr.chars().count() == 1 {
        return format!("heatmap::CellFill::Custom({})", parse_custom_char(repr, "fill"));
    }
    panic!(
        "Invalid fill: {repr} - expected one of: {}, Custom(character) or a single character",
        crate::utils::join_strings(FILLS)
    );
}

fn parse_cell_attribute(repr: &str) -> String {
    let repr = repr.trim();
    if let Some(fncall) = crate::fncall::FnCall::new(repr) {
        let name = fncall.name();
        if crate::utils::equal_ignore_case(name, "range") {
            assert!(
                fncall.params_count() == 2,
                "Invalid attr - expecting Range(start, end), for example Range(blue, red) !"
            );
            let start = parse_bar_attr(fncall.param(0).unwrap(), "attr");
            let end = parse_bar_attr(fncall.param(1).unwrap(), "attr");
            return format!("heatmap::CellAttribute::Range({start}, {end})");
        }
        if crate::utils::equal_ignore_case(name, "fixed") {
            assert!(
                fncall.params_count() == 1,
                "Invalid attr - expecting Fixed(attribute), for example Fixed(red) !"
            );
            return format!("heatmap::CellAttribute::Fixed({})", parse_bar_attr(fncall.param(0).unwrap(), "attr"));
        }
    }
    format!("heatmap::CellAttribute::Fixed({})", parse_bar_attr(repr, "attr"))
}

fn parse_values(list: &mut [Value], columns: u32, rows: u32, param_list: &str) -> String {
    if list.is_empty() {
        return String::new();
    }
    let rows_of_cells = list.iter().any(|item| item.is_list());
    if rows_of_cells && list.iter().any(|item| !item.is_list()) {
        panic!("Invalid values format - use either a grid of rows ([[1, 2], [3, 4]]) or a flat list ([1, 2, 3, 4]), not both");
    }
    let mut code = String::new();
    if rows_of_cells {
        if list.len() as u32 > rows {
            panic!("Invalid values format - the grid has {rows} rows but the list has {} !", list.len());
        }
        for (row, item) in list.iter_mut().enumerate() {
            let Some(row_list) = item.get_list() else {
                panic!("Invalid values format - every row must be a list of cells");
            };
            if row_list.len() as u32 > columns {
                panic!(
                    "Invalid values format - row {} has {} cells but the grid has {columns} columns !",
                    row + 1,
                    row_list.len()
                );
            }
            append_row(&mut code, row_list, row as u32, param_list);
        }
    } else {
        let count = (columns as usize).saturating_mul(rows as usize);
        if list.len() > count {
            panic!("Invalid values format - the grid has {count} cells but the list has {} !", list.len());
        }
        for (index, item) in list.iter_mut().enumerate() {
            if item.is_list() {
                panic!("Invalid values format - a flat list cannot contain a row. Use [[1, 2], [3, 4]] for rows");
            }
            let column = (index as u32) % columns;
            let row = (index as u32) / columns;
            if let Some(expr) = parse_cell(item, param_list) {
                code.push_str(&format!("control.set({row}, {column}, {expr});\n"));
            }
        }
    }
    code
}

fn append_row(code: &mut String, row_list: &mut [Value], row: u32, param_list: &str) {
    for (column, item) in row_list.iter_mut().enumerate() {
        if item.is_list() {
            panic!("Invalid values format - a cell must be a number, none, or a dictionary {{value, label, fill, attr}} !");
        }
        if let Some(expr) = parse_cell(item, param_list) {
            code.push_str(&format!("control.set({row}, {column}, {expr});\n"));
        }
    }
}

fn parse_cell(item: &mut Value, param_list: &str) -> Option<String> {
    if let Some(dict) = item.get_dict() {
        return Some(parse_cell_from_dict(dict, param_list));
    }
    let repr = item.get_string().trim();
    if is_empty_cell(repr) {
        return None;
    }
    validate_bar_number(repr);
    Some(repr.to_string())
}

fn is_empty_cell(repr: &str) -> bool {
    repr.is_empty() || repr == "_" || crate::utils::equal_ignore_case(repr, "none")
}

fn parse_cell_from_dict(dict: &mut NamedParamsMap, param_list: &str) -> String {
    dict.validate_positional_parameters(param_list, CELL_POSITIONAL).unwrap();
    dict.validate_named_parameters(param_list, CELL_NAMED).unwrap();
    dict.check_unkwnon_params(param_list, CELL_POSITIONAL, CELL_NAMED, None).unwrap();
    let value = match dict.get("value") {
        Some(v) => {
            let text = v.get_string().trim().to_string();
            validate_bar_number(&text);
            text
        }
        None => panic!("Invalid values format - missing 'value' ! Expected {{value, label: 'text', fill: Dots, attr: red}}"),
    };
    let mut code = format!("heatmap::CellBuilder::new({value})");
    if let Some(v) = dict.get("label") {
        let label = v.get_string();
        code.push_str(&format!(".label({label:?})"));
    }
    if let Some(v) = dict.get("fill") {
        code.push_str(&format!(".fill({})", parse_fill(v.get_string())));
    }
    let attr = match dict.get_mut("attr") {
        Some(v) if v.is_dict() => {
            let inner = v.get_dict().expect("attr was checked to be a dictionary");
            Some(format!(
                "heatmap::CellAttribute::Fixed({})",
                crate::chars::builder::create_attr_from_dict(param_list, inner)
            ))
        }
        Some(v) => Some(parse_cell_attribute(v.get_string())),
        None => None,
    };
    if let Some(attr) = attr {
        code.push_str(&format!(".attr({attr})"));
    }
    code.push_str(".build()");
    code
}

fn parse_selected(dict: &mut NamedParamsMap, param_list: &str) -> (u32, u32) {
    dict.validate_positional_parameters(param_list, SELECTED_POSITIONAL).unwrap();
    dict.validate_named_parameters(param_list, SELECTED_NAMED).unwrap();
    dict.check_unkwnon_params(param_list, SELECTED_POSITIONAL, SELECTED_NAMED, None).unwrap();
    let row = required_index(dict, "row");
    let column = required_index(dict, "column");
    (row, column)
}

fn required_index(dict: &mut NamedParamsMap, key: &str) -> u32 {
    let Some(value) = dict.get_mut(key).and_then(|value| value.get_i32()) else {
        panic!("Parameter 'selected' in heatmap is missing '{key}' ! Expected {{row, column}}");
    };
    if value < 0 {
        panic!("Parameter 'selected' in heatmap: '{key}' must be a positive number but got {value} !");
    }
    value as u32
}
