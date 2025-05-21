use crate::prelude::*;
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use listview::Group;

struct TestItem {}
impl listview::ListItem for TestItem {
    fn render_method(&self, _column_index: u16) -> Option<listview::RenderMethod> {
        Some(listview::RenderMethod::Text("abc"))
    }
    fn compare(&self, _other: &Self, _column_index: u16) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

struct Person {
    name: &'static str,
    age: &'static str,
    city: &'static str,
}
impl Person {
    fn new(name: &'static str, age: &'static str, city: &'static str) -> Self {
        Self { name, age, city }
    }
    fn populate(l: &mut ListView<Person>) {
        let g1 = l.add_group("USA");
        let g2 = l.add_group("Europe");
        let g3 = l.add_group("Asia");
        let g4 = l.add_group("Romania");
        l.add_to_group(
            Vec::from([
                Person::new("John", "25", "New York"),
                Person::new("Mike", "70", "Washington"),
                Person::new("Todd", "85", "San Francisco"),
            ]),
            g1,
        );
        l.add_to_group(
            Vec::from([
                Person::new("Sancez", "30", "Madrid"),
                Person::new("Etiene", "65", "Paris"),
                Person::new("Karl", "45", "Berlin"),
                Person::new("Jonas", "22", "Norway"),
            ]),
            g2,
        );
        l.add_to_group(
            Vec::from([
                Person::new("Yu Law", "33", "Tokyo"),
                Person::new("Kai", "45", "Beijing"),
                Person::new("Chen", "55", "Shanghai"),
                Person::new("Chan Li", "55", "Vietnam"),
                Person::new("Chen Li", "55", "Vietnam"),
            ]),
            g3,
        );
        l.add_to_group(
            Vec::from([
                Person::new("Andrei", "20", "Iasi"),
                Person::new("Mihai", "35", "Brasov"),
                Person::new("Vlad", "40", "Cluj"),
                Person::new("Ion", "45", "Bucharest"),
                Person::new("Gheorghe", "50", "Timisoara"),
                Person::new("Marin", "55", "Bucharest"),
                Person::new("Teodor", "60", "Arad"),
            ]),
            g4,
        );
    }
    fn populate_for_sort(l: &mut ListView<Person>) {
        l.add(Person::new("Dragos", "90", "Zanzibar"));
        l.add(Person::new("Zig", "70", "Albania"));
        l.add(Person::new("Bruce", "40", "Bucharest"));
        l.add(Person::new("Conrad", "80", "Dresden"));
        l.add(Person::new("Peter", "20", "Estonia"));
        l.add(Person::new("Tom", "30", "Iasi"));
        l.add(Person::new("John", "50", "Madrid"));
        l.add(Person::new("Alex", "10", "Cairo"));
    }
    fn populate_for_sort2(l: &mut ListView<Person>) {
        l.add(Person::new("Dragos", "90", "Zanzibar"));
        l.add(Person::new("Zig", "70", "Albania"));
        l.add(Person::new("Bruce", "40", "Bucharest"));
        l.add(Person::new("Conrad", "80", "Dresden"));
        l.add(Person::new("Peter", "20", "Estonia"));
        l.add(Person::new("Tom", "30", "Iasi"));
        l.add(Person::new("John", "50", "Madrid"));
        l.add(Person::new("Alex", "10", "Cairo"));
        l.add(Person::new("George", "5", "Brasov"));
        l.add(Person::new("Mihai", "35", "Brasov"));
        l.add(Person::new("Vlad", "40", "Cluj"));
        l.add(Person::new("Ion", "45", "Bucharest"));
        l.add(Person::new("Gheorghe", "50", "Timisoara"));
        l.add(Person::new("Marin", "55", "Bucharest"));
        l.add(Person::new("Teodor", "60", "Arad"));
        l.add(Person::new("Andrei", "20", "Iasi"));
        l.add(Person::new("Mircea", "35", "Brasov"));
        l.add(Person::new("Vasile", "40", "Cluj"));
        l.add(Person::new("Ilie", "45", "Bucharest"));
        l.add(Person::new("Gigi", "50", "Timisoara"));
        l.add(Person::new("Marian", "55", "Bucharest"));
        l.add(Person::new("Tudor", "60", "Arad"));
        l.add(Person::new("Marius", "20", "Iasi"));
        l.add(Person::new("Mihaela", "35", "Brasov"));
        l.add(Person::new("Viorica", "40", "Cluj"));
        l.add(Person::new("Ileana", "45", "Bucharest"));
        l.add(Person::new("Geta", "50", "Timisoara"));
    }
    fn populate_with_icon(l: &mut ListView<Person>) {
        let g1 = l.add_group("Group 1");
        l.add_item(listview::Item::new(
            Person {
                name: "Popescu",
                age: "20",
                city: "Barcelona",
            },
            false,
            None,
            ['*', '*'],
            g1,
        ));
        l.add_item(listview::Item::new(
            Person {
                name: "Marin",
                age: "30",
                city: "Iasi",
            },
            false,
            None,
            ['@', '@'],
            g1,
        ));
    }
}
impl listview::ListItem for Person {
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::Text(self.age)),
            2 => Some(listview::RenderMethod::Text(self.city)),
            _ => None,
        }
    }
    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.age.cmp(other.age),
            2 => self.city.cmp(other.city),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

