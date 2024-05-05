use appcui::prelude::*;


#[CustomControl(overwrite = OnPaint)]
struct MyControl<T> {
    value: T
}
impl<T> MyControl<T> {
    fn new(value: T)->Self {
        Self {
            base: ControlBase::new(Layout::new("d:c,w:10,h:20"), true),
            value,
        }
    }
}

impl<T> OnPaint for MyControl<T> {
    fn on_paint(&self, _surface: &mut Surface, _theme: &Theme) {}
}




fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Test,x:1,y:1,w:30,h:10");
    w.add(MyControl::new(10));
    a.add_window(w);
    a.run();
    Ok(())
}
