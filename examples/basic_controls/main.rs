use appcui::prelude::*;


fn add_description(tab: &mut Tab, index: u32, desc: &str) {
    tab.add(index, vline!("r:30,t:0,b:0"));
    tab.add(index, Label::new(desc, layout!("x:100%,y:0,w:29,h:100%,p:tr")));
}

fn add_buttons(tab: &mut Tab, index: u32) {
    add_description(tab, index, "Buttons are controls that can be used to trigger a command when:\n- the user clicks on them\n- you have focus and you press Enter of Space\n- there is a hot key associated and the user pressed Alt+<HotKey>");
    tab.add(index, button!("'&Normal button',x: 1,y:1, w: 18"));
    tab.add(index, button!("'Inactive button',x: 1,y:4, w: 18, enable: false"));
    tab.add(index, button!("'&  Flat button',x: 1,y:7, w: 18, type: Flat"));
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(120, 30)).single_window().build()?;
    let mut w = window!("'Basic Controls',dock:fill");
    let mut t = tab!("d:f, tabs:[Buttons,CheckBoxes], type: OnLeft");
    add_buttons(&mut t, 0);
    w.add(t);
    a.add_window(w);
    a.run();
    Ok(())
}
