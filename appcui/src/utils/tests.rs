use super::KeyValueParser;
use super::ValueType;
use super::VectorIndex;
use super::Strategy;

#[test]
fn check_key_value_parser_single()
{
    let mut p = KeyValueParser::new("abc=2");
    let k = p.next().unwrap();
    assert_eq!(k.key,"abc");
    assert_eq!(k.numerical_value,2);
    assert_eq!(k.value_type,ValueType::Number);
    assert_eq!(p.next(),None);
}
#[test]
fn check_key_value_parser_double()
{
    let mut p = KeyValueParser::new("abc=2,xyz=10%");
    let k = p.next().unwrap();
    assert_eq!(k.key,"abc");
    assert_eq!(k.numerical_value,2);
    assert_eq!(k.value_type,ValueType::Number);
    let k = p.next().unwrap();
    assert_eq!(k.key,"xyz");
    assert_eq!(k.numerical_value,1000);
    assert_eq!(k.value_type,ValueType::Percentage);    
    assert_eq!(p.next(),None);
}
#[test]
fn check_key_value_parser_text()
{
    let mut p = KeyValueParser::new("  abc  =  2 ,  xyz=10%   , some_value : another_value   ");
    let k = p.next().unwrap();
    assert_eq!(k.key,"abc");
    assert_eq!(k.numerical_value,2);
    assert_eq!(k.value_type,ValueType::Number);
    let k = p.next().unwrap();
    assert_eq!(k.key,"xyz");
    assert_eq!(k.numerical_value,1000);
    assert_eq!(k.value_type,ValueType::Percentage);    
    let k = p.next().unwrap();
    assert_eq!(k.key,"some_value");
    assert_eq!(k.value_type,ValueType::String);
    assert_eq!(k.value,"another_value");  
    assert_eq!(p.next(),None);
}

#[test]
fn check_index()
{
    let mut i = VectorIndex::first();
    assert_eq!(i.index(),0);
    i = VectorIndex::last(6);
    assert_eq!(i.index(),5);
    i.set(10,5,true);
    assert_eq!(i.index(),4);
    i.set(10,5,false);
    assert_eq!(i.is_valid(),false);
    i = VectorIndex::with_value(3);
    assert_eq!(i.index(),3);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(),2);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(),1);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(),0);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(),0);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(),0);
    i = VectorIndex::with_value(3);
    i.sub(125, 10, Strategy::Clamp);
    assert_eq!(i.index(),0);
    i = VectorIndex::with_value(3);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(),9);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(),5);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(),1);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(),7);
    i.add(1,9,Strategy::Clamp);
    assert_eq!(i.index(),8);
    i.add(1,9,Strategy::Clamp);
    assert_eq!(i.index(),8);
    i.add(100,9,Strategy::Clamp);
    assert_eq!(i.index(),8);
    i.add(3,9,Strategy::Rotate);
    assert_eq!(i.index(),2);
    i.add(3,9,Strategy::Rotate);
    assert_eq!(i.index(),5);
    i.add(2,9,Strategy::Rotate);
    assert_eq!(i.index(),7);

    i = VectorIndex::with_value(5);
    assert_eq!(i.in_range(10),true);
    assert_eq!(i.in_range(6),true);
    assert_eq!(i.in_range(5),false);
    assert_eq!(i.in_range(usize::MAX),false);


}