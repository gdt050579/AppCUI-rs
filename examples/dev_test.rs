use appcui::prelude::*;
use components::Column;

struct TestItem {
    name: &'static str,
    age: &'static str,
    city: &'static str,
}
impl listview::ListItem for TestItem {
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::Text(self.age)),
            2 => Some(listview::RenderMethod::Text(self.city)),
            _ => None,
        }
    }
    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.age.cmp(other.age),
            2 => self.city.cmp(other.city),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

#[Window(events=ListViewEvents<TestItem>)]
struct MyWin {}
impl ListViewEvents<TestItem> for MyWin {
    // fn on_current_item_changed(&mut self, handle: Handle<ListView<TestItem>>) -> EventProcessStatus {
    //     let name = if let Some(lv) = self.control(handle) {
    //         if let Some(item) = lv.current_item() {
    //             item.name.to_string()
    //         } else {
    //             "None".to_string()
    //         }
    //     } else {
    //         "None".to_string()
    //     };
    //     self.set_title(&format!("Current Item: {}", name));
    //     return EventProcessStatus::Processed;
    // }

    fn on_group_collapsed(&mut self, handle: Handle<ListView<TestItem>>, group: listview::Group) -> EventProcessStatus {
        let n = if let Some(lv) = self.control(handle) {
            lv.group_name(group).unwrap_or("None")
        } else {
            "None"
        }
        .to_string();
        self.set_title(&format!("Group Collapsed: {}", n));

        EventProcessStatus::Processed
    }
}
impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("Test,d:c,w:70,h:12,flags: Sizeable"),
        };
        let mut l = listview!("TestItem,d:c,view:Details,flags: ScrollBars+CheckBoxes+ShowGroups+SearchBar+SmallIcons,columns=[{&Name,20,Left},{&Age,10,Right},{&City,10,Center}]");
        let g1 = l.add_group("USA");
        let g2 = l.add_group("Europe");
        let g3 = l.add_group("Asia");
        let g4 = l.add_group("Romania");
        l.add_batch(|l| {
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "1.John",
                    age: "25",
                    city: "New York",
                },
                g1,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "2.Sancez",
                    age: "15",
                    city: "Madrid",
                },
                g2,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "3.Etiene",
                    age: "20",
                    city: "Paris",
                },
                g2,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "4.Karl",
                    age: "30",
                    city: "London",
                },
                g2,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "5.Mihai",
                    age: "35",
                    city: "Bucharest",
                },
                g4,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "6.Vlad",
                    age: "40",
                    city: "Bucharest",
                },
                g4,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "7.Ion",
                    age: "45",
                    city: "Bucharest",
                },
                g4,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "8.Gheorghe",
                    age: "50",
                    city: "Bucharest",
                },
                g4,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "9.Mihai",
                    age: "55",
                    city: "Bucharest",
                },
                g4,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "10.Vlad",
                    age: "60",
                    city: "Bucharest",
                },
                g4,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "11.Ion",
                    age: "65",
                    city: "Bucharest",
                },
                g4,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "12.Gheorghe",
                    age: "70",
                    city: "Washington",
                },
                g1,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "13.Mihai",
                    age: "75",
                    city: "Tokio",
                },
                g3,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "14.Vlad",
                    age: "80",
                    city: "Tokio",
                },
                g3,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "15.Ion",
                    age: "85",
                    city: "San Francisco",
                },
                g1,
            ));
            l.add_item(listview::Item::with_group(
                TestItem {
                    name: "16.Gheorghe",
                    age: "90",
                    city: "Viena",
                },
                g2,
            ));
        });
        w.add(l);
        w
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
