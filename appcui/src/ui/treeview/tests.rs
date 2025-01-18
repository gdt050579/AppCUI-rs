use crate::prelude::*;
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


#[derive(ListViewItem)]
struct Course {
    #[Column(name = "&Name", width = 30)]
    name: String,
    #[Column(name = "&Relevance", width = 15)]
    relevance: u32,
    #[Column(name = "&Credits", width = 15)]
    credits: u32,
}
impl Course {
    fn new(name: &str, relevance: u32, credits: u32) -> Self {
        Self {
            name: name.to_string(),
            relevance,
            credits,
        }
    }
    fn populate_with_courses(tv:&mut TreeView<Course>) {
        tv.add(Course::new("John", 20, 10));
        let h2 = tv.add(Course::new("Alice", 21, 11));
        let h2_2 = tv.add_to_parent(Course::new("Math", 1, 9), h2);
        tv.add_to_parent(Course::new("Advance Calculus", 2, 8), h2_2);
        tv.add_to_parent(Course::new("Geometry", 1, 6), h2_2); 
        tv.add_to_parent(Course::new("Logic", 3, 9), h2_2);
    
        let h3 = tv.add_to_parent(Course::new("English", 2, 7), h2);
        let h3_1 = tv.add_to_parent(Course::new("Grammar", 1, 8), h3);
        tv.add_to_parent(Course::new("Syntax", 1, 9), h3_1);
        tv.add_to_parent(Course::new("Semantics", 2, 8), h3_1);
        tv.add_to_parent(Course::new("Literature", 2, 9), h3);
        let h3_3 = tv.add_to_parent(Course::new("Poetry", 3, 10), h3);
        let h4_3 = tv.add_to_parent(Course::new("Haiku", 1, 10), h3_3);
        tv.add_to_parent(Course::new("v1", 2, 9), h4_3);
        tv.add_to_parent(Course::new("v2", 3, 5), h4_3);

        tv.add_to_parent(Course::new("Sonnet", 2, 9), h3_3);
    
        let h2_3 = tv.add_to_parent(Course::new("AI", 1, 10), h2);
        tv.add_to_parent(Course::new("Neural Networks", 2, 8), h2_3);
        tv.add_to_parent(Course::new("Deep Learning", 3, 9), h2_3);
    
        tv.add(Course::new("Bob", 22, 12));        
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

#[test]
fn check_init() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state ')
        CheckHash(0x754C65468149AA5)  
        Key.Pressed(Ctrl+Right)
        Key.Pressed(Right,6)    
        Paint('2. Name column increase by 6 characters')
        CheckHash(0x4D937071C0592AFF)  
        Key.Pressed(Down,10)
        Paint('3. Cursors on: Literature')
        CheckHash(0xEBF7B023783A3585)  
    ";
    let mut a = App::debug(60, 25, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars|treeview::Flags::SearchBar);
    Course::populate_with_courses(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}