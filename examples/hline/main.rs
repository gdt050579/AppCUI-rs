use appcui::prelude::*;

#[Window(events=ButtonEvents)]
struct MyWin{
    hl: Handle<HLine>,
    counter: i32
}

impl MyWin{
    fn new() -> Self{
        let mut w = Self{
            base: window!("Example, d:c, w:40, h:10"),
            hl: Handle::None,
            counter: 0,  
        };
        w.hl = w.add(hline!("0, x:1, y:1, w:20,flags:HasTitle"));
        w.add(button!("Add, x:10, y:6, w:15"));
        w
    }
}

impl ButtonEvents for MyWin{
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        self.counter += 1;
        let f = format!("{}", self.counter);
        let h = self.hl;
        if let Some(line_control) = self.control_mut(h){
            line_control.set_title(&f);
        }
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}