struct ColorInfo {
    name: &'static str,
    value: u32,
    b: bool,
}
impl ColorInfo {
    fn populate(lv: &mut ListView<ColorInfo>) {
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "Red",
                value: 0xFF0000,
                b: true,
            },
            true,
            Some(CharAttribute::with_fore_color(Color::Red)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "Green",
                value: 0xFF00,
                b: true,
            },
            true,
            Some(CharAttribute::with_fore_color(Color::Green)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "Blue",
                value: 0xFF,
                b: true,
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Blue)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "White",
                value: 0xFFFFFF,
                b: false,
            },
            true,
            Some(CharAttribute::with_fore_color(Color::White)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "Yellow",
                value: 0xFFFF00,
                b: false,
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Yellow)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "Dark Red",
                value: 0x800000,
                b: false,
            },
            true,
            Some(CharAttribute::with_fore_color(Color::DarkRed)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "Dark Blue on Yellow",
                value: 0x80,
                b: true,
            },
            true,
            Some(CharAttribute::with_color(Color::DarkBlue, Color::Yellow)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "Dark Green",
                value: 0x8000,
                b: false,
            },
            true,
            Some(CharAttribute::with_fore_color(Color::DarkGreen)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "Black",
                value: 0x00,
                b: false,
            },
            true,
            Some(CharAttribute::with_fore_color(Color::Black)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "Aqua",
                value: 0x8080,
                b: false,
            },
            true,
            Some(CharAttribute::with_fore_color(Color::Aqua)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "Silver",
                value: 0xC0C0C0,
                b: true,
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Silver)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
        lv.add_item(listview::Item::new(
            ColorInfo {
                name: "Gray",
                value: 0x808080,
                b: true,
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Gray)),
            [0u8 as char, 0u8 as char],
            Group::None,
        ));
    }
}
impl listview::ListItem for ColorInfo {
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            0 => Some(listview::RenderMethod::Ascii(self.name)),
            1 => Some(listview::RenderMethod::UInt64(self.value as u64, listview::NumericFormat::Hex32)),
            2 => Some(listview::RenderMethod::Bool(self.b, listview::BoolFormat::CheckmarkMinus)),
            _ => None,
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.value.cmp(&other.value),
            2 => self.b.cmp(&other.b),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

#[test]
fn check_create() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x668442408FDE500C)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_navigate_with_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x668442408FDE500C)
        Key.Pressed(Ctrl+Right)
        Paint('2. Column-1 selected')
        CheckHash(0x644101CC8184BE50)
        Key.Pressed(Ctrl+Right)
        Paint('3.cColumn-2 selected')
        CheckHash(0x9C1B04EFDBB90110)
        Key.Pressed(Ctrl+Right)
        Paint('4. Column-3 selected')
        CheckHash(0xB5979A0999E0E4B8)
        Key.Pressed(Ctrl+Right)
        Paint('5. Column-3 remains selected')
        CheckHash(0xB5979A0999E0E4B8)
        Key.Pressed(Ctrl+Left)
        Paint('6. Column-2 selected')
        CheckHash(0x9C1B04EFDBB90110)
        Key.Pressed(Ctrl+Left)
        Paint('7. Column-1 selected')
        CheckHash(0x644101CC8184BE50)
        Key.Pressed(Ctrl+Left)
        Paint('8. Column-1 re-selected')
        CheckHash(0x644101CC8184BE50)
        Key.Pressed(Escape)
        Paint('9. Initial state')
        CheckHash(0x668442408FDE500C)
        Key.Pressed(Escape)
        Paint('10. Window is closed')
        CheckHash(0x3900AF2CBDF4157D)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_sort_with_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x74843BBEF41FE568)
        Key.Pressed(Ctrl+N)
        Paint('2. Sort by Name, ascending')
        CheckHash(0x13077B05CC85E1FB)
        Key.Pressed(Ctrl+N)
        Paint('3. Sort by Name, descending')
        CheckHash(0x9D6ABDB48B9E12D1)
        Key.Pressed(Ctrl+N)
        Paint('4. Sort by Name, ascending (again)')
        CheckHash(0x13077B05CC85E1FB)
        Key.Pressed(Ctrl+A)
        Paint('5. Sort by Age, ascending')
        CheckHash(0xC6A1C5264E888EF)
        Key.Pressed(Ctrl+A)
        Paint('6. Sort by Age, descending')
        CheckHash(0xDC85EFD1D900459D)
        Key.Pressed(Ctrl+C)
        Paint('7. Sort by City, ascending, scroll moved to view City column')
        CheckHash(0x5B4578ED022A0523)
        Key.Pressed(Ctrl+C)
        Paint('8. Sort by City, descending, scroll moved to view City column')
        CheckHash(0x65980AC71BB1FCCD)
        Key.Pressed(Ctrl+N)
        Paint('9. Sort by Name, ascending, scroll moved to view Name column')
        CheckHash(0x13077B05CC85E1FB)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:30,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{&Name,15,Left},{&Age,10,Right},{&City,10,Center}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_resize_with_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x668442408FDE500C)
        Key.Pressed(Ctrl+Right)
        Paint('2. Column-1 selected')
        CheckHash(0x644101CC8184BE50)
        Key.Pressed(Right)
        Paint('3. Column-1 size increased by 1')
        CheckHash(0xC8CF6D36FDDFF890)
        Key.Pressed(Left)
        Paint('4. Column-1 size decreased by 1')
        CheckHash(0x644101CC8184BE50)
        Key.Pressed(Right,14)
        Paint('5. Column-1 size increased by 14')
        CheckHash(0xD53CDF5512B92450)
        Key.Pressed(Right,1)
        Paint('6. Column-1 size increased by 1, horzontal scroll not active')
        CheckHash(0x225D535A77E87490)
        Key.Pressed(Right,1)
        Paint('7. Column-1 size increased by 1, horzontal scroll is ACTIVE')
        CheckHash(0x2BEF9B5B61807A46)
        Mouse.Click(52,9,left)
        Paint('8. Scroll to right, last column is fully visible,Header starts with Name')
        CheckHash(0x2D4C94CE23C61191)
        Mouse.Click(52,9,left)
        Paint('9. Identical to the previous one')
        CheckHash(0x2D4C94CE23C61191)
        Mouse.Click(52,9,left)
        Paint('10. Identical to the previous one')
        CheckHash(0x2D4C94CE23C61191)
        Mouse.Click(6,9,left)
        Paint('11. Scrool to first element, horzontal scroll is ACTIVE')
        CheckHash(0x60E537F7DC6646FD)
        Mouse.Click(6,9,left)
        Paint('12. Scrool to first element, horzontal scroll is ACTIVE (identical to step 11')
        CheckHash(0x60E537F7DC6646FD)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_ensure_visible_when_changing_columns() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA6F8DC05418FA9A5)
        Key.Pressed(Ctrl+Right)
        Paint('2. C1 selected')
        CheckHash(0xA854AE3EEBECBE79)
        Key.Pressed(Ctrl+Right)
        Paint('3. C2 selected')
        CheckHash(0x1B2BF1D59A37B3C1)
        Key.Pressed(Ctrl+Right)
        Paint('4. C3 selected (header starts with C1-10, C3 is fully visible at the end)')
        CheckHash(0x7A19C8ECD3975FC9)
        Key.Pressed(Ctrl+Right)
        Paint('5. C4 selected (C4 is visible at the end of the header)')
        CheckHash(0x12531D79D88BFB31)
        Key.Pressed(Ctrl+Right)
        Paint('6. C5 selected (C5 is visible at the end of the header)')
        CheckHash(0x2610F5BA8B809EAB)
        Key.Pressed(Ctrl+Right)
        Paint('7. C5 remains selected (same as 6)')
        CheckHash(0x2610F5BA8B809EAB)
        Key.Pressed(Ctrl+Left)
        Paint('8. C4 selected (view is not changed)')
        CheckHash(0xBFEE2A312CD05D53)
        Key.Pressed(Ctrl+Left)
        Paint('9. C3 selected (from the start of the header)')
        CheckHash(0xF0A53B98F0DD9917)
        Key.Pressed(Ctrl+Left)
        Paint('10. C2 selected (from the start of the header)')
        CheckHash(0xEC65923D88D014E2)
        Key.Pressed(Ctrl+Left)
        Paint('11. C1 selected (from the start of the header)')
        CheckHash(0xA854AE3EEBECBE79)
        Key.Pressed(Ctrl+Left)
        Paint('12. C1 remains selected (view is not changed)')
        CheckHash(0xA854AE3EEBECBE79)
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{C1-10,10},{C2-12,12},{C3-14,14},{C4-16,16},{C5-10,10}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_resize_outside_visible() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA6F8DC05418FA9A5)
        Key.Pressed(Ctrl+Right,3)
        Paint('2. C3 selected')
        CheckHash(0x7A19C8ECD3975FC9)
        Key.Pressed(Right);
        Paint('3. C3 increased by 1')
        CheckHash(0x1205B31BB5A2E8E6)
        Key.Pressed(Right,4);
        Paint('4. C3 increased by 4, C2 line under T from Test')
        CheckHash(0xBE314AF843DA7B17)
        Key.Pressed(Right,18);
        Paint('5. C3 increased by 18, C3 has the entire width of the window')
        CheckHash(0x88EB41332D598118)
        Key.Pressed(Right);
        Paint('6. C3 increased by 1, Header starts with C3..., right line visible')
        CheckHash(0xFE373AA6CB9FCB9C)
        Key.Pressed(Right);
        Paint('7. C3 increased by 1, Header starts with 3..., right line visible')
        CheckHash(0xC99586142C1EE75B)
        Key.Pressed(Left,2);
        Paint('8. C3 decreased by 2 - should take the entire width of the window')
        CheckHash(0x88EB41332D598118)
        Key.Pressed(Left,20);
        Paint('9. C3 decreased by 20, Line under letter T from Test')
        CheckHash(0x886948F2CFA28E07)
        Key.Pressed(Ctrl+Left);
        Paint('10. C2 is visible')
        CheckHash(0x1D6B48ED61CF0D6A)
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{C1-10,10},{C2-12,12},{C3-14,14},{C4-16,16},{C5-10,10}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_move_scroll_when_enter_column_resize_mode() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA6F8DC05418FA9A5)
        Key.Pressed(Ctrl+Right,3)
        Paint('2. C3 selected')
        CheckHash(0x7A19C8ECD3975FC9)
        Key.Pressed(Right,10);
        Paint('3. C3 increased by 10')
        CheckHash(0x5836CFFB8679E7A7)
        Key.Pressed(Escape)
        Paint('4. No column selected')
        CheckHash(0x4663870AFB3F133B)
        Key.Pressed(Ctrl+Right)
        Paint('5. Fist column selected and scroll moved')
        CheckHash(0xA854AE3EEBECBE79)
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{C1-10,10},{C2-12,12},{C3-14,14},{C4-16,16},{C5-10,10}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_scroll_update_when_control_is_resized() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (h-scroll inactive)')
        CheckHash(0x9CC1A20A64CF25BC)
        Mouse.Drag(49,9,40,9)
        Paint('2. Resize window (h-scroll active)')
        CheckHash(0x96AD3061803219B9)   
        Key.Pressed(Right,100)     
        Paint('3. Move h-scroll to end (starts with | C2...)')
        CheckHash(0x1C85B6AB50E32DB9) 
        Mouse.Drag(40,9,30,9)  
        Paint('4. Move h-scroll to middle but still (starts with | C2...)')
        CheckHash(0x53F55F8778EDFF3F) 
        Mouse.Drag(30,9,60,9)  
        Paint('5. Scroll bar is not inactive (starts with C1 ...)')
        CheckHash(0x6CA64A075D529384) 
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!("TestItem,d:c,flags: ScrollBars,columns=[{C1,6},{C2,6},{C3,6},{C4,6},{C5,6}]"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_click() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (h-scroll inactive)')
        CheckHash(0xD91C4D1725F901B4)
        Mouse.Move(20,2)
        Paint('2. C1-hover')
        CheckHash(0xBCF1AC194596E088)   
        Mouse.Move(26,2)
        Paint('3. C2-hover')
        CheckHash(0x8B967934E497DCF4)   
        Mouse.Move(40,2)
        Paint('4. C3-hover')
        CheckHash(0xF97A147330A43988)   
        Mouse.Click(40,2,left)
        Paint('5. C3-hover, clicked, sorted up')
        CheckHash(0xB1A45A7496098B32)   
        Mouse.Click(40,2,left)
        Paint('6. C3-hover, clicked, sorted down')
        CheckHash(0x93AB741282C03DC4)   
        Mouse.Move(44,2)
        Paint('7. C4-hover, C3-selected and sorted down')
        CheckHash(0xC0D6BDF3C435DEA6)   
        Mouse.Click(44,2,left)
        Paint('8. C4-hover, clicked, sorted down, scroll move to see C3 entirely at the end')
        CheckHash(0x33D2C0CF30EAEE5C)   
        Mouse.Move(0,0)
        Paint('9. C4-clicked, sorted down, scroll move to see C3 entirely at the end')
        CheckHash(0x70C7CCCB3A1B805D)   
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{C1,10,L},{C2,10,C},{C3,10,R},{C4,10},{C5,10}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_column_left_right_scroll() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xD91C4D1725F901B4)
        Key.Pressed(Right)
        Paint('2. Scroll to right')
        CheckHash(0xF2231DF83A92FFDC)   
        Key.Pressed(Right,2)
        Paint('3. Scroll to right (C1 is not visible)')
        CheckHash(0x3073E33BB34F3EDE)   
        Key.Pressed(Right,12)
        Paint('4. Scroll to right (C2 is first on header)')
        CheckHash(0x903858619D4CE328)   
        Key.Pressed(Right,2)
        Paint('5. Scroll to right most - C5 is fully visible')
        CheckHash(0xA1F242FCA8DDE105)   
        Key.Pressed(Right)
        Paint('6. Scroll to right most - nothing changes')
        CheckHash(0xA1F242FCA8DDE105)   
        Key.Pressed(Left)
        Paint('7. Scroll to left - C5 bar is no longer visible')
        CheckHash(0xDD6D52AB8FFB4FCF)   
        Key.Pressed(Left,10)
        Paint('8. Scroll to left - 10 pos')
        CheckHash(0x5F47E2CF754E559A)   
        Key.Pressed(Left,10)
        Paint('9. Scroll to left - back to the initial state')
        CheckHash(0xD91C4D1725F901B4)   
   ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    w.add(listview!(
        "TestItem,d:c,flags: ScrollBars,columns=[{C1,10,L},{C2,10,C},{C3,10,R},{C4,10},{C5,10}]"
    ));
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigate_keys_mode_details() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from USA)')
        CheckHash(0x26CAF8B68785A02E)
        Key.Pressed(Down)
        Paint('2. Cursor on John')
        CheckHash(0x301A8308DA139160)
        Key.Pressed(Down,3)
        Paint('3. Cursor on Europe')
        CheckHash(0xD6C610938F9C72A)
        Key.Pressed(Down)
        Paint('4. Cursor on Sancez')
        CheckHash(0xE1A70659B8D510E4)
        Key.Pressed(Down)
        Paint('5. Cursor on Etiene (scroll starts from John)')
        CheckHash(0x8D44731FF0A10E46)
        Key.Pressed(PageDown)
        Paint('6. Cursor on Chen (scroll starts from Karl)')
        CheckHash(0x8FE3F71236F08FBF)
        Key.Pressed(Up,3)
        Paint('7. Cursor on Asia (scroll starts from Karl)')
        CheckHash(0x2FC0D55F820F6B29)
        Key.Pressed(PageUp)
        Paint('8. Cursor on Todd (scroll starts from Todd)')
        CheckHash(0x18BEF8422643688E)
        Key.Pressed(PageDown,2)
        Paint('9. Cursor on Romania (scroll starts from Yu Law)')
        CheckHash(0xF2A08AF4AC654EF0)
        Key.Pressed(PageDown,1)
        Paint('10. Cursor on Marin (scroll starts from Andrei)')
        CheckHash(0xA24264FFCA9BC64F)
        Key.Pressed(Down)
        Paint('11. Cursor on Teodor (scroll starts from Mihai)')
        CheckHash(0xF1971EDE81CFFDA9)
        Key.Pressed(Down,2)
        Paint('12. Cursor remains on Teodor (scroll starts from Mihai)')
        CheckHash(0xF1971EDE81CFFDA9)
        Key.Pressed(Home)
        Paint('13. Back to initial state (scroll starts from USA)')
        CheckHash(0x26CAF8B68785A02E)
        Key.Pressed(End)
        Paint('14. Cursor on Teodor (scroll starts from Mihai)')
        CheckHash(0xF1971EDE81CFFDA9)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags: ScrollBars+CheckBoxes+ShowGroups,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigate_keys_mode_details_no_checkboxes() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from USA)')
        CheckHash(0x44D6237F035CB4)
        Key.Pressed(Down)
        Paint('2. Cursor on John')
        CheckHash(0x4FF67D3ED33528DC)
        Key.Pressed(Down,3)
        Paint('3. Cursor on Europe')
        CheckHash(0xD15C9F9836A5F800)
        Key.Pressed(Down)
        Paint('4. Cursor on Sancez')
        CheckHash(0x4C68E6453D9DC020)
        Key.Pressed(Down)
        Paint('5. Cursor on Etiene (scroll starts from John)')
        CheckHash(0xC10EE32199F7F18D)
        Key.Pressed(PageDown)
        Paint('6. Cursor on Chen (scroll starts from Karl)')
        CheckHash(0x4D388B4A5B13AF5C)
        Key.Pressed(Up,3)
        Paint('7. Cursor on Asia (scroll starts from Karl)')
        CheckHash(0xA5CC3CFE29AAA254)
        Key.Pressed(PageUp)
        Paint('8. Cursor on Todd (scroll starts from Todd)')
        CheckHash(0x1DE1FDA6045B5C81)
        Key.Pressed(PageDown,2)
        Paint('9. Cursor on Romania (scroll starts from Yu Law)')
        CheckHash(0xE5BC67F9B74C0971)
        Key.Pressed(PageDown,1)
        Paint('10. Cursor on Marin (scroll starts from Andrei)')
        CheckHash(0x21A49DA95E68A6CA)
        Key.Pressed(Down)
        Paint('11. Cursor on Teodor (scroll starts from Mihai)')
        CheckHash(0xBE2A1EC6ECF34E3C)
        Key.Pressed(Down,2)
        Paint('12. Cursor remains on Teodor (scroll starts from Mihai)')
        CheckHash(0xBE2A1EC6ECF34E3C)
        Key.Pressed(Home)
        Paint('13. Back to initial state (scroll starts from USA)')
        CheckHash(0x44D6237F035CB4)
        Key.Pressed(End)
        Paint('14. Cursor on Teodor (scroll starts from Mihai)')
        CheckHash(0xBE2A1EC6ECF34E3C)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags: ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigate_keys_mode_details_no_checkboxes_no_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from John)')
        CheckHash(0xF73F60131F7F0467)
        Key.Pressed(Down)
        Paint('2. Cursor on Mike')
        CheckHash(0xBF4735812C9CD2DB)
        Key.Pressed(Down,3)
        Paint('3. Cursor on Etiene')
        CheckHash(0x2A2068FA8E607478)
        Key.Pressed(Down)
        Paint('4. Cursor on Karl')
        CheckHash(0x26B57503482F9B93)
        Key.Pressed(Down)
        Paint('5. Cursor on Jonas (scroll starts from Mike)')
        CheckHash(0x68340B9ED77B19D1)
        Key.Pressed(PageDown)
        Paint('6. Cursor on Andrei (scroll starts from Yu Law)')
        CheckHash(0x2B11F3A179CD4F81)
        Key.Pressed(Up,3)
        Paint('7. Cursor on Chen (scroll starts from Yu Law)')
        CheckHash(0xAC5B9F570F67E521)
        Key.Pressed(PageUp)
        Paint('8. Cursor on Sances (scroll starts from Sancez)')
        CheckHash(0x1D734C68954DFB8D)
        Key.Pressed(PageDown,2)
        Paint('9. Cursor on Ion (scroll starts from Chan Li)')
        CheckHash(0xEF826F07A56D06EA)
        Key.Pressed(PageDown,1)
        Paint('10. Cursor on Teodor (scroll starts from Mihai)')
        CheckHash(0x2AE34EBB0F458B7C)
        Key.Pressed(Down,2)
        Paint('11. Cursor remains on Teodor (scroll starts from Mihai)')
        CheckHash(0x2AE34EBB0F458B7C)
        Key.Pressed(Home)
        Paint('12. Back to initial state (scroll starts from USA)')
        CheckHash(0xF73F60131F7F0467)
        Key.Pressed(Up)
        Paint('13. Remains on initial state (scroll starts from USA)')
        CheckHash(0xF73F60131F7F0467)
        Key.Pressed(End)
        Paint('14. Cursor on Teodor (scroll starts from Mihai)')
        CheckHash(0x2AE34EBB0F458B7C)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigate_keys_mode_columns_2() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from USA)')
        CheckHash(0x16AC19856A27F20D)
        Key.Pressed(Down)
        Paint('2. Cursor on John')
        CheckHash(0xB580C8E374CEF837)
        Key.Pressed(Down,3)
        Paint('3. Cursor on Europe')
        CheckHash(0x37301EBF0F11BE59)
        Key.Pressed(Down,2)
        Paint('4. Cursor on Etiene')
        CheckHash(0x1CF6302F59814073)
        Key.Pressed(Down)
        Paint('5. Cursor on Karl [column:2] - (scroll starts from USA)')
        CheckHash(0x2A35FFD8F52C2AA7)
        Key.Pressed(PageDown)
        Paint('6. Cursor on Marin (scroll starts from Jonas)')
        CheckHash(0xBC64B5DA32836D23)
        Key.Pressed(Up,3)
        Paint('7. Cursor on Vlad (scroll starts from Jonas)')
        CheckHash(0xCC9FCBBD329194D8)
        Key.Pressed(PageUp)
        Paint('8. Cursor on Europe (scroll starts from Europe)')
        CheckHash(0x37AF100A1C11EC81)
        Key.Pressed(Right)
        Paint('9. Cursor on Kai [Column:2] (scroll starts from Europe)')
        CheckHash(0x8336F762C229A53C)
        Key.Pressed(Right)
        Paint('10. Cursor on Vlad [Column:2,last] (scroll starts from Sancez)')
        CheckHash(0x372127E998C80E4E)
        Key.Pressed(Left)
        Paint('11. Cursor on Kai [Column:1] (scroll starts from Sancez)')
        CheckHash(0x473F99708B7FF71)
        Key.Pressed(Left)
        Paint('12. Cursor on Europe [Column:1] (scroll starts from Europe)')
        CheckHash(0x37AF100A1C11EC81)
        Key.Pressed(Right,3)
        Paint('13. Cursor on Teodor [Column:2,last] (scroll starts from Asia)')
        CheckHash(0xA1CCA28B22095A21)
        Key.Pressed(Right,3)
        Paint('14. Cursor remains on Teodor [Column:2,last] (scroll starts from Asia)')
        CheckHash(0xA1CCA28B22095A21)
        Key.Pressed(PageDown)
        Paint('15. Cursor remains on Teodor [Column:2,last] (scroll starts from Asia)')
        CheckHash(0xA1CCA28B22095A21)
        Key.Pressed(Home)
        Paint('16. Cursor back to initial state')
        CheckHash(0x16AC19856A27F20D)
        Key.Pressed(End)
        Paint('17. Cursor back on Teodor [Column:2,last] (scroll starts from Asia)')
        CheckHash(0xA1CCA28B22095A21)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Columns(2),flags: ScrollBars+ShowGroups+CheckBoxes,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigate_keys_mode_columns_2_no_checkboxes() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from USA)')
        CheckHash(0x60D51AAE8B7F6E34)
        Key.Pressed(Down)
        Paint('2. Cursor on John')
        CheckHash(0x95523E36244631A4)
        Key.Pressed(Down,3)
        Paint('3. Cursor on Europe')
        CheckHash(0x2B642A9C6C4902B0)
        Key.Pressed(Down,2)
        Paint('4. Cursor on Etiene')
        CheckHash(0xCAD2C89EA8CC0A60)
        Key.Pressed(Down)
        Paint('5. Cursor on Karl [column:2] - (scroll starts from USA)')
        CheckHash(0x5EDBEEDAA1E502AC)
        Key.Pressed(PageDown)
        Paint('6. Cursor on Marin (scroll starts from Jonas)')
        CheckHash(0xD87FD7BC875245AD)
        Key.Pressed(Up,3)
        Paint('7. Cursor on Vlad (scroll starts from Jonas)')
        CheckHash(0x404DBC1C5A2DE18E)
        Key.Pressed(PageUp)
        Paint('8. Cursor on Europe (scroll starts from Europe)')
        CheckHash(0xECE6473D5BC71152)
        Key.Pressed(Right)
        Paint('9. Cursor on Kai [Column:2] (scroll starts from Europe)')
        CheckHash(0xDBBFD67279E8C84D)
        Key.Pressed(Right)
        Paint('10. Cursor on Vlad [Column:2,last] (scroll starts from Sancez)')
        CheckHash(0x4033F6A65856AD24)
        Key.Pressed(Left)
        Paint('11. Cursor on Kai [Column:1] (scroll starts from Sancez)')
        CheckHash(0x2F3A88149A8DAB6B)
        Key.Pressed(Left)
        Paint('12. Cursor on Europe [Column:1] (scroll starts from Europe)')
        CheckHash(0xECE6473D5BC71152)
        Key.Pressed(Right,3)
        Paint('13. Cursor on Teodor [Column:2,last] (scroll starts from Asia)')
        CheckHash(0xE43AFA143E2A58CF)
        Key.Pressed(Right,3)
        Paint('14. Cursor remains on Teodor [Column:2,last] (scroll starts from Asia)')
        CheckHash(0xE43AFA143E2A58CF)
        Key.Pressed(PageDown)
        Paint('15. Cursor remains on Teodor [Column:2,last] (scroll starts from Asia)')
        CheckHash(0xE43AFA143E2A58CF)
        Key.Pressed(Home)
        Paint('16. Cursor back to initial state')
        CheckHash(0x60D51AAE8B7F6E34)
        Key.Pressed(End)
        Paint('17. Cursor back on Teodor [Column:2,last] (scroll starts from Asia)')
        CheckHash(0xE43AFA143E2A58CF)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(2),flags: ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigate_keys_mode_columns_3() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from USA)')
        CheckHash(0xF678FC0FAD8F3DEF)
        Key.Pressed(Down)
        Paint('2. Cursor on John')
        CheckHash(0xDC5C08DC80FFDA68)
        Key.Pressed(Down,3)
        Paint('3. Cursor on Europe')
        CheckHash(0x3ECD2C4091135017)
        Key.Pressed(Down,2)
        Paint('4. Cursor on Etiene')
        CheckHash(0x3AA89DF3DFA5895C)
        Key.Pressed(Down)
        Paint('5. Cursor on Karl [column:2] - (scroll starts from USA)')
        CheckHash(0xACCF1076BFEEE270)
        Key.Pressed(PageDown)
        Paint('6. Cursor on Teodor (scroll starts from MIke)')
        CheckHash(0x800820C35EA0583)
        Key.Pressed(Up,3)
        Paint('7. Cursor on Ion[Column:3] (scroll starts from Mike)')
        CheckHash(0x27035D1EDABABE1C)
        Key.Pressed(PageUp)
        Paint('8. Cursor on USA (scroll starts from USA)')
        CheckHash(0xF678FC0FAD8F3DEF)
        Key.Pressed(Right)
        Paint('9. Cursor on Karl [Column:2] (scroll starts from USA)')
        CheckHash(0xACCF1076BFEEE270)
        Key.Pressed(Right)
        Paint('10. Cursor on Chen Li [Column:3] (scroll starts from USA)')
        CheckHash(0x51FD55B0D2974627)
        Key.Pressed(Left)
        Paint('11. Cursor on Karl [Column:2] (scroll starts from USA)')
        CheckHash(0xACCF1076BFEEE270)
        Key.Pressed(Down,3)
        Key.Pressed(Left)
        Paint('12. Cursor on Todd [Column:1] (scroll starts from USA)')
        CheckHash(0xD52B1C7B8CE54E48)
        Key.Pressed(Right,3)
        Paint('13. Cursor on Teodor [Column:3,last] (scroll starts from Mike)')
        CheckHash(0x800820C35EA0583)
        Key.Pressed(Right,3)
        Paint('14. Cursor remains on Teodor [Column:3,last] (scroll starts from Mike)')
        CheckHash(0x800820C35EA0583)
        Key.Pressed(PageDown)
        Paint('15. Cursor remains on Teodor [Column:3,last] (scroll starts from Mike)')
        CheckHash(0x800820C35EA0583)
        Key.Pressed(Home)
        Paint('16. Cursor back to initial state')
        CheckHash(0xF678FC0FAD8F3DEF)
        Key.Pressed(Down,4)
        Key.Pressed(Right)
        Paint('17. Cursor on Kai [Column:2] (scroll starts from USA)')
        CheckHash(0xB9F9EE8A9C47A8AB)
        Key.Pressed(Right)
        Paint('18. Cursor on Vlad [Column:3] (scroll starts from USA)')
        CheckHash(0x3981DEA800956584)
        Key.Pressed(Left,2)
        Paint('19. Cursor on Europe [Column:1] (scroll starts from USA)')
        CheckHash(0x3ECD2C4091135017)
        Key.Pressed(Left)
        Paint('20. Cursor on USA [Column:1,first] (scroll starts from USA)')
        CheckHash(0xF678FC0FAD8F3DEF)
        Key.Pressed(End)
        Paint('21. Cursor back on Teodor [Column:3,last] (scroll starts from Mike)')
        CheckHash(0x800820C35EA0583)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Columns(3),flags: ScrollBars+ShowGroups+CheckBoxes,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigate_keys_mode_columns_3_no_checkboxes() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from USA)')
        CheckHash(0xEDE3F53EE0F55861)
        Key.Pressed(Down)
        Paint('2. Cursor on John')
        CheckHash(0x10039C38FF6B29F5)
        Key.Pressed(Down,3)
        Paint('3. Cursor on Europe')
        CheckHash(0xDA81B28223F6A185)
        Key.Pressed(Down,2)
        Paint('4. Cursor on Etiene')
        CheckHash(0x325BF7481BBA48F1)
        Key.Pressed(Down)
        Paint('5. Cursor on Karl [column:2] - (scroll starts from USA)')
        CheckHash(0x796B1099C8BC7C5)
        Key.Pressed(PageDown)
        Paint('6. Cursor on Teodor (scroll starts from MIke)')
        CheckHash(0xD65AB689B1378ECA)
        Key.Pressed(Up,3)
        Paint('7. Cursor on Ion[Column:3] (scroll starts from Mike)')
        CheckHash(0xDD926EC3064B6945)
        Key.Pressed(PageUp)
        Paint('8. Cursor on USA (scroll starts from USA)')
        CheckHash(0xEDE3F53EE0F55861)
        Key.Pressed(Right)
        Paint('9. Cursor on Karl [Column:2] (scroll starts from USA)')
        CheckHash(0x796B1099C8BC7C5)
        Key.Pressed(Right)
        Paint('10. Cursor on Chen Li [Column:3] (scroll starts from USA)')
        CheckHash(0x97A493C688F286D2)
        Key.Pressed(Left)
        Paint('11. Cursor on Karl [Column:2] (scroll starts from USA)')
        CheckHash(0x796B1099C8BC7C5)
        Key.Pressed(Down,3)
        Key.Pressed(Left)
        Paint('12. Cursor on Todd [Column:1] (scroll starts from USA)')
        CheckHash(0x86FB4A2D1A73C1B5)
        Key.Pressed(Right,3)
        Paint('13. Cursor on Teodor [Column:3,last] (scroll starts from Mike)')
        CheckHash(0xD65AB689B1378ECA)
        Key.Pressed(Right,3)
        Paint('14. Cursor remains on Teodor [Column:3,last] (scroll starts from Mike)')
        CheckHash(0xD65AB689B1378ECA)
        Key.Pressed(PageDown)
        Paint('15. Cursor remains on Teodor [Column:3,last] (scroll starts from Mike)')
        CheckHash(0xD65AB689B1378ECA)
        Key.Pressed(Home)
        Paint('16. Cursor back to initial state')
        CheckHash(0xEDE3F53EE0F55861)
        Key.Pressed(Down,4)
        Key.Pressed(Right)
        Paint('17. Cursor on Kai [Column:2] (scroll starts from USA)')
        CheckHash(0x911F078D69FD5F26)
        Key.Pressed(Right)
        Paint('18. Cursor on Vlad [Column:3] (scroll starts from USA)')
        CheckHash(0xFAF82C34F4DD9929)
        Key.Pressed(Left,2)
        Paint('19. Cursor on Europe [Column:1] (scroll starts from USA)')
        CheckHash(0xDA81B28223F6A185)
        Key.Pressed(Left)
        Paint('20. Cursor on USA [Column:1,first] (scroll starts from USA)')
        CheckHash(0xEDE3F53EE0F55861)
        Key.Pressed(End)
        Paint('21. Cursor back on Teodor [Column:3,last] (scroll starts from Mike)')
        CheckHash(0xD65AB689B1378ECA)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(3),flags: ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_empty_list_navigation_view_details() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from USA)')
        CheckHash(0xD23097345BC6E84)
        Key.Pressed(Down,3)
        Key.Pressed(Up,2)
        Key.Pressed(Left,3)
        Key.Pressed(Right,2)
        Key.Pressed(PageDown)
        Key.Pressed(Home)
        Key.Pressed(PageUp)
        Key.Pressed(End)
        Paint('2. State remains the same')
        CheckHash(0xD23097345BC6E84)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let lv = listview!("Person,d:c,view:Details,flags: ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_empty_list_navigation_view_columns_2() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from USA)')
        CheckHash(0x9F818C330754220D)
        Key.Pressed(Down,3)
        Key.Pressed(Up,2)
        Key.Pressed(Left,3)
        Key.Pressed(Right,2)
        Key.Pressed(PageDown)
        Key.Pressed(Home)
        Key.Pressed(PageUp)
        Key.Pressed(End)
        Paint('2. State remains the same')
        CheckHash(0x9F818C330754220D)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let lv = listview!("Person,d:c,view:Columns(2),flags: ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_empty_list_navigation_view_columns_4() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from USA)')
        CheckHash(0xACF4FE52E73E65AD)
        Key.Pressed(Down,3)
        Key.Pressed(Up,2)
        Key.Pressed(Left,3)
        Key.Pressed(Right,2)
        Key.Pressed(PageDown)
        Key.Pressed(Home)
        Key.Pressed(PageUp)
        Key.Pressed(End)
        Paint('2. State remains the same')
        CheckHash(0xACF4FE52E73E65AD)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let lv = listview!("Person,d:c,view:Columns(4),flags: ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_navigate_keys_mode_columns_2_no_checkboxes_no_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from John)')
        CheckHash(0x1FF6D1E0707A694D)
        Key.Pressed(Down)
        Paint('2. Cursor on Mke')
        CheckHash(0x7F062EB63E123D61)
        Key.Pressed(Down,4)
        Paint('3. Cursor on Karl')
        CheckHash(0x15D0346B0F03A37D)
        Key.Pressed(Down,2)
        Paint('4. Cursor on Yu Law (second column)')
        CheckHash(0x8F1CCA3480CF4A51)
        Key.Pressed(Down)
        Paint('5. Cursor on Kay [column:2] - (scroll starts from John)')
        CheckHash(0xE925FC7CF494E7CA)
        Key.Pressed(PageDown)
        Paint('6. Cursor on Teodor (scroll starts from Karl)')
        CheckHash(0xEE17AB263D429E21)
        Key.Pressed(Up,3)
        Paint('7. Cursor on Ion (scroll starts from Karl)')
        CheckHash(0x9D027C0106B8652E)
        Key.Pressed(PageUp)
        Paint('8. Cursor on Mike (scroll starts from Mike)')
        CheckHash(0xEF5EA31F6C5E0B65)
        Key.Pressed(Right)
        Paint('9. Cursor on Kai [Column:2] (scroll starts from Mike)')
        CheckHash(0x2BFC39436CB5DD7A)
        Key.Pressed(Right)
        Paint('10. Cursor on IOn [Column:2,last] (scroll starts from Todd)')
        CheckHash(0x6E131F53141C63C3)
        Key.Pressed(Left)
        Paint('11. Cursor on Kai [Column:1] (scroll starts from Todd)')
        CheckHash(0x5A2224983DCB672B)
        Key.Pressed(Left)
        Paint('12. Cursor on Mike [Column:1] (scroll starts from Mike)')
        CheckHash(0xEF5EA31F6C5E0B65)
        Key.Pressed(Right,3)
        Paint('13. Cursor on Teodor [Column:2,last] (scroll starts from Karl)')
        CheckHash(0xEE17AB263D429E21)
        Key.Pressed(Right,3)
        Paint('14. Cursor remains on Teodor [Column:2,last] (scroll starts from Karl)')
        CheckHash(0xEE17AB263D429E21)
        Key.Pressed(PageDown)
        Paint('15. Cursor remains on Teodor [Column:2,last] (scroll starts from Karl)')
        CheckHash(0xEE17AB263D429E21)
        Key.Pressed(Home)
        Paint('16. Cursor back to initial state')
        CheckHash(0x1FF6D1E0707A694D)
        Key.Pressed(End)
        Paint('17. Cursor back on Teodor [Column:2,last] (scroll starts from Karl)')
        CheckHash(0xEE17AB263D429E21)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(2),flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_changing_scroll_with_keys_viewmode_details() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from John,cursor on John)')
        CheckHash(0xF73F60131F7F0467)
        Key.Pressed(Ctrl+Alt+Down)
        Paint('2. Scroll starts with Mke (selection is not visible)')
        CheckHash(0x5A3081E291EE83D6)
        Key.Pressed(Ctrl+Alt+Down,4)
        Paint('3. Scroll starts with on Karl')
        CheckHash(0x78569F84D9FAC3CC)
        Key.Pressed(Down,2)
        // the first Down key restore the scroll position
        // the second just moves the cursor
        Paint('4. Scroll starts with Mike, Selection is on Todd')
        CheckHash(0xF7174883C3455E5E)
        Key.Pressed(Down,3)
        Key.Pressed(Ctrl+Alt+Down,5)
        Paint('5. Scroll starts with Jonas (seletion not visible)')
        CheckHash(0x8A110AE9F8257F26)
        Key.Pressed(Ctrl+Alt+Up,3)
        Paint('6. Scroll starts with Sancez (cursor on Karl)')
        CheckHash(0x9FB87E41DC728305)
        Key.Pressed(Ctrl+Alt+Up,100)
        Paint('7. Scroll starts with John (cursor on Karl)')
        CheckHash(0x26B57503482F9B93)
        Key.Pressed(Ctrl+Alt+Down,100)
        Paint('9. Scroll starts with Mihai (cursor not visible)')
        CheckHash(0xF0D100CA0D8572E8)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Details,flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_changing_scroll_with_keys_viewmode_columns_3() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from John,cursor on John)')
        CheckHash(0xA79704A84622FE8)
        Key.Pressed(Ctrl+Alt+Down)
        Paint('2. Nothing happens (all items are visible)')
        CheckHash(0xA79704A84622FE8)
        Key.Pressed(Ctrl+Alt+Up,100)
        Paint('3. Nothing happens (all items are visible)')
        CheckHash(0xA79704A84622FE8)
        Key.Pressed(Ctrl+Alt+Down,100)
        Paint('4. Nothing happens (all items are visible)')
        CheckHash(0xA79704A84622FE8)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(3),flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_changing_scroll_with_keys_viewmode_columns_2() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from John,cursor on John)')
        CheckHash(0x1FF6D1E0707A694D)
        Key.Pressed(Ctrl+Alt+Down)
        Paint('2. Scroll starts with Mke (selection is not visible)')
        CheckHash(0x14D74B1E74093625)
        Key.Pressed(Ctrl+Alt+Down,4)
        Paint('3. Scroll starts with on Karl')
        CheckHash(0xCED2467A7C73CC1D)
        Key.Pressed(Down,2)
        // the first Down key restore the scroll position
        // the second just moves the cursor
        Paint('4. Scroll starts with on Mike, Selection is on Todd')
        CheckHash(0x2C05D94E78980379)
        Key.Pressed(Down,2)
        Key.Pressed(Ctrl+Alt+Down,4)
        Paint('5. Scroll starts with on Karl (seletion not visible)')
        CheckHash(0xCED2467A7C73CC1D)
        Key.Pressed(Ctrl+Alt+Up,3)
        Paint('6. Scroll starts with on Todd (cursor or Etienne)')
        CheckHash(0x7A858D4B7295B16C)
        Key.Pressed(Ctrl+Alt+Up,100)
        Paint('7. Scroll starts with on JOhn (cursor on Etienne)')
        CheckHash(0x6DAA747E7F6FA299)
        Key.Pressed(Ctrl+Alt+Down,100)
        Paint('8. Scroll starts with on Karl (cursor not visible)')
        CheckHash(0xCED2467A7C73CC1D)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(2),flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_select_with_mouse_viewmode_details() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from John,cursor on John)')
        CheckHash(0xF73F60131F7F0467)
        Mouse.Click(14,7,left)
        Paint('2. Selected: Etiene')
        CheckHash(0x2A2068FA8E607478)
        Mouse.Click(40,4,left)
        Paint('3. Selected: Mike')
        CheckHash(0xBF4735812C9CD2DB)
        Mouse.Click(44,5,left)
        Paint('4. Selected: Todd')
        CheckHash(0x97CFE2808BE3E483)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Details,flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_select_with_mouse_viewmode_colums_3() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from John,cursor on John)')
        CheckHash(0xA79704A84622FE8)
        Mouse.Click(14,7,left)
        Paint('2. Selected: Karl')
        CheckHash(0x5A977164B3434178)
        Mouse.Click(40,4,left)
        Paint('3. Selected: Gheorghe')
        CheckHash(0xED518AB3281FBAB8)
        Mouse.Click(44,5,left)
        Paint('4. Selected: Marin')
        CheckHash(0xD98AFEAD0115DF63)
        Mouse.Click(28,7,left)
        Paint('5. Selected: Andrei')
        CheckHash(0x4EF079CB13627C50)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(3),flags: ScrollBars,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_select_with_mouse_viewmode_details_with_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from USA,cursor on USA)')
        CheckHash(0x44D6237F035CB4)
        Mouse.Click(14,7,left)
        Paint('2. Selected: Europe Group')
        CheckHash(0xD15C9F9836A5F800)
        Mouse.Click(40,4,left)
        Paint('3. Selected: John')
        CheckHash(0x4FF67D3ED33528DC)
        Mouse.Click(44,5,left)
        Paint('4. Selected: Mike')
        CheckHash(0x9ED84080FA5D6E8)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Details,flags: ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_select_with_mouse_viewmode_colums_3_with_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from USA,cursor on USA)')
        CheckHash(0xE13E48CE0A47B95C)
        Mouse.Click(14,6,left)
        Paint('2. Selected: Europe group')
        CheckHash(0xF6BB083B07DE4BF8)
        Mouse.Click(40,4,left)
        Paint('3. Selected: Andrei')
        CheckHash(0xB0C7FF24585E02BA)
        Mouse.Click(44,5,left)
        Paint('4. Selected: Mihai')
        CheckHash(0xA6D0974B957768E1)
        Mouse.Click(28,7,left)
        Paint('5. Selected: Chen')
        CheckHash(0x719FF1F63D3DAAB2)
        Mouse.Click(34,4,left)
        Paint('6. Selected: Asia group')
        CheckHash(0xE31AD6B686C17620)        
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:50,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(3),flags: ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_resize_window_view_columns_3() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC3AF94EDA108D941)     
        Mouse.Drag(39,8,50,8) 
        Paint('2. Resized')
        CheckHash(0xFDB09C3407374AAB)     
        Mouse.Drag(50,8,59,8) 
        Paint('3. Resized')
        CheckHash(0x715076E1432BC0A4)     
        Mouse.Drag(59,8,19,8) 
        Paint('4. Resized')
        CheckHash(0xB569C25F98391083)     
        Mouse.Drag(19,8,19,10) 
        Paint('5. Height increased')
        CheckHash(0x2D0F2E9B2F5BE149)     
        Mouse.Drag(19,10,59,10) 
        Paint('6. Height increased & resized')
        CheckHash(0x426BD2A131C85C3)     
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,x:0,y:0,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(3),flags: ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_resize_window_view_columns_3_from_end_width_checkboxes() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(End)
        Paint('1. Initial state')
        CheckHash(0x5CD50C2F808C5543)     
        Mouse.Drag(39,8,50,8) 
        Paint('2. Resized')
        CheckHash(0xB86DDA803EA56BBD)     
        Mouse.Drag(50,8,59,8) 
        Paint('3. Resized')
        CheckHash(0xEBF753AFB3027CC5)     
        Mouse.Drag(59,8,19,8) 
        Paint('4. Resized (just x nothing else)')
        CheckHash(0x60575ED72DBCB516)     
        Mouse.Drag(19,8,19,10) 
        Paint('5. Height increased')
        CheckHash(0x597ED679C93FA214)     
        Mouse.Drag(19,10,59,10) 
        Paint('6. Height increased & resized')
        CheckHash(0x2B7002EAA7237AF3)     
        Mouse.Drag(59,10,59,6) 
        Paint('7. Height decreased (selection is not visible)')
        CheckHash(0x7EB4328ED146EBDA)    
        Key.Pressed(Up) 
        Paint('8. Selection is visible (at Marin, scroll starts from Karl)')
        CheckHash(0xA53E52B8F3CAD4A8)    
        Key.Pressed(Down) 
        Paint('9. Selection is visible (at Teodor, scroll starts from Jonas)')
        CheckHash(0x94E5EA9F2F544358)    
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,x:0,y:0,w:40,h:9,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Columns(3),flags: ScrollBars+ShowGroups+CheckBoxes,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_hover_on_items_view_details() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x5985022EA6C2FAFE)     
        Mouse.Move(3,3) 
        Paint('2. Hover over Checkbox of John')
        CheckHash(0x88C320739D191D2B)       
        Mouse.Move(3,5) 
        Paint('3. Hover over Checkbox of Todd')
        CheckHash(0xC6A9F47B2236ADBF)       
        Mouse.Move(5,6) 
        Paint('4. Hover over Checkbox of Europe')
        CheckHash(0xF4BD6ABEC424C94F)       
        Mouse.Move(2,6) 
        Paint('5. Hover over Collapse from Europe')
        CheckHash(0xDBB0BE8ACC8F62D4)       
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,x:0,y:0,w:40,h:9,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Details,flags: ScrollBars+ShowGroups+CheckBoxes,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_hover_on_items_view_columns_3() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x7643ACC81F1299EF)     
        Mouse.Move(3,3) 
        Paint('2. Hover over Checkbox of Mike')
        CheckHash(0x3B8A9DF94913744E)       
        Mouse.Move(41,4) 
        Paint('3. Hover over Checkbox of Marin')
        CheckHash(0x218F4C7A00BA82CE)       
        Mouse.Move(5,5) 
        Paint('4. Hover over Checkbox of Europe')
        CheckHash(0x1759181AD5188D7A)       
        Mouse.Move(2,5) 
        Paint('5. Hover over Collapse from Europe')
        CheckHash(0x1E47CABD03FB6609)       
        Mouse.Move(25,1) 
        Paint('6. Hover over Checkbox from Asia')
        CheckHash(0x920A175AE663AC2E)       
        Mouse.Move(21,1) 
        Paint('7. Hover over Collapse from Asia')
        CheckHash(0xEC6C7F255C3B02AD)       
        Mouse.Move(21,7) 
        Paint('8. Hover over Collapse from Romania')
        CheckHash(0xEF6824047808E96D)       
        Mouse.Move(23,7) 
        Paint('9. Hover over CheckBox from Romania')
        CheckHash(0xD2D6E880C63AC376)
        Mouse.Move(2,1) 
        Paint('10. Hover over Collapse from USA')
        CheckHash(0x458DE426E8429F94)       
        Mouse.Move(5,1) 
        Paint('11. Hover over CheckBox from USA')
        CheckHash(0x4A75556AFD43CC90)       
        Mouse.Click(2,1,left) 
        Paint('12. Collapse USA group')
        CheckHash(0x8BC7D199CFC4E928)       
        Mouse.Move(2,2) 
        Paint('13. Hover over Collapse from Europe')
        CheckHash(0xD2D9184AFBA550A5)       
        Mouse.Move(24,4) 
        Paint('14. Hover over CheckBox from Romania')
        CheckHash(0x942FB17B2D95584A)       
        Mouse.Move(41,2) 
        Paint('15. Hover over CheckBox from Teodor')
        CheckHash(0xBD7FC79FC8422A06)       
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,x:0,y:0,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Columns(3),flags: ScrollBars+ShowGroups+CheckBoxes,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_sort_no_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xD478C89C88EB3A43)  
        Mouse.Click(14,2,left)   
        Paint('2. Sort by name (ascendent) cursor on Dragos')
        CheckHash(0x55D548C867D25465)  
        Mouse.Click(14,2,left)   
        Paint('3. Sort by name (descendent) cursor on Dragos')
        CheckHash(0x38A1122D44D9752C)  
        Key.Pressed(Down,2)
        Paint('4. Now cursor on Bruce')
        CheckHash(0x9BB34118E2A15658)  
        Mouse.Click(35,2,left)   
        Paint('5. Sort by city (ascendent) cursor on Bruce')
        CheckHash(0x9451DB3381206649)  
        Mouse.Click(35,2,left)   
        Paint('6. Sort by city (descendent) cursor on Bruce')
        CheckHash(0x651A5FDAFB45DD7F)  
        Key.Pressed(Home)
        Paint('7. Now cursor on Dragos (from Zanzibar descending)')
        CheckHash(0x765886DE6E162CFE)  
        Key.Pressed(Ctrl+N)
        Paint('8. Sort by name (ascendent) cursor on Dragos')
        CheckHash(0x612571FC4FD23D58)  
        Key.Pressed(Ctrl+S)
        Paint('9. Sort by size (ascendent) cursor on Dragos')
        CheckHash(0x4BB0CFA7D1F5D1F3)  
        Key.Pressed(Ctrl+S)
        Paint('10. Sort by size (descendent) cursor on Dragos')
        CheckHash(0x173E33D8E1484115)  
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:8,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars,columns=[{&Name,10,Left},{&Size,10,Right},{&City,10,Center}]");
    Person::populate_for_sort(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_sort_no_groups_3_columns() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x3032B710D6D3752)  
        Key.Pressed(Ctrl+N)
        Paint('2. Nothing should happen (as we are on a list view mode)')
        CheckHash(0x3032B710D6D3752)  
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:8,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(3),flags:ScrollBars,columns=[{&Name,10,Left},{&Size,10,Right},{&City,10,Center}]");
    Person::populate_for_sort2(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_sort_no_groups_3_columns_with_commands() {
    #[Window(events: CommandBarEvents, internal: true, commands:[ByName,ByCity,BySize])]
    struct MyWin {
        lv: Handle<ListView<Person>>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,d:c,w:40,h:8,flags: Sizeable"),
                lv: Handle::None,
            };
            let mut lv = listview!("Person,d:c,view:Columns(3),flags:ScrollBars,columns=[{&Name,10,Left},{&Size,10,Right},{&City,10,Center}]");
            Person::populate_for_sort2(&mut lv);
            w.lv = w.add(lv);
            w
        }
    }
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Sort by name", mywin::Commands::ByName);
            commandbar.set(key!("F2"), "Sort by age", mywin::Commands::BySize);
            commandbar.set(key!("F3"), "Sort by city", mywin::Commands::ByCity);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            let h = self.lv;
            match command_id {
                mywin::Commands::ByName => {
                    if let Some(c) = self.control_mut(h) {
                        c.sort(0, true)
                    }
                }
                mywin::Commands::ByCity => {
                    if let Some(c) = self.control_mut(h) {
                        c.sort(2, true)
                    }
                }
                mywin::Commands::BySize => {
                    if let Some(c) = self.control_mut(h) {
                        c.sort(1, true)
                    }
                }
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x493F449BF2AB2E62)  
        Key.Pressed(F1)
        Paint('2. Sort by name (ascendent) cursor on Dragos')
        CheckHash(0x60FE67CF20690673)  
        Key.Pressed(F2)
        Paint('3. Sort by age (ascendent) cursor on Dragos')
        CheckHash(0xB0CC910515391AE8)  
        Key.Pressed(F3)
        Paint('4. Sort by city (ascendent) cursor on Dragos')
        CheckHash(0xE68EB359CA5885EC) 
    ";
    let mut a = App::debug(60, 11, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_sort_groups_details() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x242A1971CD3A8F69)  
        Key.Pressed(Ctrl+N)
        Paint('2. Sort by name but withing the grops (ascendent) cursor on USA')
        CheckHash(0x9B82DD7A98DB388A)  
        Key.Pressed(Down,5)
        Paint('3. cursor on Etiene')
        CheckHash(0xD50D7122063D6C19)  
        Key.Pressed(Ctrl+S)
        Paint('4. Sort by size but withing the grops (ascendent) cursor on Etiene (last from group)')
        CheckHash(0xC2B87EBA596BC4A5)  
        Mouse.Click(30,1,left)
        Paint('5. Sort by city but withing the grops (ascendent) cursor on Etiene')
        CheckHash(0x1C2E94E45BFC87FB)  
        Mouse.Click(30,1,left)
        Paint('5. Sort by city but withing the grops (descendent) cursor on Etiene')
        CheckHash(0xFE8457732E43FDB6)  
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Details,flags:ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_fold_groups_details_with_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x71ADDF80B0EFE8B2)  
        Key.Pressed(Space)
        Paint('2. USA group collapsed')
        CheckHash(0x535F3B7FE3F4D04D)
        Key.Pressed(Down,6)
        Paint('3. Cursor on Asia group')
        CheckHash(0x9CAFE69E0B109B55)
        Key.Pressed(Space)
        Paint('4. Asia group collapsed (cursor on Asia)')
        CheckHash(0x1CBA3D43D2DBB54A)
        Key.Pressed(Up,5)
        Paint('5. Cursor on Europe group')
        CheckHash(0x6A0FDCBDA65DD476)
        Key.Pressed(Space)
        Paint('6. Europe group collapsed (cursor on Europe)')
        CheckHash(0xC9BC26938B2FB7FF)
        Key.Pressed(Home)
        Paint('7. Cusor on USA group')
        CheckHash(0x6C10887728CA5353)
        Key.Pressed(Space)
        Paint('8. USA group unfold (cursor on USA)')
        CheckHash(0x67E9B5D4C5E675AF)
        Key.Pressed(Down,5)
        Paint('9. Cursor on Asia group')
        CheckHash(0xB28030965912FC8F)
        Key.Pressed(Space)
        Paint('10. Asia group unfold (cursor on Asia)')
        CheckHash(0x155538C17E24C351)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Details,flags:ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_fold_groups_details_with_keys_and_checkboxes() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x50422D61A602CC20)  
        Key.Pressed(Enter)
        Paint('2. USA group collapsed')
        CheckHash(0xDE22812284FE93D4)
        Key.Pressed(Down,6)
        Paint('3. Cursor on Asia group')
        CheckHash(0xFD2CFC2BA7BE00F0)
        Key.Pressed(Enter)
        Paint('4. Asia group collapsed (cursor on Asia)')
        CheckHash(0x9240A66A407C5C08)
        Key.Pressed(Up,5)
        Paint('5. Cursor on Europe group')
        CheckHash(0xCB7455B23017C7D4)
        Key.Pressed(Enter)
        Paint('6. Europe group collapsed (cursor on Europe)')
        CheckHash(0xEA4756A25DD61F68)
        Key.Pressed(Home)
        Paint('7. Cusor on USA group')
        CheckHash(0xFB9073E8ED3DCA68)
        Key.Pressed(Enter)
        Paint('8. USA group unfold (cursor on USA)')
        CheckHash(0x4BBC1B520F8ED749)
        Key.Pressed(Down,5)
        Paint('9. Cursor on Asia group')
        CheckHash(0x177CC811829ADF49)
        Key.Pressed(Enter)
        Paint('10. Asia group unfold (cursor on Asia)')
        CheckHash(0xEF2D25D25D5E0188)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Details,flags:ScrollBars+ShowGroups+CheckBoxes,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_fold_groups_details_with_mouse() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x71ADDF80B0EFE8B2)  
        Mouse.Click(2,2,left)
        Paint('2. USA group collapsed')
        CheckHash(0x1BA863A229396672)
        Mouse.Click(5,8,left)
        Paint('3. Cursor on Asia group')
        CheckHash(0x9CAFE69E0B109B55)
        Mouse.Click(2,8,left)
        Paint('4. Asia group collapsed (cursor on Asia)')
        CheckHash(0xE4F55C50CFD16FD5)
        Mouse.Click(20,3,left)
        Paint('5. Cursor on Europe group')
        CheckHash(0x6A0FDCBDA65DD476)
        Mouse.Click(2,3,left)
        Paint('6. Europe group collapsed (cursor on Europe)')
        CheckHash(0xC74B25B3A4363C30)
        Mouse.Click(35,2,left)
        Paint('7. Cusor on USA group')
        CheckHash(0x6C10887728CA5353)
        Mouse.Click(2,2,left)
        Paint('8. USA group unfold (cursor on USA)')
        CheckHash(0xD879BF4A3088DD4C)
        Mouse.Click(19,7,left)
        Paint('9. Cursor on Asia group')
        CheckHash(0xB28030965912FC8F)
        Mouse.Click(2,7,left)
        Paint('10. Asia group unfold (cursor on Asia)')
        CheckHash(0xE43DE23734DA82EA)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Details,flags:ScrollBars+ShowGroups,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_fold_groups_3_columns_with_keys_and_checkboxes() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x978DA3298F62537B)  
        Key.Pressed(Enter)
        Paint('2. USA group collapsed')
        CheckHash(0xA059090C47309633)
        Key.Pressed(Down,6)
        Paint('3. Cursor on Asia group')
        CheckHash(0xE8BE3E2ECD5B51AF)
        Key.Pressed(Enter)
        Paint('4. Asia group collapsed (cursor on Asia)')
        CheckHash(0x31D20DE72F108377)
        Key.Pressed(Up,5)
        Paint('5. Cursor on Europe group')
        CheckHash(0xCDE0075A36C6BA07)
        Key.Pressed(Enter)
        Paint('6. Europe group collapsed (cursor on Europe)')
        CheckHash(0xF99FEE798FACC7F5)
        Key.Pressed(Home)
        Paint('7. Cusor on USA group')
        CheckHash(0x42ADC0354B23819)
        Key.Pressed(Enter)
        Paint('8. USA group unfold (cursor on USA)')
        CheckHash(0x5A9FA8E4B9CB8399)
        Key.Pressed(Down,5)
        Paint('9. Cursor on Asia group')
        CheckHash(0x23F803CF70244BBD)
        Key.Pressed(Enter)
        Paint('10. Asia group unfold (cursor on Asia)')
        CheckHash(0x33C6D10E6AD0565D)
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Columns(3),flags:ScrollBars+ShowGroups+CheckBoxes,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_check_with_keys_no_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x71D21AC2A0917A86)  
        Key.Pressed(Down)
        Key.Pressed(Insert)
        Paint('2. Checked (Zig), cursor on Bruce')
        CheckHash(0x9A10FA4CEFE397B6)  
        Key.Pressed(Shift+Down,3)
        Paint('3. Checked (Zig,Bruce,Conrad,Peter), cursor on Tom')
        CheckHash(0x3995C25A1D9C18CD)  
        Key.Pressed(Down)
        Key.Pressed(Shift+Down,2)
        Paint('4. Aditional Checked (John,Alex), cursor on Alex')
        CheckHash(0x18B483F4346B4DEF)  
        Key.Pressed(Insert)
        Paint('5. Now Alex is no longer checked, cursor on Alex')
        CheckHash(0xAB5945A018A01483)  
        Key.Pressed(Ctrl+A)
        Paint('6. All items checked, cursor on Alex')
        CheckHash(0xED20BB8656251D7)  
        Key.Pressed(Ctrl+A)
        Paint('7. All items un-checked, cursor on Alex')
        CheckHash(0x2F99E654D801BEFF)  
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Details,flags:ScrollBars,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate_for_sort(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_check_with_keys_no_groups_checkboxes() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x1465916A8FFC10CE)  
        Key.Pressed(Down)
        Key.Pressed(Insert)
        Paint('2. Checked (Zig), cursor on Bruce')
        CheckHash(0x6F32BFF77BBE6E6C)  
        Key.Pressed(Shift+Down,3)
        Paint('3. Checked (Zig,Bruce,Conrad,Peter), cursor on Tom')
        CheckHash(0xBD93EF3FDB5FA981)  
        Key.Pressed(Down)
        Key.Pressed(Shift+Down,2)
        Paint('4. Aditional Checked (John,Alex), cursor on Alex')
        CheckHash(0x2EB2B5A762E89AE9)  
        Key.Pressed(Insert)
        Paint('5. Now Alex is no longer checked, cursor on Alex')
        CheckHash(0xA47409A0D9A90D41)  
        Key.Pressed(Ctrl+A)
        Paint('6. All items checked, cursor on Alex')
        CheckHash(0x8F928AC3B9D296E3)  
        Key.Pressed(Ctrl+A)
        Paint('7. All items un-checked, cursor on Alex')
        CheckHash(0xDF4BEC272C349C3F)  
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Details,flags:ScrollBars+CheckBoxes,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate_for_sort(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_check_with_keys_no_groups_columns_2() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xAE1344FF63648BFF)  
        Key.Pressed(Down)
        Key.Pressed(Insert)
        Paint('2. Checked (Mike), cursor on Todd')
        CheckHash(0x77944DF19376DFDF)  
        Key.Pressed(Shift+Down,3)
        Paint('3. Checked (Mike,Todd,Sancez,Etiene), cursor on Karl')
        CheckHash(0x164D3CE5EE1DC0EF)  
        Key.Pressed(Down)
        Key.Pressed(Shift+Down,2)
        Paint('4. Aditional Checked (Jonas,Yu Law), cursor on Kai')
        CheckHash(0xEC9E634BE3565948)  
        Key.Pressed(Down,2)
        Key.Pressed(Shift+Right,1)
        Paint('5. Aditioanl checked (Chan Li -> Teodor), cursor on Teodor')
        CheckHash(0x6C2B5D5912B3144D)  
        Key.Pressed(Ctrl+A)
        Paint('6. All items checked, cursor on Teodor')
        CheckHash(0x8284190F3F743821)  
        Key.Pressed(Ctrl+A)
        Paint('7. All items un-checked, cursor on Teodor')
        CheckHash(0xB4DC008E98FB2E95)  
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(2),flags:ScrollBars,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_check_with_keys_and_groups_details() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xFE38CA7939ABB271)  
        Key.Pressed(Space)
        Paint('2. All items under USA checked (cursor on USA)')
        CheckHash(0xFCD452704760FDB7)  
        Key.Pressed(Down,2)
        Key.Pressed(Space)
        Paint('3. Cursor on Mike, MIke is not checked, USA is partially checked [?]')
        CheckHash(0x7796299CD2A6A152)  
        Key.Pressed(Up,1)
        Key.Pressed(Space)
        Paint('4. Cursor on John, John is not checked, USA is partially checked [?]')
        CheckHash(0xA0DC5F386E00ECC)  
        Key.Pressed(Down,2)
        Key.Pressed(Space)
        Paint('5. Cursor on Todd, Todd is not checked, USA is NOT checked')
        CheckHash(0x4BB58C50C5A54978)  
        Key.Pressed(Space)
        Paint('6. Cursor on Todd, Todd IS checked, USA is partially checked [?]')
        CheckHash(0x5CDAE3CD95977039)  
        Key.Pressed(Up,3)
        Key.Pressed(Space)
        Paint('7. Cursor on USA, USA (and all items from group) are checked')
        CheckHash(0xFCD452704760FDB7)  
        Key.Pressed(Space)
        Paint('8. Cursor on USA, USA (and all items from group) are NOT checked')
        CheckHash(0xFE38CA7939ABB271)
        Key.Pressed(Down,10)  
        Key.Pressed(Space)
        Paint('9. Cursor on Yu Law, Yu Law is CHECKED, Asia group is partially checked [?]')
        CheckHash(0x7BE6764444725CA)
        Key.Pressed(Up,6)
        Paint('10. Cursor on Europe group')
        CheckHash(0xE9C2B6DC0B08EAF1)
        Key.Pressed(Insert)
        Paint('11. Cursor on Sancez, Europe group is CHECKED, Sancez is CHECKED')
        CheckHash(0x48F56FFBFF8C20B)
        Key.Pressed(Shift+Down,2)
        Paint('12. Cursor on Karl, Sancez and Etiene are NOT checked, Europe group is partially checked [?]')
        CheckHash(0xBAF0CB5D3F573110)
        Key.Pressed(Down,2)
        Key.Pressed(Shift+Down)
        Paint('13. Cursor on Yu Law, entire Asia Group is checked')
        CheckHash(0xCC5A2ECB64768F48)
        Key.Pressed(Up)
        Key.Pressed(Shift+Down)
        Paint('14. Cursor on Yu Law, entire Asia Group is NOT checked')
        CheckHash(0x935CE1341C87F596)
        Key.Pressed(Shift+End)
        Paint('15. Cursor on Teodor, Everything from Yu Law until Teodor are checked')
        CheckHash(0x182B8ADA67A9D620)
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Details,flags:ScrollBars+CheckBoxes+ShowGroups,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_check_with_keys_and_groups_3_columns() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x978DA3298F62537B)  
        Key.Pressed(Space)
        Paint('2. All items under USA checked (cursor on USA)')
        CheckHash(0x42800E01138F6BB1)  
        Key.Pressed(Down,2)
        Key.Pressed(Space)
        Paint('3. Cursor on Mike, MIke is not checked, USA is partially checked [?]')
        CheckHash(0xE0F57F230BB4474)  
        Key.Pressed(Up,1)
        Key.Pressed(Space)
        Paint('4. Cursor on John, John is not checked, USA is partially checked [?]')
        CheckHash(0x7DCC3D9301D68142)  
        Key.Pressed(Down,2)
        Key.Pressed(Space)
        Paint('5. Cursor on Todd, Todd is not checked, USA is NOT checked')
        CheckHash(0x64701E617B8ADE85)  
        Key.Pressed(Space)
        Paint('6. Cursor on Todd, Todd IS checked, USA is partially checked [?]')
        CheckHash(0xCA4FD09FD78B5D3C)  
        Key.Pressed(Up,3)
        Key.Pressed(Space)
        Paint('7. Cursor on USA, USA (and all items from group) are checked')
        CheckHash(0x42800E01138F6BB1)  
        Key.Pressed(Space)
        Paint('8. Cursor on USA, USA (and all items from group) are NOT checked')
        CheckHash(0x978DA3298F62537B)
        Key.Pressed(Down,10)  
        Key.Pressed(Space)
        Paint('9. Cursor on Yu Law, Yu Law is CHECKED, Asia group is partially checked [?]')
        CheckHash(0x7B1CC85F09967CE0)
        Key.Pressed(Up,6)
        Paint('10. Cursor on Europe group')
        CheckHash(0x485349D41CD03558)
        Key.Pressed(Insert)
        Paint('11. Cursor on Sancez, Europe group is CHECKED, Sancez is CHECKED')
        CheckHash(0x4EF76BCD88430F8E)
        Key.Pressed(Shift+Down,2)
        Paint('12. Cursor on Karl, Sancez and Etiene are NOT checked, Europe group is partially checked [?]')
        CheckHash(0xEDE4A47E1B7A85C9)
        Key.Pressed(Down,2)
        Key.Pressed(Shift+Down)
        Paint('13. Cursor on Yu Law, entire Asia Group is checked')
        CheckHash(0x532FB9AD91F6988A)
        Key.Pressed(Up)
        Key.Pressed(Shift+Down)
        Paint('14. Cursor on Yu Law, entire Asia Group is NOT checked')
        CheckHash(0xB3C1122B99B50E00)
        Key.Pressed(Shift+End)
        Paint('15. Cursor on Teodor, Everything from Yu Law until Teodor are checked')
        CheckHash(0xF244A913EB6F0A26)
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Columns(3),flags:ScrollBars+CheckBoxes+ShowGroups,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_check_with_mouse_and_groups_details() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xFE38CA7939ABB271)  
        Mouse.Click(5,2,left)
        Paint('2. All items under USA checked (cursor on USA)')
        CheckHash(0xDEB1E877BA139A50)  
        Mouse.Click(3,4,left)        
        Paint('3. Cursor on Mike, MIke is not checked, USA is partially checked [?]')
        CheckHash(0xB67F230F678231F9)  
        Mouse.Click(3,3,left)
        Paint('4. Cursor on John, John is not checked, USA is partially checked [?]')
        CheckHash(0xF83AB5CBED2C25D3)  
        Mouse.Click(3,5,left)
        Paint('5. Cursor on Todd, Todd is not checked, USA is NOT checked')
        CheckHash(0x6DD6CE76D4526683)  
        Mouse.Click(3,5,left)
        Paint('6. Cursor on Todd, Todd IS checked, USA is partially checked [?]')
        CheckHash(0x992DB861555C4ACE)  
        Mouse.Click(6,2,left)
        Paint('7. Cursor on USA, USA (and all items from group) are checked')
        CheckHash(0xDEB1E877BA139A50)  
        Mouse.Click(4,2,left)
        Paint('8. Cursor on USA, USA (and all items from group) are NOT checked')
        CheckHash(0xB6A7F5858E3BE05E)
        // click outside the listvew (on the deactivated scroll bar)
        Mouse.Click(3,9,left)
        Paint('9. Nothing happens: Cursor on USA, USA (and all items from group) are NOT checked')
        CheckHash(0xFE38CA7939ABB271)
        Mouse.Wheel(20,5,down,1)
        Mouse.Click(3,8,left)
        Paint('10. Cursor on Karl; Karl is CHECKED, Europe group is partially checked [?]')
        CheckHash(0x5288789D13EDB1FC)
        Mouse.Click(3,8,left)
        Paint('11. Cursor on Karl, Karl is CHECKED, Europe group is NOT checked')
        CheckHash(0xBA5F5EA77525A1D5)
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Details,flags:ScrollBars+CheckBoxes+ShowGroups,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_check_with_mouse_and_groups_2_columns() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x12903703CFE1E8FC)  
        Mouse.Click(5,1,left)
        Paint('2. All items under USA checked (cursor on USA)')
        CheckHash(0xED44841F41F3FD2D)  
        Mouse.Click(3,3,left)        
        Paint('3. Cursor on Mike, MIke is not checked, USA is partially checked [?]')
        CheckHash(0x387C8EECCCB9799C)  
        Mouse.Click(3,2,left)
        Paint('4. Cursor on John, John is not checked, USA is partially checked [?]')
        CheckHash(0xD046C4670EF97AEE)  
        Mouse.Click(3,4,left)
        Paint('5. Cursor on Todd, Todd is not checked, USA is NOT checked')
        CheckHash(0x7361598DD13BAE41)  
        Mouse.Click(3,4,left)
        Paint('6. Cursor on Todd, Todd IS checked, USA is partially checked [?]')
        CheckHash(0xEF25AC57F70BAA60)  
        Mouse.Click(6,1,left)
        Paint('7. Cursor on USA, USA (and all items from group) are checked')
        CheckHash(0xED44841F41F3FD2D)  
        Mouse.Click(4,1,left)
        Paint('8. Cursor on USA, USA (and all items from group) are NOT checked')
        CheckHash(0x2EFAB7CE3FAAFCF3)
        // click outside the listvew (on the deactivated scroll bar)
        Mouse.Click(3,9,left)
        Paint('9. Nothing happens: Cursor on USA, USA (and all items from group) are NOT checked')
        CheckHash(0x12903703CFE1E8FC)
        Mouse.Wheel(20,5,down,1)
        Mouse.Click(3,8,left)
        Paint('10. Cursor on Jonas; Jonas is CHECKED, Europe group is partially checked [?]')
        CheckHash(0xE6F54937614C904F)
        Mouse.Click(3,8,left)
        Paint('11. Cursor on Jonas, Jonas is CHECKED, Europe group is NOT checked')
        CheckHash(0xC9B7230B891373A6)
        Mouse.Click(34,1,left)
        Paint('12. Cursor on Asia, Asia group is checked')
        CheckHash(0x9AB839D34AF7B771)
        Mouse.Click(32,3,left)
        Paint('13. Cursor on Kai, Kai is not Checked, Asia group is partially checked [?]')
        CheckHash(0x81B6DB7A22EFE80B)
        Mouse.Click(31,1,left)
        Paint('14. Cursor on Asia, Asia group is Folded')
        CheckHash(0xD63CEE5AC1373074)
        Mouse.Click(32,8,left)
        Paint('15. Cursor on Marin, Marin is checked, Romania group is partially checked [?]')
        CheckHash(0x4BA0D17034FCE2B7)
        Mouse.Wheel(20,5,down,1)
        Paint('16. Cursor on Marin, Marin is NOT checked, Romania group is partially checked [?], View statys from Mike')
        CheckHash(0xA7A4854C58820250)
        Mouse.Click(2,8,left)
        Paint('17. Asia group is unfolded, cursor on Asia')
        CheckHash(0xFC94AD7FE5C436EE)
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Columns(2),flags:ScrollBars+CheckBoxes+ShowGroups,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_datetime_normal() {
    struct Student {
        name: &'static str,
        born: NaiveDateTime,
    }
    impl listview::ListItem for Student {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Text(self.name)),
                1 => Some(listview::RenderMethod::DateTime(self.born, listview::DateTimeFormat::Normal)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1 => self.born.cmp(&other.born),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x5B1C10DAD00281CD) 
        Mouse.Click(5,1,left) 
        Paint('2. Sort by name (ascendent) - cursor on John')
        CheckHash(0x7C578B003F18F032) 
        Mouse.Click(5,1,left) 
        Paint('3. Sort by name (descendent) - cursor on John')
        CheckHash(0x6DF5332E78517E18) 
        Mouse.Click(35,1,left) 
        Paint('4. Sort by datetime (ascendent) - cursor on John')
        CheckHash(0x7D9E569032489D4E) 
        Mouse.Click(35,1,left) 
        Paint('5. Sort by datetime (descendent) - cursor on John')
        CheckHash(0x4CA258340F6023C4) 
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Student,d:c,flags:ScrollBars,columns=[{&Name,10,Left},{'&Born on',30,Center}]");

    let students = vec![
        Student {
            name: "John",
            born: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap().and_hms_opt(12, 34, 56).unwrap(),
        },
        Student {
            name: "Mike",
            born: NaiveDate::from_ymd_opt(1997, 5, 20).unwrap().and_hms_opt(1, 2, 3).unwrap(),
        },
        Student {
            name: "Alex",
            born: NaiveDate::from_ymd_opt(1997, 5, 20).unwrap().and_hms_opt(23, 59, 59).unwrap(),
        },
        Student {
            name: "Zig",
            born: NaiveDate::from_ymd_opt(2005, 12, 31).unwrap().and_hms_opt(18, 30, 0).unwrap(),
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_datetime_short() {
    struct Student {
        name: &'static str,
        born: NaiveDateTime,
    }
    impl listview::ListItem for Student {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Text(self.name)),
                1 => Some(listview::RenderMethod::DateTime(self.born, listview::DateTimeFormat::Short)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1 => self.born.cmp(&other.born),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x4554C81261452634) 
        Mouse.Click(5,1,left) 
        Paint('2. Sort by name (ascendent) - cursor on John')
        CheckHash(0x3145CFCC818A291B) 
        Mouse.Click(5,1,left) 
        Paint('3. Sort by name (descendent) - cursor on John')
        CheckHash(0xA203F179479E3459) 
        Mouse.Click(35,1,left) 
        Paint('4. Sort by datetime (ascendent) - cursor on John')
        CheckHash(0x509F3FB14B0C86B7) 
        Mouse.Click(35,1,left) 
        Paint('5. Sort by datetime (descendent) - cursor on John')
        CheckHash(0xC647BD4ABEBDAEF5) 
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Student,d:c,flags:ScrollBars,columns=[{&Name,10,Left},{'&Born on',30,Center}]");

    let students = vec![
        Student {
            name: "John",
            born: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap().and_hms_opt(12, 34, 56).unwrap(),
        },
        Student {
            name: "Mike",
            born: NaiveDate::from_ymd_opt(1997, 5, 20).unwrap().and_hms_opt(1, 2, 3).unwrap(),
        },
        Student {
            name: "Alex",
            born: NaiveDate::from_ymd_opt(1997, 5, 20).unwrap().and_hms_opt(23, 59, 59).unwrap(),
        },
        Student {
            name: "Zig",
            born: NaiveDate::from_ymd_opt(2005, 12, 31).unwrap().and_hms_opt(18, 30, 0).unwrap(),
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_datetime_full() {
    struct Student {
        name: &'static str,
        born: NaiveDateTime,
    }
    impl listview::ListItem for Student {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Text(self.name)),
                1 => Some(listview::RenderMethod::DateTime(self.born, listview::DateTimeFormat::Full)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1 => self.born.cmp(&other.born),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x3B552F39E1B2838) 
        Mouse.Click(5,1,left) 
        Paint('2. Sort by name (ascendent) - cursor on John')
        CheckHash(0xD966E7A05562E6B7) 
        Mouse.Click(5,1,left) 
        Paint('3. Sort by name (descendent) - cursor on John')
        CheckHash(0x413A21454B652FD) 
        Mouse.Click(35,1,left) 
        Paint('4. Sort by datetime (ascendent) - cursor on John')
        CheckHash(0x27D7CADCCBE1383B) 
        Mouse.Click(35,1,left) 
        Paint('5. Sort by datetime (descendent) - cursor on John')
        CheckHash(0x59264D9D3D81C531) 
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Student,d:c,flags:ScrollBars,columns=[{&Name,10,Left},{'&Born on',30,Center}]");

    let students = vec![
        Student {
            name: "John",
            born: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap().and_hms_opt(12, 34, 56).unwrap(),
        },
        Student {
            name: "Mike",
            born: NaiveDate::from_ymd_opt(1997, 5, 20).unwrap().and_hms_opt(1, 2, 3).unwrap(),
        },
        Student {
            name: "Alex",
            born: NaiveDate::from_ymd_opt(1997, 5, 20).unwrap().and_hms_opt(23, 59, 59).unwrap(),
        },
        Student {
            name: "Zig",
            born: NaiveDate::from_ymd_opt(2005, 12, 31).unwrap().and_hms_opt(18, 30, 0).unwrap(),
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_filter_details_without_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xCC575D7AD387F6A5)  
        Key.TypeText('mi')
        Paint('2. Cursor on Mike, [Mike,Mihai and Gheorghe are visible]')
        CheckHash(0x5707D2B5B51914B3)
        CheckCursor(4,11)
        Key.Pressed(Escape)  
        Paint('3. All items (curson on Mike)')
        CheckHash(0x24EE099411C373FD)  
        Key.TypeText('3')
        Paint('4. Cursor on Sancez, [Sancez, Yu Law and Mihai are visible] ')
        CheckHash(0xCEDF6F82B6C5CFC2)
        Key.Pressed(Escape)
        Key.TypeText('Bucharest')
        Paint('5. Cursor on Ion, [Ion and Marin are visible]')
        CheckHash(0x5C51D19861045FDD)
        CheckCursor(11,11)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+CheckBoxes+SearchBar,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_filter_details_with_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x68027AE2F1046FCD)  
        Key.TypeText('mi')
        Paint('2. Cursor on Mike, USA and Romania groups are visible')
        CheckHash(0x72A1E99690D4D151)
        CheckCursor(4,11)
        Key.Pressed(Escape)  
        Paint('3. All items (curson on Mike)')
        CheckHash(0x93075CC668AB036B)  
        Key.TypeText('3')
        Paint('4. Cursor on Sancez, Europe/Asia/Romania groups are visible (with 1 item each) ')
        CheckHash(0xFCDD19BD738E0FB4)
        Key.Pressed(Escape)
        Key.TypeText('Bucharest')
        Paint('5. Cursor on Ion, Romania group is visible with two items')
        CheckHash(0x4DC512A82409096)
        CheckCursor(11,11)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,flags:ScrollBars+CheckBoxes+ShowGroups+SearchBar,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_filter_details_with_groups_no_result_search() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x68027AE2F1046FCD)  
        Key.TypeText('xyztBC')
        Paint('2. No items visible')
        CheckHash(0x8A741CE652294875)
        Key.Pressed(Escape)  
        Paint('3. All items (curson on John)')
        CheckHash(0x6657F8ED70CD626B)  
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,flags:ScrollBars+CheckBoxes+ShowGroups+SearchBar,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_filter_details_without_groups_no_result_search() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xCC575D7AD387F6A5)  
        Key.TypeText('xyztBC')
        Paint('2. No items visible')
        CheckHash(0x8A741CE652294875)
        Key.Pressed(Escape)  
        Paint('3. All items (curson on John)')
        CheckHash(0xCC575D7AD387F6A5)  
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+CheckBoxes+SearchBar,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_filter_columns_search_age() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x9A8829E6C6794A19)  
        Key.TypeText('3')
        Paint('2. No items visible (even if there are items with 3 in the age)')
        CheckHash(0xB93C3D60914AF1C4)
        Key.Pressed(Escape)  
        Paint('3. All items (cursor on John)')
        CheckHash(0x9A8829E6C6794A19)  
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Columns(3),flags:ScrollBars+CheckBoxes+SearchBar,columns=[{&Name,10,Left},{&Size,10,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_columns_autoresize_double_click_with_groups_with_checkboxes() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x147356C458F6062E)  
        Mouse.DoubleClick(11,1,left)
        Paint('2. First column auto-resized')
        CheckHash(0xCFEC96AD446AD7A2)  
        Mouse.DoubleClick(24,1,left)
        Paint('3. Second column auto-resized')
        CheckHash(0x51BD609B37E3766)  
        Mouse.DoubleClick(27,1,left)
        Paint('4. Fourth column auto-resized')
        CheckHash(0x42E317708303853E)  
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,flags:ScrollBars+CheckBoxes+SearchBar+ShowGroups,columns=[{&Name,10,Left},{&Size,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_columns_autoresize_double_click_without_groups_with_checkboxes() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xD2238E5F6838AE09)  
        Mouse.DoubleClick(6,1,left)
        Paint('2. First column auto-resized')
        CheckHash(0x7FC371038C3A9B6E)  
        Mouse.DoubleClick(17,1,left)
        Paint('3. Second column auto-resized')
        CheckHash(0xD80A9DD7B6AC354C)  
        Mouse.DoubleClick(20,1,left)
        Paint('4. Fourth column auto-resized')
        CheckHash(0x996E9331B3B8B186)  
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+CheckBoxes+SearchBar,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_columns_autoresize_double_click_without_groups_without_checkboxes() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC763EBB1152DC0CF)  
        Mouse.DoubleClick(6,1,left)
        Paint('2. First column auto-resized')
        CheckHash(0x1782931BF7F0EC66)  
        Mouse.DoubleClick(15,1,left)
        Paint('3. Second column auto-resized')
        CheckHash(0x2675D88BDD8AED8C)  
        Mouse.DoubleClick(18,1,left)
        Paint('4. Fourth column auto-resized')
        CheckHash(0x8606E26D34ED1136)  
    ";
    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_columns_autoresize_with_icon_small_details_no_checboxes_no_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x17100C2B7B5B024E)  
        Mouse.DoubleClick(6,1,left)
        Paint('2. First column auto-resized')
        CheckHash(0xC6B07B4F773B3674)   
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+SearchBar+SmallIcons,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_columns_autoresize_with_icon_small_details_no_checboxes_with_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x97B4CEA673F134B8)  
        Mouse.DoubleClick(6,1,left)
        Paint('2. First column auto-resized')
        CheckHash(0x52DB5FE005D711DA)   
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+SearchBar+SmallIcons+ShowGroups,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_columns_autoresize_with_icon_small_details_with_checboxes_with_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC5DD39381C542B08)  
        Mouse.DoubleClick(6,1,left)
        Paint('2. First column auto-resized')
        CheckHash(0x2BE25D50F47BD397)   
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,flags:ScrollBars+SearchBar+SmallIcons+ShowGroups+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_columns_autoresize_with_icon_small_details_with_checboxes_without_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x3C69A16A94AD801E)  
        Mouse.DoubleClick(6,1,left)
        Paint('2. First column auto-resized')
        CheckHash(0xE5941496700CD360)   
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+SearchBar+SmallIcons+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_columns_autoresize_with_icon_large_details_no_checboxes_no_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC86741106818EBCE)  
        Mouse.DoubleClick(6,1,left)
        Paint('2. First column auto-resized')
        CheckHash(0x58141EFF73E68E75)   
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+SearchBar+LargeIcons,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_columns_autoresize_with_icon_large_details_no_checboxes_with_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x3360AF925AAB2A8F)  
        Mouse.DoubleClick(6,1,left)
        Paint('2. First column auto-resized')
        CheckHash(0x75C1F9304674E084)   
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+SearchBar+LargeIcons+ShowGroups,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_columns_autoresize_with_icon_large_details_with_checboxes_with_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC5DD39381C542B08)  
        Mouse.DoubleClick(6,1,left)
        Paint('2. First column auto-resized')
        CheckHash(0x78039A575E457BE1)   
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,flags:ScrollBars+SearchBar+LargeIcons+ShowGroups+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_columns_autoresize_with_icon_large_details_with_checboxes_without_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xFC318D489409A585)  
        Mouse.DoubleClick(6,1,left)
        Paint('2. First column auto-resized')
        CheckHash(0x8FD1D236E8DE1A41)   
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+SearchBar+LargeIcons+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_icon_small_3_columns_no_checboxes_no_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x91E2941BE32F8DAF)  
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+SmallIcons,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_icon_small_3_columns_no_checboxes_with_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x551ED6316723F225)     
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!(
        "Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+SmallIcons+ShowGroups,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]"
    );
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_icon_small_3_columns_with_checboxes_with_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xD87F10C1D22554B1)  
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+SmallIcons+ShowGroups+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_icon_small_3_columns_with_checboxes_without_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xD5848AD1F98892E3)  
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!(
        "Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+SmallIcons+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]"
    );
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_icon_large_3_columns_no_checboxes_no_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6190A2B1430BD412)  
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+LargeIcons,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_icon_large_3_columns_no_checboxes_with_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x9B4A6D714FAF88F)     
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!(
        "Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+LargeIcons+ShowGroups,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]"
    );
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_icon_large_3_columns_with_checboxes_with_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x7FA38519D4D7F5C3)  
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+LargeIcons+ShowGroups+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]");
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_icon_large_3_columns_with_checboxes_without_groups() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x128B0BA6B5DAA76)  
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!(
        "Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+LargeIcons+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]"
    );
    Person::populate_with_icon(&mut lv);

    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_custom_filter() {
    struct Student {
        name: &'static str,
        grade: i32,
    }
    impl listview::ListItem for Student {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Text(self.name)),
                1 => Some(listview::RenderMethod::Int64(self.grade as i64, listview::NumericFormat::Normal)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1 => self.grade.cmp(&other.grade),
                _ => std::cmp::Ordering::Equal,
            }
        }

        fn matches(&self, text: &str) -> bool {
            if let Ok(number) = text.parse() {
                self.grade >= number
            } else {
                false
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xEA6531BEDEBEEDB7)
        Key.TypeText('8')
        // only values bigger than 8 should be provided
        Paint('2. John and Zig are visible')
        CheckHash(0x2F06B7915860706B)
        Key.Pressed(Escape)
        Key.TypeText('6')
        Paint('3. John,Alex and Zig are visible')
        CheckHash(0xFC5BE975A3ECC1A5)
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Student,d:c,flags:ScrollBars+SearchBar+CustomFilter,columns=[{&Name,10,Left},{&Grade,10,Right}]");

    let students = vec![
        Student { name: "John", grade: 10 },
        Student { name: "Mike", grade: 5 },
        Student { name: "Alex", grade: 7 },
        Student { name: "Zig", grade: 8 },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_numeric_formater_renderer() {
    struct Employee {
        name: &'static str,
        salary: u64,
        rgb: u32,
        debt: i64,
    }
    impl listview::ListItem for Employee {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::UInt64(self.salary, listview::NumericFormat::Separator)),
                2 => Some(listview::RenderMethod::UInt64(self.rgb as u64, listview::NumericFormat::Hex32)),
                3 => Some(listview::RenderMethod::Int64(self.debt, listview::NumericFormat::Normal)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1 => self.salary.cmp(&other.salary),
                2 => self.rgb.cmp(&other.rgb),
                3 => self.debt.cmp(&other.debt),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x2856BE886DE7BDDE)
        Key.TypeText('1,0')
        Paint('2. Alex is visible')
        CheckHash(0x1CD641E5712E4D26)
        Key.Pressed(Escape)
        Key.TypeText('FFFF')
        Paint('3. Alex and Zig are visible')
        CheckHash(0x1FD2B0113FA2344B)
        Key.Pressed(Escape)
        Mouse.DoubleClick(22,1,left)
        Paint('4. Salary column auto-resized')
        CheckHash(0x7158F839E845EB0E)
        Mouse.DoubleClick(34,1,left)
        Paint('5. RGB column auto-resized')
        CheckHash(0x127C73F6B58F3962)
        Mouse.DoubleClick(43,1,left)
        Paint('6. Debt column auto-resized')
        CheckHash(0xFD92A12162C24BA)
        Mouse.Click(17,1,left)
        Paint('7. Sort by salary (ascendent)')
        CheckHash(0xB6204AF8F9D68E2C)
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Employee,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,10,Left},{&Salary,10,Right},{&RGB,12,Center},{&Debt,10,Right}]");

    let students = vec![
        Employee {
            name: "John",
            salary: 150000,
            rgb: 0xFFAABB,
            debt: 0,
        },
        Employee {
            name: "Mike",
            salary: 45000,
            rgb: 0xAA,
            debt: -3000,
        },
        Employee {
            name: "Alex",
            salary: 1000000,
            rgb: 0xFFFF,
            debt: -123456,
        },
        Employee {
            name: "Zig",
            salary: 12500,
            rgb: 0xFFFF0000,
            debt: -25,
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_numeric_formater_renderer_with_macro() {
    #[derive(ListItem)]
    struct Employee {
        #[Column(name = "&Name", align = "Left", size = 10)]
        name: &'static str,
        #[Column(name = "&Salary", align = "Right", size = 10, render: "UInt64", format: "Separator")]
        salary: u64,
        #[Column(name = "&RGB", align = "Center", size = 12, render: "UInt64", format: "Hex32")]
        rgb: u32,
        #[Column(name = "&Debt", align = "Right", size = 10, render: "Int64", format: "Normal")]
        debt: i64,
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x2856BE886DE7BDDE)
        Key.TypeText('1,0')
        Paint('2. Alex is visible')
        CheckHash(0x1CD641E5712E4D26)
        Key.Pressed(Escape)
        Key.TypeText('FFFF')
        Paint('3. Alex and Zig are visible')
        CheckHash(0x1FD2B0113FA2344B)
        Key.Pressed(Escape)
        Mouse.DoubleClick(22,1,left)
        Paint('4. Salary column auto-resized')
        CheckHash(0x7158F839E845EB0E)
        Mouse.DoubleClick(34,1,left)
        Paint('5. RGB column auto-resized')
        CheckHash(0x127C73F6B58F3962)
        Mouse.DoubleClick(43,1,left)
        Paint('6. Debt column auto-resized')
        CheckHash(0xFD92A12162C24BA)
        Mouse.Click(17,1,left)
        Paint('7. Sort by salary (ascendent)')
        CheckHash(0xB6204AF8F9D68E2C)
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Employee,d:c,flags:ScrollBars+SearchBar");

    let students = vec![
        Employee {
            name: "John",
            salary: 150000,
            rgb: 0xFFAABB,
            debt: 0,
        },
        Employee {
            name: "Mike",
            salary: 45000,
            rgb: 0xAA,
            debt: -3000,
        },
        Employee {
            name: "Alex",
            salary: 1000000,
            rgb: 0xFFFF,
            debt: -123456,
        },
        Employee {
            name: "Zig",
            salary: 12500,
            rgb: 0xFFFF0000,
            debt: -25,
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bool_formater_renderer() {
    struct Employee {
        name: &'static str,
        v1: bool,
        v2: bool,
        v3: bool,
        v4: bool,
    }
    impl listview::ListItem for Employee {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Bool(self.v1, listview::BoolFormat::YesNo)),
                2 => Some(listview::RenderMethod::Bool(self.v2, listview::BoolFormat::CheckmarkMinus)),
                3 => Some(listview::RenderMethod::Bool(self.v3, listview::BoolFormat::XMinus)),
                4 => Some(listview::RenderMethod::Bool(self.v4, listview::BoolFormat::TrueFalse)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1 => self.v1.cmp(&other.v1),
                2 => self.v2.cmp(&other.v2),
                3 => self.v3.cmp(&other.v3),
                4 => self.v4.cmp(&other.v4),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x436B2F176AEDC868)
        Key.TypeText('Yes')
        Paint('2. John and Alex are visible')
        CheckHash(0x7A6AF8F43FABC95E)
        Key.Pressed(Escape)
        Key.TypeText('False')
        Paint('3. Alex and Mike are visible')
        CheckHash(0x57ACA971D48BC22)
        Key.Pressed(Escape)
        Mouse.DoubleClick(22,1,left)
        Paint('4. V1 column auto-resized')
        CheckHash(0x94C0D0AC7DF68702)
        Mouse.DoubleClick(28,1,left)
        Paint('5. V2 column auto-resized')
        CheckHash(0xC0889D8BCBAF323E)
        Mouse.DoubleClick(39,1,left)
        Paint('6. V4 column auto-resized')
        CheckHash(0x28A6BDE924E5114E)
        Mouse.Click(31,1,left)
        Paint('7. Sort by V4 (ascendent)')
        CheckHash(0xD53023C5C3820831)
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("Employee,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,10,Left},{V1,10,Right},{V2,12,Center},{V3,10,Right},{V4,10,Right}]");

    let students = vec![
        Employee {
            name: "John",
            v1: true,
            v2: false,
            v3: true,
            v4: false,
        },
        Employee {
            name: "Mike",
            v1: false,
            v2: true,
            v3: false,
            v4: true,
        },
        Employee {
            name: "Alex",
            v1: true,
            v2: true,
            v3: false,
            v4: true,
        },
        Employee {
            name: "Zig",
            v1: false,
            v2: false,
            v3: true,
            v4: false,
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bool_formater_renderer_with_macro() {
    // columns=[{&Name,10,Left},{V1,10,Right},{V2,12,Center},{V3,10,Right},{V4,10,Right}]
    #[derive(ListItem)]
    struct Employee {
        #[Column(name: "&Name", w: 10, a: l, r:Ascii)]
        name: &'static str,
        #[Column(name: "V1", w:10, a:r, render: bool, format: YesNo)]
        v1: bool,
        #[Column(name: "V2", w:12, a:c, render: bool, format: CheckmarkMinus)]
        v2: bool,
        #[Column(name: "V3", w:10, a:r, render: bool, format: XMinus)]
        v3: bool,
        #[Column(name: "V4", w:10, a:r, render: bool, format: TrueFalse)]
        v4: bool,
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x436B2F176AEDC868)
        Key.TypeText('Yes')
        Paint('2. John and Alex are visible')
        CheckHash(0x7A6AF8F43FABC95E)
        Key.Pressed(Escape)
        Key.TypeText('False')
        Paint('3. Alex and Mike are visible')
        CheckHash(0x57ACA971D48BC22)
        Key.Pressed(Escape)
        Mouse.DoubleClick(22,1,left)
        Paint('4. V1 column auto-resized')
        CheckHash(0x94C0D0AC7DF68702)
        Mouse.DoubleClick(28,1,left)
        Paint('5. V2 column auto-resized')
        CheckHash(0xC0889D8BCBAF323E)
        Mouse.DoubleClick(39,1,left)
        Paint('6. V4 column auto-resized')
        CheckHash(0x28A6BDE924E5114E)
        Mouse.Click(31,1,left)
        Paint('7. Sort by V4 (ascendent)')
        CheckHash(0xD53023C5C3820831)
    ";
    let mut a = App::debug(60, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Employee,d:c,flags:ScrollBars+SearchBar");

    let students = vec![
        Employee {
            name: "John",
            v1: true,
            v2: false,
            v3: true,
            v4: false,
        },
        Employee {
            name: "Mike",
            v1: false,
            v2: true,
            v3: false,
            v4: true,
        },
        Employee {
            name: "Alex",
            v1: true,
            v2: true,
            v3: false,
            v4: true,
        },
        Employee {
            name: "Zig",
            v1: false,
            v2: false,
            v3: true,
            v4: false,
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_size_formater_renderer_simple() {
    struct FileInfo {
        name: &'static str,
        size: u64,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::Bytes)),
                2 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::KiloBytes)),
                3 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::MegaBytes)),
                4 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::GigaBytes)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=4 => self.size.cmp(&other.size),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x580385D00925B270)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{B,20,Right},{KB,15,Right},{MB,10,Right},{GB,10,Right}]");

    let students = vec![
        FileInfo { name: "NUll", size: 0 },
        FileInfo { name: "Small", size: 300 },
        FileInfo {
            name: "Regular",
            size: 12345,
        },
        FileInfo { name: "Song", size: 3200000 },
        FileInfo {
            name: "Movie",
            size: 7950000000,
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_size_formater_renderer_with_decimals() {
    struct FileInfo {
        name: &'static str,
        size: u64,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::Bytes)),
                2 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::KiloBytesWithDecimals)),
                3 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::MegaBytesWithDecimals)),
                4 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::GigaBytesWithDecimals)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=4 => self.size.cmp(&other.size),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xBF3A416D456E4D96)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{B,20,Right},{KB,15,Right},{MB,12,Right},{GB,10,Right}]");

    let students = vec![
        FileInfo { name: "NUll", size: 0 },
        FileInfo { name: "Small", size: 345 },
        FileInfo {
            name: "Regular",
            size: 12345,
        },
        FileInfo {
            name: "Song",
            size: 3_123_456,
        },
        FileInfo {
            name: "Movie",
            size: 7_950_123_456,
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_size_formater_renderer_with_auto() {
    struct FileInfo {
        name: &'static str,
        size: u64,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::Bytes)),
                2 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::Auto)),
                3 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::AutoWithDecimals)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=3 => self.size.cmp(&other.size),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xD59FF1A8E969B065)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{B,20,Right},{Auto,20,Right},{AutoDec,20,Right}]");

    let students = vec![
        FileInfo { name: "NUll", size: 0 },
        FileInfo { name: "Small", size: 345 },
        FileInfo {
            name: "Regular",
            size: 12_345,
        },
        FileInfo {
            name: "Song",
            size: 3_123_456,
        },
        FileInfo {
            name: "Movie",
            size: 7_950_123_456,
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_time_formater_renderer() {
    struct FileInfo {
        name: &'static str,
        tm: NaiveTime,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Time(self.tm, listview::TimeFormat::Short)),
                2 => Some(listview::RenderMethod::Time(self.tm, listview::TimeFormat::AMPM)),
                3 => Some(listview::RenderMethod::Time(self.tm, listview::TimeFormat::Normal)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=3 => self.tm.cmp(&other.tm),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x2A167C2A8155CCF)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{Short,20,Right},{AM-PM,20,Right},{Normal,20,Right}]");

    let files = vec![
        FileInfo {
            name: "f1",
            tm: NaiveTime::from_hms_milli_opt(18, 32, 20, 123).unwrap(),
        },
        FileInfo {
            name: "f2",
            tm: NaiveTime::from_hms_milli_opt(23, 59, 59, 999).unwrap(),
        },
        FileInfo {
            name: "f3",
            tm: NaiveTime::from_hms_milli_opt(00, 00, 00, 000).unwrap(),
        },
        FileInfo {
            name: "f4",
            tm: NaiveTime::from_hms_milli_opt(12, 00, 00, 000).unwrap(),
        },
        FileInfo {
            name: "f5",
            tm: NaiveTime::from_hms_milli_opt(5, 20, 30, 234).unwrap(),
        },
        FileInfo {
            name: "f6",
            tm: NaiveTime::from_hms_milli_opt(21, 50, 1, 5).unwrap(),
        },
    ];
    lv.add_items(files);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_date_formater_renderer() {
    struct FileInfo {
        name: &'static str,
        dt: NaiveDate,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Date(self.dt, listview::DateFormat::Full)),
                2 => Some(listview::RenderMethod::Date(self.dt, listview::DateFormat::DayMonthYear)),
                3 => Some(listview::RenderMethod::Date(self.dt, listview::DateFormat::YearMonthDay)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=3 => self.dt.cmp(&other.dt),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xA57A8DA7D0578F54)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{Full,20,Right},{YMD,20,Right},{DMY,20,Right}]");

    let files = vec![
        FileInfo {
            name: "f1",
            dt: NaiveDate::from_ymd_opt(2024, 12, 30).unwrap(),
        },
        FileInfo {
            name: "f2",
            dt: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        },
        FileInfo {
            name: "f3",
            dt: NaiveDate::from_ymd_opt(2024, 5, 5).unwrap(),
        },
        FileInfo {
            name: "f4",
            dt: NaiveDate::from_ymd_opt(2024, 3, 4).unwrap(),
        },
        FileInfo {
            name: "f5",
            dt: NaiveDate::from_ymd_opt(2024, 10, 21).unwrap(),
        },
        FileInfo {
            name: "f6",
            dt: NaiveDate::from_ymd_opt(2024, 4, 2).unwrap(),
        },
    ];
    lv.add_items(files);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_percentage_formater_renderer() {
    struct FileInfo {
        name: &'static str,
        proc: f64,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Percentage(self.proc, listview::PercentageFormat::Normal)),
                2 => Some(listview::RenderMethod::Percentage(self.proc, listview::PercentageFormat::Decimals)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x3AC07717D31D060F)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{Normal,20,Right},{Decimals,20,Right}]");

    let files = vec![
        FileInfo { name: "f1", proc: 0.99 },
        FileInfo { name: "f2", proc: 0.0 },
        FileInfo { name: "f3", proc: 1.0 },
        FileInfo { name: "f4", proc: 0.125 },
        FileInfo { name: "f5", proc: 0.0625 },
        FileInfo { name: "f6", proc: 0.5 },
    ];
    lv.add_items(files);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_float_formater_renderer() {
    struct FileInfo {
        name: &'static str,
        value: f64,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Float(self.value, listview::FloatFormat::Normal)),
                2 => Some(listview::RenderMethod::Float(self.value, listview::FloatFormat::TwoDigits)),
                3 => Some(listview::RenderMethod::Float(self.value, listview::FloatFormat::ThreeDigits)),
                4 => Some(listview::RenderMethod::Float(self.value, listview::FloatFormat::FourDigits)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xE9C5BED230AEFA5E)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!(
        "FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{Normal,15,Right},{Two,15,Right},{Three,15,Right},{Four,15,Right}]"
    );

    let files = vec![
        FileInfo { name: "f1", value: 0.123456 },
        FileInfo { name: "f2", value: 0.0 },
        FileInfo { name: "f3", value: -1.9876 },
        FileInfo {
            name: "f4",
            value: 12345.125,
        },
        FileInfo {
            name: "f5",
            value: 123456.0625,
        },
        FileInfo { name: "f6", value: 0.5 },
    ];
    lv.add_items(files);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_on_change_current_item_event() {
    #[Window(events=ListViewEvents<Person>, internal: true)]
    struct MyWin {}
    impl ListViewEvents<Person> for MyWin {
        fn on_current_item_changed(&mut self, handle: Handle<ListView<Person>>) -> EventProcessStatus {
            let name = if let Some(lv) = self.control(handle) {
                if let Some(item) = lv.current_item() {
                    item.name.to_string()
                } else {
                    "None".to_string()
                }
            } else {
                "None".to_string()
            };
            self.set_title(&format!("Person: {}", name));
            EventProcessStatus::Processed
        }
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
            };
            let mut lv = listview!(
                "Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+ShowGroups,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]"
            );
            Person::populate(&mut lv);
            w.add(lv);
            w
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xBD45D8B7079B61BA)     
        Key.Pressed(Down)
        Paint('2. Title is John')
        CheckHash(0xF142CB62EEFE710)     
        Key.Pressed(Right)
        Paint('3. Title is None (curson on Asia group)')
        CheckHash(0x8EC8CEB7874BE721)     
        Key.Pressed(Down)
        Paint('4. Title is Yu Law')
        CheckHash(0x1B935153863C86E1)     
        Key.TypeText('Gheo')
        Paint('5. Title is Gheorghe')
        CheckHash(0xA2144919E5610F47)   
        Key.Pressed(Escape)  
        Paint('6. Cursor on Gheorghe')
        CheckHash(0xEE12444688EA14DE)   
        Mouse.Click(25,4,left)
        Paint('7. Title is Kai')
        CheckHash(0x1AC178C84EC4FB24)   
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_status_formater_renderer() {
    struct FileInfo {
        name: &'static str,
        status: listview::Status,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Status(self.status, listview::StatusFormat::Hashtag)),
                2 => Some(listview::RenderMethod::Status(self.status, listview::StatusFormat::Graphical)),
                3 => Some(listview::RenderMethod::Status(self.status, listview::StatusFormat::Arrow)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false) 
        Paint('1. Initial state')
        CheckHash(0x3C5D207D90FE87F8)
    ";
    let mut a = App::debug(80, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,6,Left},{Normal,20,Right},{Two,20,Right},{Three,20,Right}]");

    let files = vec![
        FileInfo {
            name: "f1",
            status: listview::Status::Running(0.5),
        },
        FileInfo {
            name: "f2",
            status: listview::Status::Running(0.75),
        },
        FileInfo {
            name: "f3",
            status: listview::Status::Running(0.0),
        },
        FileInfo {
            name: "f4",
            status: listview::Status::Running(1.0),
        },
        FileInfo {
            name: "f5",
            status: listview::Status::Completed,
        },
        FileInfo {
            name: "f6",
            status: listview::Status::Error,
        },
        FileInfo {
            name: "f7",
            status: listview::Status::Paused(0.33),
        },
        FileInfo {
            name: "f8",
            status: listview::Status::Paused(0.99),
        },
        FileInfo {
            name: "f9",
            status: listview::Status::Stopped,
        },
        FileInfo {
            name: "f1-",
            status: listview::Status::Queued,
        },
    ];
    lv.add_items(files);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_temperature_formater_renderer() {
    struct FileInfo {
        name: &'static str,
        temperature: f64,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Temperature(
                    self.temperature,
                    listview::TemperatureFormat::Celsius,
                )),
                2 => Some(listview::RenderMethod::Temperature(
                    self.temperature,
                    listview::TemperatureFormat::Fahrenheit,
                )),
                3 => Some(listview::RenderMethod::Temperature(self.temperature, listview::TemperatureFormat::Kelvin)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xD28567B414CAD555)
    ";
    let mut a = App::debug(80, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,6,Left},{Celsius,20,Right},{Fahrenheit,20,Right},{Kelvin,20,Right}]");

    let files = vec![
        FileInfo {
            name: "f1",
            temperature: 37.0,
        },
        FileInfo {
            name: "f2",
            temperature: 100.5,
        },
        FileInfo {
            name: "f3",
            temperature: -20.4,
        },
        FileInfo {
            name: "f4",
            temperature: 0.0,
        },
    ];
    lv.add_items(files);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_on_selection_changed_event() {
    #[Window(events=ListViewEvents<Person>, internal: true)]
    struct MyWin {}
    impl ListViewEvents<Person> for MyWin {
        fn on_selection_changed(&mut self, handle: Handle<ListView<Person>>) -> EventProcessStatus {
            let data = if let Some(lv) = self.control(handle) {
                format!("{}/{}", lv.selected_items_count(), lv.items_count())
            } else {
                "?".to_string()
            };
            self.set_title(&data);
            EventProcessStatus::Processed
        }
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
            };
            let mut lv = listview!(
                "Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+ShowGroups+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]"
            );
            Person::populate(&mut lv);
            w.add(lv);
            w
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x39603D1E8E3FDC2E) 
        Mouse.Click(5,1,left)      
        Paint('2. Usa group selected (3/19) in title')
        CheckHash(0xB2F707F4D8D99119) 
        Key.Pressed(Down,2)
        Key.Pressed(Insert)
        Paint('3. Cursor on Todd, Mike is not selected (2/19) in title')
        CheckHash(0xB106D78FE8BEDA4F) 
        Key.Pressed(Right)
        Key.Pressed(Insert,3)
        Paint('4. Cursor on Chen Li, (5/19) in title')
        CheckHash(0x773CD4F7667F489E) 
        Key.Pressed(Ctrl+A)
        Paint('5. All selected, (19/19) in title')
        CheckHash(0xF24EA396455B2322) 
        Key.Pressed(Ctrl+A)
        Paint('6. All clear, (0/19) in title')
        CheckHash(0x2C7437424D694517) 
        Key.Pressed(Shift+Home)
        Paint('7. Cursor on USA, USA+Europa+Asia group selected, (12/19) in title')
        CheckHash(0x5A053793E9C9EB8)
        Mouse.Click(41,4,left) 
        Paint('8. Ion Selected, (13/19) in title')
        CheckHash(0x39347317263325BF)
        Mouse.Click(41,4,left) 
        Paint('8. Ion un-Selected, (12/19) in title')
        CheckHash(0x5ADEEB27CB2DACE7)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_on_item_action_event() {
    #[Window(events=ListViewEvents<Person>, internal: true)]
    struct MyWin {}
    impl ListViewEvents<Person> for MyWin {
        fn on_item_action(&mut self, handle: Handle<ListView<Person>>, index: usize) -> EventProcessStatus {
            let txt = if let Some(lv) = self.control(handle) {
                if let Some(p) = lv.item(index) {
                    format!("idx: {} -> {}", index, p.name)
                } else {
                    "??".to_string()
                }
            } else {
                "?".to_string()
            };
            self.set_title(&txt);
            EventProcessStatus::Processed
        }
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
            };
            let mut lv = listview!(
                "Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+ShowGroups+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]"
            );
            Person::populate(&mut lv);
            w.add(lv);
            w
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x39603D1E8E3FDC2E)  
        Key.Pressed(Down,2)
        Key.Pressed(Enter)       
        Paint('2. Title: idx: 1 -> Mike')
        CheckHash(0x9325B0C7B506A531)  
        Mouse.DoubleClick(7,7,left)
        Paint('3. Title: idx: 4 -> Etiene')
        CheckHash(0x40E86D7984F481B0)  
        Mouse.DoubleClick(30,2,left)
        Paint('4. Asia group folded')
        CheckHash(0x893167EC1EFCEEBE)
        Key.Pressed(Enter)       
        Paint('5. Asia group un-folded')
        CheckHash(0x242F515807D881E)          
        Mouse.DoubleClick(41,1,left)
        Paint('6. Title: idx: 12 -> Andrei (Andrei is not checked)')
        CheckHash(0xEC9291622870C230)          
        Mouse.Click(41,1,left)
        Paint('7. Andrei is checked')
        CheckHash(0x8C0624825955E4B1)          
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_area_formater_renderer_simple() {
    struct FileInfo {
        name: &'static str,
        size: u64,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Area(self.size, listview::AreaFormat::SquaredKilometers)),
                2 => Some(listview::RenderMethod::Area(self.size, listview::AreaFormat::SquareFeet)),
                3 => Some(listview::RenderMethod::Area(self.size, listview::AreaFormat::SquaredCentimeters)),
                4 => Some(listview::RenderMethod::Area(self.size, listview::AreaFormat::SquareYards)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=4 => self.size.cmp(&other.size),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x11C90827B8B8029D)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{KMP,20,Right},{FTP,15,Right},{CMP,10,Right},{YD,10,Right}]");

    let students = vec![
        FileInfo { name: "NUll", size: 0 },
        FileInfo { name: "Small", size: 300 },
        FileInfo {
            name: "Regular",
            size: 12345,
        },
        FileInfo { name: "Song", size: 3200000 },
        FileInfo {
            name: "Movie",
            size: 7950000000,
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_distance_formater_renderer_simple() {
    struct FileInfo {
        name: &'static str,
        dist: u64,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Distance(self.dist, listview::DistanceFormat::Centimeters)),
                2 => Some(listview::RenderMethod::Distance(self.dist, listview::DistanceFormat::Meters)),
                3 => Some(listview::RenderMethod::Distance(self.dist, listview::DistanceFormat::Inches)),
                4 => Some(listview::RenderMethod::Distance(self.dist, listview::DistanceFormat::Feet)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=4 => self.dist.cmp(&other.dist),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x5B6BB90EC503446A)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{CM,20,Right},{M,15,Right},{IN,10,Right},{FT,10,Right}]");

    let students = vec![
        FileInfo { name: "NUll", dist: 0 },
        FileInfo { name: "Small", dist: 300 },
        FileInfo {
            name: "Regular",
            dist: 12345,
        },
        FileInfo { name: "Song", dist: 3200000 },
        FileInfo {
            name: "Movie",
            dist: 7950000000,
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_volume_formater_renderer_simple() {
    struct FileInfo {
        name: &'static str,
        vol: u64,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Volume(self.vol, listview::VolumeFormat::CubicCentimeters)),
                2 => Some(listview::RenderMethod::Volume(self.vol, listview::VolumeFormat::Liters)),
                3 => Some(listview::RenderMethod::Volume(self.vol, listview::VolumeFormat::Gallons)),
                4 => Some(listview::RenderMethod::Volume(self.vol, listview::VolumeFormat::CubicFeet)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=4 => self.vol.cmp(&other.vol),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x5DE42374940E54E)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!(
        "FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{CM**3,20,Right},{Liters,15,Right},{Galons,10,Right},{CubicFeets,15,Right}]"
    );

    let students = vec![
        FileInfo { name: "NUll", vol: 0 },
        FileInfo { name: "Small", vol: 300 },
        FileInfo { name: "Regular", vol: 12345 },
        FileInfo { name: "Large", vol: 3200000 },
        FileInfo {
            name: "Huge",
            vol: 795000000,
        },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_weight_formater_renderer_simple() {
    struct FileInfo {
        name: &'static str,
        w: u64,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Weight(self.w, listview::WeightFormat::Kilograms)),
                2 => Some(listview::RenderMethod::Weight(self.w, listview::WeightFormat::Pounds)),
                3 => Some(listview::RenderMethod::Weight(self.w, listview::WeightFormat::Tons)),
                4 => Some(listview::RenderMethod::Weight(self.w, listview::WeightFormat::Milligrams)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=4 => self.w.cmp(&other.w),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xF708A8445B8E4B4E)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!(
        "FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{KG,20,Right},{Pounds,15,Right},{Tons,10,Right},{Millgrams,15,Right}]"
    );

    let students = vec![
        FileInfo { name: "NUll", w: 0 },
        FileInfo { name: "Small", w: 300 },
        FileInfo { name: "Regular", w: 12345 },
        FileInfo { name: "Large", w: 3200000 },
        FileInfo { name: "Huge", w: 795000000 },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_speed_formater_renderer_simple() {
    struct FileInfo {
        name: &'static str,
        speed: u64,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Speed(self.speed, listview::SpeedFormat::KilometersPerHour)),
                2 => Some(listview::RenderMethod::Speed(self.speed, listview::SpeedFormat::Mach)),
                3 => Some(listview::RenderMethod::Speed(self.speed, listview::SpeedFormat::MilesPerHour)),
                4 => Some(listview::RenderMethod::Speed(self.speed, listview::SpeedFormat::MetersPerSecond)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=4 => self.speed.cmp(&other.speed),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xCBF8030B31CD0A41)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{KM/H,20,Right},{Mach,15,Right},{MPH,10,Right},{M/S,15,Right}]");

    let students = vec![
        FileInfo { name: "NUll", speed: 0 },
        FileInfo { name: "Small", speed: 140 },
        FileInfo {
            name: "Regular",
            speed: 1800,
        },
        FileInfo { name: "Large", speed: 32000 },
        FileInfo { name: "Huge", speed: 79500 },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_rating_formater_renderer_simple() {
    struct FileInfo {
        name: &'static str,
        r: u32,
        scor: u32,
    }
    impl listview::ListItem for FileInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Rating(self.r, listview::RatingFormat::Stars(5))),
                2 => Some(listview::RenderMethod::Rating(self.r, listview::RatingFormat::Circles(5))),
                3 => Some(listview::RenderMethod::Rating(self.r, listview::RatingFormat::Asterix(5))),
                4 => Some(listview::RenderMethod::Rating(self.r, listview::RatingFormat::Numerical(5))),
                5 => Some(listview::RenderMethod::Rating(self.scor, listview::RatingFormat::Numerical(1000))),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=4 => self.r.cmp(&other.r),
                5 => self.scor.cmp(&other.scor),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xDF3716DCCA6AE2E1)
    ";
    let mut a = App::debug(80, 8, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!(
        "FileInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{Stars,10,c},{Circle,10,c},{Asterix,10,c},{Numric,10,c},{Scor,10,r}]"
    );

    let students = vec![
        FileInfo { name: "#1", r: 0, scor: 123 },
        FileInfo { name: "#2", r: 2, scor: 999 },
        FileInfo { name: "#3", r: 5, scor: 0 },
        FileInfo {
            name: "#4",
            r: 4,
            scor: 1000,
        },
        FileInfo { name: "#5", r: 1, scor: 150 },
        FileInfo { name: "#6", r: 3, scor: 200 },
    ];
    lv.add_items(students);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_duration_formater_renderer() {
    struct ItemInfo {
        name: &'static str,
        d: Duration,
    }
    impl listview::ListItem for ItemInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Duration(self.d, listview::DurationFormat::Auto)),
                2 => Some(listview::RenderMethod::Duration(self.d, listview::DurationFormat::Details)),
                3 => Some(listview::RenderMethod::Duration(self.d, listview::DurationFormat::Seconds)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1..=4 => self.d.cmp(&other.d),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x920AA22037535EF1)
    ";
    let mut a = App::debug(80, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("ItemInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{Auto,15,r},{Detailes,20,r},{Seconds,15,r}]");

    let items = vec![
        ItemInfo {
            name: "#1",
            d: Duration::seconds(6),
        },
        ItemInfo {
            name: "#2",
            d: Duration::seconds(15),
        },
        ItemInfo {
            name: "#3",
            d: Duration::seconds(123),
        },
        ItemInfo {
            name: "#4",
            d: Duration::minutes(42) + Duration::seconds(12),
        },
        ItemInfo {
            name: "#5",
            d: Duration::minutes(2),
        },
        ItemInfo {
            name: "#6",
            d: Duration::hours(1) + Duration::minutes(42) + Duration::seconds(12),
        },
        ItemInfo {
            name: "#7",
            d: Duration::hours(19) + Duration::minutes(42) + Duration::seconds(12),
        },
        ItemInfo {
            name: "#8",
            d: Duration::days(5) + Duration::hours(1) + Duration::minutes(42) + Duration::seconds(12),
        },
        ItemInfo {
            name: "#9",
            d: Duration::days(123) + Duration::hours(15) + Duration::minutes(42) + Duration::seconds(12),
        },
    ];
    lv.add_items(items);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_custom_attribute_for_items_3_columns_view() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xCC70B68242ACB2DD)
    ";
    let mut a = App::debug(80, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("ColorInfo,d:c,view:Columns(3),flags:ScrollBars+SearchBar+CheckBoxes,columns=[{&Name,15,Left},{Value,15,r},{Bool,7,r}]");
    ColorInfo::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_custom_attribute_for_items_detail_view() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xAC7AF095CF9BFFC1)
    ";
    let mut a = App::debug(80, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("ColorInfo,d:c,flags:ScrollBars+SearchBar+CheckBoxes,columns=[{&Name,30,Left},{Value,15,r},{Bool,7,r}]");
    ColorInfo::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_currency_formater_renderer() {
    struct ItemInfo {
        name: &'static str,
        sum: f64,
    }
    impl listview::ListItem for ItemInfo {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Ascii(self.name)),
                1 => Some(listview::RenderMethod::Currency(self.sum, listview::CurrencyFormat::USDSymbol)),
                2 => Some(listview::RenderMethod::Currency(self.sum, listview::CurrencyFormat::USD)),
                3 => Some(listview::RenderMethod::Currency(self.sum, listview::CurrencyFormat::EUR)),
                4 => Some(listview::RenderMethod::Currency(self.sum, listview::CurrencyFormat::EURSymbol)),
                5 => Some(listview::RenderMethod::Currency(self.sum, listview::CurrencyFormat::Bitcoin)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x88CAA81438278221)
    ";
    let mut a = App::debug(80, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("ItemInfo,d:c,flags:ScrollBars+SearchBar,columns=[{&Name,8,Left},{USD,15,r},{USD,10,r},{EUR,15,r},{EUR,15,r},{BTC,15,r}]");

    let items = vec![
        ItemInfo { name: "#1", sum: 0.0 },
        ItemInfo { name: "#2", sum: 123.45 },
        ItemInfo { name: "#3", sum: 12345.67 },
        ItemInfo { name: "#4", sum: 123456.78 },
        ItemInfo { name: "#5", sum: 1234567.89 },
        ItemInfo {
            name: "#6",
            sum: 12345678.90,
        },
        ItemInfo {
            name: "#7",
            sum: 123456789.01,
        },
        ItemInfo {
            name: "#8",
            sum: 1234567890.12,
        },
        ItemInfo {
            name: "#9",
            sum: 12345678901.23,
        },
    ];
    lv.add_items(items);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_one_frozen_columns() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x22C1646F5C353990)
        Key.Pressed(Right,3) 
        Paint('2. Scroll to the right (+3)')
        CheckHash(0x50885C1E81B85F41)
        Key.Pressed(Right,3) 
        Paint('3. Scroll to the right (+6)')
        CheckHash(0xFF41F67BA5B335E3)
        Key.Pressed(Right,5) 
        Paint('4. Scroll to the right (+11)')
        CheckHash(0xED73BC0A861B2DCF)
        Key.Pressed(Right,5) 
        Paint('5. Scroll to the right (+16)')
        CheckHash(0xE9B89F0C42BBFEF)
    ";
    let mut a = App::debug(30, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+CheckBoxes+SearchBar,columns=[{&Name,15,Left},{&Size,6,Right},{&City,20,Center}]");
    lv.set_frozen_columns(1);
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_two_frozen_columns() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x1BB3696F21FF26F)
        Key.Pressed(Right,3) 
        Paint('2. Scroll to the right (+3)')
        CheckHash(0x6018648802FBF734)   
        Key.Pressed(Right,3) 
        Paint('3. Scroll to the right (+6)')
        CheckHash(0xB306B4AAFE5EAD3A)
        Key.Pressed(Right,5) 
        Paint('4. Scroll to the right (+11)')
        CheckHash(0x73704811ECA7AEAD)
        Key.Pressed(Right,5) 
        Paint('5. Scroll remains the same')
        CheckHash(0x73704811ECA7AEAD)
    ";
    let mut a = App::debug(35, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+CheckBoxes+SearchBar,columns=[{&Name,15,Left},{&Size,6,Right},{&City,20,Center}]");
    lv.set_frozen_columns(2);
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_folding_and_filtering() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x9E3C11ECF6744DC9)
        Mouse.Click(2,2,left)
        Paint('2. USA group folded (has 3 items)')
        CheckHash(0x7B7A27AC3DA2D909)
        Key.TypeText('5')
        Paint('3. USA group folded (now with 2 items)')
        CheckHash(0x8C68CEC5F34F1974)
        Mouse.Click(5,2,left)
        Paint('4. USA group checked (all 2 items) and folded')
        CheckHash(0x50838713A6A6E95C)
        Mouse.Click(2,2,left)
        Paint('5. USA group un-folded and checked (has 2 items - cheked)')
        CheckHash(0x6E8813699531D03E)
        Key.Pressed(Backspace)
        Paint('6. USA group un-folded and partially checked, (3 items in total / 2 checked)')
        CheckHash(0x7CDF4998CFDC03A2)
        Key.TypeText('5')
        Paint('7. USA group not-folded but fully checked (with 2 items - both checked)')
        CheckHash(0x74242E241371D8E1)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+CheckBoxes+SearchBar+ShowGroups,columns=[{&Name,15,Left},{&Size,6,Right},{&City,20,Center}]");
    lv.set_frozen_columns(2);
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_filtering_and_select_while_folded() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x9E3C11ECF6744DC9)
        Mouse.Click(2,2,left)
        Paint('2. USA group folded (has 3 items)')
        CheckHash(0x7B7A27AC3DA2D909)
        Key.TypeText('5')
        Paint('3. USA group folded (now with 2 items)')
        CheckHash(0x8C68CEC5F34F1974)
        Mouse.Click(5,2,left)
        Paint('4. USA group folded and checked (has 2 items - cheked)')
        CheckHash(0x50838713A6A6E95C)
        Mouse.Click(2,2,left)
        Paint('5. USA group un-folded checked, (2 items in total / 2 checked)')
        CheckHash(0x6E8813699531D03E)
        Key.Pressed(Backspace)
        Paint('6. USA group un-folded and partially checked, (3 items in total / 2 checked)')
        CheckHash(0x7CDF4998CFDC03A2)
        Key.TypeText('5')
        Paint('7. USA group not-folded but fully checked (with 2 items - both checked)')
        CheckHash(0x74242E241371D8E1)
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+CheckBoxes+SearchBar+ShowGroups,columns=[{&Name,15,Left},{&Size,6,Right},{&City,20,Center}]");
    lv.set_frozen_columns(2);
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_wheel_details() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x936838B8F2CC884C)
        Mouse.Wheel(20,5,down,1)
        Paint('2. MOve scroll down +1 (starts with John)')
        CheckHash(0x55826CA1E6C2BD04)
        Mouse.Wheel(20,5,down,3)
        Paint('3. MOve scroll down +4 (starts with Europe)')
        CheckHash(0x27E6B994F8F94109)
        Mouse.Wheel(20,5,up,2)
        Paint('4. MOve scroll up +2 (starts with Mike)')
        CheckHash(0x7431930EE340A2A5)
        Mouse.Wheel(20,5,right,2)
        Paint('5. MOve scroll right +2')
        CheckHash(0x86C5AF4111531E4E)
        Mouse.Wheel(20,5,right,4)
        Paint('6. MOve scroll right +4')
        CheckHash(0x8272911FBFE6BE84)
        Mouse.Wheel(20,5,left,2)
        Paint('7. MOve scroll left +2')
        CheckHash(0x9B2A2A7535388CE)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("Person,d:c,flags:ScrollBars+CheckBoxes+SearchBar+ShowGroups,columns=[{&Name,15,Left},{&Size,6,Right},{&City,20,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_wheel_columns_3() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x55D64E63B3A6C855)
        Mouse.Wheel(20,5,right,2)
        Paint('2. Move scroll right +2 (on Andrei)')
        CheckHash(0x8F1B6B93F51BA08E)
        Mouse.Wheel(20,5,right,1)
        Paint('3. Move scroll right +1 (on Teodor - last)')
        CheckHash(0x95AA6943E1770B3E)
        Mouse.Wheel(20,5,left,2)
        Paint('4. Move scroll left +2 (on Etiene)')
        CheckHash(0xB2BBCFA0C87610BA)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!(
        "Person,d:c,view: Columns(3),flags:ScrollBars+CheckBoxes+SearchBar+ShowGroups,columns=[{&Name,15,Left},{&Size,6,Right},{&City,20,Center}]"
    );
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_dynamically_change_view() {
    struct GreekLetter {
        name: &'static str,
        description: &'static str,
    }
    impl listview::ListItem for GreekLetter {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Text(self.name)),
                1 => Some(listview::RenderMethod::Text(self.description)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1 => self.description.cmp(other.description),
                _ => std::cmp::Ordering::Equal,
            }
        }
    }
    #[Window(events = RadioBoxEvents, internal = true)]
    struct Win {
        rb_detailed: Handle<RadioBox>,
        rb_columns_2: Handle<RadioBox>,
        rb_columns_3: Handle<RadioBox>,
        lv: Handle<ListView<GreekLetter>>,
    }

    impl Win {
        fn new() -> Self {
            let mut me = Self {
                base: window!("'Greek Letters',d:c,w:70,h:15,flags: Sizeable"),
                rb_detailed: Handle::None,
                rb_columns_2: Handle::None,
                rb_columns_3: Handle::None,
                lv: Handle::None,
            };
            let mut lv = listview!(
                "class: GreekLetter,l:0,t:5,r:0,b:0,flags: ScrollBars+SearchBar+LargeIcons+CheckBoxes, columns:[{&Name,15,l},{&Description,80,l}]"
            );
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Alpha",
                    description: "The first letter of the Greek alphabet, used to denote the beginning of something.",
                },
                false,
                None,
                ['α', 'Α'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Beta",
                    description: "The second letter of the Greek alphabet, used to denote the second element in a sequence.",
                },
                false,
                None,
                ['β', 'Β'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Gamma",
                    description: "The third letter of the Greek alphabet, used to denote the third element in a sequence.",
                },
                false,
                None,
                ['γ', 'Γ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Delta",
                    description: "The fourth letter of the Greek alphabet, used to denote the fourth element in a sequence.",
                },
                false,
                None,
                ['δ', 'Δ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Epsilon",
                    description: "The fifth letter of the Greek alphabet, used to denote the fifth element in a sequence.",
                },
                false,
                None,
                ['ε', 'Ε'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Zeta",
                    description: "The sixth letter of the Greek alphabet, used to denote the sixth element in a sequence.",
                },
                false,
                None,
                ['ζ', 'Ζ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Eta",
                    description: "The seventh letter of the Greek alphabet, used to denote the seventh element in a sequence.",
                },
                false,
                None,
                ['η', 'Η'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Theta",
                    description: "The eighth letter of the Greek alphabet, used to denote the eighth element in a sequence.",
                },
                false,
                None,
                ['θ', 'Θ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Iota",
                    description: "The ninth letter of the Greek alphabet, used to denote the ninth element in a sequence.",
                },
                false,
                None,
                ['ι', 'Ι'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Kappa",
                    description: "The tenth letter of the Greek alphabet, used to denote the tenth element in a sequence.",
                },
                false,
                None,
                ['κ', 'Κ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Lambda",
                    description: "The eleventh letter of the Greek alphabet, used to denote the eleventh element in a sequence.",
                },
                false,
                None,
                ['λ', 'Λ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Mu",
                    description: "The twelfth letter of the Greek alphabet, used to denote the twelfth element in a sequence.",
                },
                false,
                None,
                ['μ', 'Μ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Nu",
                    description: "The thirteenth letter of the Greek alphabet, used to denote the thirteenth element in a sequence.",
                },
                false,
                None,
                ['ν', 'Ν'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Xi",
                    description: "The fourteenth letter of the Greek alphabet, used to denote the fourteenth element in a sequence.",
                },
                false,
                None,
                ['ξ', 'Ξ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Omicron",
                    description: "The fifteenth letter of the Greek alphabet, used to denote the fifteenth element in a sequence.",
                },
                false,
                None,
                ['ο', 'Ο'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Pi",
                    description: "The sixteenth letter of the Greek alphabet, used to denote the sixteenth element in a sequence.",
                },
                false,
                None,
                ['π', 'Π'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Rho",
                    description: "The seventeenth letter of the Greek alphabet, used to denote the seventeenth element in a sequence.",
                },
                false,
                None,
                ['ρ', 'Ρ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Sigma",
                    description: "The eighteenth letter of the Greek alphabet, used to denote the eighteenth element in a sequence.",
                },
                false,
                None,
                ['σ', 'Σ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Tau",
                    description: "The nineteenth letter of the Greek alphabet, used to denote the nineteenth element in a sequence.",
                },
                false,
                None,
                ['τ', 'Τ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Upsilon",
                    description: "The twentieth letter of the Greek alphabet, used to denote the twentieth element in a sequence.",
                },
                false,
                None,
                ['υ', 'Υ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Phi",
                    description: "The twenty-first letter of the Greek alphabet, used to denote the twenty-first element in a sequence.",
                },
                false,
                None,
                ['φ', 'Φ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Chi",
                    description: "The twenty-second letter of the Greek alphabet, used to denote the twenty-second element in a sequence.",
                },
                false,
                None,
                ['χ', 'Χ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Psi",
                    description: "The twenty-third letter of the Greek alphabet, used to denote the twenty-third element in a sequence.",
                },
                false,
                None,
                ['ψ', 'Ψ'],
                listview::Group::None,
            ));
            lv.add_item(listview::Item::new(
                GreekLetter {
                    name: "Omega",
                    description: "The twenty-fourth letter of the Greek alphabet, used to denote the twenty-fourth element in a sequence.",
                },
                false,
                None,
                ['ω', 'Ω'],
                listview::Group::None,
            ));
            me.add(hline!("l:0,t:4,r:0"));
            me.rb_detailed = me.add(radiobox!("'&Detailed view',x:1,y:1,w:20,selected: true"));
            me.rb_columns_2 = me.add(radiobox!("'&2 columns view',x:1,y:2,w:20,selected: false"));
            me.rb_columns_3 = me.add(radiobox!("'&3 columns view',x:1,y:3,w:20, selected: false"));
            me.lv = me.add(lv);
            me
        }
    }

    impl RadioBoxEvents for Win {
        fn on_selected(&mut self, handle: Handle<RadioBox>) -> EventProcessStatus {
            let lvh = self.lv;
            let d = self.rb_detailed;
            let c_2 = self.rb_columns_2;
            let c_3 = self.rb_columns_3;
            if let Some(lv) = self.control_mut(lvh) {
                if handle == d {
                    lv.set_view_mode(listview::ViewMode::Details);
                } else if handle == c_2 {
                    lv.set_view_mode(listview::ViewMode::Columns(2));
                } else if handle == c_3 {
                    lv.set_view_mode(listview::ViewMode::Columns(3));
                }
                EventProcessStatus::Processed
            } else {
                EventProcessStatus::Ignored
            }
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xCDB1162625E29682)
        Key.Pressed(Down,4)
        Key.Pressed(Space)
        Paint('2. Cursor on Epsilon (checked)')
        CheckHash(0x431CC067276F0972)
        Key.Pressed(Alt+2)
        Key.Pressed(Tab,2)
        Paint('3. Cursor on Epsilon(checked), View: 2 columns')
        CheckHash(0x72D8AF5482A2ABAD)
        Key.Pressed(Alt+3)
        Key.Pressed(Tab)
        Paint('4. Cursor on Epsilon(checked), View: 3 columns')
        CheckHash(0x5B6FBE546286070A)
        Key.Pressed(Insert,4)
        Key.Pressed(Alt+D)
        Key.Pressed(Tab,3)
        Paint('5. Cursor on Iota, View: Details [Zeta,Eta and Theta are checked]')
        CheckHash(0xF63284DD9388073E)
    ";
    let mut a = App::debug(80, 20, script).build().unwrap();
    a.add_window(Win::new());
    a.run();
}

#[test]
fn check_item_custom_paint() {
    struct MyItem {
        name: &'static str,
        size: u32,
        city: &'static str,
    }
    impl listview::ListItem for MyItem {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Text(self.name)),
                1 => Some(listview::RenderMethod::Custom),
                2 => Some(listview::RenderMethod::Text(self.city)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.name.cmp(other.name),
                1 => self.size.cmp(&other.size),
                2 => self.city.cmp(other.city),
                _ => std::cmp::Ordering::Equal,
            }
        }

        fn paint(&self, column_index: u32, width: u16, surface: &mut Surface, _theme: &Theme, attr: Option<CharAttribute>) {
            if column_index != 1 {
                return;
            }
            let attr = attr.unwrap_or(CharAttribute::with_color(Color::White, Color::Black));
            surface.fill_horizontal_line_with_size(0, 0, self.size, Character::with_attributes('+', attr));
            surface.write_string(0, 0, format!("{width}").as_str(), attr, false);
            surface.write_char(width as i32 - 1, 0, char!(">,red,black"));
        }
    }
    let script = "
        Paint.Enable(false)
        //|10++++   >|
        // width should be 10
        Paint('1. Initial state')
        CheckHash(0x29F74CEB38FFCAF9)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("MyItem,d:c,flags:ScrollBars+CheckBoxes+SearchBar,columns=[{&Name,15,Left},{Value,10,Right},{&City,20,Center}]");
    lv.add(MyItem {
        name: "John",
        size: 5,
        city: "New York",
    });
    lv.add(MyItem {
        name: "Mike",
        size: 4,
        city: "Los Angeles",
    });
    lv.add(MyItem {
        name: "Etiene",
        size: 2,
        city: "Chicago",
    });
    lv.add(MyItem {
        name: "Andrei",
        size: 6,
        city: "San Francisco",
    });
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_custom_paint_first_column_details() {
    struct MyItem {
        name: &'static str,
        size: u32,
        city: &'static str,
    }
    impl listview::ListItem for MyItem {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Custom),
                1 => Some(listview::RenderMethod::Text(self.name)),
                2 => Some(listview::RenderMethod::Text(self.city)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.size.cmp(&other.size),
                1 => self.name.cmp(other.name),
                2 => self.city.cmp(other.city),
                _ => std::cmp::Ordering::Equal,
            }
        }

        fn paint(&self, column_index: u32, width: u16, surface: &mut Surface, _theme: &Theme, attr: Option<CharAttribute>) {
            if column_index != 0 {
                return;
            }
            let attr = attr.unwrap_or(CharAttribute::with_color(Color::White, Color::Black));
            surface.fill_horizontal_line_with_size(0, 0, self.size, Character::with_attributes('+', attr));
            surface.write_string(0, 0, format!("{width}").as_str(), attr, false);
            surface.write_char(width as i32 - 1, 0, char!(">,red,black"));
        }
    }
    let script = "
        //Paint.Enable(false)
        //│x 13+++       >│
        // 123456789012345
        // the width should be 13 (as we have the 'x ' prefix)
        Paint('1. Initial state')
        CheckHash(0x45E307F308558E86)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv = listview!("MyItem,d:c,flags:ScrollBars+CheckBoxes+SearchBar,columns=[{&Name,15,Left},{Value,10,Right},{&City,20,Center}]");
    lv.add(MyItem {
        name: "John",
        size: 5,
        city: "New York",
    });
    lv.add(MyItem {
        name: "Mike",
        size: 7,
        city: "Los Angeles",
    });
    lv.add(MyItem {
        name: "Etiene",
        size: 8,
        city: "Chicago",
    });
    lv.add(MyItem {
        name: "Andrei",
        size: 6,
        city: "San Francisco",
    });
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_item_custom_paint_first_column_columns_3() {
    struct MyItem {
        name: &'static str,
        size: u32,
        city: &'static str,
    }
    impl listview::ListItem for MyItem {
        fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
            match column_index {
                0 => Some(listview::RenderMethod::Custom),
                1 => Some(listview::RenderMethod::Text(self.name)),
                2 => Some(listview::RenderMethod::Text(self.city)),
                _ => None,
            }
        }

        fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
            match column_index {
                0 => self.size.cmp(&other.size),
                1 => self.name.cmp(other.name),
                2 => self.city.cmp(other.city),
                _ => std::cmp::Ordering::Equal,
            }
        }

        fn paint(&self, column_index: u32, width: u16, surface: &mut Surface, _theme: &Theme, attr: Option<CharAttribute>) {
            if column_index != 0 {
                return;
            }
            let attr = attr.unwrap_or(CharAttribute::with_color(Color::White, Color::Black));
            surface.fill_horizontal_line_with_size(0, 0, self.size, Character::with_attributes('+', attr));
            surface.write_string(0, 0, format!("{width}").as_str(), attr, false);
            surface.write_char(width as i32 - 1, 0, char!(">,red,black"));
        }
    }
    let script = "
        Paint.Enable(false)
        //│x 10++++   >│
        // the width should be 10 (12 if we count the 'x ' prefix)
        Paint('1. Initial state')
        CheckHash(0x99664F10C2C18948)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut lv =
        listview!("MyItem,d:c,view:Columns(3),flags:ScrollBars+CheckBoxes+SearchBar,columns=[{&Name,15,Left},{Value,10,Right},{&City,20,Center}]");
    lv.add(MyItem {
        name: "John",
        size: 5,
        city: "New York",
    });
    lv.add(MyItem {
        name: "Mike",
        size: 7,
        city: "Los Angeles",
    });
    lv.add(MyItem {
        name: "Etiene",
        size: 8,
        city: "Chicago",
    });
    lv.add(MyItem {
        name: "Andrei",
        size: 6,
        city: "San Francisco",
    });
    w.add(lv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_proc_macro_listviewitem() {
    #[derive(ListItem)]
    struct DownloadItem {
        #[Column(name: "&Name", width: 12, align: Left)]
        name: &'static str,
        #[Column(name: "&Age", width: 10, align: Center)]
        age: u32,
        #[Column(name: "&Server")]
        server: &'static str,
        #[Column(name: "&Stars", width: 10, align: Center, render: Rating, format:Stars)]
        stars: u8,
        #[Column(name: "Download", width:15)]
        download: listitem::Status,
        #[Column(name: "Created", w: 20, align: Center, render: DateTime, format: Short)]
        created: chrono::NaiveDateTime,
        #[Column(name: "Enabled", align: Center)]
        enabled: bool,
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x3F9D82EE992A190)
    ";
    let mut a = App::debug(100, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut l = listview!("DownloadItem,d:c,view:Details,flags: ScrollBars+CheckBoxes");
    l.add(DownloadItem {
        name: "John.mp3",
        age: 21,
        server: "London",
        stars: 4,
        download: listview::Status::Running(0.5),
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: true,
    });
    l.add(DownloadItem {
        name: "Mihai.png",
        age: 30,
        server: "Bucharest",
        stars: 3,
        download: listview::Status::Paused(0.25),
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: false,
    });
    l.add(DownloadItem {
        name: "Ion.exe",
        age: 40,
        server: "Bucharest",
        stars: 5,
        download: listitem::Status::Completed,
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: true,
    });
    w.add(l);
    a.add_window(w);
    a.run();
}


#[test]
fn check_proc_macro_listviewitem_order_from_1() {
    #[derive(ListItem)]
    struct DownloadItem {
        #[Column(name: "&Name", width: 12, align: Left, index: 2)]
        name: &'static str,
        #[Column(name: "&Age", width: 10, align: Center, index:4)]
        age: u32,
        #[Column(name: "&Server", idx: 1)]
        server: &'static str,
        #[Column(name: "&Stars", width: 10, align: Center, render: Rating, format:Stars, idx: 3)]
        stars: u8,
        #[Column(name: "Download", width:15, idx: 7)]
        download: listitem::Status,
        #[Column(name: "Created", w: 20, align: Center, render: DateTime, format: Short, index: 6)]
        created: chrono::NaiveDateTime,
        #[Column(name: "Enabled", align: Center, index: 5)]
        enabled: bool,
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x232187AB9FA9838)
    ";
    let mut a = App::debug(100, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut l = listview!("DownloadItem,d:c,view:Details,flags: ScrollBars+CheckBoxes");
    l.add(DownloadItem {
        name: "John.mp3",
        age: 21,
        server: "London",
        stars: 4,
        download: listview::Status::Running(0.5),
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: true,
    });
    l.add(DownloadItem {
        name: "Mihai.png",
        age: 30,
        server: "Bucharest",
        stars: 3,
        download: listview::Status::Paused(0.25),
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: false,
    });
    l.add(DownloadItem {
        name: "Ion.exe",
        age: 40,
        server: "Bucharest",
        stars: 5,
        download: listview::Status::Completed,
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: true,
    });
    w.add(l);
    a.add_window(w);
    a.run();
}

#[test]
fn check_proc_macro_listviewitem_order_from_0() {
    #[derive(ListItem)]
    struct DownloadItem {
        #[Column(name: "&Name", width: 12, align: Left, index: 1)]
        name: &'static str,
        #[Column(name: "&Age", width: 10, align: Center, index:3)]
        age: u32,
        #[Column(name: "&Server", idx: 0)]
        server: &'static str,
        #[Column(name: "&Stars", width: 10, align: Center, render: Rating, format:Stars, idx: 2)]
        stars: u8,
        #[Column(name: "Download", width:15, idx: 6)]
        download: listitem::Status,
        #[Column(name: "Created", w: 20, align: Center, render: DateTime, format: Short, index: 5)]
        created: chrono::NaiveDateTime,
        #[Column(name: "Enabled", align: Center, index: 4)]
        enabled: bool,
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x232187AB9FA9838)
    ";
    let mut a = App::debug(100, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut l = listview!("DownloadItem,d:c,view:Details,flags: ScrollBars+CheckBoxes");
    l.add(DownloadItem {
        name: "John.mp3",
        age: 21,
        server: "London",
        stars: 4,
        download: listview::Status::Running(0.5),
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: true,
    });
    l.add(DownloadItem {
        name: "Mihai.png",
        age: 30,
        server: "Bucharest",
        stars: 3,
        download: listview::Status::Paused(0.25),
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: false,
    });
    l.add(DownloadItem {
        name: "Ion.exe",
        age: 40,
        server: "Bucharest",
        stars: 5,
        download: listview::Status::Completed,
        created: chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap(),
        enabled: true,
    });
    w.add(l);
    a.add_window(w);
    a.run();
}


#[test]
fn check_select_item_method() {
    #[Window(events=ListViewEvents<Person>, internal: true)]
    struct MyWin {}
    impl ListViewEvents<Person> for MyWin {
        fn on_item_action(&mut self, handle: Handle<ListView<Person>>, index: usize) -> EventProcessStatus {
            let txt = if let Some(lv) = self.control_mut(handle) {
                let status = lv.is_item_selected(index);
                lv.select_item(index, !status);
                let count = lv.items_count();
                let count_selected = lv.selected_items_count();
                format!("{}/{}", count_selected, count)
            } else {
                "?".to_string()
            };
            self.set_title(&txt);
            EventProcessStatus::Processed
        }
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
            };
            let mut lv = listview!(
                "Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+ShowGroups+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]"
            );
            Person::populate(&mut lv);
            w.add(lv);
            w
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x39603D1E8E3FDC2E)  
        Key.Pressed(Down,2)
        Key.Pressed(Enter)       
        Paint('2. Title: idx: 1/19 -> cursor at: Mike')
        CheckHash(0x2A9E3C47926E86F1)  
        Mouse.DoubleClick(7,7,left)
        Paint('3. Title: idx: 2/19 -> cursor at: Etiene')
        CheckHash(0xA68B0D8A9786CC61)  
        Mouse.DoubleClick(30,2,left)
        Paint('4. Asia group folded')
        CheckHash(0x7385F8EC17FB9F6D)
        Key.Pressed(Enter)       
        Paint('5. Asia group un-folded')
        CheckHash(0xF8048EA09FD73499)          
        Mouse.DoubleClick(41,1,left)
        Paint('6. Title: 3/12 -> cursor at: Andrei')
        CheckHash(0xF2BAC8A723FBC04)          
        Mouse.DoubleClick(41,1,left)
        Paint('7. Title: 2/12 -> cursor at: Andrei')
        CheckHash(0x59B04A60BB56A8C)          
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}


#[test]
fn check_clear_method() {
    #[Window(events=ListViewEvents<Person>, internal: true)]
    struct MyWin {}
    impl ListViewEvents<Person> for MyWin {
        fn on_item_action(&mut self, handle: Handle<ListView<Person>>, _index: usize) -> EventProcessStatus {
            if let Some(lv) = self.control_mut(handle) {
                lv.clear();
            }
            EventProcessStatus::Processed
        }
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
            };
            let mut lv = listview!(
                "Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+ShowGroups+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]"
            );
            Person::populate(&mut lv);
            w.add(lv);
            w
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x39603D1E8E3FDC2E)  
        Key.Pressed(Down,6)
        Key.Pressed(Enter)       
        Paint('2. Elements are cleared')
        CheckHash(0xFFF6487CAF22C71F)  
        Key.Pressed(Down,6)
        Key.Pressed(Up,2)
        Key.Pressed(Home)
        Key.Pressed(End)
        Key.Pressed(Left,3)
        Key.Pressed(Down,2)
        Paint('3. Nothing changes')
        CheckHash(0xFFF6487CAF22C71F)  
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}


#[test]
fn check_clear_search_method() {
    #[Window(events=ListViewEvents<Person>, internal: true)]
    struct MyWin {}
    impl ListViewEvents<Person> for MyWin {
        fn on_item_action(&mut self, handle: Handle<ListView<Person>>, _index: usize) -> EventProcessStatus {
            if let Some(lv) = self.control_mut(handle) {
                lv.clear_search();
            }
            EventProcessStatus::Processed
        }
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,d:c,w:100%,h:100%,flags: Sizeable"),
            };
            let mut lv = listview!(
                "Person,d:c,view:Columns(3),flags:ScrollBars+SearchBar+ShowGroups+CheckBoxes,columns=[{&Name,5,Left},{&Size,5,Right},{&City,5,Center}]"
            );
            Person::populate(&mut lv);
            w.add(lv);
            w
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x39603D1E8E3FDC2E)  
        Key.TypeText('e')     
        Paint('2. Filter based on `e` character -> cursor on Mike')
        CheckHash(0x61CC5A5FBEAF5C39)  
        Key.Pressed(Enter)
        Paint('3. Search cleared - all items restored -> cursor on Mike')
        CheckHash(0x6C49F7AAE9F24E9C)  
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_no_selection_mode() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state (scroll starts from John,cursor on John)')
        CheckHash(0xF73F60131F7F0467)
        Key.Pressed(Insert,2)
        Paint('2. Nothing select (cursor at ...)')
        CheckHash(0x97CFE2808BE3E483)
        Key.Pressed(Shift+Down)
        Paint('3. Nothing select (cursor at ...)')
        CheckHash(0x3AF8F1EBE5EB2013)
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = window!("Test,d:c,w:40,h:9,flags: Sizeable");
    let mut lv = listview!("Person,d:c,view:Details,flags: ScrollBars+NoSelection,columns=[{&Name,10,Left},{&Age,10,Right},{&City,10,Center}]");
    Person::populate(&mut lv);
    w.add(lv);
    a.add_window(w);
    a.run();
}


#[test]
fn check_column_methods() {
    #[derive(ListItem)]
    struct DownloadItem {
        #[Column(name: "&Name", width: 12, align: Left, index: 1)]
        name: &'static str,
        #[Column(name: "&Age", width: 10, align: Center, index:3)]
        age: u32,
        #[Column(name: "&Server", idx: 0)]
        server: &'static str,
        #[Column(name: "&Stars", width: 10, align: Center, render: Rating, format:Stars, idx: 2)]
        stars: u8,
        #[Column(name: "Download", width:15, idx: 6)]
        download: listitem::Status,
        #[Column(name: "Created", w: 20, align: Center, render: DateTime, format: Short, index: 5)]
        created: chrono::NaiveDateTime,
        #[Column(name: "Enabled", align: Center, index: 4)]
        enabled: bool,
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6963EDDD254CC945)
    ";
    let mut a = App::debug(100, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut l = listview!("DownloadItem,d:c,view:Details,flags: ScrollBars+CheckBoxes");
    assert_eq!(l.column(0).unwrap().name(),"Server");
    assert_eq!(l.column(1).unwrap().name(),"Name");
    assert_eq!(l.column(3).unwrap().name(),"Age");
    assert_eq!(l.column(2).unwrap().name(),"Stars");
    assert_eq!(l.column(7), None);
    assert_eq!(l.column(1).unwrap().width(), 12);
    assert_eq!(l.column(4).unwrap().alignment(), TextAlignament::Center);
    assert_eq!(l.column(1).unwrap().tooltip(), "");
    // set a new name for the first column
    l.column_mut(0).unwrap().set_name("Server2");
    assert_eq!(l.column(0).unwrap().name(),"Server2");
    // set a new tooltip for the first column
    l.column_mut(0).unwrap().set_tooltip("Server2 tooltip");
    assert_eq!(l.column(0).unwrap().tooltip(),"Server2 tooltip");
    // set right alignment for column with index 6
    l.column_mut(6).unwrap().set_alignment(TextAlignament::Right);
    assert_eq!(l.column(6).unwrap().alignment(), TextAlignament::Right);
    w.add(l);
    a.add_window(w);
    a.run();
}

#[test]
fn check_autoresize_temperature() {
    #[derive(ListItem)]
    struct MyItem {
        #[Column(name: "V1", width: 3, align: Left, render: temperature, format: Celsius)]
        v1: f32,
        #[Column(name: "V2", width: 3, align: Left, render: temperature, format: Fahrenheit)]
        v2: f32,
        #[Column(name: "V3", width: 3, align: Left, render: temperature, format: Kelvin)]
        v3: f32,
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x1577B8621F47427A)
        Mouse.DoubleClick(12,1,left)
        Paint('2. v3 - auto resized')
        CheckHash(0x42CB9B627940BB31)
        Mouse.DoubleClick(8,1,left)
        Paint('3. v2 - auto resized')
        CheckHash(0xAC616B4142E7FF0D)
        Mouse.DoubleClick(4,1,left)
        Paint('4. v1 - auto resized')
        CheckHash(0xCD289EDA906FADFE)
    ";
    let mut a = App::debug(100, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut l = listview!("MyItem,d:c,view:Details,flags: ScrollBars+CheckBoxes");
    l.add(MyItem {v1: -123.75f32, v2: 1234.625f32, v3: 12345.0625f32});
    w.add(l);
    a.add_window(w);
    a.run();
}

#[test]
fn check_autoresize_volume() {
    #[derive(ListItem)]
    struct MyItem {
        #[Column(name: "V1", width: 3, align: Left, render: Volume, format: CubicMilimeters )]
        v1: u32,
        #[Column(name: "V2", width: 3, align: Left, render: Volume, format: CubicMeters)]
        v2: u32,
        #[Column(name: "V3", width: 3, align: Left, render: Volume, format: CubicKilometers )]
        v3: u32,
        #[Column(name: "V4", width: 3, align: Left, render: Volume, format: Milliliters )]
        v4: u32,
        #[Column(name: "V5", width: 3, align: Left, render: Volume, format: CubicInches )]
        v5: u32,        
        #[Column(name: "V5", width: 3, align: Left, render: Volume, format: CubicYards )]
        v6: u32,        
        #[Column(name: "V5", width: 3, align: Left, render: Volume, format: CubicMiles )]
        v7: u32,        
    }

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x870BEA0E27BE58FD)
        Mouse.DoubleClick(28,1,left)
        Mouse.DoubleClick(24,1,left)
        Mouse.DoubleClick(20,1,left)
        Mouse.DoubleClick(16,1,left)
        Mouse.DoubleClick(12,1,left)
        Mouse.DoubleClick(8,1,left)
        Mouse.DoubleClick(4,1,left)
        Paint('2. auto resized')
        CheckHash(0x8115367D61AC3EB1)
    ";
    let mut a = App::debug(100, 10, script).build().unwrap();
    let mut w = window!("Test,d:c,w:100%,h:100%,flags: Sizeable");
    let mut l = listview!("MyItem,d:c,view:Details,flags: ScrollBars+CheckBoxes");
    l.add(MyItem {v1: 1, v2: 11, v3: 123 ,v4: 1234, v5: 12345, v6: 123456, v7: 1234567});
    w.add(l);
    a.add_window(w);
    a.run();
}