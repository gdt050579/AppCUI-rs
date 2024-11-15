use appcui::prelude::*;
use chrono::NaiveDate;

struct File {
    name: &'static str,
    size: u64,
    created: NaiveDate,
}
impl listview::ListItem for File {
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::Size(self.size, listview::SizeFormat::Auto)),
            2 => Some(listview::RenderMethod::Date(self.created, listview::DateFormat::YearMonthDay)),
            _ => None,
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.size.cmp(&other.size),
            2 => self.created.cmp(&other.created),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

#[Window()]
pub(crate) struct Win {}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("Files,d:c,w:70,h:10,flags: Sizeable"),
        };
        let mut lv = listview!("class: File,x:0,y:0,w:100%,h:100%,flags: ScrollBars+SearchBar+LargeIcons, columns:[{&Name,25,l},{&Size,12,r}, {&Created,12,c}]");

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

        me.add(lv);
        me
    }
}
