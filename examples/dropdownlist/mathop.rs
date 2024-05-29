use appcui::prelude::*;

pub struct MathOp {
    name: String,
    description: String,
    symbol: String,
    f: fn(&[i32]) -> i32,
}
impl MathOp {
    pub fn new(name: &str, description: &str, symbol: &str, f: fn(&[i32]) -> i32) -> MathOp {
        MathOp {
            name: name.to_string(),
            description: description.to_string(),
            symbol: symbol.to_string(),
            f,
        }
    }
    pub fn run(&self, data: &[i32]) -> i32 {
        (self.f)(data)
    }   
}

impl DropDownListType for MathOp {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn symbol(&self) -> &str {
        &self.symbol
    }
}
