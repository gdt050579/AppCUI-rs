use appcui::prelude::*;
use chrono::NaiveDate;

#[derive(ListItem)]
struct File {
    #[Column(name: "&Name", width: 25)]
    name: &'static str,
    #[Column(name: "&Size", width: 12, align: right, render: size, format: auto)]
    size: u64,
    #[Column(name: "&Created", width: 12, align: center, render: date, format: YearMonthDay)]
    created: NaiveDate,
}

#[Window(events = ToggleButtonEvents)]
pub(crate) struct Win { 
    l: Handle<ListView<File>>,
    details: Handle<ToggleButton>,
    columns: Handle<ToggleButton>,   
}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("Files,d:c,w:70,h:10,flags: Sizeable"),
            l: Handle::None,
            details: Handle::None,
            columns: Handle::None,
        };
        me.details = me.add(togglebutton!("'▤','Show details for each file',r:1,t:0,w:1, selected: true, group: true"));
        me.columns = me.add(togglebutton!("'‖','Simplified mode (only the file name on multiple columns)',r:2,t:0,w:1,group:true "));
        let mut lv = listview!("class: File,l:0,t:1,r:0,b:0,flags: ScrollBars+SearchBar+LargeIcons");

        let g_folder = lv.add_group("Folders");
        lv.add_item(listview::Item::new(
            File {
                name: "Applicatons",
                size: 0,
                created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::White)),
            ['📁', ' '],
            g_folder,
        ));
        lv.add_item(listview::Item::new(
            File {
                name: "Users",
                size: 0,
                created: NaiveDate::from_ymd_opt(2019, 5, 10).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::White)),
            ['📁', ' '],
            g_folder,
        ));
        lv.add_item(listview::Item::new(
            File {
                name: "Games",
                size: 0,
                created: NaiveDate::from_ymd_opt(20213, 12, 24).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::White)),
            ['📁', ' '],
            g_folder,
        ));

        let g_folder = lv.add_group("Executables");
        lv.add_item(listview::Item::new(
            File {
                name: "RunMe.exe",
                size: 1024 * 1024 * 10,
                created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Yellow)),
            ['🗔', ' '], 
            g_folder,
        ));
        lv.add_item(listview::Item::new(
            File {
                name: "MyGames.exe",
                size: 1024 * 15,
                created: NaiveDate::from_ymd_opt(2019, 5, 10).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Yellow)),
            ['🗔', ' '],
            g_folder,
        ));

        let g_scripts = lv.add_group("Scripts");
        lv.add_item(listview::Item::new(
            File {
                name: "Install.bat",
                size: 1024 * 1024 * 2,
                created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Green)),
            ['✎', ' '],
            g_scripts,
        ));

        let g_archives = lv.add_group("Archives");
        lv.add_item(listview::Item::new(
            File {
                name: "MyBackup.zip",
                size: 1024 * 1024 * 10,
                created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Red)),
            ['📦', ' '],
            g_archives,
        ));
        lv.add_item(listview::Item::new(
            File {
                name: "OldStuff.rar",
                size: 1024 * 1024 * 15,
                created: NaiveDate::from_ymd_opt(2019, 5, 10).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Red)),
            ['📦', ' '],
            g_archives,
        ));

        let g_images = lv.add_group("Images");
        lv.add_item(listview::Item::new(
            File {
                name: "MyPhoto.jpg",
                size: 1024 * 1024 * 2,
                created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Silver)),
            ['📷', ' '],
            g_images,
        ));
        lv.add_item(listview::Item::new(
            File {
                name: "Dog and cats.png",
                size: 1024 * 1024 * 3,
                created: NaiveDate::from_ymd_opt(2019, 5, 10).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Silver)),
            ['📷', ' '],
            g_images,
        ));

        let g_videos = lv.add_group("Videos");
        lv.add_item(listview::Item::new(
            File {
                name: "MyHoliday.mp4",
                size: 1024 * 1024 * 100,
                created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Silver)),
            ['🎥', ' '],
            g_videos,
        ));
        lv.add_item(listview::Item::new(
            File {
                name: "FunnyCats.mp4",
                size: 1024 * 1024 * 50,
                created: NaiveDate::from_ymd_opt(2019, 5, 10).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Silver)),
            ['🎥', ' '],
            g_videos,
        ));

        let g_music = lv.add_group("Music");
        lv.add_item(listview::Item::new(
            File {
                name: "MySong.mp3",
                size: 1024 * 1024 * 5,
                created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Silver)),
            ['🎵', ' '],
            g_music,
        ));
        lv.add_item(listview::Item::new(
            File {
                name: "BestOf2024.mp3",
                size: 1024 * 1024 * 10,
                created: NaiveDate::from_ymd_opt(2019, 5, 10).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Silver)),
            ['🎵', ' '],
            g_music,
        ));

        let g_documents = lv.add_group("Documents");
        lv.add_item(listview::Item::new(
            File {
                name: "MyCV.docx",
                size: 1024 * 1024,
                created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Silver)),
            ['📄', ' '],
            g_documents,
        ));
        lv.add_item(listview::Item::new(
            File {
                name: "MyBook.pdf",
                size: 1024 * 1024 * 2,
                created: NaiveDate::from_ymd_opt(2019, 5, 10).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Silver)),
            ['📄', ' '],
            g_documents,
        ));

        let g_other = lv.add_group("Other");
        lv.add_item(listview::Item::new(
            File {
                name: "Manual.txt",
                size: 22345,
                created: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            },
            false,
            Some(CharAttribute::with_fore_color(Color::Gray)),
            ['?', ' '],
            g_other,
        ));

        me.l = me.add(lv);
        me
    }
    fn set_view_mode(&mut self, mode: listview::ViewMode) {
        let h = self.l;
        if let Some(lv) = self.control_mut(h) {
            lv.set_view_mode(mode);
            lv.request_focus();
        }
    }
}

impl ToggleButtonEvents for Win {
    fn on_selection_changed(&mut self, handle: Handle<ToggleButton>, _selected: bool) -> EventProcessStatus {
        if handle == self.details {
            self.set_view_mode(listview::ViewMode::Details);
        } else if handle == self.columns {
            self.set_view_mode(listview::ViewMode::Columns(3));
        }
        EventProcessStatus::Processed
    }
}