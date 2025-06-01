use super::StructDefinition;

#[test]
fn check_struct_definition() {
    let sd = StructDefinition::from("struct    MyStruct { ");
    assert!(sd.access.is_empty());
    assert_eq!(sd.name, "MyStruct");
    assert!(sd.template_def.is_empty());
    assert!(sd.template_type.is_empty());

    let sd2 = StructDefinition::from("pub(crate) struct Test<T> { ");
    assert_eq!(sd2.access, "pub(crate) ");
    assert_eq!(sd2.name, "Test");
    assert_eq!(sd2.template_def, "<T> ");
    assert_eq!(sd2.template_type, "<T>");

    let sd3 = StructDefinition::from("pub   struct    MyControl  <A,B> where  A:Copy+Clone, B:Debug { ");
    assert_eq!(sd3.name, "MyControl");
    assert_eq!(sd3.template_def, "<A,B> where  A:Copy+Clone, B:Debug ");
    assert_eq!(sd3.template_type, "<A,B>");

    let sd4 = StructDefinition::from("pub   struct MyControl  <TT> where TT: Copy+Clone> { ");
    assert_eq!(sd4.name, "MyControl");
    assert_eq!(sd4.template_def, "<TT> where TT: Copy+Clone> ");
    assert_eq!(sd4.template_type, "<TT>");
}
