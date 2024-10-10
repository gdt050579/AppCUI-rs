use appcui::prelude::*;

struct Melody {
    name: &'static str,
    author: &'static str,
    duration: chrono::Duration,
    year: u16,
    stars: u8,
}
impl listview::ListItem for Melody {
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::Text(&self.author)),
            2 => Some(listview::RenderMethod::UInt64(self.year as u64, listview::NumericFormat::Normal)),
            _ => None,
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.author.cmp(other.author),
            2 => self.year.cmp(&other.year),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

fn populate_listview(lv: &mut ListView<Melody>) {
    // rock music
    let g_rock = lv.add_group("Rock");
    let v = vec![
        Melody { name: "Bohemian Rhapsody",  author: "Queen", duration: chrono::Duration::minutes(6) + chrono::Duration::seconds(7), year: 1975, stars: 5 },
        Melody { name: "Stairway to Heaven", author: "Led Zeppelin", duration: chrono::Duration::minutes(8) + chrono::Duration::seconds(2), year: 1971, stars: 3 },
        Melody { name: "Hotel California",   author: "Eagles", duration: chrono::Duration::minutes(6) + chrono::Duration::seconds(30), year: 1976, stars: 4 },
        Melody { name: "Imagine",            author: "John Lennon", duration: chrono::Duration::minutes(3) + chrono::Duration::seconds(3), year: 1971, stars: 5 },
    ];
    lv.add_to_group(v, g_rock);
}

#[Window()]
pub(crate) struct Win {}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("Music,d:c,w:70,h:10,flags: Sizeable"),
        };
        let mut lv = listview!("class: Melody,x:0,y:0,w:100%,h:100%,flags: ScrollBars+SearchBar+ShowGroups, columns:[{&Name,20,l},{&Author,16,l}, {&Year,6,c}]");
        populate_listview(&mut lv);
        me.add(lv);
        me
    }
}
