use appcui::prelude::*;
use std::fs;

fn main() -> Result<(), appcui::system::Error> {
    let file_path = "examples\\exemple.md";
    let content = fs::read_to_string(file_path).unwrap_or_else(|_| String::new());
    
    let mut a = App::new().build()?;
    let mut w = window!("Test,d:c,w:50,h:15,flags:sizeable");
    let mut m = Markdown::new(content,
        Layout::new("d: c"),
        markdown::Flags::ScrollBars,
    );
    w.add(m);
    a.add_window(w);
    a.run();
    Ok(())
}
