use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let app = App::new().app_bar().build()?;
    app.run();
    Ok(())
}
