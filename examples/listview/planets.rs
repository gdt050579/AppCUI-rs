use appcui::prelude::*;

struct Planet {
    name: &'static str,
    volume: u64,
    temperature: f64,
    distance_to_sun: u64,
}
impl listview::ListItem for Planet {
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::Volume(self.volume, listview::VolumeFormat::CubicKilometers)),
            2 => Some(listview::RenderMethod::Temperature(
                self.temperature,
                listview::TemperatureFormat::Celsius,
            )),
            3 => Some(listview::RenderMethod::Distance(
                self.distance_to_sun,
                listview::DistanceFormat::Kilometers,
            )),
            _ => None,
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.volume.cmp(&other.volume),
            2 => match () {
                _ if self.temperature > other.temperature => std::cmp::Ordering::Greater,
                _ if self.temperature < other.temperature => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            },
            3 => self.distance_to_sun.cmp(&other.distance_to_sun),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

#[Window()]
pub(crate) struct Win {}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("Countries,d:c,w:70,h:10,flags: Sizeable"),
        };
        let mut lv = listview!("class: Planet,x:0,y:0,w:100%,h:100%,flags: ScrollBars+SearchBar+SmallIcons, columns:[{&Name,15,l},{&Volume,27,r}, {&Temperature,14,r}, {'&Distance to Sun',25,r}]");
        lv.add_item(listview::Item::new(
            Planet {
                name: "Pluto",
                volume: 1_123_456,
                temperature: -225.0,
                distance_to_sun: 5_906_376_272,
            },
            false,
            None,
            ['♇', 0 as char],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            Planet {
                name: "Mercury",
                volume: 6_083_000_000,
                temperature: 167.0,
                distance_to_sun: 57_910_000,
            },
            false,
            None,
            ['☿', 0 as char],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            Planet {
                name: "Uranus",
                volume: 6_833_000_000_000_000,
                temperature: -195.0,
                distance_to_sun: 2_870_658_186,
            },
            false,
            None,
            ['♅', 0 as char],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            Planet {
                name: "Neptune",
                volume: 6_254_000_000_000_000,
                temperature: -200.0,
                distance_to_sun: 4_498_396_441,
            },
            false,
            None,
            ['♆', 0 as char],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            Planet {
                name: "Venus",
                volume: 928_415_000_000,
                temperature: 464.0,
                distance_to_sun: 108_200_000,
            },
            false,
            None,
            ['♀', 0 as char],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            Planet {
                name: "Earth",
                volume: 1_083_210_000_000,
                temperature: 15.0,
                distance_to_sun: 149_600_000,
            },
            false,
            None,
            ['♁', 0 as char],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            Planet {
                name: "Mars",
                volume: 163_180_000_000,
                temperature: -65.0,
                distance_to_sun: 227_940_000,
            },
            false,
            None,
            ['♂', 0 as char],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            Planet {
                name: "Jupiter",
                volume: 1_431_280_000_000_000,
                temperature: -110.0,
                distance_to_sun: 778_330_000,
            },
            false,
            None,
            ['♃', 0 as char],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            Planet {
                name: "Saturn",
                volume: 8_271_300_000_000_000,
                temperature: -140.0,
                distance_to_sun: 1_426_666_000,
            },
            false,
            None,
            ['♄', 0 as char],
            listview::Group::None,
        ));

        me.add(lv);
        me
    }
}
