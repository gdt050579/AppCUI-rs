use std::{str::FromStr, vec};

use appcui::prelude::*;

struct Person {
    name: &'static str,
    hire_date: chrono::NaiveDate,
    salary: u32,
    position: &'static str,
    age: u8,
    location: &'static str,
    evaluation: u8,
}
impl Person {
    fn new(name: &'static str, hire_date: &str, salary: u32, position: &'static str, age: u8, location: &'static str, evaluation: u8) -> Self {
        Self {
            name,
            hire_date: chrono::NaiveDate::from_str(hire_date).unwrap(),
            salary,
            position,
            age,
            location,
            evaluation,
        }
    }
}
impl listview::ListItem for Person {
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::Ascii(self.position)),
            2 => Some(listview::RenderMethod::Currency(self.salary as f64, listview::CurrencyFormat::USDSymbol)),
            3 => Some(listview::RenderMethod::Date(self.hire_date, listview::DateFormat::DayMonthYear)),
            4 => Some(listview::RenderMethod::UInt64(self.age as u64, listview::NumericFormat::Normal)),
            5 => Some(listview::RenderMethod::Rating(self.evaluation as u32, listview::RatingFormat::Stars(8))),
            6 => Some(listview::RenderMethod::Ascii(self.location)),
            _ => None,
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.position.cmp(&other.position),
            2 => self.salary.cmp(&other.salary),
            3 => self.hire_date.cmp(&other.hire_date),
            4 => self.age.cmp(&other.age),
            5 => self.evaluation.cmp(&other.evaluation),
            6 => self.location.cmp(&other.location),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

fn populate_listview(lv: &mut ListView<Person>) {
    // C-Level
    let g_c_level = lv.add_group("C-Level");
    let v = vec![
        Person::new("John Doe", "2010-01-01", 100_000, "CEO", 45, "New York", 5),
        Person::new("Jane Doe", "2015-01-01", 80_000, "CFO", 40, "New York", 4),
        Person::new("Alice Doe", "2018-01-01", 70_000, "COO", 35, "San Francisco", 3),
        Person::new("Bob Doe", "2019-01-01", 60_000, "CTO", 30, "Dallas", 6),
    ];
    lv.add_to_group(v, g_c_level);
    // C-managers
    let g_c_managers = lv.add_group("Managers");
    let v = vec![
        Person::new("Jeremy Mike", "2022-05-08", 50_000, "Manager", 50, "Los Angeles", 7),
        Person::new("Michael Johnson", "2020-01-01", 45_000, "Manager", 45, "Chicago", 6),
        Person::new("Jessica Smith", "2018-01-01", 40_000, "Manager", 40, "Miami", 5),
        Person::new("Samantha Brown", "2015-01-01", 35_000, "Manager", 35, "Seattle", 6),
    ];
    lv.add_to_group(v, g_c_managers);
    // personal
    let g_personal = lv.add_group("Personal");
    let v = vec![
        Person::new("John Smith", "2010-01-01", 30_000, "Engineer", 30, "New York", 5),
        Person::new("Sam Brown", "2015-01-01", 25_000, "HR", 25, "Paris", 4),
        Person::new("Alice Johnson", "2018-01-01", 20_000, "QA", 20, "London", 3),
        Person::new("Bob White", "2019-01-01", 15_000, "Intern", 18, "Berlin", 6),
    ];
    lv.add_to_group(v, g_personal);
}

#[Window(events = ListViewEvents<Person>)]
pub(crate) struct Win {
    lb: Handle<toolbar::Label>,
    lv: Handle<ListView<Person>>,
}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("'Personal list',d:c,w:70,h:10,flags: Sizeable"),
            lb: Handle::None,
            lv: Handle::None,
        };
        let mut lv = listview!(
            "class: Person,
                                x:0,y:0,w:100%,h:100%,
                                flags: [ScrollBars,SearchBar,ShowGroups,CheckBoxes], 
                                columns:[
                                    {&Name,20,l},
                                    {&Position,12,l}, 
                                    {&Salary,12,c}, 
                                    {&Hired,12,c},
                                    {&Age,7,r}, 
                                    {&Evaluation, 12, c},
                                    {&Location, 12, c}
                                ]"
        );
        lv.set_frozen_columns(1);
        lv.set_components_toolbar_margins(6, 1);
        populate_listview(&mut lv);
        me.lv = me.add(lv);

        let info_group = me.toolbar().create_group(toolbar::GroupPosition::BottomLeft);
        me.lb = me.toolbar().add(info_group, toolbar::Label::new("0/12"));

        me
    }
    fn update_info(&mut self) {
        let (checked, total) = if let Some(lv) = self.control(self.lv) {
            (lv.checked_items_count(), lv.items_count())
        } else {
            (0, 0)
        };
        let data = format!("{}/{}", checked, total);
        let h = self.lb;
        if let Some(lb) = self.toolbar().get_mut(h) {
            lb.set_content(data.as_str());
        }
    }
}

impl ListViewEvents<Person> for Win {
    fn on_selection_changed(&mut self, _: Handle<ListView<Person>>) -> EventProcessStatus {
        self.update_info();
        EventProcessStatus::Ignored
    }
}
