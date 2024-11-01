use super::structure::Structure;
use proc_macro::*;
use std::str::FromStr;

static TEMPLATE: &str = r#"
impl listview::ListItem for $(STRUCT_NAME) {
    const COLUMNS_COUNT: u16 = $(COLUMNS_COUNT);
    fn column(index: u16) -> listview::Column{ 
        match index {
            $(COLUMNS)
            _ => listview::Column::new("", 10, TextAlignament::Left)
        }
    }
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            $(RENDER_METHODS)
            _ => None
        }
    }
    fn compare(&self, other: &Self, column_index: u16) -> Ordering {
        match column_index {
            $(COMPARE_METHODS)
            _ => std::cmp::Ordering::Equal
        }    
    }
}
"#;

fn build_derive_code(s: &Structure) -> TokenStream {
    let output = TEMPLATE.replace("$(STRUCT_NAME)", &s.name);
    TokenStream::from_str(&output).expect("Fail to convert structure derived definition to token stream")
}

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    match Structure::from(input) {
        Ok(s) => build_derive_code(&s),
        Err(desc) => panic!("Fail to derive ListItem for structure: {}", desc),
    }
}
