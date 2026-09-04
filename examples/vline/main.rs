use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new().window(|| {
        let mut w = Window::new("Title", layout!("a:c,w:40,h:20"), window::Flags::None);

        w.add(vline!("x:1,y:1,h:15,flags:DoubleLine"));
        w
    }).run()
}