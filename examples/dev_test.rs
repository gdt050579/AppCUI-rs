use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let app = App::new().app_bar().run()?;
    app.run();
    Ok(())
}
