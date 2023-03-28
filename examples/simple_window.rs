use appcui::controls::*;

fn main()->Result<(),appcui::system::Error> {
    let mut a = appcui::system::App::debug()?;
    let mut w = Window::new("Simple window", Layout::new("d:c,w:40,h:10"), WindowFlags::None);    
    w.add(CheckBox::new(
        "This is a checkbox that &enables a certain property that is required by this program",
        Layout::new("x:1,y:1,w:35,h:3"),
        true,
    ));
    a.add(w);
    a.run();
    Ok(())
}
