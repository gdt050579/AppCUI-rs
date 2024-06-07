use appcui::prelude::*;



fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().command_bar().build()?;
    let mut w = window!("x:1,y:1,w:60,h:20,title:Win");
    let mut vs = VSplitter::new(5,Layout::new("d:c,w:100%,h:100%"),vsplitter::Flags::None);
    vs.set_left_min_size(2);
    vs.set_right_min_size(0.5);
    w.add(vs);
    a.add_window(w);
    a.run();
    Ok(())
}
