use appcui::prelude::*;


fn main() -> Result<(), appcui::system::Error> {
    let file_path = "examples\\exemple.md";
    let content = fs::read_to_string(file_path).unwrap_or_else(|_| String::new());
    
    let mut a = App::new().build()?;
    let mut w = window!("Test,d:c,w:60,h:10");
    a.add_window(w);
    a.run();
    Ok(())
}
