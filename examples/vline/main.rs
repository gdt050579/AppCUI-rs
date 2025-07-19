use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = Window::new("Title", Layout::new("a:c,w:40,h:20"), window::Flags::None);
    
    w.add(vline!("x:1,y:1,h:15,flags:DoubleLine"));
    app.add_window(w);
    app.run();
    Ok(())
}