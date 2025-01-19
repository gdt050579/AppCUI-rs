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
    fn populate_with_courses(tv: &mut TreeView<Course>) {
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
    fn populate_with_courses_batch(tv: &mut TreeView<Course>) {
        tv.add_batch(|tv| {
            Course::populate_with_courses(tv);
        });
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
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_key_movement_left_right() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x461AC404AD7793D0) 
        Key.Pressed(Right,3) 
        Paint('2. Scroll to right (+3 position)')
        CheckHash(0x54D5076D51FBEB36) 
        Key.Pressed(Right,10)
        Paint('3. Scroll to right (+10 position)')
        CheckHash(0x90A1F3104F8A79EB) 
        Key.Pressed(Right,100)
        Paint('4. Scroll to max right')
        CheckHash(0x7A388F4BD11BAFA2) 
        Key.Pressed(Left,10)
        Paint('5. Scroll to left (-10 positions)')
        CheckHash(0x96B04C8EA9FDCB3B) 
        Key.Pressed(Left,100)
        Paint('6. Scroll to initial state')
        CheckHash(0x461AC404AD7793D0) 
    ";
    let mut a = App::debug(40, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_key_movement_up_down() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state ')
        CheckHash(0xB43934B962A84918) 
        Key.Pressed(Down,3) 
        Paint('2. Focus on Advance Calculus')
        CheckHash(0x84D046006246A1DB) 
        Key.Pressed(Down,2)
        Paint('3. Focus on Logic')
        CheckHash(0xAB83685BE65F2050) 
        Key.Pressed(Down)
        Paint('4. Focus on English')
        CheckHash(0x4C56B11AB66903F7) 
        Key.Pressed(Down,3)
        Paint('5. Focus on Semantic')
        CheckHash(0x85CDF875454B8881) 
        Key.Pressed(Down,300)
        Paint('6. Last element: Focus on Bob')
        CheckHash(0xC27C2E646E71DB79) 
        Key.Pressed(Up,2)
        Paint('7. Focus on Neural Network')
        CheckHash(0xE0578657B45848C1) 
        Key.Pressed(Up,4)
        Paint('8. Focus on v2, last view element is Bob')
        CheckHash(0x4ABAE32EA5027079) 
        Key.Pressed(Up,400)
        Paint('9.Back to initial state')
        CheckHash(0xB43934B962A84918) 
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_key_movement_pageup_pagedown() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state ')
        CheckHash(0xB43934B962A84918) 
        Key.Pressed(PageDown)
        Paint('2. Focus on Grammar')
        CheckHash(0x4677CEAFEDD4405D) 
        Key.Pressed(PageDown)
        Paint('3. Focus on v2')
        CheckHash(0x6A918816F85C4198) 
        Key.Pressed(PageDown)
        Paint('4. Focus on Bob')
        CheckHash(0xC27C2E646E71DB79) 
        Key.Pressed(PageDown)
        Paint('5. Focus remains on Bob')
        CheckHash(0xC27C2E646E71DB79) 
        Key.Pressed(PageUp)
        Paint('6. Focus on Haiku')
        CheckHash(0xBB62323FB39A99A7) 
        Key.Pressed(PageUp)
        Paint('7. Focus on Logic')
        CheckHash(0xB46529F8F791A898) 
        Key.Pressed(PageUp)
        Paint('8. Back to initial state')
        CheckHash(0xB43934B962A84918) 
        Key.Pressed(End)
        Paint('9. Focus on BOB (last state)')
        CheckHash(0xC27C2E646E71DB79) 
        Key.Pressed(Home)
        Paint('10. Back to initial state')
        CheckHash(0xB43934B962A84918) 
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_key_movement_scroll_up_down() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state ')
        CheckHash(0xB43934B962A84918) 
        Key.Pressed(Ctrl+Alt+Down)
        Paint('2. Scroll starts with Alice')
        CheckHash(0x369BEDE4869DDF36) 
        Key.Pressed(Ctrl+Alt+Down,5)
        Paint('3. Scroll starts with English')
        CheckHash(0x34D177D1FF0AD4CA) 
        Key.Pressed(Down)
        Paint('4. Scroll starts with Alice, focus on Alice')
        CheckHash(0xAA3869BE66A2218E) 
        Key.Pressed(Ctrl+Alt+Down,7)
        Paint('5. Scroll starts with Syntax')
        CheckHash(0x8D4E28A201B257DF) 
        Key.Pressed(Ctrl+Alt+Down,100)
        Paint('6. Scroll starts with v1, last visible item is Bob')
        CheckHash(0x8989B7D7CEA6D066) 
        Key.Pressed(Ctrl+Alt+Up,10)
        Paint('7. Scroll starts with Advance Calculus')
        CheckHash(0x3CBDDECEEEA2BEF6) 
        Key.Pressed(Ctrl+Alt+Up)
        Paint('8. Scroll starts with Math')
        CheckHash(0x6110677A6E4D7487) 
        Key.Pressed(Ctrl+Alt+Up,10)
        Paint('9. Scroll starts with John, Selected item is Alice')
        CheckHash(0xA3A7B2F7F66BA9AC) 
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_sort() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Down,3)
        Paint('1. Initial state (focus on Advance Calculus)')
        CheckHash(0xEB97DB5992DF3F85) 
        Key.Pressed(Ctrl+N)
        Paint('2. Sort by name (ascendent)')
        CheckHash(0x35EDBA55B7A7C9CB) 
        Key.Pressed(Ctrl+N)
        Paint('3. Sort by name (descendent)')
        CheckHash(0xE3C543AB79E3E38C) 
        Key.Pressed(Ctrl+R)
        Paint('4. Sort by relevance (ascendent)')
        CheckHash(0xE86218F3A4195C13) 
        Key.Pressed(Ctrl+R)
        Paint('5. Sort by relevance (descendent)')
        CheckHash(0xDEBE625515EF7278) 
    ";
    let mut a = App::debug(60, 25, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_sort_by_mouse() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Down,3)
        Paint('1. Initial state (focus on Advance Calculus)')
        CheckHash(0xEB97DB5992DF3F85) 
        Mouse.Click(10,1,left)
        Paint('2. Sort by name (ascendent)')
        CheckHash(0x58B4BE1D9E03E25F) 
        Mouse.Click(10,1,left)
        Paint('3. Sort by name (descendent)')
        CheckHash(0xC4F3642B25FBC9F8) 
        Mouse.Click(40,1,left)
        Paint('4. Sort by relevance (ascendent)')
        CheckHash(0x3E3BBCEC77D1EA72) 
        Mouse.Click(40,1,left)
        Paint('5. Sort by relevance (descendent)')
        CheckHash(0xCB769FD963C7F0DD) 
    ";
    let mut a = App::debug(60, 25, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses_batch(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}


#[test]
fn check_key_movement_up_down_without_header() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state ')
        CheckHash(0xDF48D35D7AEBD3B6) 
        Key.Pressed(Down,3) 
        Paint('2. Focus on Advance Calculus')
        CheckHash(0x3725D55464171ADD) 
        Key.Pressed(Down,2)
        Paint('3. Focus on Logic')
        CheckHash(0x7684D00A6F3535A6) 
        Key.Pressed(Down)
        Paint('4. Focus on English')
        CheckHash(0x79BF7AF04C1EEDC9) 
        Key.Pressed(Down,3)
        Paint('5. Focus on Semantic')
        CheckHash(0x238E1F8AA368BB82) 
        Key.Pressed(Down,300)
        Paint('6. Last element: Focus on Bob')
        CheckHash(0xE3FEA7288EC382B6) 
        Key.Pressed(Up,2)
        Paint('7. Focus on Neural Network')
        CheckHash(0x3BCC08D73E174E8E) 
        Key.Pressed(Up,4)
        Paint('8. Focus on v1, last view element is Bob')
        CheckHash(0x75A257EB67E95A0E) 
        Key.Pressed(Up,400)
        Paint('9.Back to initial state')
        CheckHash(0xDF48D35D7AEBD3B6) 
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::HideHeader);
    Course::populate_with_courses(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_key_mouse_does_not_work_on_columns_on_hide_header_flag() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state ')
        CheckHash(0xDF48D35D7AEBD3B6) 
        Key.Pressed(Down,3) 
        Paint('2. Focus on Advance Calculus')
        CheckHash(0x3725D55464171ADD) 
        Mouse.Move(5,1)
        Paint('3. Nothing changes')
        CheckHash(0x3725D55464171ADD) 
        Mouse.Click(5,1,left)
        Paint('4. John is selected (NO sort by name happens)')
        CheckHash(0xDF48D35D7AEBD3B6) 
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::HideHeader);
    Course::populate_with_courses(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}
