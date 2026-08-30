use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .window(|| {
            let mut win = window!("'First Window',a:c,w:30,h:9");
            win.add(label!("'Hello World !',a:c,w:13,h:1"));
            win
        })
        .run()?
        .run();
    Ok(())
}
