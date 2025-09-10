use appcui::prelude::*;

struct Animal {
    name: &'static str,
    speed: u64,
    weight: u64,
    predator: bool,
}
impl listview::ListItem for Animal {
    fn render_method(&'_ self, column_index: u16) -> Option<listview::RenderMethod<'_>> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::Speed(self.speed, listview::SpeedFormat::KilometersPerHour)),
            2 => Some(listview::RenderMethod::Weight(
                self.weight,
                listview::WeightFormat::Kilograms,
            )),
            3 => Some(listview::RenderMethod::Bool(
                self.predator,
                listview::BoolFormat::CheckmarkMinus,
            )),
            _ => None,
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.speed.cmp(&other.speed),
            2 => self.weight.cmp(&other.weight),
            3 => self.predator.cmp(&other.predator),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

#[Window()]
pub(crate) struct Win {}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("Animals,a:c,w:70,h:10,flags: Sizeable"),
        };
        let mut lv = listview!("class: Animal,x:0,y:0,w:100%,h:100%,flags: ScrollBars+SearchBar+LargeIcons+CheckBoxes+ShowGroups, columns:[{&Name,15,l},{&Speed,12,r}, {&Weight,12,r}, {&Predator,12,c}]");

        let g_birds = lv.add_group("Birds");
        lv.add_item(listview::Item::new(
            Animal {
                name: "Penguin",
                speed: 5,
                weight: 20,
                predator: false,
            },
            false,
            None,
            ['🐧', 0 as char],
            g_birds,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Owl",
                speed: 10,
                weight: 2,
                predator: true,
            },
            false,
            None,
            ['🦉', 0 as char],
            g_birds,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Duck",
                speed: 10,
                weight: 2,
                predator: false,
            },
            false,
            None,
            ['🦆', 0 as char],
            g_birds,
        ));

        let g_fish = lv.add_group("Fish");
        lv.add_item(listview::Item::new(
            Animal {
                name: "Shark",
                speed: 50,
                weight: 1000,
                predator: true,
            },
            false,
            None,
            ['🦈', 0 as char],
            g_fish,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Crab",
                speed: 1,
                weight: 2,
                predator: false,
            },
            false,
            None,
            ['🦞', 0 as char],
            g_fish,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Small Fish",
                speed: 5,
                weight: 1,
                predator: false,
            },
            false,
            None,
            ['🐟', 0 as char],
            g_fish,
        ));

        let g_insects = lv.add_group("Insects");

        lv.add_item(listview::Item::new(
            Animal {
                name: "Butterfly",
                speed: 10,
                weight: 0,
                predator: false,
            },
            false,
            None,
            ['🦋', 0 as char],
            g_insects
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Bee",
                speed: 5,
                weight: 0,
                predator: false,
            },
            false,
            None,
            ['🐝', 0 as char],
            g_insects
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Ant",
                speed: 1,
                weight: 0,
                predator: false,
            },
            false,
            None,
            ['🐜', 0 as char],
            g_insects
        ));

        let g_mammals = lv.add_group("Mammals");
        lv.add_item(listview::Item::new(
            Animal {
                name: "Tiger",
                speed: 60,
                weight: 300,
                predator: true,
            },
            false,
            None,
            ['🐯', 0 as char],
            g_mammals
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Lion",
                speed: 80,
                weight: 200,
                predator: true,
            },
            false,
            None,
            ['🦁', 0 as char],
            g_mammals,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Elephant",
                speed: 40,
                weight: 5000,
                predator: false,
            },
            false,
            None,
            ['🐘', 0 as char],
            g_mammals,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Panda",
                speed: 20,
                weight: 100,
                predator: false,
            },
            false,
            None,
            ['🐼', 0 as char],
            g_mammals,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Cheetah",
                speed: 100,
                weight: 60,
                predator: true,
            },
            false,
            None,
            ['🐆', 0 as char],
            g_mammals,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Giraffe",
                speed: 60,
                weight: 800,
                predator: false,
            },
            false,
            None,
            ['🦒', 0 as char],
            g_mammals,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Polar Bear",
                speed: 40,
                weight: 500,
                predator: true,
            },
            false,
            None,
            ['🐻', 0 as char],
            g_mammals,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Gorilla",
                speed: 20,
                weight: 200,
                predator: false,
            },
            false,
            None,
            ['🦍', 0 as char],
            g_mammals,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Horse",
                speed: 50,
                weight: 500,
                predator: false,
            },
            false,
            None,
            ['🐴', 0 as char],
            g_mammals,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Dog",
                speed: 30,
                weight: 30,
                predator: false,
            },
            false,
            None,
            ['🐕', 0 as char],
            g_mammals,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Cat",
                speed: 20,
                weight: 5,
                predator: false,
            },
            false,
            None,
            ['🐈', 0 as char],
            g_mammals,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Cow",
                speed: 20,
                weight: 500,
                predator: false,
            },
            false,
            None,
            ['🐄', 0 as char],
            g_mammals,
        ));
        lv.add_item(listview::Item::new(
            Animal {
                name: "Kangaroo",
                speed: 70,
                weight: 80,
                predator: false,
            },
            false,
            None,
            ['🦘', 0 as char],
            g_mammals,
        ));


        me.add(lv);
        me
    }
}
