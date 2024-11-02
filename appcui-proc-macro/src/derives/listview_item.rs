use super::structure::Structure;
use super::structure::StructureField;
use core::panic;
use proc_macro::*;
use std::str::FromStr;
use std::u32;

static TEMPLATE: &str = r#"
impl listview::ListItem for $(STRUCT_NAME) {
    const COLUMNS_COUNT: u16 = $(COLUMNS_COUNT);
    fn column(index: u16) -> components::Column{ 
        match index {
            $(COLUMNS)
            _ => components::Column::new("", 10, TextAlignament::Left)
        }
    }
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            $(RENDER_METHODS)
            _ => None
        }
    }
    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            $(COMPARE_METHODS)
            _ => std::cmp::Ordering::Equal
        }    
    }
}
"#;

struct Column {
    name: String,
    width: u16,
    index: u32,
    align: &'static str,
    varname: String,
    vartype: String,
}
impl Column {
    fn align(value: &str) -> Option<&'static str> {
        if crate::utils::equal_ignore_case(value, "left") {
            Some("Left")
        } else if crate::utils::equal_ignore_case(value, "l") {
            return Some("Left");
        } else if crate::utils::equal_ignore_case(value, "right") {
            return Some("Right");
        } else if crate::utils::equal_ignore_case(value, "r") {
            return Some("Right");
        } else if crate::utils::equal_ignore_case(value, "center") {
            return Some("Center");
        } else if crate::utils::equal_ignore_case(value, "c") {
            return Some("Center");
        } else {
            None
        }
    }
    fn try_from(field: &StructureField) -> Option<Self> {
        if field.attributes.is_empty() {
            return None;
        }
        let mut name = None;
        let mut align = None;
        let mut width = None;
        let mut idx = u32::MAX;
        for (attr_name, value) in field.attributes.iter() {
            if !attr_name.starts_with("Column.") {
                continue;
            }
            match attr_name[7..].trim() {
                "align" | "a" | "alignament" => {
                    align = Column::align(value);
                    if align.is_none() {
                        panic!(
                            "Unknown alignament value: '{}' for field '{}'. Allowed values are 'left', 'right' or 'center' !",
                            value, field.name
                        );
                    }
                }
                "name" | "text" | "caption" => {
                    name = Some(value.to_string());
                }
                "width" | "w" | "size" => {
                    if let Ok(v) = value.parse::<u16>() {
                        if v > 200 {
                            panic!(
                                "Invalid width value: '{}' for field '{}'. Maximum allowed value is 200 !",
                                value, field.name
                            );
                        }
                        width = Some(v);
                    } else {
                        panic!("Invalid numerical value for width: '{}' for field '{}'", value, field.name);
                    }
                }
                "index" | "idx" => {
                    if let Ok(idx_value) = value.parse::<u32>() {
                        idx = idx_value;
                    } else {
                        panic!("Invalid numerical value for column index: '{}' for field '{}'", value, field.name);
                    }
                }
                _ => {
                    panic!(
                        "Unknown attribute: '{}' for field '{}'. Available attributes are: 'name', 'align', 'width'",
                        &attr_name[7..],
                        field.name
                    );
                }
            }
        }
        if name.is_none() {
            panic!("Missing 'name' attribute for field '{}'", field.name);
        }
        Some(Self {
            name: name.unwrap(),
            index: idx,
            width: width.unwrap_or(10),
            align: align.unwrap_or("Left"),
            varname: field.name.clone(),
            vartype: field.ty.clone(),
        })
    }
    fn to_column_code(&self, index: usize) -> String {
        format!(
            "{} => components::Column::new(\"{}\", {}, TextAlignament::{}),\n",
            index, self.name, self.width, self.align
        )
    }
    fn to_compare_code(&self, index: usize) -> String {
        match self.vartype.as_str() {
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => {
                format!(
                    "{} => self.{}.cmp(&other.{}),\n",
                    index, self.varname, self.varname
                )
            }
            "f32" | "f64" => {
                format!(
                    "{} => self.{}.partial_cmp(&other.{}).unwrap_or(std::cmp::Ordering::Equal),\n",
                    index, self.varname, self.varname
                )
            }
            "bool" => {
                format!(
                    "{} => self.{}.cmp(&other.{}),\n",
                    index, self.varname, self.varname
                )
            }
            "char" => {
                format!(
                    "{} => self.{}.cmp(&other.{}),\n",
                    index, self.varname, self.varname
                )
            }
            "String" => {
                format!(
                    "{} => self.{}.cmp(&other.{}),\n",
                    index, self.varname, self.varname
                )
            }
            "&str" => {
                format!(
                    "{} => self.{}.cmp(other.{}),\n",
                    index, self.varname, self.varname
                )
            }
            _ => panic!("Unsupported type '{}' for field '{}'. Implement ListItem manually to provide explicit implementation for this type !", self.vartype, self.varname),
        }
    }
    fn to_render_code(&self, index: usize) -> String {
        format!(
            "{} => Some(listview::RenderMethod::Text(self.{})),\n",
            index, self.varname
        )
    }
}

fn build_derive_code(s: &Structure) -> TokenStream {
    let mut columns = Vec::with_capacity(s.fields.len());
    let mut last_index = u32::MAX;
    let mut first_index = u32::MAX;
    for field in &s.fields {
        if let Some(mut column) = Column::try_from(field) {
            if column.index == u32::MAX {
                if last_index == u32::MAX {
                    column.index = 0;
                } else {
                    column.index = last_index + 1;
                }
            }
            if first_index == u32::MAX {
                first_index = column.index;
            }
            last_index = column.index;
            columns.push(column);
        }
    }
    if columns.is_empty() {
        panic!(
            "No columns defined for structure '{}' (use #[Column(...)] before structure fields to specify column information)",
            s.name
        );
    }
    if first_index > 1 {
        panic!("Invalid column index. The first column index must be 0 or 1, but it is {}", first_index);
    }
    if last_index + 1 - first_index != columns.len() as u32 {
        panic!("Invalid column indexes. Make sure that all column indexes are unique and sequential starting from 0 or 1");
    }
    columns.sort_by(|a, b| a.index.cmp(&b.index));
    let mut columns_code = String::new();
    let mut compare_code = String::new();
    let mut render_code = String::new();
    for (index, column) in columns.iter().enumerate() {
        columns_code.push_str(&column.to_column_code(index));
        compare_code.push_str(&column.to_compare_code(index));
        render_code.push_str(&column.to_render_code(index));
    }

    let output = TEMPLATE
        .replace("$(STRUCT_NAME)", &s.name)
        .replace("$(COLUMNS_COUNT)", &columns.len().to_string())
        .replace("$(COLUMNS)", &columns_code)
        .replace("$(COMPARE_METHODS)", &compare_code)
        .replace("$(RENDER_METHODS)", &render_code);
    TokenStream::from_str(&output).expect("Fail to convert structure derived definition to token stream")
}

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    match Structure::from(input) {
        Ok(s) => build_derive_code(&s),
        Err(desc) => panic!("Fail to derive ListItem for structure: {}", desc),
    }
}
