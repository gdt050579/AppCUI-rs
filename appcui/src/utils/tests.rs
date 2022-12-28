use super::KeyValuePair;
use super::KeyValueParser;
use super::ValueType;

#[test]
fn check_parser_1()
{
    let mut p = KeyValueParser::new("abc=2");
    let k = p.next().unwrap();
    println!("{:?}",k);
}