use appcui::prelude::*;
use appcui::ui::numericslider::*;
use appcui::ui::common::number::*;

fn main() -> Result<(), appcui::system::Error> {
    let a = App::new().build()?;
    a.run();
    Ok(())
}
