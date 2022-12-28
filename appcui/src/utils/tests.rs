use super::KeyValueParser;
use super::ValueType;

#[test]
fn check_KeyValueParser_single()
{
    let mut p = KeyValueParser::new("abc=2");
    let k = p.next().unwrap();
    assert_eq!(k.key,"abc");
    assert_eq!(k.numerical_value,2);
    assert_eq!(k.value_type,ValueType::Number);
    assert_eq!(p.next(),None);
}
#[test]
fn check_KeyValueParser_double()
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
fn check_KeyValueParser_text()
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