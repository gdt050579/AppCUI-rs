use appcui::prelude::*;
use appcui::ui::numericslider::*;
use appcui::ui::common::number::*;


fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().log_file("debug.log",false).build()?;
    a.run();
    Ok(())
}
