use appcui::prelude::*;

struct Student {
    name: &'static str,
    grade: u32
}
impl listview::ListItem for Student {
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::UInt64(self.grade as u64, listview::NumericFormat::Normal)),
            _ => None,
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.grade.cmp(&other.grade),
            _ => std::cmp::Ordering::Equal,
        }
    }

    fn matches(&self, text: &str) -> bool {
        if text.is_empty() {
            return true;
        }

        if let Some(text_value) = text.strip_prefix(">=") {
            if let Ok(value) = text_value.parse::<u32>() {
                return self.grade >= value;
            } else {
                return false;
            }
        }
        if let Some(text_value) = text.strip_prefix(">") {
            if let Ok(value) = text_value.parse::<u32>() {
                return self.grade > value;
            } else {
                return false;
            }
        }
        if let Some(text_value) = text.strip_prefix("<=") {
            if let Ok(value) = text_value.parse::<u32>() {
                return self.grade <= value;
            } else {
                return false;
            }
        }
        if let Some(text_value) = text.strip_prefix("<") {
            if let Ok(value) = text_value.parse::<u32>() {
                return self.grade < value;
            } else {
                return false;
            }
        }
        if let Some(text_value) = text.strip_prefix("=") {
            if let Ok(value) = text_value.parse::<u32>() {
                return self.grade == value;
            } else {
                return false;
            }
        }
        self.name.contains(text)
    }
}

#[Window()]
pub(crate) struct Win {}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("Students,d:c,w:70,h:10,flags: Sizeable"),
        };
        me.add(label!("'Use the following patter to filter the list: <, <=, >, >=, =, or the name of the student.\n\nFor example: >5 will show only students with grade greater than 5',x:0,y:0,w:40%,h:100%"));
        let mut lv = listview!("class: Student,x:50%,y:0,w:100%,h:100%,flags: SearchBar+CustomFilter, columns:[{&Name,15,l},{&Grade,12,r}]");
        lv.set_components_toolbar_margins(2, 1);
        lv.add(Student { name: "Alice", grade: 10 });
        lv.add(Student { name: "Zack", grade: 4 });
        lv.add(Student { name: "Bob", grade: 9 });
        lv.add(Student { name: "Charlie", grade: 8 });
        lv.add(Student { name: "David", grade: 7 });
        lv.add(Student { name: "Eve", grade: 6 });
        lv.add(Student { name: "Marjorie", grade: 10 });
        lv.add(Student { name: "Nancy", grade: 9 });
        lv.add(Student { name: "Oscar", grade: 6 });
        lv.add(Student { name: "Peter", grade: 7 });
        lv.add(Student { name: "Quincy", grade: 6 });
        lv.add(Student { name: "Rita", grade: 10 });
        lv.add(Student { name: "Steve", grade: 5 });
        lv.add(Student { name: "Tina", grade: 8 });
        lv.add(Student { name: "Ursula", grade: 5 });
        lv.add(Student { name: "Victor", grade: 6 });
        lv.add(Student { name: "Wendy", grade: 10 });
        lv.add(Student { name: "Xavier", grade: 9 });
        lv.add(Student { name: "Yvonne", grade: 8 });
        lv.sort(0, true);
        me.add(lv);
        me
    }
}
