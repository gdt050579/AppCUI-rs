use super::command_parser::CommandParser;

#[test]
fn check_command_parser_simple() {
    let cp = CommandParser::new("run(1,2)").unwrap();
    assert_eq!(cp.get_command(), "run");
    assert_eq!(cp.get_params_count(), 2);
    assert_eq!(cp.get_param(0), Some("1"));
    assert_eq!(cp.get_param(1), Some("2"));
}
#[test]
fn check_command_parser_spaced() {
    let cp = CommandParser::new("  run     (    1  ,   left  ,   -200   )    ").unwrap();
    assert_eq!(cp.get_command(), "run");
    assert_eq!(cp.get_params_count(), 3);
    assert_eq!(cp.get_param(0), Some("1"));
    assert_eq!(cp.get_param(1), Some("left"));
    assert_eq!(cp.get_param(2), Some("-200"));
}
#[test]
fn check_command_parser_string() {
    let cp =
        CommandParser::new("  run     (    'some string '  ,   left  ,   -200   )    ").unwrap();
    assert_eq!(cp.get_command(), "run");
    assert_eq!(cp.get_params_count(), 3);
    assert_eq!(cp.get_param(0), Some("some string "));
    assert_eq!(cp.get_param(1), Some("left"));
    assert_eq!(cp.get_param(2), Some("-200"));
}

#[test]
fn test_command_only() {
    let command = "help";
    let parser = CommandParser::new(command).unwrap();
    assert_eq!(parser.get_command(), "help");
    assert_eq!(parser.get_params_count(), 0);
    assert_eq!(parser.get_param(0), None);
    assert_eq!(parser.get_param(1), None);
    assert_eq!(parser.get_param(2), None);
}

#[test]
fn test_single_param() {
    let command = "set(value)";
    let parser = CommandParser::new(command).unwrap();
    assert_eq!(parser.get_command(), "set");
    assert_eq!(parser.get_params_count(), 1);
    assert_eq!(parser.get_param(0), Some("value"));
    assert_eq!(parser.get_param(1), None);
    assert_eq!(parser.get_param(2), None);
}

#[test]
fn test_multiple_params() {
    let command = "create(user,\"John ,Smith\",30)";
    let parser = CommandParser::new(command).unwrap();
    assert_eq!(parser.get_command(), "create");
    assert_eq!(parser.get_params_count(), 3);
    assert_eq!(parser.get_param(0), Some("user"));
    assert_eq!(parser.get_param(1), Some("John ,Smith"));
    assert_eq!(parser.get_param(2), Some("30"));
}

#[test]
fn test_too_many_params() {
    let command = "update(id,name,age,location)";
    let parser = CommandParser::new(command);
    assert!(parser.is_err());
    assert_eq!(
        parser.unwrap_err().get_error(),
        "Too many parameters (max allowed is 3)"
    );
}

#[test]
fn test_missing_command() {
    let command = "";
    let parser = CommandParser::new(command);
    assert!(parser.is_err());
    assert_eq!(parser.unwrap_err().get_error(), "Expecting a valid command (not an empty line)");
}

#[test]
fn test_missing_param() {
    let command = "create(user)";
    let parser = CommandParser::new(command).unwrap();
    assert_eq!(parser.get_command(), "create");
    assert_eq!(parser.get_params_count(), 1);
    assert_eq!(parser.get_param(0), Some("user"));
    assert_eq!(parser.get_param(1), None);
    assert_eq!(parser.get_param(2), None);
}

#[test]
fn test_invalid_syntax() {
    let command = "update(, id)";
    let parser = CommandParser::new(command);
    assert!(parser.is_err());
    assert_eq!(
        parser.unwrap_err().get_error(),
        "Expecting a word but found ',' separator !"
    );
}

#[test]
fn test_invalid_character() {
    let command = "create( #user)";
    let parser = CommandParser::new(command);
    assert!(parser.is_err());
    assert_eq!(parser.unwrap_err().get_error(), "Invalid character (expecting a word)");
}

#[test]
fn test_invalid_string() {
    let command = "create (user, \"John Smith)";
    let parser = CommandParser::new(command);
    assert!(parser.is_err());
    assert_eq!(
        parser.unwrap_err().get_error(),
        "Invalid string (no ending '\"' character found)"
    );
}


#[test]
fn check_command_parser_bool_params() {
    let cp = CommandParser::new("  validate(1,true,false)").unwrap();
    assert_eq!(cp.get_command(), "validate");
    assert_eq!(cp.get_params_count(), 3);
    assert_eq!(cp.get_bool(0), None);
    assert_eq!(cp.get_bool(1), Some(true));
    assert_eq!(cp.get_bool(2), Some(false));
}

#[test]
fn check_command_parser_i32_params() {
    let cp = CommandParser::new("  validate(123,-1276,false)").unwrap();
    assert_eq!(cp.get_command(), "validate");
    assert_eq!(cp.get_params_count(), 3);
    assert_eq!(cp.get_i32(0), Some(123));
    assert_eq!(cp.get_i32(1), Some(-1276));
    assert_eq!(cp.get_i32(2), None);
}