use crate::prelude::*;


#[test]
fn check_creation() {
    let script = "
        //Paint.Enable(false)
        Error.Disable(true)
        Paint('initial state')   
        CheckHash(0x8E402A80F606DBF1)
    ";
    let mut a = App::debug(40, 10, script).build().unwrap();
    let mut w = window!("Title,d:c,w:36,h:7");
    w.add(NumericSelector::<i32>::new(5,1,8,1,Layout::new("x:1,y:1,w:10"),numericselector::Flags::None));
    a.add_window(w);
    a.run();
}