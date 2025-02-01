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

impl FileInfo {
    fn new(name: &str, size: u64, folder: bool) -> treeview::Item<Self> {
        treeview::Item::new(Self {
            name: name.to_string(),
            size,
            folder,
        },
        false,
        Some(if folder {charattr!("white")} else {charattr!("darkgreen")}),
        [0 as char, 0 as char]
        )
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = Window::new("Folders", Layout::new("d:c,w:60,h:20"), window::Flags::None);
    let mut tv = treeview!("FileInfo,d:c,flags:Scrollbars+SearchBar");
    let h = tv.add_item(FileInfo::new("Games", 0, true));
    let h1 = tv.add_item_to_parent(FileInfo::new("Tetris", 0, true),h);
    tv.add_item_to_parent(FileInfo::new("tetris.exe", 100000, false),h1);
    tv.add_item_to_parent(FileInfo::new("tetris.ico", 102, false),h1);
    tv.add_item_to_parent(FileInfo::new("readme.txt", 1024, false),h1);
    tv.add_item_to_parent(FileInfo::new("sounds.mp3", 1024123, false),h1);
    let h2 = tv.add_item_to_parent(FileInfo::new("Snake", 0, true),h);
    tv.add_item_to_parent(FileInfo::new("game.exe", 100000, false),h2);
    tv.add_item_to_parent(FileInfo::new("snake.ico", 102, false),h2);
    tv.add_item_to_parent(FileInfo::new("readme.txt", 1024, false),h2);
    tv.add_item_to_parent(FileInfo::new("movie.mavi", 102412312, false),h2);
    tv.add_item_to_parent(FileInfo::new("run.exe", 12345, false),h);
    let h = tv.add_item(FileInfo::new("Documents", 0, true)); 
    tv.add_item_to_parent(FileInfo::new("clients.docx", 100000, false),h);
    tv.add_item_to_parent(FileInfo::new("presentation.ppt", 10000, false),h);
    tv.add_item_to_parent(FileInfo::new("salaries.xsls", 12345, false),h);
    let h = tv.add_item(FileInfo::new("Utilities", 0, true)); 
    tv.add_item_to_parent(FileInfo::new("scripts.exe", 12345, false),h);
    tv.add_item_to_parent(FileInfo::new("hash.py", 123, false),h);
    w.add(tv);
    app.add_window(w);
    app.run();
    Ok(())
}