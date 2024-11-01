use proc_macro::*;
use std::str::FromStr;

static TEMPLATE: &str = r#"
impl ListItem for $(STRUCT_NAME) {
    const COLUMNS_COUNT: u16 = $(COLUMNS_COUNT);
    fn column(index: u16) -> Column{ 
        match index {
            $(COLUMNS)
            _ => Column::new("", 10, TextAlignament::Left)
        }
    }
    fn render_method(&self, column_index: u16) -> Option<RenderMethod> {
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

pub(crate) fn derive(input: TokenStream) -> TokenStream {

    let s = TEMPLATE.to_string();
    TokenStream::from_str(&s).expect("Fail to convert structure derived definition to token stream")
}