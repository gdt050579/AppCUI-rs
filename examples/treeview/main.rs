use appcui::prelude::*;

#[derive(ListItem)]
struct FileInfo {
    #[Column(name = "&Name", width: 40)]
    name: String,
    #[Column(name = "&Size", width: 10, align: right)]
    size: u64,
    #[Column(name = "Folder", width: 6, align: center)]
    folder: bool,
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = Window::new("Folders", Layout::new("d:c,w:60,h:20"), window::Flags::None);
    let mut tv = treeview!("FileInfo,d:c,flags:Scrollbars+SearchBar");
    w.add(tv);
    app.add_window(w);
    app.run();
    Ok(())
}