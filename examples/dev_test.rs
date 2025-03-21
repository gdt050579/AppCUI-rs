use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let a = App::new().build()?;
    a.run();
    Ok(())
}
