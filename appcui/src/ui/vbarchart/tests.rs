use crate::prelude::*;

#[test]
fn check_creation() {
    let _chart = VBarChart::<i32>::new(layout!("x:1,y:1,w:10,h:5"), vbarchart::Flags::None);
}
