use flat_string::FlatString;

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
        crate::prelude::Column::new("", 50, crate::prelude::TextAlignament::Left)
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
    fn populate_with_icons(tv: &mut TreeView<Course>) {
        let h_math = tv.add_item(Item::new(Course::new("Math", 20, 10), false, None, ['M', 'M']));
        let h_geom = tv.add_item_to_parent(Item::new(Course::new("Geometry", 2, 5), false, None, ['G', 'G']), h_math);
        tv.add_item_to_parent(Item::new(Course::new("1-0-1", 4, 3), false, None, ['1', '1']), h_geom);
        tv.add_item_to_parent(Item::new(Course::new("2-0-2", 8, 2), false, None, ['2', '2']), h_geom);
        let h_calculus = tv.add_item_to_parent(Item::new(Course::new("Calculus", 4, 3), false, None, ['C', 'C']), h_math);
        tv.add_item_to_parent(Item::new(Course::new("Simple", 4, 3), false, None, ['S', 'S']), h_calculus);
        let h_adv = tv.add_item_to_parent(Item::new(Course::new("Advanced", 8, 2), false, None, ['A', 'A']), h_calculus);
        tv.add_item_to_parent(Item::new(Course::new("1-0-1", 4, 3), false, None, ['1', '1']), h_adv);
        tv.add_item_to_parent(Item::new(Course::new("2-0-2", 8, 2), false, None, ['2', '2']), h_adv);
        let h_logic = tv.add_item_to_parent(Item::new(Course::new("Logic", 4, 3), false, None, ['L', 'L']), h_math);
        tv.add_item_to_parent(Item::new(Course::new("Boolean", 8, 8), false, None, ['B', 'B']), h_logic);
        tv.add_item_to_parent(Item::new(Course::new("Prop", 8, 8), false, None, ['P', 'P']), h_logic);
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

#[test]
fn check_hover_over_fold_button() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Down,3)
        Paint('1. Initial state (focus on Advance Calculus)')
        CheckHash(0x7265C73AC95890D9) 
        Mouse.Move(2,3)
        Paint('2. Hover over Alice [-] button')
        CheckHash(0x21DB0D48AF84A8FF) 
        Mouse.Move(14,9)
        Paint('3. Hover over Grammer [-] button')
        CheckHash(0xDB9B92DC20777933) 
        Key.Pressed(Right,10)
        Paint('4. Change scroll X (nothing should be hovered)')
        CheckHash(0x351C7A95B324BD1C) 
        Key.Pressed(Ctrl+Alt+Down,4)
        Paint('5. Change scroll Y (nothing should be hovered)')
        CheckHash(0x7120D733E0AACA30) 
        Mouse.Move(16,10)
        Paint('6. Hover over Grammer [-] Haiku')
        CheckHash(0x5891B0DD550D20CA) 
        Mouse.Move(4,4)
        Paint('7. Hover over Grammer [-] English')
        CheckHash(0x75A6E85CF8407EAE) 
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses_batch(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_fold_button_with_mouse() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Down,3)
        Paint('1. Initial state (focus on Advance Calculus)')
        CheckHash(0x7265C73AC95890D9) 
        Mouse.Move(2,3)
        Paint('2. Hover over Alice [-] button')
        CheckHash(0x21DB0D48AF84A8FF) 
        Mouse.Click(2,3,left)
        Paint('3. Alice is collapsed [+]')
        CheckHash(0x30313469FD320C65) 
        Mouse.Click(2,3,left)
        Paint('4. Alice is expanded [-]')
        CheckHash(0x9B48A3C26FEC3EDA) 
        Mouse.Click(14,9,left)
        Paint('5. Grammar is collapse [+]')
        CheckHash(0x508A56E0D23C18F6) 
        Mouse.Click(2,3,left)
        Paint('6. Alice is collapsed [+]')
        CheckHash(0x30313469FD320C65) 
        Mouse.Click(2,3,left)
        Paint('7. Alice is expanded [-] (and grammer is collapsed)')
        CheckHash(0x48543562D4D9E092) 
        Key.Pressed(Right,15)
        Paint('8. Scroll right characters to the right')
        CheckHash(0x5EFDABDA00591910) 
        Mouse.Click(4,8,left)
        Paint('9. English is collapsed [+]')
        CheckHash(0x2BF6024818A131A7) 
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses_batch(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_fold_using_space() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Down,2)
        Paint('1. Initial state (focus on Math)')
        CheckHash(0x2D9B66CBE781DDFE) 
        Key.Pressed(Space)
        Paint('2. Math is collapsed [+]')
        CheckHash(0x4C84D0C161FAA53E) 
        Key.Pressed(Space)
        Paint('3. Math is expanded [+]')
        CheckHash(0x2D9B66CBE781DDFE) 
        Key.Pressed(Down,5)
        Key.Pressed(Space)
        Paint('4. Grammar is now collapsed [+]')
        CheckHash(0x250802DA587A60A1) 
        Key.Pressed(Up)
        Key.Pressed(Space)
        Paint('5. Englosh is now collapsed [+]')
        CheckHash(0x408F23B73C4E37C6) 
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses_batch(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_inactive() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(Down,3)
        Paint('1. Initial state (no focus because it is inactive)')
        CheckHash(0x674C02FC96FC3BC9) 
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::NoSelection);
    tv.set_enabled(false);
    Course::populate_with_courses_batch(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_filter() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xF4F13809AFE2629) 
        Key.TypeText('ai')
        Paint('2. filter based on <ai> text - cursor on Haiku')
        CheckHash(0x6D6BDBD3081248C1) 
        Key.Pressed(Enter)
        Paint('3. Move to next item with <ai> -> AI')
        CheckHash(0xE19B5FD646D31CCA) 
        Key.Pressed(Enter)
        Paint('4. Back on Haiku')
        CheckHash(0x6D6BDBD3081248C1) 
        Mouse.Click(10,13,left)
        Paint('5. Click outside the list (nothing happens)')
        CheckHash(0x6D6BDBD3081248C1) 
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses_batch(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_resize() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x754C65468149AA5) 
        Key.Pressed(Ctrl+Right)
        Paint('2. First resize column selected')
        CheckHash(0x15A3000A16F00EAF) 
        Key.Pressed(Left,10)
        Paint('3. Minimize first column by 10 characters')
        CheckHash(0x9DE0D61EB6DDDB47) 
        Key.Pressed(Ctrl+Right)
        Paint('4. Seconds resize column selected')
        CheckHash(0xB4DC3AF051686A8F) 
        Key.Pressed(Right,10)
        Paint('5. Increase seconds column by 10 characters')
        CheckHash(0x32AB02708F5CE014) 
        Mouse.DoubleClick(21,1,left)
        Paint('6. Autoresze column 1')
        CheckHash(0x8C7B09FCFA339E6E) 
        Key.Pressed(Escape)
        Key.Pressed(Right,100)
        Paint('7. Scroll to last column')
        CheckHash(0x4FC46C5F91982D49) 
        Mouse.DoubleClick(58,1,left)
        Paint('8. Autoresze column 3')
        CheckHash(0x9BF79D529343665A) 
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
fn check_mouse_wheel() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC0FE26FDDDC694DF) 
        Mouse.Wheel(20,5,down,1)
        Paint('2. Scroll starst with Alice')
        CheckHash(0xB3B90D215AD057EF) 
        Mouse.Wheel(20,5,right,10)
        Paint('3. Scroll on right')
        CheckHash(0xAE208272BB8A18D8) 
        Mouse.Wheel(20,5,down,4)
        Paint('4. Scroll starts with Logic')
        CheckHash(0x734C6A359873E7A9) 
        Mouse.Wheel(20,5,left,2)
        Paint('5. Scroll to left 5 positions')
        CheckHash(0x79B339256DF9AC29) 
        Mouse.Wheel(20,5,up,2)
        Paint('6. Scroll is not on Advance Calculus')
        CheckHash(0x5D9406DE9A1F664E) 
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SearchBar);
    Course::populate_with_courses_batch(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_small_icons() {
    let script = "
        Paint.Enable(false)
        Mouse.Drag(31,1,40,1)
        Paint('1. Initial state')
        CheckHash(0x463764FFB7A47293) 
        Mouse.Click(19,8,left)
        Paint('2. Advanced is folded [+]')
        CheckHash(0x4CDB8F4C685DFC7C) 
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::SmallIcons);
    Course::populate_with_icons(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_larger_icons() {
    let script = "
        Paint.Enable(false)
        Mouse.Drag(31,1,40,1)
        Paint('1. Initial state')
        CheckHash(0xED408410B85B1EE1) 
        Mouse.Click(21,8,left)
        Paint('2. Advanced is folded [+]')
        CheckHash(0x25A50B0D5E4C7061) 
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars | treeview::Flags::LargeIcons);
    Course::populate_with_icons(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_change_item_event() {
    #[Window(events = TreeViewEvents<Course>, internal: true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
            };
            let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::None);
            Course::populate_with_icons(&mut tv);
            w.add(tv);
            w
        }
    }
    impl TreeViewEvents<Course> for MyWin {
        fn on_current_item_changed(&mut self, handle: Handle<TreeView<Course>>, item_handle: Handle<treeview::Item<Course>>) -> EventProcessStatus {
            if let Some(tv) = self.control(handle) {
                if let Some(item) = tv.item(item_handle) {
                    let s: FlatString<32> = FlatString::from_str(item.name.as_str());
                    self.set_title(&s);
                }
            }
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Mouse.Drag(31,1,40,1)
        Paint('1. Initial state')
        CheckHash(0x9797BA2144E8815E) 
        Key.Pressed(Down)
        Paint('2. Focus on Geometry, windows title: Geometry ')
        CheckHash(0xD5A2BDB1F85A1FE2) 
        Key.Pressed(End)
        Paint('3. Focus on Prop, windows title: Prop ')
        CheckHash(0x988BC5D85727D801) 
        Mouse.Click(30,8,left)
        Paint('4. Focus on Advanced, windows title: Advanced ')
        CheckHash(0xEFD5EED5912A30EA) 
        Mouse.Click(30,8,left)
        Paint('5. Nothing changes')
        CheckHash(0xEFD5EED5912A30EA) 
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_change_item_event_change() {
    #[Window(events = TreeViewEvents<Course>, internal: true)]
    struct MyWin {
        counter: u32,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
                counter: 0,
            };
            let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::None);
            Course::populate_with_icons(&mut tv);
            w.add(tv);
            w
        }
    }
    impl TreeViewEvents<Course> for MyWin {
        fn on_current_item_changed(&mut self, handle: Handle<TreeView<Course>>, item_handle: Handle<treeview::Item<Course>>) -> EventProcessStatus {
            self.counter += 1;
            let c = self.counter;
            if let Some(tv) = self.control_mut(handle) {
                if let Some(item) = tv.item_mut(item_handle) {
                    item.name.push_str(format!("{},", c).as_str());
                }
            }
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Mouse.Drag(31,1,40,1)
        Paint('1. Initial state')
        CheckHash(0x9797BA2144E8815E) 
        Key.Pressed(Down)
        Paint('2. Focus on Geometry, new name: Geometry1,') ')
        CheckHash(0x65491C2002F10483) 
        Key.Pressed(End)
        Paint('3. Focus on Prop, new name: Prop2,')
        CheckHash(0xA6D85A0961EC73A1) 
        Mouse.Click(30,8,left)
        Paint('4. Focus on Advanced, new name: Advanced3,')
        CheckHash(0x7E4D5CAFD1D987CE) 
        Mouse.Click(30,8,left)
        Paint('5. Nothing changes')
        CheckHash(0x7E4D5CAFD1D987CE) 
        Key.Pressed(Up,5)
        Paint('6. Focus on Geometry, new name: Geometry1,8, and [Simple4,] [Calculus5,] [2-0-26,] [1-0-17,]')
        CheckHash(0xFB14BB51038A327A) 
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_on_item_action_event() {
    #[Window(events = TreeViewEvents<Course>, internal: true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
            };
            let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::None);
            Course::populate_with_icons(&mut tv);
            w.add(tv);
            w
        }
    }
    impl TreeViewEvents<Course> for MyWin {
        fn on_item_action(&mut self, handle: Handle<TreeView<Course>>, item_handle: Handle<treeview::Item<Course>>) -> EventProcessStatus {
            if let Some(tv) = self.control(handle) {
                if let Some(item) = tv.item(item_handle) {
                    let s: FlatString<32> = FlatString::from_str(item.name.as_str());
                    self.set_title(&s);
                }
            }
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Mouse.Drag(31,1,40,1)
        Paint('1. Initial state')
        CheckHash(0x9797BA2144E8815E) 
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('2. Focus on Geometry, windows title: Geometry ')
        CheckHash(0xD5A2BDB1F85A1FE2) 
        Key.Pressed(End)
        Paint('3. Focus on Prop, windows title: Geometry ')
        CheckHash(0xF62E72ECDCF31E36) 
        Key.Pressed(Enter)
        Paint('4. Focus on Prop, windows title: Prop ')
        CheckHash(0x988BC5D85727D801) 
        Mouse.DoubleClick(30,8,left)
        Paint('5. Focus on Advanced, windows title: Advanced ')
        CheckHash(0xEFD5EED5912A30EA) 
        Mouse.Click(30,6,left)
        Paint('6. Focus on Calculus, windows title: Advanced')
        CheckHash(0xBA98FC217CA536FE) 
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_on_item_colapse_expanded() {
    #[Window(events = TreeViewEvents<Course>, internal: true)]
    struct MyWin {}
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
            };
            let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::None);
            Course::populate_with_icons(&mut tv);
            w.add(tv);
            w
        }
    }
    impl TreeViewEvents<Course> for MyWin {
        fn on_item_collapsed(
            &mut self,
            handle: Handle<TreeView<Course>>,
            item_handle: Handle<treeview::Item<Course>>,
            _: bool,
        ) -> EventProcessStatus {
            if let Some(tv) = self.control(handle) {
                if let Some(item) = tv.item(item_handle) {
                    let s: FlatString<32> = FlatString::from_str(item.name.as_str());
                    self.set_title(format!("CLP: {}", &s).as_str());
                }
            }
            EventProcessStatus::Processed
        }

        fn on_item_expanded(&mut self, handle: Handle<TreeView<Course>>, item_handle: Handle<treeview::Item<Course>>, _: bool) -> EventProcessStatus {
            if let Some(tv) = self.control(handle) {
                if let Some(item) = tv.item(item_handle) {
                    let s: FlatString<32> = FlatString::from_str(item.name.as_str());
                    self.set_title(format!("EXP: {}", &s).as_str());
                }
            }
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Mouse.Drag(31,1,40,1)
        Paint('1. Initial state')
        CheckHash(0x9797BA2144E8815E) 
        Key.Pressed(Space)
        Paint('2. Math collapsed')
        CheckHash(0xED587153AFDDF179)         
        Key.Pressed(Space)
        Paint('3. Math Expanded')
        CheckHash(0xAE04F8FC3C9970B0)   
        Mouse.Click(7,6,left)      
        Paint('4. Collapse Calculus')
        CheckHash(0xB22991CB13229E05)   
        Mouse.Click(7,6,left)      
        Paint('5. Expands Calculus')
        CheckHash(0xFFF9FB0BF9B3C7F5)   
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_on_item_dynamic_colapse_expanded() {
    #[Window(events = TreeViewEvents<TestData>, internal: true)]
    struct MyWin {
        count: u32,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
                count: 0,
            };
            let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::None);
            tv.add_item(treeview::Item::expandable(TestData::new("Root"), false));
            w.add(tv);
            w
        }
    }
    impl TreeViewEvents<TestData> for MyWin {
        fn on_item_collapsed(
            &mut self,
            handle: Handle<TreeView<TestData>>,
            item_handle: Handle<treeview::Item<TestData>>,
            _: bool,
        ) -> EventProcessStatus {
            if let Some(tv) = self.control_mut(handle) {
                tv.delete_item_children(item_handle);
            }
            EventProcessStatus::Processed
        }

        fn on_item_expanded(
            &mut self,
            handle: Handle<TreeView<TestData>>,
            item_handle: Handle<treeview::Item<TestData>>,
            _: bool,
        ) -> EventProcessStatus {
            let mut c = self.count;
            self.count += 3;
            if let Some(tv) = self.control_mut(handle) {
                for _ in 0..3 {
                    c += 1;
                    let item = treeview::Item::expandable(TestData::new(format!("Item {}", c).as_str()), true);
                    tv.add_item_to_parent(item, item_handle);
                }
            }
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x1C291EE6B3316985)   
        Key.Pressed(Space,2)
        Paint('2. Root has [1,2,3]')
        CheckHash(0xB8572D4F123DE46E)   
        Key.Pressed(Down,2)
        Key.Pressed(Space)
        Paint('3. Item 2 has [4,5,6]')
        CheckHash(0x1132F01163CD9041)   
        Key.Pressed(Down,1)
        Key.Pressed(Space)
        Paint('4. Item 4 has [7,8,9]')
        CheckHash(0x6E11F38A6C816AFE)   
        Key.Pressed(Down,2)
        Key.Pressed(Space)
        Paint('5. Item 8 has [10,11,12]')
        CheckHash(0xE2CB37B127653FAF)   
        Mouse.Click(8,4,left)
        Paint('6. Item 2 collapsed')
        CheckHash(0xEA5B053EC01EC531)   
        Mouse.Click(8,4,left)
        Paint('7. Item 2 expanded [13,14,15]')
        CheckHash(0x174C3A2DE2587A19)   
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_delete_item() {
    #[Window(events = TreeViewEvents<Course>+CommandBarEvents, commands: Delete, internal: true)]
    struct MyWin {
        tv: Handle<TreeView<Course>>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,x:0,y:0,w:100%,h:19,flags: Sizeable"),
                tv: Handle::None,
            };
            let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::None);
            Course::populate_with_icons(&mut tv);
            w.tv = w.add(tv);
            w
        }
    }
    impl TreeViewEvents<Course> for MyWin {
        fn on_current_item_changed(&mut self, handle: Handle<TreeView<Course>>, item_handle: Handle<treeview::Item<Course>>) -> EventProcessStatus {
            if let Some(tv) = self.control(handle) {
                if let Some(item) = tv.item(item_handle) {
                    let s: FlatString<32> = FlatString::from_str(item.name.as_str());
                    self.set_title(format!("Current: {}", &s).as_str());
                }
            }
            EventProcessStatus::Processed
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("Delete"), "Delete", mywin::Commands::Delete);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            if command_id == mywin::Commands::Delete {
                let h = self.tv;
                if let Some(tv) = self.control_mut(h) {
                    if let Some(current) = tv.current_item() {
                        tv.delete_item(current);
                    }
                }
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x4C82513A69DE59BB)  
        Key.Pressed(Down,4)
        Paint('2. Now on Calculus (title is Calculus)')
        CheckHash(0x66D6BBEC6238BD47)  
        Key.Pressed(Delete)
        Paint('3. Calculus deleted, now on Logic (title is Logic)')
        CheckHash(0xD15CDF09517B60C2)  
        Key.Pressed(Delete)
        Paint('4. Logic deleted, now on 2-0-2 (title is 2-0-2)')
        CheckHash(0x5287C4F19F3FFD3C)  
        Key.Pressed(Up,2)
        Key.Pressed(Delete)
        Paint('5. Geometry deleted, now on Math (title is Math)')
        CheckHash(0xEA032D83E92961D4)  
        Key.Pressed(Delete)
        Paint('6. Math deleted, no more items (title remains Math)')
        CheckHash(0xBB7F61AA35475FAC)  
    ";
    let mut a = App::debug(60, 20, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_clear() {
    #[Window(events = CommandBarEvents, commands: Clear+Add, internal: true)]
    struct MyWin {
        tv: Handle<TreeView<Course>>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,x:0,y:0,w:100%,h:9,flags: Sizeable"),
                tv: Handle::None,
            };
            let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars);
            Course::populate_with_icons(&mut tv);
            w.tv = w.add(tv);
            w
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Clear", mywin::Commands::Clear);
            commandbar.set(key!("F2"), "Add", mywin::Commands::Add);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            let h = self.tv;
            match command_id {
                mywin::Commands::Clear => {
                    if let Some(tv) = self.control_mut(h) {
                        tv.clear();
                    }
                }
                mywin::Commands::Add => {
                    if let Some(tv) = self.control_mut(h) {
                        Course::populate_with_courses_batch(tv);
                    }
                }
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xF39ABDF241CD6DA1)  
        Key.Pressed(End)
        Paint('2. Now at end (focus on Prop)') 
        CheckHash(0xCFC9DC4133EA1DDB)
        Mouse.Click(10,1,left)
        Mouse.Click(10,1,left)
        Paint('3. Sort by name (descendent)') 
        CheckHash(0xE9CD4715B31F1AE5)
        Key.Pressed(F2)
        Paint('4. Add Math root one more time (scroll should change)') 
        CheckHash(0xDC67902C4A15F6E9)
        Mouse.Drag(59,2,59,4)
        Paint('5. Drag srollbar down') 
        CheckHash(0x7F6FFC68D93D3C69)
        Mouse.Click(2,3,left)
        Paint('6. Alice folded') 
        CheckHash(0xD2A1B4C7A63107E8)
        Key.Pressed(Home)
        Key.Pressed(Space)
        Paint('7. Math folded') 
        CheckHash(0x6E4E2276E101E5CE)
        Key.Pressed(End)
        Key.Pressed(Space)
        Paint('8. Alice expanded') 
        CheckHash(0x908E90024D7E599B)
        Key.Pressed(F1)
        Paint('9. All items cleared (scrollbars should be disabled)') 
        CheckHash(0xC91809CA90AC8ADF)
        Key.Pressed(F2)
        Paint('10. Repopulate (scroll bars should be enabled)') 
        CheckHash(0xB3B4C273E84B43EA)
        Mouse.Drag(59,2,59,6)
        Paint('11. Scroll to the end') 
        CheckHash(0x520D93E7092CDBE9)
    ";
    let mut a = App::debug(60, 10, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_collapse_expand_via_methods() {
    #[Window(events = CommandBarEvents, commands: Collapse+Expand, internal: true)]
    struct MyWin {
        tv: Handle<TreeView<Course>>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,x:0,y:0,w:100%,h:9,flags: Sizeable"),
                tv: Handle::None,
            };
            let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars);
            Course::populate_with_courses(&mut tv);
            w.tv = w.add(tv);
            w
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Expand", mywin::Commands::Expand);
            commandbar.set(key!("F2"), "Collapse", mywin::Commands::Collapse);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            let h = self.tv;
            match command_id {
                mywin::Commands::Expand => {
                    if let Some(tv) = self.control_mut(h) {
                        if let Some(ci) = tv.current_item() {
                            tv.expand_item(ci);
                        }
                    }
                }
                mywin::Commands::Collapse => {
                    if let Some(tv) = self.control_mut(h) {
                        if let Some(ci) = tv.current_item() {
                            tv.collapse_item(ci);
                        }
                    }
                }
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x3C07A73059F403C8)  
        Key.Pressed(Down)
        Key.Pressed(F2)
        Paint('2. Alice is collapsed')
        CheckHash(0x886AA1E9B0BB0A4C)  
        Key.Pressed(F2)
        Paint('3. Nothing changes')
        CheckHash(0x886AA1E9B0BB0A4C)  
        Key.Pressed(F1)
        Paint('4. Alice is expanded')
        CheckHash(0xD8BC7870BC98F71C)  
    ";
    let mut a = App::debug(60, 10, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_expand_collapese_recursively() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA413FE97269933E0) 
        Key.Pressed(Ctrl+Space)
        Paint('2. Math collapsed recusivelly')
        CheckHash(0xAB4CC479A2C48A4F) 
        Key.Pressed(Space)
        Paint('3. Math expanded (Geometry, Calculus and Logic are collapsed)')
        CheckHash(0x262090A713F7BCFD) 
        Key.Pressed(Down,2)
        Paint('4. Focus on calculus')
        CheckHash(0xF892BEA47C253AD) 
        Key.Pressed(Ctrl+Space)
        Paint('5. Expand recursively Calculus')
        CheckHash(0x4DF25F1D06E1F137) 
        Key.Modifier(Ctrl)
        Mouse.Click(2,2,left)
        Key.Modifier(None)
        Paint('6. Math recusivelly collapse via mouse')
        CheckHash(0xFC357E031BEC5A60) 
        Key.Pressed(Space)
        Paint('7. Math expanded (Geometry, Calculus and Logic are collapsed)')
        CheckHash(0x98F8001FACD5814A) 
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::ScrollBars);
    Course::populate_with_icons(&mut tv);
    w.add(tv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_on_item_dynamic_colapse_expanded_recursively() {
    #[Window(events = TreeViewEvents<TestData>, internal: true)]
    struct MyWin {
        count: u32,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = MyWin {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
                count: 0,
            };
            let mut tv = TreeView::new(Layout::new("d:c"), treeview::Flags::None);
            tv.add_item(treeview::Item::expandable(TestData::new("Root"), false));
            w.add(tv);
            w
        }
    }
    impl TreeViewEvents<TestData> for MyWin {
        fn on_item_collapsed(
            &mut self,
            handle: Handle<TreeView<TestData>>,
            item_handle: Handle<treeview::Item<TestData>>,
            recursive: bool,
        ) -> EventProcessStatus {
            if recursive {
                self.set_title("Collapsed recursively");
            } else {
                if let Some(tv) = self.control_mut(handle) {
                    tv.delete_item_children(item_handle);
                }
            }
            EventProcessStatus::Processed
        }

        fn on_item_expanded(
            &mut self,
            handle: Handle<TreeView<TestData>>,
            item_handle: Handle<treeview::Item<TestData>>,
            recursive: bool,
        ) -> EventProcessStatus {
            let mut c = self.count;
            self.count += 3;
            if recursive {
                self.set_title("Expanded recursively");
            } else {
                if let Some(tv) = self.control_mut(handle) {
                    for _ in 0..3 {
                        c += 1;
                        let item = treeview::Item::expandable(TestData::new(format!("Item {}", c).as_str()), true);
                        tv.add_item_to_parent(item, item_handle);
                    }
                }
            }
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x1C291EE6B3316985)   
        Key.Pressed(Space,2)
        Paint('2. Root has [1,2,3]')
        CheckHash(0xB8572D4F123DE46E)   
        Key.Pressed(Down,2)
        Key.Pressed(Space)
        Paint('3. Item 2 has [4,5,6]')
        CheckHash(0x1132F01163CD9041)   
        Key.Pressed(Down,1)
        Key.Pressed(Space)
        Paint('4. Item 4 has [7,8,9]')
        CheckHash(0x6E11F38A6C816AFE)   
        Key.Pressed(Down,2)
        Key.Pressed(Space)
        Paint('5. Item 8 has [10,11,12]')
        CheckHash(0xE2CB37B127653FAF)   
        Mouse.Click(8,4,left)
        Paint('6. Item 2 collapsed')
        CheckHash(0xEA5B053EC01EC531)   
        Mouse.Click(8,4,left)
        Paint('7. Item 2 expanded [13,14,15]')
        CheckHash(0x174C3A2DE2587A19)   
        Key.Pressed(Home)
        Key.Pressed(Ctrl+Space)
        Paint('8. Collapse all items recursively')
        CheckHash(0x380ED262CEBBE322)   
        Key.Pressed(Ctrl+Space)
        Mouse.Move(0,0)
        Paint('9. Expand all items recursively')
        CheckHash(0xA025143D3B47AB1A)   
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

