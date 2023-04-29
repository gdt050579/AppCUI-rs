use super::command_parser::CommandParser;


#[test]
fn check_command_parser_simple() {
    let cp = CommandParser::new("run(1,2)").unwrap();
    assert_eq!(cp.get_command(),"run");
    assert_eq!(cp.get_params_count(),2);
    assert_eq!(cp.get_param(0),Some("1"));
    assert_eq!(cp.get_param(1),Some("2"));
}
#[test]
fn check_command_parser_spaced() {
    let cp = CommandParser::new("  run     (    1  ,   left  ,   -200   )    ").unwrap();
    assert_eq!(cp.get_command(),"run");
    assert_eq!(cp.get_params_count(),3);
    assert_eq!(cp.get_param(0),Some("1"));
    assert_eq!(cp.get_param(1),Some("left"));
    assert_eq!(cp.get_param(2),Some("-200"));
}
#[test]
fn check_command_parser_string() {
    let cp = CommandParser::new("  run     (    'some string '  ,   left  ,   -200   )    ").unwrap();
    assert_eq!(cp.get_command(),"run");
    assert_eq!(cp.get_params_count(),3);
    assert_eq!(cp.get_param(0),Some("some string "));
    assert_eq!(cp.get_param(1),Some("left"));
    assert_eq!(cp.get_param(2),Some("-200"));
}