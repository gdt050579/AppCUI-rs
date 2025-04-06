use super::enumeration::Enum;
use super::enumeration::EnumVariant;
use core::panic;
use proc_macro::*;
use std::str::FromStr;

static TEMPLATE: &str = r#"
impl DropDownListType for $(ENUM_NAME) {
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

    fn symbol(&self) -> &'static str {
        match self {
            $(SYMBOL_MATCH)
            _ => "",
        }
    }
}
"#;

fn generate_variant_match_arms(variant: &EnumVariant) -> (String, String, String) {
    let mut name: Option<&String> = None;
    let mut description: Option<&String> = None;
    let mut symbol: Option<&String> = None;

    for (attr_name, value) in variant.attributes.iter() {
        if let Some(next_str) = attr_name.strip_prefix("VariantInfo.")
        {
            match next_str.trim() {
                "name" | "Name" | "N" | "n" => {
                    if name.is_some() {
                        panic!("Duplicate 'name' attributes found for variant '{}'.", variant.name);
                    }
                    name = Some(value);
                }
                "description" | "Description" | "details" | "Details" | "D" | "d" => {
                    if description.is_some() {
                        panic!("Duplicate 'description' attributes found for variant '{}'.", variant.name);
                    }
                    description = Some(value);
                }
                "symbol" | "Symbol" | "S" | "s" => {
                    if symbol.is_some() {
                        panic!("Duplicate 'symbol' attributes found for variant '{}'.", variant.name);
                    }
                    symbol = Some(value);
                }
                _ => {
                    panic!(
                        "Unknown attribute: '{}' for field '{}'. Allowed: 'name', 'description', 'symbol'.",
                        attr_name, variant.name
                    );
                }
            }
        }
    }

    let name_value = name.unwrap_or(&variant.name);
    let empty = String::new();
    let description_value = description.unwrap_or(&empty);
    let symbol_value = symbol.unwrap_or(&empty);
    let fields_placeholder = if variant.fields.is_none() { "".to_string() } else { "{ .. }".to_string() };

    let name_arm = format!("Self::{} {} => \"{}\",", variant.name, fields_placeholder, name_value);
    let description_arm = format!("Self::{} {} => \"{}\",", variant.name, fields_placeholder, description_value);
    let symbol_arm = format!("Self::{} {} => \"{}\",", variant.name, fields_placeholder, symbol_value);

    (name_arm, description_arm, symbol_arm)
}

fn build_dropdownlisttype_code(en: &Enum) -> TokenStream {
    let mut name_match = String::new();
    let mut description_match = String::new();
    let mut symbol_match = String::new();

    for variant in &en.variants {
        let (nm, dm, sm) = generate_variant_match_arms(variant);
        name_match.push_str(&nm);
        name_match.push('\n');
        description_match.push_str(&dm);
        description_match.push('\n');
        symbol_match.push_str(&sm);
        symbol_match.push('\n');
    }
    
    let output = TEMPLATE
        .replace("$(ENUM_NAME)", &en.name)
        .replace("$(NAME_MATCH)", &name_match)
        .replace("$(DESCRIPTION_MATCH)", &description_match)
        .replace("$(SYMBOL_MATCH)", &symbol_match);

    TokenStream::from_str(&output)
        .expect("Failed to generate DropDownListType implementation")
}

pub fn derive(input: TokenStream) -> TokenStream {
    match Enum::from(input) {
        Ok(en) => build_dropdownlisttype_code(&en),
        Err(err) => panic!("Fail to derive DropDownListType for enum: {}", err),
    }
}
