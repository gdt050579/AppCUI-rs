use appcui::controls::*;

fn main() {
    let mut a = appcui::system::App::new();
    a.add(CheckBox::new("a checkbox", Layout::new("x:1,y:1,w:30"),true));
    a.run();
}