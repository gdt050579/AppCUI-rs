use crate::system::Handle;
use crate::system::HandleSupport;
use crate::utils::HandleManager;

use super::KeyValueParser;
use super::Strategy;
use super::ValueType;
use super::VectorIndex;

#[test]
fn check_key_value_parser_single() {
    let mut p = KeyValueParser::new("abc=2");
    let k = p.next().unwrap();
    assert_eq!(k.key, "abc");
    assert_eq!(k.numerical_value, 2);
    assert_eq!(k.value_type, ValueType::Number);
    assert_eq!(p.next(), None);
}
#[test]
fn check_key_value_parser_double() {
    let mut p = KeyValueParser::new("abc=2,xyz=10%");
    let k = p.next().unwrap();
    assert_eq!(k.key, "abc");
    assert_eq!(k.numerical_value, 2);
    assert_eq!(k.value_type, ValueType::Number);
    let k = p.next().unwrap();
    assert_eq!(k.key, "xyz");
    assert_eq!(k.numerical_value, 1000);
    assert_eq!(k.value_type, ValueType::Percentage);
    assert_eq!(p.next(), None);
}
#[test]
fn check_key_value_parser_text() {
    let mut p = KeyValueParser::new("  abc  =  2 ,  xyz=10%   , some_value : another_value   ");
    let k = p.next().unwrap();
    assert_eq!(k.key, "abc");
    assert_eq!(k.numerical_value, 2);
    assert_eq!(k.value_type, ValueType::Number);
    let k = p.next().unwrap();
    assert_eq!(k.key, "xyz");
    assert_eq!(k.numerical_value, 1000);
    assert_eq!(k.value_type, ValueType::Percentage);
    let k = p.next().unwrap();
    assert_eq!(k.key, "some_value");
    assert_eq!(k.value_type, ValueType::String);
    assert_eq!(k.value, "another_value");
    assert_eq!(p.next(), None);
}

#[test]
fn check_index() {
    let mut i = VectorIndex::First;
    assert_eq!(i.index(), 0);
    i = VectorIndex::last(6);
    assert_eq!(i.index(), 5);
    i.set(10, 5, true);
    assert_eq!(i.index(), 4);
    i.set(10, 5, false);
    assert_eq!(i.is_valid(), false);
    i = VectorIndex::with_value(3);
    assert_eq!(i.index(), 3);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(), 2);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(), 1);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(), 0);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(), 0);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(), 0);
    i = VectorIndex::with_value(3);
    i.sub(125, 10, Strategy::Clamp);
    assert_eq!(i.index(), 0);
    i = VectorIndex::with_value(3);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(), 9);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(), 5);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(), 1);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(), 7);
    i.add(1, 9, Strategy::Clamp);
    assert_eq!(i.index(), 8);
    i.add(1, 9, Strategy::Clamp);
    assert_eq!(i.index(), 8);
    i.add(100, 9, Strategy::Clamp);
    assert_eq!(i.index(), 8);
    i.add(3, 9, Strategy::Rotate);
    assert_eq!(i.index(), 2);
    i.add(3, 9, Strategy::Rotate);
    assert_eq!(i.index(), 5);
    i.add(2, 9, Strategy::Rotate);
    assert_eq!(i.index(), 7);

    i = VectorIndex::with_value(5);
    assert_eq!(i.in_range(10), true);
    assert_eq!(i.in_range(6), true);
    assert_eq!(i.in_range(5), false);
}

#[test]
fn check_hanlde_manager() {
    struct MyData {
        text: String,
        value: i32,
        handle: Handle<MyData>,
    }
    impl MyData {
        fn new(text: &str, value: i32) -> Self {
            Self {
                text: text.to_string(),
                value,
                handle: Handle::None,
            }
        }
    }
    impl HandleSupport<MyData> for MyData {
        fn get_handle(&self) -> crate::system::Handle<MyData> {
            self.handle
        }

        fn set_handle(&mut self, handle: crate::system::Handle<MyData>) {
            self.handle = handle;
        }
    }

    let mut man: HandleManager<MyData> = HandleManager::new(16);
    let h1_123 = man.add(MyData::new("handle1", 123));
    let habc_1 = man.add(MyData::new("abc", 1));
    let hgdt_2345 = man.add(MyData::new("GDT", 2345));
    assert!(man.allocated_objects() == 3);
    assert!(man.free_spaces() == 0);
    let o1 = man.get(h1_123).unwrap();
    assert!(o1.handle == h1_123);
    assert!(o1.text == "handle1");
    assert!(o1.value == 123);
    let o2 = man.get(habc_1).unwrap();
    assert!(o2.handle == habc_1);
    assert!(o2.text == "abc");
    assert!(o2.value == 1);
    let o3 = man.get(hgdt_2345).unwrap();
    assert!(o3.handle == hgdt_2345);
    assert!(o3.text == "GDT");
    assert!(o3.value == 2345);
    // delete first element
    assert!(man.remove(h1_123) == true);
    // second time it shoudl return false
    assert!(man.remove(h1_123) == false);
    assert!(man.get(h1_123).is_none());
    // check o3 again
    let o3 = man.get(hgdt_2345).unwrap();
    assert!(o3.handle == hgdt_2345);
    assert!(o3.text == "GDT");
    assert!(o3.value == 2345);
    // validate o2
    assert!(man.get(habc_1).is_some());
    // we have deleted one element it should be on the free list
    assert!(man.free_spaces() == 1);
    assert!(man.allocated_objects() == 3);
    // add a new element
    let h_new = man.add(MyData::new("new_handle", 1234));
    assert!(h_new != h1_123);
    assert!(h_new.get_index() == 0);
    assert!(h1_123.get_index() == 0);
    // no free spacess stored
    assert!(man.free_spaces() == 0);
    assert!(man.allocated_objects() == 3);
}
