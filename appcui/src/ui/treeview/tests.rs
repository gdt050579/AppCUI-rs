use super::ListItem;
use super::TreeDataManager;

struct TestData {
    text: &'static str,
}
impl TestData {
    pub fn new(text: &'static str) -> Self {
        Self { text }
    }
}
impl ListItem for TestData {
    fn render_method(&self, column_index: u16) -> Option<super::RenderMethod> {
        match column_index {
            0 => Some(super::RenderMethod::Text(self.text)),
            _ => None,
        }
    }
    
    fn columns_count() -> u16 {
        1
    }    
    fn column(_: u16) -> crate::prelude::Column {
        crate::prelude::Column::new("", 10, crate::prelude::TextAlignament::Left)
    }    
}

#[test]
fn check_tree_manager() {
    let mut tm = TreeDataManager::<TestData>::with_capacity(10);
    assert!(tm.first().is_none());
    let h1 = tm.add(TestData::new("Item 1"));
    assert_eq!(tm.first(),h1);
}