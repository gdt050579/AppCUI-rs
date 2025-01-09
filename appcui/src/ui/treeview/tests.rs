use crate::system::Handle;

use super::Item;
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
    assert!(tm.roots().is_empty());
    let h1 = tm.add(Item::from(TestData::new("Item 1")), Handle::None);
    let h2 = tm.add(Item::from(TestData::new("Item 2")), Handle::None);
    assert_eq!(tm.roots().len(), 2);
    assert_eq!(tm.roots()[0], h1);
    assert_eq!(tm.roots()[1], h2);
    assert_eq!(tm.get(h1).unwrap().value().value(), "Item 1");
    assert_eq!(tm.get(h2).unwrap().value().value(), "Item 2");
    tm.delete(h1);
    assert_eq!(tm.roots().len(), 1);
    assert_eq!(tm.roots()[0], h2);
    tm.delete(h2);
    assert!(tm.roots().is_empty());
}

#[test]
fn check_tree_manager_chldren() {
    let mut tm = TreeDataManager::<TestData>::with_capacity(10);
    assert!(tm.roots().is_empty());
    let h1 = tm.add(Item::from(TestData::new("1")), Handle::None);
    let h2 = tm.add(Item::from(TestData::new("2")), Handle::None);
    assert_eq!(tm.roots().len(), 2);
    assert_eq!(tm.roots()[0], h1);
    assert_eq!(tm.roots()[1], h2);
    for i in 0..10 {
        tm.add(Item::from(TestData::new(&format!("1.{i}"))), h1);
    }
    for i in 0..20 {
        tm.add(Item::from(TestData::new(&format!("2.{i}"))), h2);
    }
    assert_eq!(tm.free_list().len(), 0);
    for (i, h) in tm.get(h1).unwrap().children.iter().enumerate() {
        assert_eq!(tm.get(*h).unwrap().value().value(), &format!("1.{i}"));
    }
    for (i, h) in tm.get(h2).unwrap().children.iter().enumerate() {
        assert_eq!(tm.get(*h).unwrap().value().value(), &format!("2.{i}"));
    }
    assert_eq!(tm.get(h1).unwrap().children.len(), 10);
    assert_eq!(tm.get(h2).unwrap().children.len(), 20);

    tm.delete_children(h1);
    assert_eq!(tm.free_list().len(), 10);
    assert_eq!(tm.get(h1).unwrap().children.len(), 0);
    tm.delete(h2);
    assert_eq!(tm.free_list().len(), 31);
    assert_eq!(tm.roots().len(), 1);
    assert_eq!(tm.roots()[0], h1);
}

#[test]
fn check_tree_manager_delete_middle_child() {
    let mut tm = TreeDataManager::<TestData>::with_capacity(10);
    assert!(tm.roots().is_empty());
    let h1 = tm.add(Item::from(TestData::new("1")), Handle::None);
    let h2 = tm.add(Item::from(TestData::new("2")), Handle::None);
    let h3 = tm.add(Item::from(TestData::new("3")), Handle::None);
    assert_eq!(tm.roots().len(), 3);
    assert_eq!(tm.roots()[0], h1);
    assert_eq!(tm.roots()[1], h2);
    assert_eq!(tm.roots()[2], h3);

    tm.delete(h2);
    assert_eq!(tm.roots().len(), 2);
    assert_eq!(tm.roots()[0], h1);
    assert_eq!(tm.roots()[1], h3);
    
    assert_eq!(tm.free_list().len(), 1);
}
