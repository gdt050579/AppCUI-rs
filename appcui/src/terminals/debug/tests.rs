use super::command_parser::CommandParser;


#[test]
fn test_1() {
    let cp = CommandParser::new("run(1,2)").unwrap();
    assert_eq!(cp.get_command(),"run");
    assert_eq!(cp.get_params_count(),2);
}