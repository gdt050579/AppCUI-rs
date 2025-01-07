use super::ListItem;
use super::TreeDataManager;

struct TestData {
    text: String,
}
impl TestData {
    pub fn new(text: &'static str) -> Self {
        Self { text: text.to_string() }   
    }
    pub fn value(&self) -> &str {
        &self.text
    }
}
impl ListItem for TestData {
    fn render_method(&self, column_index: u16) -> Option<super::RenderMethod> {
        match column_index {
            0 => Some(super::RenderMethod::Text(self.text.as_str())),
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
fn check_tree_manager_test_root() {
    let mut tm = TreeDataManager::<TestData>::with_capacity(10);
    assert!(tm.first().is_none());
    let h1 = tm.add(TestData::new("Item 1"));
    assert_eq!(tm.first(),h1);
    let h2 = tm.add(TestData::new("Item 2"));
    assert_eq!(tm.first(),h2);
    assert_eq!(tm.next(h2),h1);
    assert_eq!(tm.get(h1).unwrap().value().value(),"Item 1");
    assert_eq!(tm.get(h2).unwrap().value().value(),"Item 2");
    tm.delete(h2);
    assert_eq!(tm.first(),h1);
    tm.delete(h1);
    assert!(tm.first().is_none());
}