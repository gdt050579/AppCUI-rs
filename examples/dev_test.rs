use appcui::prelude::*;


fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Test,d:c,w:60,h:10");
    a.add_window(w);
    a.run();
    Ok(())
}
