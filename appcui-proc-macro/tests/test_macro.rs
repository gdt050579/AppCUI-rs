use AppCUIProcMacro::AppCUIControl;

struct A {
    value: i32,
}
#[AppCUIControl(base: A, overwrite: OnPaint)]
pub struct MyButton {
    x: i32,
    y: i32
}
#[test]
fn test_const_get_value() {

}