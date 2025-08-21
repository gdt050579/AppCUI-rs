use appcui::graphics::Color;
use appcui::prelude::*;
use chrono::NaiveDate;

#[derive(ListItem)]
struct Task {
    #[Column(name: "&Description", width: 30)]
    desc: &'static str,
    #[Column(name: "&Created", width: 12, align: center, render: date, format: YearMonthDay)]
    created: NaiveDate,
    #[Column(name: "&Status", width: 20, align: center, render: status, format: Block)]
    status: listitem::Status,
}

#[Window()]
pub(crate) struct Win {}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("Tasks,a:c,w:70,h:12,flags: Sizeable"),
        };
        let mut lv = listview!("class: Task,d:f,flags: [ScrollBars, SearchBar, LargeIcons, ShowGroups]");

        let g_in_progress = lv.add_group("In progress");
        let attr = Some(CharAttribute::with_fore_color(Color::White));
        lv.add_item(listview::Item::new(
            Task {
                desc: "Develop a new software",
                created: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                status: listitem::Status::Running(0.25),
            },
            false,
            attr,
            ['⏯', ' '],
            g_in_progress,
        ));
        lv.add_item(listview::Item::new(
            Task {
                desc: "Writing unit tests",
                created: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
                status: listitem::Status::Running(0.75),
            },
            false,
            attr,
            ['⏯', ' '],
            g_in_progress,
        ));
        lv.add_item(listview::Item::new(
            Task {
                desc: "Working on documentation",
                created: NaiveDate::from_ymd_opt(2025, 2, 6).unwrap(),
                status: listitem::Status::Running(0.85),
            },
            false,
            attr,
            ['⏯', ' '],
            g_in_progress,
        ));

        let g_paused = lv.add_group("Paused");
        let attr = Some(CharAttribute::with_fore_color(Color::Olive));
        lv.add_item(listview::Item::new(
            Task {
                desc: "Linux support",
                created: NaiveDate::from_ymd_opt(2025, 2, 1).unwrap(),
                status: listitem::Status::Paused(0.60),
            },
            false,
            attr,
            ['⏸', ' '],
            g_paused,
        ));
        lv.add_item(listview::Item::new(
            Task {
                desc: "MAC/OSX support",
                created: NaiveDate::from_ymd_opt(2025, 2, 5).unwrap(),
                status: listitem::Status::Paused(0.20),
            },
            false,
            attr,
            ['⏸', ' '],
            g_paused,
        ));   

        let g_stopped = lv.add_group("Stopped");
        let attr = Some(CharAttribute::with_fore_color(Color::Red));
        lv.add_item(listview::Item::new(
            Task {
                desc: "Old version compatibility",
                created: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
                status: listitem::Status::Stopped
            },
            false,
            attr,
            ['⏏', ' '],
            g_stopped,
        ));             

        me.add(lv);
        me
    }
}
