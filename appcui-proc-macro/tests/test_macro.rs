use AppCUIProcMacro::AppCUIControl;

#[AppCUIControl(base: int, overwrite: OnPaint)]
pub struct MyButton {
    x: i32,
    y: i32
}

#[test]
fn test_const_get_value() {

}