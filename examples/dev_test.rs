use appcui::prelude::*;



fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().command_bar().build()?;
    let mut w = window!("Test,d:c,w:50,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(0.5,Layout::new("d:c,w:100%,h:100%"),vsplitter::ResizeBehavior::PreserveRightPanelSize);
    vs.add(vsplitter::Panel::Left,panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right,panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
    Ok(())
}
