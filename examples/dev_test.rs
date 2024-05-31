use appcui::{prelude::*, ui::dropdownlist::DropDownList};

enum MathOp {
    Sum,
    Product,
    Integral,
    Radical,
    Different
}
impl DropDownListType for MathOp {
    fn name(&self) -> &str {
        match self {
            MathOp::Sum => "Sum",
            MathOp::Product => "Product",
            MathOp::Integral => "Integral",
            MathOp::Radical => "Radical",
            MathOp::Different => "Different"
        }
    }
    fn description(&self) -> &str {
        match self {
            MathOp::Sum => "(Add multiple numbers)",
            MathOp::Product => "(Multiply multiple numbers)",
            MathOp::Integral => "(Calculate the integral of a function)",
            MathOp::Radical => "(Calculate the radical of a number  )",
            MathOp::Different => "(Check if all elements from a set are different)"
        }
    }
    fn symbol(&self) -> &str {
        match self {
            MathOp::Sum => "∑",
            MathOp::Product => "∏",
            MathOp::Integral => "∫",
            MathOp::Radical => "√",
            MathOp::Different => "≠"
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().command_bar().build()?;
    let mut w = window!("x:1,y:1,w:60,h:20,title:Win");
    let mut db = DropDownList::<MathOp>::with_symbol(1, Layout::new("x:1,y:1,w:50"), dropdownlist::Flags::ShowDescription);
    db.add(MathOp::Sum);
    db.add(MathOp::Product);
    db.add(MathOp::Integral);
    db.add(MathOp::Radical);
    db.add(MathOp::Different);
    w.add(db);
    a.add_window(w);
    a.run();
    Ok(())
}
