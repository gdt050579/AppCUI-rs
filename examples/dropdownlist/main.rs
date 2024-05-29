use appcui::prelude::*;

mod mathop;
use mathop::MathOp;

#[Window(events = ButtonEvents+TextFieldEvents)]
struct MyWin {
    h_txt: Handle<TextField>,
    h_op: Handle<DropDownList<MathOp>>,
}
impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("d:c,w:50,h:10,caption:Win"),
            h_txt: Handle::None,
            h_op: Handle::None,
        };
        w.add(label!("Operation,x:1,y:1,w:15"));
        let mut db = DropDownList::<MathOp>::with_symbol(1, Layout::new("x:20,y:1,w:26"), dropdownlist::Flags::ShowDescription);
        db.add(MathOp::new("Sum", "(Add multiple numbers)", "∑", |x| x.iter().sum::<i32>()));
        db.add(MathOp::new("Product", "(Multiply multiple numbers)", "∏", |x| x.iter().product::<i32>()));
        db.add(MathOp::new("Average", "(Calculate the average of multiple numbers)", "∅", |x| x.iter().sum::<i32>() / x.len() as i32));
        db.add(MathOp::new("Max", "(Calculate the maximum of multiple numbers)", "⊔", |x| *x.iter().max().unwrap()));   
        db.add(MathOp::new("Min", "(Calculate the minimum of multiple numbers)", "⊓", |x| *x.iter().min().unwrap()));
        w.h_op = w.add(db);
        w.add(label!("Numbers,x:1,y:3,w:15"));
        w.h_txt = w.add(textfield!("'1,2,3,4',x:20,y:3,w:26,flags:ProcessEnter"));
        w.add(button!("&Calculate,x:50%,y:100%,a:b,w:20"));
        w
    }
    fn compute(&self) {
        let h = self.h_txt;
        if let Some(txt) = self.control(h) {
            let numbers = txt.text().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let h = self.h_op;
            if let Some(op) = self.control(h) {
                if let Some(result) = op.selected_item() {
                    let r = result.run(&numbers);
                    let msg = format!("Result: {}", r);
                    dialogs::message("Rsult", &msg);
                }
            }
        }
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        self.compute();
        EventProcessStatus::Processed
    }
}
impl TextFieldEvents for MyWin {
    fn on_validate(&mut self, _handle: Handle<TextField>, _text: &str) -> EventProcessStatus {
        self.compute();
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
