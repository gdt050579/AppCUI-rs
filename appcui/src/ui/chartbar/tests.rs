use crate::prelude::*;
use chartbar::{Fit,Value,Type,Flags,YAxes};

#[test]
fn check_chartbar_creation() {
    let script = "
        //Paint.Enable(false)
        Paint('nine labels across al corners and center')   
        CheckHash(0xF7D704CAB062ED5C)   
    ";
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Title", Layout::new("d:c,w:40,h:9"), window::Flags::None);

    let mut c = ChartBar::new(
        "aaaaa",
        Type::VerticalBar,
        0,
        Flags::AutoScroll | Flags::ScrollBars,
        1,
        YAxes::MinMax(0, 100),
        Fit::None,
        Layout::new("d:c,w:100%,h:100%"),
    );
    
    let v = [ 100, 15, 20, 30, 25, 37, 50, 50, 70, 78, 90, 100 ];

    for i in v 
    {
        c.add_value(Value::with_label_color(i, format!("{i}").as_str(), charattr!("back:red")));
    }
    c.set_axes_left_space(7);
    c.set_yaxes("lalddsD", 0, 70, 2, 7);
    w.add(c);
    a.add_window(w);
    a.run();
}