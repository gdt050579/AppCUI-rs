use appcui::prelude::*;

struct Melody {
    name: &'static str,
    author: &'static str,
    duration: chrono::Duration,
    year: u16,
    stars: u8,
}
impl listview::ListItem for Melody {
    fn render_method(&'_ self, column_index: u16) -> Option<listview::RenderMethod<'_>> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::Text(self.author)),
            2 => Some(listview::RenderMethod::UInt64(self.year as u64, listview::NumericFormat::Normal)),
            3 => Some(listview::RenderMethod::Rating(self.stars as u32, listview::RatingFormat::Stars(5))),
            4 => Some(listview::RenderMethod::Duration(self.duration, listview::DurationFormat::Details)),
            _ => None,
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.author.cmp(other.author),
            2 => self.year.cmp(&other.year),
            3 => self.stars.cmp(&other.stars),
            4 => self.duration.cmp(&other.duration),
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
    // pop music
    let g_pop = lv.add_group("Pop");
    let v = vec![
        Melody { name: "Thriller", author: "Michael Jackson", duration: chrono::Duration::minutes(5) + chrono::Duration::seconds(57), year: 1982, stars: 5 },
        Melody { name: "Billie Jean", author: "Michael Jackson", duration: chrono::Duration::minutes(4) + chrono::Duration::seconds(54), year: 1982, stars: 4 },
        Melody { name: "Like a Virgin", author: "Madonna", duration: chrono::Duration::minutes(3) + chrono::Duration::seconds(11), year: 1984, stars: 3 },
        Melody { name: "I Will Always Love You", author: "Whitney Houston", duration: chrono::Duration::minutes(4) + chrono::Duration::seconds(31), year: 1992, stars: 5 },
    ];
    lv.add_to_group(v, g_pop);
    // dance music
    let g_dance = lv.add_group("Dance");
    let v = vec![
        Melody { name: "Around the World", author: "Daft Punk", duration: chrono::Duration::minutes(7) + chrono::Duration::seconds(9), year: 1997, stars: 4 },
        Melody { name: "One More Time", author: "Daft Punk", duration: chrono::Duration::minutes(5) + chrono::Duration::seconds(20), year: 2000, stars: 5 },
        Melody { name: "Get Lucky", author: "Daft Punk", duration: chrono::Duration::minutes(6) + chrono::Duration::seconds(9), year: 2013, stars: 5 },
        Melody { name: "Happy", author: "Pharrell Williams", duration: chrono::Duration::minutes(3) + chrono::Duration::seconds(53), year: 2013, stars: 4 },
    ];
    lv.add_to_group(v, g_dance);
    // jazz music
    let g_jazz = lv.add_group("Jazz");
    let v = vec![
        Melody { name: "Take Five", author: "Dave Brubeck", duration: chrono::Duration::minutes(5) + chrono::Duration::seconds(24), year: 1959, stars: 5 },
        Melody { name: "So What", author: "Miles Davis", duration: chrono::Duration::minutes(9) + chrono::Duration::seconds(22), year: 1959, stars: 4 },
        Melody { name: "My Favorite Things", author: "John Coltrane", duration: chrono::Duration::minutes(13) + chrono::Duration::seconds(41), year: 1961, stars: 3 },
        Melody { name: "A Love Supreme", author: "John Coltrane", duration: chrono::Duration::minutes(32) + chrono::Duration::seconds(18), year: 1965, stars: 5 },
    ];
    lv.add_to_group(v, g_jazz);
    // classical music
    let g_classical = lv.add_group("Classical");
    let v = vec![
        Melody { name: "Symphony No. 9", author: "Ludwig van Beethoven", duration: chrono::Duration::minutes(1) + chrono::Duration::seconds(5), year: 1824, stars: 5 },
        Melody { name: "The Four Seasons", author: "Antonio Vivaldi", duration: chrono::Duration::minutes(9) + chrono::Duration::seconds(12), year: 1725, stars: 4 },
        Melody { name: "Swan Lake", author: "Pyotr Ilyich Tchaikovsky", duration: chrono::Duration::minutes(2) + chrono::Duration::seconds(41), year: 1876, stars: 3 },
        Melody { name: "The Nutcracker", author: "Pyotr Ilyich Tchaikovsky", duration: chrono::Duration::minutes(2) + chrono::Duration::seconds(41), year: 1892, stars: 5 },
    ];
    lv.add_to_group(v, g_classical);
    // country music
    let g_country = lv.add_group("Country");
    let v = vec![
        Melody { name: "Jolene", author: "Dolly Parton", duration: chrono::Duration::minutes(2) + chrono::Duration::seconds(41), year: 1973, stars: 5 },
        Melody { name: "Ring of Fire", author: "Johnny Cash", duration: chrono::Duration::minutes(2) + chrono::Duration::seconds(37), year: 1963, stars: 4 },
        Melody { name: "Take Me Home, Country Roads", author: "John Denver", duration: chrono::Duration::minutes(3) + chrono::Duration::seconds(8), year: 1971, stars: 3 },
        Melody { name: "The Gambler", author: "Kenny Rogers", duration: chrono::Duration::minutes(3) + chrono::Duration::seconds(31), year: 1978, stars: 5 },
    ];
    lv.add_to_group(v, g_country);
    // blues music
    let g_blues = lv.add_group("Blues");
    let v = vec![
        Melody { name: "The Thrill is Gone", author: "B.B. King", duration: chrono::Duration::minutes(5) + chrono::Duration::seconds(24), year: 1969, stars: 4 },
        Melody { name: "Born Under a Bad Sign", author: "Albert King", duration: chrono::Duration::minutes(3) + chrono::Duration::seconds(22), year: 1967, stars: 2 },        
    ];
    lv.add_to_group(v, g_blues);
}

#[Window()]
pub(crate) struct Win {}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("Music,a:c,w:70,h:10,flags: Sizeable"),
        };
        let mut lv = listview!("class: Melody,
                                x:0,y:0,w:100%,h:100%,
                                flags: ScrollBars+SearchBar+ShowGroups, 
                                columns:[
                                    {&Name,20,l},
                                    {&Author,16,l}, 
                                    {&Year,6,c}, 
                                    {&Stars,7,c}, 
                                    {&Duration, 12, r}
                                ]");
        populate_listview(&mut lv);
        me.add(lv);
        me
    }
}
