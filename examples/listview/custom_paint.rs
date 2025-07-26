use appcui::prelude::*;

struct Face {
    name: &'static str,
    repr: &'static str,
    color: Color,
}
impl listview::ListItem for Face {
    fn render_method(&self, column_index: u16) -> Option<listview::RenderMethod> {
        match column_index {
            0 => Some(listview::RenderMethod::Text(self.name)),
            1 => Some(listview::RenderMethod::Custom),
            _ => None,
        }
    }

    fn compare(&self, other: &Self, column_index: u16) -> std::cmp::Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            _ => std::cmp::Ordering::Equal,
        }
    }

    fn paint(&self, column_index: u32, _width: u16, surface: &mut Surface, _theme: &Theme, attr: Option<CharAttribute>) {
        if column_index != 1 {
            return;
        }
        let attr = attr.unwrap_or(CharAttribute::with_fore_color(self.color));
        surface.write_string(0, 0, self.repr, attr, false);
    }
}

#[Window()]
pub(crate) struct Win {}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("'Custom Paint',a:c,w:70,h:10,flags: Sizeable"),
        };
        let mut lv = listview!(
            "class: Face,x:0,y:0,w:100%,h:100%,flags: ScrollBars+SearchBar, columns:[{&Name,15,l},{&Representation,20,c}]"
        );
        lv.add(Face {
            name: "Smile",
            repr: ":-)",
            color: Color::Green,
        });
        lv.add(Face {
            name: "Distant",
            repr: "(0-0)",
            color: Color::Pink,
        });
        lv.add(Face {
            name: "Pouty",
            repr: "(_^_)",
            color: Color::Magenta,
        });
        lv.add(Face {
            name: "Birdy Eyes",
            repr: "(*v*)",
            color: Color::Aqua,
        });
        lv.add(Face {
            name: "Sad",
            repr: "(-_-)",
            color: Color::White,
        });
        lv.add(Face {
            name: "Angry",
            repr: "(`_´)",
            color: Color::Red,
        });
        lv.add(Face {
            name: "Surprised",
            repr: "(⊙_☉)",
            color: Color::Yellow,
        });
        lv.add(Face {
            name: "Wink",
            repr: "(^_-)",
            color: Color::Green,
        });

        me.add(lv);
        me
    }
}
