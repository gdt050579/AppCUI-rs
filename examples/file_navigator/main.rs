use appcui::prelude::*;
use chrono::{DateTime, Local, NaiveDateTime};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(ListItem)]
struct Entry {
    #[Column(name: "&Name", width: 32)]
    name: String,
    #[Column(name: "&Size", width: 12, align: right, render: size, format: auto)]
    size: u64,
    #[Column(name: "&Modified", width: 18, align: center, render: datetime, format: short)]
    modified: NaiveDateTime,
    // not a column: it only tells folders apart from files
    is_dir: bool,
}

fn to_local_time(time: Option<SystemTime>) -> NaiveDateTime {
    match time {
        Some(time) => DateTime::<Local>::from(time).naive_local(),
        None => NaiveDateTime::default(),
    }
}

#[Window(events = ListViewEvents<Entry>)]
struct Win {
    path: PathBuf,
    list: Handle<ListView<Entry>>,
    status: Handle<Label>,
}

impl Win {
    fn new() -> Self {
        let mut me = Self {
            base: window!("'File navigator',a:c,w:80,h:20,flags: Sizeable"),
            path: PathBuf::new(),
            list: Handle::None,
            status: Handle::None,
        };
        me.status = me.add(label!("'',l:1,t:0,r:1,h:1"));
        me.list = me.add(listview!("class: Entry,l:0,t:1,r:0,b:0,flags: ScrollBars+SearchBar+LargeIcons"));

        // start in the current folder
        let start = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        if let Err(error) = me.open_folder(start) {
            me.show_status(&error);
        }
        me
    }

    fn show_status(&mut self, text: &str) {
        let h = self.status;
        if let Some(label) = self.control_mut(h) {
            label.set_caption(text);
        }
    }

    fn read_folder(path: &Path) -> Result<Vec<Entry>, String> {
        let reader = fs::read_dir(path).map_err(|e| format!("Can't open '{}': {}", path.display(), e))?;
        let mut entries = Vec::new();
        for item in reader.flatten() {
            // entries that can't be inspected are still listed (with an empty size and date)
            let metadata = item.metadata().ok();
            entries.push(Entry {
                name: item.file_name().to_string_lossy().into_owned(),
                size: metadata.as_ref().filter(|m| m.is_file()).map(|m| m.len()).unwrap_or(0),
                modified: to_local_time(metadata.as_ref().and_then(|m| m.modified().ok())),
                is_dir: metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false),
            });
        }
        Ok(entries)
    }

    /// Lists the content of `path`. If the folder can't be read, the current one is kept
    /// and the reason is returned as an error.
    fn open_folder(&mut self, path: PathBuf) -> Result<(), String> {
        let (mut folders, mut files): (Vec<Entry>, Vec<Entry>) = Self::read_folder(&path)?.into_iter().partition(|e| e.is_dir);
        // folders first, then files, both sorted by name
        folders.sort_by_key(|e| e.name.to_lowercase());
        files.sort_by_key(|e| e.name.to_lowercase());
        let parent = path.parent().map(|parent| Entry {
            name: "..".to_string(),
            size: 0,
            modified: to_local_time(fs::metadata(parent).and_then(|m| m.modified()).ok()),
            is_dir: true,
        });

        let h = self.list;
        if let Some(lv) = self.control_mut(h) {
            lv.clear();
            lv.clear_search();
            for folder in parent.into_iter().chain(folders) {
                lv.add_item(listview::Item::new(
                    folder,
                    false,
                    Some(CharAttribute::with_fore_color(Color::White)),
                    ['📁', ' '],
                    listview::Group::None,
                ));
            }
            for file in files {
                lv.add_item(listview::Item::new(
                    file,
                    false,
                    Some(CharAttribute::with_fore_color(Color::Silver)),
                    ['📄', ' '],
                    listview::Group::None,
                ));
            }
            lv.request_focus();
        }
        self.show_status(&path.display().to_string());
        self.path = path;
        Ok(())
    }
}

impl ListViewEvents<Entry> for Win {
    fn on_item_action(&mut self, handle: Handle<ListView<Entry>>, item_index: usize) -> EventProcessStatus {
        let target = match self.control(handle).and_then(|lv| lv.item(item_index)) {
            Some(entry) if entry.is_dir => {
                if entry.name == ".." {
                    self.path.parent().map(Path::to_path_buf)
                } else {
                    Some(self.path.join(&entry.name))
                }
            }
            _ => None,
        };
        if let Some(target) = target {
            if let Err(error) = self.open_folder(target) {
                self.show_status(&error);
            }
        }
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().window(Win::new).run()
}
