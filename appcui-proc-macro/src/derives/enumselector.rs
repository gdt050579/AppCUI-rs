use super::enumeration::Enum;
use super::enumeration::EnumVariant;
use core::panic;
use proc_macro::*;
use std::str::FromStr;

static TEMPLATE: &str = r#"
impl EnumSelector for $(ENUM_NAME) {
    const COUNT: u32 = $(COUNT);

    fn from_index(index: u32) -> Option<Self> {
        match index {
            $(FROM_INDEX_MATCH)
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            $(NAME_MATCH)
            _ => "$(ENUM_NAME)",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            $(DESCRIPTION_MATCH)
            _ => "",
        }
    }
}
"#;

fn generate_variant_match_arms(variant: &EnumVariant, index: usize) -> (String, String, String) {
    let mut name: Option<&String> = None;
    let mut description: Option<&String> = None;
    let mut init_code: Option<&String> = None;

    for (attr_name, value) in variant.attributes.iter() {
        match attr_name.trim() {
            "name" | "Name" | "N" | "n" => {
                if name.is_some() {
                    panic!("Duplicate 'name' attributes found for variant '{}'. Use only one!", variant.name);
                }
                name = Some(value);
            }
            "description" | "Description" | "details" | "Details" | "D" | "d" => {
                if description.is_some() {
                    panic!("Duplicate 'description' attributes found for variant '{}'. Use only one!", variant.name);
                }
                description = Some(value);
            }
            "init" => {
                if init_code.is_some() {
                    panic!("Duplicate 'initialization' attributes found for variant '{}'. Use only one!", variant.name);
                }
                init_code = Some(value)
            }
            _ => {
                panic!(
                    "Unknown attribute: '{}' for field '{}'. Available attributes are: 'name' and 'description' and 'init'.",
                    &attr_name,
                    variant.name
                );
            }
        }
    }

    let name_value = name.unwrap_or(&variant.name);
    let empty = String::new();
    let description_value = description.unwrap_or(&empty);
    let fields_placeholder = if variant.fields.is_none() { "".to_string() } else { "{ .. }".to_string() };
    let init_value = init_code.unwrap_or(&empty);

    let from_index_arm = format!("{} => Some(Self::{} {}),", index, variant.name, init_value);
    let name_arm = format!("Self::{} {} => \"{}\",", variant.name, fields_placeholder, name_value);
    let description_arm = format!("Self::{} {} => \"{}\",", variant.name, fields_placeholder, description_value);

    (from_index_arm, name_arm, description_arm)
}



fn build_enumselector_code(en: &Enum) -> TokenStream {
    let count = en.variants.len();
    let mut from_index_match = String::new();
    let mut name_match = String::new();
    let mut description_match = String::new();

    for (i, variant) in en.variants.iter().enumerate() {
        let (fi, nm, dm) = generate_variant_match_arms(variant, i);
        from_index_match.push_str(&fi);
        from_index_match.push('\n');
        name_match.push_str(&nm);
        name_match.push('\n');
        description_match.push_str(&dm);
        description_match.push('\n');
    }
    
    let output = TEMPLATE
        .replace("$(ENUM_NAME)", &en.name)
        .replace("$(COUNT)", &count.to_string())
        .replace("$(FROM_INDEX_MATCH)", &from_index_match)
        .replace("$(NAME_MATCH)", &name_match)
        .replace("$(DESCRIPTION_MATCH)", &description_match);

    TokenStream::from_str(&output)
        .expect("Failed to convert enumselector derived definition to token stream")
}

pub fn derive(input: TokenStream) -> TokenStream {
    match Enum::from(input) {
        Ok(en) => build_enumselector_code(&en),
        Err(err) => panic!("Fail to derive EnumSelector for enum: {}", err),
    }
}
