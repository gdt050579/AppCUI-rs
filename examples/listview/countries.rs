use appcui::prelude::*;

struct Country {
    name: &'static str,
    capital: &'static str,
    population: u32,
    area: u32,
}
impl listview::ListItem for Country {
    fn render_method(&'_ self, column_index: u16) -> Option<listview::RenderMethod<'_>> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::Text(self.capital)),
            2 => Some(listview::RenderMethod::UInt64(self.population as u64, listview::NumericFormat::Separator)),
            3 => Some(listview::RenderMethod::Area(self.area as u64, listview::AreaFormat::SquaredKilometers)),
            _ => None,
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.capital.cmp(other.capital),
            2 => self.population.cmp(&other.population),
            3 => self.area.cmp(&other.area),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

fn data() -> Vec<Country> {
    vec![
        Country {
            name: "Romania",
            capital: "Bucharest",
            population: 19_000_000,
            area: 238_397,
        },
        Country {
            name: "France",
            capital: "Paris",
            population: 67_000_000,
            area: 551_695,
        },
        Country {
            name: "Germany",
            capital: "Berlin",
            population: 83_000_000,
            area: 357_386,
        },
        Country {
            name: "Italy",
            capital: "Rome",
            population: 60_000_000,
            area: 301_340,
        },
        Country {
            name: "Spain",
            capital: "Madrid",
            population: 47_000_000,
            area: 505_992,
        },
        Country {
            name: "United Kingdom",
            capital: "London",
            population: 66_000_000,
            area: 242_495,
        },
        Country {
            name: "Greece",
            capital: "Athens",
            population: 10_000_000,
            area: 131_957,
        },
        Country {
            name: "Sweden",
            capital: "Stockholm",
            population: 10_000_000,
            area: 450_295,
        },
        Country {
            name: "Norway",
            capital: "Oslo",
            population: 5_000_000,
            area: 323_802,
        },
        Country {
            name: "Finland",
            capital: "Helsinki",
            population: 5_000_000,
            area: 338_424,
        },
        Country {
            name: "Denmark",
            capital: "Copenhagen",
            population: 6_000_000,
            area: 42_924,
        },
        Country {
            name: "Netherlands",
            capital: "Amsterdam",
            population: 17_000_000,
            area: 41_543,
        },
        Country {
            name: "Belgium",
            capital: "Brussels",
            population: 11_000_000,
            area: 30_528,
        },
        Country {
            name: "Austria",
            capital: "Vienna",
            population: 9_000_000,
            area: 83_879,
        },
        Country {
            name: "Switzerland",
            capital: "Bern",
            population: 8_000_000,
            area: 41_290,
        },
        Country {
            name: "Portugal",
            capital: "Lisbon",
            population: 10_000_000,
            area: 92_212,
        },
        Country {
            name: "Poland",
            capital: "Warsaw",
            population: 38_000_000,
            area: 312_696,
        },
        Country {
            name: "Czech Republic",
            capital: "Prague",
            population: 10_000_000,
            area: 78_866,
        },
        Country {
            name: "Slovakia",
            capital: "Bratislava",
            population: 5_000_000,
            area: 49_035,
        },
        Country {
            name: "Hungary",
            capital: "Budapest",
            population: 10_000_000,
            area: 93_030,
        },
        Country {
            name: "Bulgaria",
            capital: "Sofia",
            population: 7_000_000,
            area: 110_994,
        },
        Country {
            name: "India",
            capital: "New Delhi",
            population: 1_300_000_000,
            area: 3_287_263,
        },
        Country {
            name: "China",
            capital: "Beijing",
            population: 1_400_000_000,
            area: 9_596_961,
        },
        Country {
            name: "Russia",
            capital: "Moscow",
            population: 146_000_000,
            area: 17_098_242,
        },
        Country {
            name: "United States",
            capital: "Washington",
            population: 328_000_000,
            area: 9_525_067,
        },
        Country {
            name: "Canada",
            capital: "Ottawa",
            population: 38_000_000,
            area: 9_984_670,
        },
        Country {
            name: "Brazil",
            capital: "Brasilia",
            population: 209_000_000,
            area: 8_515_767,
        },
        Country {
            name: "Argentina",
            capital: "Buenos Aires",
            population: 44_000_000,
            area: 2_780_400,
        },
        Country {
            name: "Australia",
            capital: "Canberra",
            population: 25_000_000,
            area: 7_692_024,
        },
        Country {
            name: "South Africa",
            capital: "Pretoria",
            population: 58_000_000,
            area: 1_221_037,
        },
        Country {
            name: "Egypt",
            capital: "Cairo",
            population: 100_000_000,
            area: 1_010_408,
        },
        Country {
            name: "Nigeria",
            capital: "Abuja",
            population: 206_000_000,
            area: 923_768,
        },
        Country {
            name: "Kenya",
            capital: "Nairobi",
            population: 53_000_000,
            area: 580_367,
        },
        Country {
            name: "Morocco",
            capital: "Rabat",
            population: 36_000_000,
            area: 446_550,
        },
        Country {
            name: "Algeria",
            capital: "Algiers",
            population: 43_000_000,
            area: 2_381_741,
        },
        Country {
            name: "Tunisia",
            capital: "Tunis",
            population: 11_000_000,
            area: 163_610,
        },
        Country {
            name: "Libya",
            capital: "Tripoli",
            population: 7_000_000,
            area: 1_759_540,
        },
        Country {
            name: "Saudi Arabia",
            capital: "Riyadh",
            population: 34_000_000,
            area: 2_149_690,
        },
        Country {
            name: "United Arab Emirates",
            capital: "Abu Dhabi",
            population: 9_000_000,
            area: 83_600,
        },
        Country {
            name: "Qatar",
            capital: "Doha",
            population: 2_000_000,
            area: 11_586,
        },
        Country {
            name: "Kuwait",
            capital: "Kuwait City",
            population: 4_000_000,
            area: 17_818,
        },
        Country {
            name: "Iraq",
            capital: "Baghdad",
            population: 40_000_000,
            area: 438_317,
        },
        Country {
            name: "Iran",
            capital: "Tehran",
            population: 83_000_000,
            area: 1_648_195,
        },
        Country {
            name: "Pakistan",
            capital: "Islamabad",
            population: 220_000_000,
            area: 881_913,
        },
        Country {
            name: "Afghanistan",
            capital: "Kabul",
            population: 38_000_000,
            area: 652_230,
        },
        Country {
            name: "Japan",
            capital: "Tokyo",
            population: 126_000_000,
            area: 377_975,
        },
        Country {
            name: "South Korea",
            capital: "Seoul",
            population: 51_000_000,
            area: 100_210,
        },
        Country {
            name: "North Korea",
            capital: "Pyongyang",
            population: 25_000_000,
            area: 120_540,
        },
        Country {
            name: "Vietnam",
            capital: "Hanoi",
            population: 97_000_000,
            area: 331_212,
        },
        Country {
            name: "Thailand",
            capital: "Bangkok",
            population: 69_000_000,
            area: 513_120,
        },
        Country {
            name: "Malaysia",
            capital: "Kuala Lumpur",
            population: 32_000_000,
            area: 329_847,
        },
        Country {
            name: "Indonesia",
            capital: "Jakarta",
            population: 270_000_000,
            area: 1_904_569,
        },
        Country {
            name: "Philippines",
            capital: "Manila",
            population: 108_000_000,
            area: 300_000,
        },
        Country {
            name: "New Zealand",
            capital: "Wellington",
            population: 5_000_000,
            area: 268_021,
        },
        Country {
            name: "Fiji",
            capital: "Suva",
            population: 1_000_000,
            area: 18_274,
        },
        Country {
            name: "Tonga",
            capital: "Nuku'alofa",
            population: 100_000,
            area: 747,
        },
        Country {
            name: "Samoa",
            capital: "Apia",
            population: 200_000,
            area: 2_842,
        },
        Country {
            name: "Tahiti",
            capital: "Papeete",
            population: 200_000,
            area: 1_042,
        },
        Country {
            name: "Hawaii",
            capital: "Honolulu",
            population: 1_500_000,
            area: 28_311,
        },
        Country {
            name: "Alaska",
            capital: "Juneau",
            population: 700_000,
            area: 1_723_337,
        },
        Country {
            name: "Iceland",
            capital: "Reykjavik",
            population: 360_000,
            area: 103_000,
        },
        Country {
            name: "Greenland",
            capital: "Nuuk",
            population: 56_000,
            area: 2_166_086,
        },
    ]
}

#[Window()]
pub(crate) struct Win {}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("Countries,a:c,w:70,h:10,flags: Sizeable"),
        };
        let mut lv = listview!("class: Country,x:0,y:0,w:100%,h:100%,flags: ScrollBars+SearchBar, columns:[{&Name,20,l},{&Capital,16,l}, {&Population,14,r}, {&Area,14,r}]");
        lv.add_items(data());
        me.add(lv);
        me
    }
}
