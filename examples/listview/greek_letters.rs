use appcui::prelude::*;

struct GreekLetter {
    name: &'static str,
    description: &'static str,
}
impl listview::ListItem for GreekLetter {
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::Text(self.description)),
            _ => None,
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.description.cmp(other.description),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

#[Window(events = RadioBoxEvents)]
pub(crate) struct Win {
    rb_detailed: Handle<RadioBox>,
    rb_columns_2: Handle<RadioBox>,
    rb_columns_3: Handle<RadioBox>,
    lv: Handle<ListView<GreekLetter>>,
}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("'Greek Letters',d:c,w:70,h:15,flags: Sizeable"),
            rb_detailed: Handle::None,
            rb_columns_2: Handle::None,
            rb_columns_3: Handle::None,
            lv: Handle::None,
        };
        let mut lv = listview!("class: GreekLetter,l:0,t:5,r:0,b:0,flags: ScrollBars+SearchBar+LargeIcons+CheckBoxes, columns:[{&Name,15,l},{&Description,80,l}]");
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Alpha",
                description: "The first letter of the Greek alphabet, used to denote the beginning of something.",
            },
            false,
            None,
            ['α', 'Α'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Beta",
                description: "The second letter of the Greek alphabet, used to denote the second element in a sequence.",
            },
            false,
            None,
            ['β', 'Β'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Gamma",
                description: "The third letter of the Greek alphabet, used to denote the third element in a sequence.",
            },
            false,
            None,
            ['γ', 'Γ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Delta",
                description: "The fourth letter of the Greek alphabet, used to denote the fourth element in a sequence.",
            },
            false,
            None,
            ['δ', 'Δ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Epsilon",
                description: "The fifth letter of the Greek alphabet, used to denote the fifth element in a sequence.",
            },
            false,
            None,
            ['ε', 'Ε'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Zeta",
                description: "The sixth letter of the Greek alphabet, used to denote the sixth element in a sequence.",
            },
            false,
            None,
            ['ζ', 'Ζ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Eta",
                description: "The seventh letter of the Greek alphabet, used to denote the seventh element in a sequence.",
            },
            false,
            None,
            ['η', 'Η'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Theta",
                description: "The eighth letter of the Greek alphabet, used to denote the eighth element in a sequence.",
            },
            false,
            None,
            ['θ', 'Θ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Iota",
                description: "The ninth letter of the Greek alphabet, used to denote the ninth element in a sequence.",
            },
            false,
            None,
            ['ι', 'Ι'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Kappa",
                description: "The tenth letter of the Greek alphabet, used to denote the tenth element in a sequence.",
            },
            false,
            None,
            ['κ', 'Κ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Lambda",
                description: "The eleventh letter of the Greek alphabet, used to denote the eleventh element in a sequence.",
            },
            false,
            None,
            ['λ', 'Λ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Mu",
                description: "The twelfth letter of the Greek alphabet, used to denote the twelfth element in a sequence.",
            },
            false,
            None,
            ['μ', 'Μ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Nu",
                description: "The thirteenth letter of the Greek alphabet, used to denote the thirteenth element in a sequence.",
            },
            false,
            None,
            ['ν', 'Ν'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Xi",
                description: "The fourteenth letter of the Greek alphabet, used to denote the fourteenth element in a sequence.",
            },
            false,
            None,
            ['ξ', 'Ξ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Omicron",
                description: "The fifteenth letter of the Greek alphabet, used to denote the fifteenth element in a sequence.",
            },
            false,
            None,
            ['ο', 'Ο'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Pi",
                description: "The sixteenth letter of the Greek alphabet, used to denote the sixteenth element in a sequence.",
            },
            false,
            None,
            ['π', 'Π'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Rho",
                description: "The seventeenth letter of the Greek alphabet, used to denote the seventeenth element in a sequence.",
            },
            false,
            None,
            ['ρ', 'Ρ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Sigma",
                description: "The eighteenth letter of the Greek alphabet, used to denote the eighteenth element in a sequence.",
            },
            false,
            None,
            ['σ', 'Σ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Tau",
                description: "The nineteenth letter of the Greek alphabet, used to denote the nineteenth element in a sequence.",
            },
            false,
            None,
            ['τ', 'Τ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Upsilon",
                description: "The twentieth letter of the Greek alphabet, used to denote the twentieth element in a sequence.",
            },
            false,
            None,
            ['υ', 'Υ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Phi",
                description: "The twenty-first letter of the Greek alphabet, used to denote the twenty-first element in a sequence.",
            },
            false,
            None,
            ['φ', 'Φ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Chi",
                description: "The twenty-second letter of the Greek alphabet, used to denote the twenty-second element in a sequence.",
            },
            false,
            None,
            ['χ', 'Χ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Psi",
                description: "The twenty-third letter of the Greek alphabet, used to denote the twenty-third element in a sequence.",
            },
            false,
            None,
            ['ψ', 'Ψ'],
            listview::Group::None,
        ));
        lv.add_item(listview::Item::new(
            GreekLetter {
                name: "Omega",
                description: "The twenty-fourth letter of the Greek alphabet, used to denote the twenty-fourth element in a sequence.", 
            },
            false,
            None,
            ['ω', 'Ω'],
            listview::Group::None,
        ));        
        me.add(hline!("l:0,t:4,r:0"));
        me.rb_detailed = me.add(radiobox!("'&Detailed view',x:1,y:1,w:20,selected: true"));
        me.rb_columns_2 = me.add(radiobox!("'&2 columns view',x:1,y:2,w:20,selected: false"));
        me.rb_columns_3 = me.add(radiobox!("'&3 columns view',x:1,y:3,w:20, selected: false"));
        me.lv = me.add(lv);
        me
    }
}

impl RadioBoxEvents for Win {
    fn on_selected(&mut self, handle: Handle<RadioBox>) -> EventProcessStatus {
        let lvh = self.lv;
        let d = self.rb_detailed;
        let c_2 = self.rb_columns_2;
        let c_3 = self.rb_columns_3;
        if let Some(lv) = self.control_mut(lvh) {
            if handle == d {
                lv.set_view_mode(listview::ViewMode::Details);
            } else if handle == c_2 {
                lv.set_view_mode(listview::ViewMode::Columns(2));
            } else if handle == c_3 {
                lv.set_view_mode(listview::ViewMode::Columns(3));
            }
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}