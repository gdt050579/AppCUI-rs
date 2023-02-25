use appcui::controls::*;

fn main() {
    let mut a = appcui::system::App::new();
    a.add(CheckBox::new(
        "This is a checkbox that &enables a certain property that is required by this program",
        Layout::new("x:1,y:1,w:35,h:3"),
        true,
    ));
    a.run();
}
