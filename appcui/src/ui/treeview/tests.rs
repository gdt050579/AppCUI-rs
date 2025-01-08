use super::ListItem;
use super::TreeDataManager;

struct TestData {
    text: String,
}
impl TestData {
    pub fn new(text: &str) -> Self {
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

#[test]
fn check_tree_manager_chldren() {
    let mut tm = TreeDataManager::<TestData>::with_capacity(10);
    assert!(tm.first().is_none());
    let h1 = tm.add(TestData::new("1"));
    assert_eq!(tm.first(),h1);
    let h2 = tm.add(TestData::new("2"));
    for i in 0..10 {
        tm.add_to_parent(TestData::new(&format!("1.{i}")),h1);
    }
    for i in 0..20 {
        tm.add_to_parent(TestData::new(&format!("2.{i}")),h2);
    }
    assert_eq!(tm.first(),h2);
    assert_eq!(tm.free_list().len(),0);
    let mut h = tm.get(h2).unwrap().child;
    for i in 19..=0 {
        assert_eq!(tm.get(h).unwrap().value().value(),&format!("2.{i}"));
        h = tm.next(h);
    }
    let mut h = tm.get(h1).unwrap().child;
    for i in 9..=0 {
        assert_eq!(tm.get(h).unwrap().value().value(),&format!("1.{i}"));
        h = tm.next(h);
    }
    tm.delete_children(h1);
    assert_eq!(tm.free_list().len(),10);
    assert!(tm.get(h1).unwrap().child.is_none());
    tm.delete(h2);
    assert_eq!(tm.first(),h1);
    assert_eq!(tm.free_list().len(),31);
}

#[test]
fn check_tree_manager_delete_middle_child() {
    let mut tm = TreeDataManager::<TestData>::with_capacity(10);
    assert!(tm.first().is_none());
    let h1 = tm.add(TestData::new("1"));
    let h2 = tm.add(TestData::new("2"));
    let h3 = tm.add(TestData::new("3"));
    assert_eq!(tm.first(),h3);
    assert_eq!(tm.next(h3),h2);
    assert_eq!(tm.next(h2),h1);
    assert!(tm.next(h1).is_none());
    tm.delete(h2);
    assert_eq!(tm.first(),h3);
    assert_eq!(tm.next(h3),h1);
    assert!(tm.next(h1).is_none());
    assert_eq!(tm.free_list().len(),1);
}