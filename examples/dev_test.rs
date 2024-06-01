use appcui::{prelude::*, ui::dropdownlist::DropDownList};



fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().command_bar().build()?;
    let mut w = window!("x:1,y:1,w:60,h:20,title:Win");
    w.add(NumericSelector::<i32>::new(
        5,
        123,
        850,
        11,
        Layout::new("x:1,y:1,w:20"),
        numericselector::Flags::HideButtons,
    ));
    w.add(NumericSelector::<i32>::new(
        5,
        1,
        6,
        1,
        Layout::new("x:1,y:3,w:20"),
        numericselector::Flags::ReadOnly,
    ));
    a.add_window(w);
    a.run();
    Ok(())
}
