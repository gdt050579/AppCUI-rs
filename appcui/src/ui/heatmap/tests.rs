use std::sync::Mutex;

use crate::prelude::*;

static HEATMAP_EVENTS: Mutex<Vec<Option<(u32, u32)>>> = Mutex::new(Vec::new());

fn in_app(test: impl FnOnce() + 'static) {
    App::new()
        .size(Size::new(40, 12))
        .debug_script("Paint.Enable(false)")
        .window(|| {
            test();
            window!("Test,d:f")
        })
        .run()
        .unwrap();
}

fn press(map: &mut HeatMap<i32>, key: impl Into<Key>) -> EventProcessStatus {
    map.on_key_pressed(key.into(), '\0')
}

fn is_processed(status: EventProcessStatus) -> bool {
    status == EventProcessStatus::Processed
}

fn click(map: &mut HeatMap<i32>, x: i32, y: i32) -> EventProcessStatus {
    map.on_mouse_event(&MouseEvent::Pressed(MouseEventData {
        x,
        y,
        button: MouseButton::Left,
        modifier: KeyModifier::None,
    }))
}

fn filled(width: u32, height: u32) -> HeatMap<i32> {
    let mut map = HeatMap::new(layout!("d:f"), Size::new(width, height), heatmap::Flags::None);
    let count = (width * height) as usize;
    map.set_values(1..=count as i32);
    map
}

#[test]
fn check_click_selects_and_clears() {
    in_app(|| {
        let mut map = filled(3, 2);
        assert_eq!(map.selected_cell(), None);
        assert!(is_processed(click(&mut map, 0, 0)));
        assert_eq!(map.selected_cell(), Some((0, 0)));
        // the same cell again does not raise another change
        assert!(!is_processed(click(&mut map, 1, 0)));
        assert_eq!(map.selected_cell(), Some((0, 0)));
        // column 1 starts at x = 2
        assert!(is_processed(click(&mut map, 2, 0)));
        assert_eq!(map.selected_cell(), Some((0, 1)));
        // row 1
        assert!(is_processed(click(&mut map, 4, 1)));
        assert_eq!(map.selected_cell(), Some((1, 2)));
        // past the grid
        assert!(is_processed(click(&mut map, 8, 0)));
        assert_eq!(map.selected_cell(), None);
        assert!(!is_processed(click(&mut map, 8, 0)));
    });
}

#[test]
fn check_click_on_empty_cell_clears() {
    in_app(|| {
        let mut map = filled(2, 2);
        map.clear_cell(0, 1);
        assert!(is_processed(click(&mut map, 0, 0)));
        assert_eq!(map.selected_cell(), Some((0, 0)));
        assert!(is_processed(click(&mut map, 2, 0)));
        assert_eq!(map.selected_cell(), None);
    });
}

#[test]
fn check_margins_shift_the_hit_test() {
    in_app(|| {
        let mut map = HeatMap::new(layout!("d:f"), Size::new(2, 2), heatmap::Flags::Margins);
        map.set_values([1, 2, 3, 4]);
        assert!(!is_processed(click(&mut map, 0, 0)));
        assert_eq!(map.selected_cell(), None);
        assert!(is_processed(click(&mut map, 1, 1)));
        assert_eq!(map.selected_cell(), Some((0, 0)));
        assert!(!is_processed(click(&mut map, 2, 1)));
        assert!(is_processed(click(&mut map, 3, 1)));
        assert_eq!(map.selected_cell(), Some((0, 1)));
        assert!(is_processed(click(&mut map, 1, 2)));
        assert_eq!(map.selected_cell(), Some((1, 0)));
    });
}

#[test]
fn check_arrow_keys_wrap_and_skip_empty_cells() {
    in_app(|| {
        let mut map = filled(2, 2);
        map.clear_cell(0, 1);
        map.clear_cell(1, 0);
        // filled cells are (0, 0) and (1, 1)
        assert!(is_processed(press(&mut map, key!("Right"))));
        assert_eq!(map.selected_cell(), Some((0, 0)));
        assert!(is_processed(press(&mut map, key!("Right"))));
        assert_eq!(map.selected_cell(), Some((1, 1)));
        assert!(is_processed(press(&mut map, key!("Right"))));
        assert_eq!(map.selected_cell(), Some((0, 0)));
        assert!(is_processed(press(&mut map, key!("Left"))));
        assert_eq!(map.selected_cell(), Some((1, 1)));
        map.set_selected_cell(None);
        assert!(is_processed(press(&mut map, key!("Left"))));
        assert_eq!(map.selected_cell(), Some((1, 1)));
        map.set_selected_cell(None);
        assert!(is_processed(press(&mut map, key!("Down"))));
        assert_eq!(map.selected_cell(), Some((0, 0)));
        assert!(is_processed(press(&mut map, key!("Down"))));
        assert_eq!(map.selected_cell(), Some((1, 1)));
        assert!(is_processed(press(&mut map, key!("Up"))));
        assert_eq!(map.selected_cell(), Some((0, 0)));
        map.set_selected_cell(None);
        assert!(is_processed(press(&mut map, key!("Up"))));
        assert_eq!(map.selected_cell(), Some((1, 1)));
    });
}

#[test]
fn check_escape_clears_selection() {
    in_app(|| {
        let mut map = filled(2, 1);
        assert!(!is_processed(press(&mut map, key!("Escape"))));
        click(&mut map, 0, 0);
        assert_eq!(map.selected_cell(), Some((0, 0)));
        assert!(is_processed(press(&mut map, key!("Escape"))));
        assert_eq!(map.selected_cell(), None);
        assert!(!is_processed(press(&mut map, key!("Escape"))));
    });
}

#[test]
fn check_ctrl_arrows_scroll_without_selecting() {
    in_app(|| {
        let mut map = filled(4, 2);
        assert!(is_processed(press(&mut map, key!("Ctrl+Right"))));
        assert_eq!(map.selected_cell(), None);
        assert!(is_processed(press(&mut map, key!("Right"))));
        assert_eq!(map.selected_cell(), Some((0, 0)));
        assert!(is_processed(press(&mut map, key!("Ctrl+Left"))));
        assert_eq!(map.selected_cell(), Some((0, 0)));
    });
}

#[test]
fn check_set_selected_cell_and_clear() {
    in_app(|| {
        let mut map = filled(3, 2);
        map.set_selected_cell(Some((1, 2)));
        assert_eq!(map.selected_cell(), Some((1, 2)));
        map.set_selected_cell(Some((9, 9)));
        assert_eq!(map.selected_cell(), Some((1, 2)));
        map.set_selected_cell(None);
        assert_eq!(map.selected_cell(), None);
        map.set_selected_cell(Some((0, 1)));
        map.clear_cell(0, 1);
        assert_eq!(map.selected_cell(), None);
        map.set_selected_cell(Some((0, 0)));
        map.clear_all();
        assert_eq!(map.selected_cell(), None);
        assert!(map.cell(0, 0).is_none());
    });
}

#[test]
fn check_set_heatmap_size() {
    in_app(|| {
        let mut map: HeatMap<i32> = HeatMap::new(layout!("d:f"), Size::new(3, 2), heatmap::Flags::None);
        let attr = heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black));
        map.set_values([1, 2, 3, 4, 5, 6]);
        map.update_cell(1, 1, |cell| {
            cell.set_label("Peak");
            cell.set_fill(heatmap::CellFill::Stars);
            cell.set_attr(attr);
        });
        map.set_selected_cell(Some((1, 1)));

        map.set_heatmap_size(Size::new(0, 4));
        map.set_heatmap_size(Size::new(4, 0));
        map.set_heatmap_size(Size::new(3, 2));
        assert_eq!(map.heatmap_size(), Size::new(3, 2));
        assert_eq!(map.selected_cell(), Some((1, 1)));

        map.set_heatmap_size(Size::new(4, 3));
        assert_eq!(map.heatmap_size(), Size::new(4, 3));
        assert_eq!(map.cell(0, 0).map(|cell| cell.value()), Some(1));
        assert_eq!(map.cell(0, 1).map(|cell| cell.value()), Some(2));
        assert_eq!(map.cell(0, 2).map(|cell| cell.value()), Some(3));
        assert!(map.cell(0, 3).is_none());
        assert_eq!(map.cell(1, 0).map(|cell| cell.value()), Some(4));
        let peak = map.cell(1, 1).unwrap();
        assert_eq!(peak.value(), 5);
        assert_eq!(peak.label(), "Peak");
        assert_eq!(peak.fill(), Some(heatmap::CellFill::Stars));
        assert_eq!(peak.attr(), Some(attr));
        assert_eq!(map.cell(1, 2).map(|cell| cell.value()), Some(6));
        assert!(map.cell(1, 3).is_none());
        assert!(map.cell(2, 0).is_none());
        assert!(map.cell(2, 3).is_none());
        assert_eq!(map.selected_cell(), Some((1, 1)));

        map.set(2, 3, 9);
        assert_eq!(map.cell(2, 3).map(|cell| cell.value()), Some(9));
        map.set_selected_cell(Some((0, 3)));
        map.set_heatmap_size(Size::new(2, 2));
        assert_eq!(map.heatmap_size(), Size::new(2, 2));
        assert_eq!(map.cell(0, 0).map(|cell| cell.value()), Some(1));
        assert_eq!(map.cell(0, 1).map(|cell| cell.value()), Some(2));
        assert_eq!(map.cell(1, 0).map(|cell| cell.value()), Some(4));
        let peak = map.cell(1, 1).unwrap();
        assert_eq!(peak.value(), 5);
        assert_eq!(peak.label(), "Peak");
        assert_eq!(peak.fill(), Some(heatmap::CellFill::Stars));
        assert_eq!(peak.attr(), Some(attr));
        assert!(map.cell(0, 2).is_none());
        assert!(map.cell(1, 2).is_none());
        assert!(map.cell(2, 0).is_none());
        assert_eq!(map.selected_cell(), None);

        map.set_selected_cell(Some((0, 0)));
        map.set_heatmap_size(Size::new(1, 2));
        assert_eq!(map.heatmap_size(), Size::new(1, 2));
        assert_eq!(map.cell(0, 0).map(|cell| cell.value()), Some(1));
        assert_eq!(map.cell(1, 0).map(|cell| cell.value()), Some(4));
        assert!(map.cell(0, 1).is_none());
        assert_eq!(map.selected_cell(), Some((0, 0)));
    });
}

#[test]
fn check_empty_map_arrows_clear_selection() {
    in_app(|| {
        let mut map: HeatMap<i32> = HeatMap::new(layout!("d:f"), Size::new(2, 2), heatmap::Flags::DimCellsOnSelection);
        map.set_selected_cell(Some((1, 1)));
        assert!(is_processed(press(&mut map, key!("Right"))));
        assert_eq!(map.selected_cell(), None);
        assert!(is_processed(press(&mut map, key!("Down"))));
        assert_eq!(map.selected_cell(), None);
    });
}

#[test]
fn check_selection_events() {
    #[Window(events = HeatMapEvents<i32>, internal: true)]
    struct Probe {}
    impl Probe {
        fn new() -> Self {
            let mut win = Self { base: window!("Test,d:f") };
            let mut map = HeatMap::<i32>::new(layout!("x:0,y:0,w:30,h:8"), Size::new(4, 3), heatmap::Flags::None);
            map.set_values(1..=12);
            win.add(map);
            win
        }
    }
    impl HeatMapEvents<i32> for Probe {
        fn on_cell_selected(&mut self, _: Handle<HeatMap<i32>>, row: u32, column: u32) -> EventProcessStatus {
            HEATMAP_EVENTS.lock().unwrap().push(Some((row, column)));
            EventProcessStatus::Processed
        }
        fn on_clear_selection(&mut self, _: Handle<HeatMap<i32>>) -> EventProcessStatus {
            HEATMAP_EVENTS.lock().unwrap().push(None);
            EventProcessStatus::Processed
        }
    }
    HEATMAP_EVENTS.lock().unwrap().clear();
    let script = "
        Paint.Enable(false)
        Mouse.Click(1,1,left)
        Mouse.Click(3,1,left)
        Key.Pressed(Right)
        Key.Pressed(Down)
        Mouse.Click(20,4,left)
        Key.Pressed(Escape)
    ";
    App::new().size(Size::new(40, 12)).debug_script(script).window(Probe::new).run().unwrap();
    assert_eq!(
        HEATMAP_EVENTS.lock().unwrap().as_slice(),
        &[Some((0, 0)), Some((0, 1)), Some((0, 2)), Some((1, 2)), None,]
    );
}

#[test]
fn check_cell_builder() {
    let cell = heatmap::CellBuilder::<i32>::new(10).build();
    assert_eq!(cell.value(), 10);
    assert_eq!(cell.label(), "");
    assert_eq!(cell.fill(), None);
    assert_eq!(cell.attr(), None);

    let attr = heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black));
    let cell = heatmap::CellBuilder::new(10)
        .label("first")
        .label("Peak")
        .fill(heatmap::CellFill::Shades)
        .fill(heatmap::CellFill::Custom('#'))
        .attr(attr)
        .build();
    assert_eq!(cell.value(), 10);
    assert_eq!(cell.label(), "Peak");
    assert_eq!(cell.fill(), Some(heatmap::CellFill::Custom('#')));
    assert_eq!(cell.attr(), Some(attr));

    let cell: heatmap::Cell<i32> = heatmap::CellBuilder::new(4).label("").fill(heatmap::CellFill::Stars).into();
    assert_eq!(cell.value(), 4);
    assert_eq!(cell.label(), "");
    assert_eq!(cell.fill(), Some(heatmap::CellFill::Stars));
    assert_eq!(cell.attr(), None);
}

#[test]
fn check_cell_builder_is_accepted_by_set() {
    in_app(|| {
        let mut map: HeatMap<i32> = HeatMap::new(layout!("d:f"), Size::new(2, 2), heatmap::Flags::None);
        let attr = heatmap::CellAttribute::Range(
            CharAttribute::with_color(Color::Blue, Color::Black),
            CharAttribute::with_color(Color::Red, Color::Black),
        );
        map.set(
            1,
            0,
            heatmap::CellBuilder::new(7).label("Source").fill(heatmap::CellFill::Circles).attr(attr),
        );
        let cell = map.cell(1, 0).unwrap();
        assert_eq!(cell.value(), 7);
        assert_eq!(cell.label(), "Source");
        assert_eq!(cell.fill(), Some(heatmap::CellFill::Circles));
        assert_eq!(cell.attr(), Some(attr));
    });
}

#[test]
fn check_cell_set_fill_attr_and_label() {
    in_app(|| {
        let mut map: HeatMap<i32> = HeatMap::new(layout!("d:f"), Size::new(2, 1), heatmap::Flags::None);
        map.set(0, 0, 10);
        let fixed = heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black));
        let range = heatmap::CellAttribute::Range(
            CharAttribute::with_color(Color::Blue, Color::Black),
            CharAttribute::with_color(Color::Yellow, Color::Black),
        );
        map.update_cell(0, 0, |cell| {
            cell.set_fill(heatmap::CellFill::Circles);
            cell.set_attr(fixed);
            cell.set_label("Peak");
        });
        let cell = map.cell(0, 0).unwrap();
        assert_eq!(cell.value(), 10);
        assert_eq!(cell.fill(), Some(heatmap::CellFill::Circles));
        assert_eq!(cell.attr(), Some(fixed));
        assert_eq!(cell.label(), "Peak");

        map.update_cell(0, 0, |cell| cell.set_fill(heatmap::CellFill::Stars));
        assert_eq!(map.cell(0, 0).unwrap().fill(), Some(heatmap::CellFill::Stars));
        assert_eq!(map.cell(0, 0).unwrap().attr(), Some(fixed));
        assert_eq!(map.cell(0, 0).unwrap().label(), "Peak");

        map.update_cell(0, 0, |cell| cell.clear_fill());
        assert_eq!(map.cell(0, 0).unwrap().fill(), None);
        assert_eq!(map.cell(0, 0).unwrap().attr(), Some(fixed));
        assert_eq!(map.cell(0, 0).unwrap().label(), "Peak");

        map.update_cell(0, 0, |cell| cell.set_attr(range));
        assert_eq!(map.cell(0, 0).unwrap().attr(), Some(range));
        map.update_cell(0, 0, |cell| cell.clear_attr());
        assert_eq!(map.cell(0, 0).unwrap().attr(), None);
        assert_eq!(map.cell(0, 0).unwrap().label(), "Peak");

        map.update_cell(0, 0, |cell| cell.set_label("Next"));
        assert_eq!(map.cell(0, 0).unwrap().label(), "Next");
        map.update_cell(0, 0, |cell| cell.set_label(""));
        assert_eq!(map.cell(0, 0).unwrap().label(), "");
        assert_eq!(map.cell(0, 0).unwrap().value(), 10);

        // an empty cell and a position outside the grid are ignored
        map.update_cell(0, 1, |cell| cell.set_label("missing"));
        assert!(map.cell(0, 1).is_none());
        map.update_cell(9, 9, |cell| cell.set_label("outside"));
    });
}

#[test]
fn check_clear_default_attr() {
    in_app(|| {
        let mut map: HeatMap<i32> = HeatMap::new(layout!("d:f"), Size::new(2, 1), heatmap::Flags::None);
        assert_eq!(map.default_attr(), None);
        map.clear_default_attr();
        assert_eq!(map.default_attr(), None);

        let default_attr = heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black));
        let cell_attr = heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Green, Color::Black));
        map.set_default_attr(default_attr);
        map.set_default_fill(heatmap::CellFill::Dots);
        map.set(0, 0, 1);
        map.set(
            0,
            1,
            heatmap::CellBuilder::new(2).label("Own").fill(heatmap::CellFill::Stars).attr(cell_attr),
        );
        assert_eq!(map.default_attr(), Some(default_attr));

        map.clear_default_attr();
        assert_eq!(map.default_attr(), None);
        assert_eq!(map.default_fill(), Some(heatmap::CellFill::Dots));
        assert_eq!(map.cell(0, 0).unwrap().attr(), None);
        assert_eq!(map.cell(0, 0).unwrap().value(), 1);
        let own = map.cell(0, 1).unwrap();
        assert_eq!(own.value(), 2);
        assert_eq!(own.label(), "Own");
        assert_eq!(own.fill(), Some(heatmap::CellFill::Stars));
        assert_eq!(own.attr(), Some(cell_attr));

        map.clear_default_attr();
        assert_eq!(map.default_attr(), None);
        assert_eq!(map.cell(0, 1).unwrap().attr(), Some(cell_attr));
    });
}

#[test]
fn check_fill_with() {
    // row 0 is Dots at 0, 25, 50, 75, 100 → " ·•●●"
    // row 1 leaves the ends empty and paints shades at 0, 50, 100
    let script = "
        Paint.Enable(false)
        Paint('fill_with')
        CheckHash(0x5E35E9574F918DC7)
    ";
    App::new()
        .size(Size::new(40, 8))
        .debug_script(script)
        .window(|| {
            let mut win = window!("Fills,d:f");
            let mut map = HeatMap::<i32>::new(layout!("d:f"), Size::new(5, 2), heatmap::Flags::None);
            map.set_cell_size(Size::new(1, 1));
            map.fill_with(|row, column| Some((row * 5 + column) as i32));
            assert_eq!(map.cell(0, 0).map(|cell| cell.value()), Some(0));
            assert_eq!(map.cell(0, 4).map(|cell| cell.value()), Some(4));
            assert_eq!(map.cell(1, 0).map(|cell| cell.value()), Some(5));
            assert_eq!(map.cell(1, 4).map(|cell| cell.value()), Some(9));
            assert_eq!(map.cell(0, 0).unwrap().label(), "");
            assert_eq!(map.cell(0, 0).unwrap().fill(), None);

            map.fill_with(|row, column| {
                if row == 1 && (column == 0 || column == 4) {
                    return None;
                }
                let value = if row == 0 { column as i32 * 25 } else { (column as i32 - 1) * 50 };
                let mut builder = heatmap::CellBuilder::new(value);
                if row == 0 {
                    builder = builder.label("top").fill(heatmap::CellFill::Dots);
                }
                Some(builder)
            });
            assert!(map.cell(1, 0).is_none());
            assert!(map.cell(1, 4).is_none());
            let top = map.cell(0, 2).unwrap();
            assert_eq!(top.value(), 50);
            assert_eq!(top.label(), "top");
            assert_eq!(top.fill(), Some(heatmap::CellFill::Dots));
            assert_eq!(map.cell(0, 0).map(|cell| cell.value()), Some(0));
            assert_eq!(map.cell(0, 4).map(|cell| cell.value()), Some(100));
            assert_eq!(map.cell(1, 1).map(|cell| cell.value()), Some(0));
            assert_eq!(map.cell(1, 1).unwrap().fill(), None);
            assert_eq!(map.cell(1, 1).unwrap().label(), "");
            assert_eq!(map.cell(1, 2).map(|cell| cell.value()), Some(50));
            assert_eq!(map.cell(1, 3).map(|cell| cell.value()), Some(100));
            assert_eq!(map.heatmap_size(), Size::new(5, 2));
            win.add(map);
            win
        })
        .run()
        .unwrap();
}

#[test]
fn check_clear_default_fill() {
    // after the clear, cells with no fill of their own use shades: " ▒█" and the last cell keeps stars "✹"
    let script = "
        Paint.Enable(false)
        Paint('clear default fill')
        CheckHash(0xB6E0E51E9AAC0912)
    ";
    App::new()
        .size(Size::new(40, 12))
        .debug_script(script)
        .window(|| {
            let mut win = window!("Fills,d:f");
            let mut map = HeatMap::<i32>::new(layout!("d:f"), Size::new(4, 1), heatmap::Flags::None);
            map.set_cell_size(Size::new(1, 1));
            assert_eq!(map.default_fill(), None);
            map.clear_default_fill();
            assert_eq!(map.default_fill(), None);

            let attr = heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black));
            map.set_default_attr(attr);
            map.set_default_fill(heatmap::CellFill::Dots);
            map.set_values([0, 50, 100, 100]);
            map.update_cell(0, 3, |cell| {
                cell.set_fill(heatmap::CellFill::Stars);
                cell.set_label("Peak");
            });
            assert_eq!(map.default_fill(), Some(heatmap::CellFill::Dots));

            map.clear_default_fill();
            assert_eq!(map.default_fill(), None);
            assert_eq!(map.default_attr(), Some(attr));
            assert_eq!(map.cell(0, 0).unwrap().fill(), None);
            assert_eq!(map.cell(0, 2).unwrap().fill(), None);
            assert_eq!(map.cell(0, 3).unwrap().fill(), Some(heatmap::CellFill::Stars));
            assert_eq!(map.cell(0, 3).unwrap().label(), "Peak");
            map.clear_default_fill();
            assert_eq!(map.default_fill(), None);
            assert_eq!(map.cell(0, 3).unwrap().fill(), Some(heatmap::CellFill::Stars));
            win.add(map);
            win
        })
        .run()
        .unwrap();
}

fn hover_script(title: &str, moves: &[(i32, i32, &str)], hashes: &[u64]) -> String {
    let mut script = format!(
        "
        Paint.Enable(false)
        Paint('{title}')
        CheckHash(0x{idle:X})
    ",
        idle = hashes[0]
    );
    for (index, (x, y, name)) in moves.iter().enumerate() {
        script.push_str(&format!(
            "        Mouse.Move({x},{y})\n        Paint('{title}: {name}')\n        CheckHash(0x{hash:X})\n",
            hash = hashes[index + 1]
        ));
    }
    script
}

fn show_heatmap(script: &str, map: impl FnOnce() -> HeatMap<i32> + 'static) {
    App::new()
        .size(Size::new(40, 12))
        .debug_script(script)
        .window(move || {
            let mut win = window!("Heat,d:f");
            win.add(map());
            win
        })
        .run()
        .unwrap();
}

fn labeled_map(format: heatmap::TooltipFormat) -> HeatMap<i32> {
    let mut map = HeatMap::new(layout!("d:f"), Size::new(4, 1), heatmap::Flags::None);
    map.set_tooltip_format(format);
    map.set(0, 0, heatmap::CellBuilder::new(0).label("Low"));
    map.set(0, 1, heatmap::CellBuilder::new(50).label("Mid"));
    map.set(0, 2, 100);
    map
}

#[test]
fn check_tooltip_formats() {
    // cells at screen (1,1), (3,1), (5,1) and an empty cell at (7,1). (2,1) stays on Low.
    let moves = [(1, 1, "Low"), (2, 1, "Low again"), (3, 1, "Mid"), (5, 1, "100"), (7, 1, "empty"), (0, 0, "left")];
    let formats = [
        heatmap::TooltipFormat::Value,
        heatmap::TooltipFormat::Label,
        heatmap::TooltipFormat::Percentage,
        heatmap::TooltipFormat::LabelAndValue,
        heatmap::TooltipFormat::LabelAndPercentage,
    ];
    for format in formats {
        // idle, Low, Low again, Mid, 100, empty, mouse left
        let hashes = match format {
            heatmap::TooltipFormat::Value => [
                0x4D077C171E7587EB,
                0xA6FD8D8CFE6B228B,
                0xA6FD8D8CFE6B228B,
                0xF9487E3ADC46430E,
                0xA78CC673D8E28BA7,
                0x4D077C171E7587EB,
                0x4D077C171E7587EB,
            ],
            heatmap::TooltipFormat::Label => [
                0x4D077C171E7587EB,
                0xD19E487401D92BBB,
                0xD19E487401D92BBB,
                0xC0FD6B9A07069793,
                0x4D077C171E7587EB,
                0x4D077C171E7587EB,
                0x4D077C171E7587EB,
            ],
            heatmap::TooltipFormat::Percentage => [
                0x4D077C171E7587EB,
                0x19A1309FD72D739E,
                0x19A1309FD72D739E,
                0x5B5C4F317FC57D5B,
                0xA7EBFB065C3335D2,
                0x4D077C171E7587EB,
                0x4D077C171E7587EB,
            ],
            heatmap::TooltipFormat::LabelAndValue => [
                0x4D077C171E7587EB,
                0x423086B8FED86B13,
                0x423086B8FED86B13,
                0xA866FEDB3EF0F7E0,
                0xA78CC673D8E28BA7,
                0x4D077C171E7587EB,
                0x4D077C171E7587EB,
            ],
            heatmap::TooltipFormat::LabelAndPercentage => [
                0x4D077C171E7587EB,
                0x9FC0A9B498E36798,
                0x9FC0A9B498E36798,
                0xE20D0B2C3A4DD25B,
                0xA7EBFB065C3335D2,
                0x4D077C171E7587EB,
                0x4D077C171E7587EB,
            ],
        };
        let script = hover_script(&format!("{format:?}"), &moves, &hashes);
        show_heatmap(&script, move || labeled_map(format));
    }
}

#[test]
fn check_percentage() {
    // 0, 1, 2, 3 and NaN → 0%, 33%, 67%, 100%, 0%
    let moves = [(1, 1, "0%"), (3, 1, "33%"), (5, 1, "67%"), (7, 1, "100%"), (9, 1, "NaN"), (0, 0, "left")];
    let script = hover_script(
        "percentage",
        &moves,
        &[
            0x49E829379152201B,
            0xC0B0136566540DAE,
            0x65E1E55E0330BA0A,
            0xFBE152EAFAB666FE,
            0x8EFC55A9A20946C2,
            0x60BB14B50BF74003,
            0x49E829379152201B,
        ],
    );
    App::new()
        .size(Size::new(40, 12))
        .debug_script(&script)
        .window(|| {
            let mut win = window!("Heat,d:f");
            let mut map = HeatMap::<f64>::new(layout!("d:f"), Size::new(5, 1), heatmap::Flags::None);
            map.set_tooltip_format(heatmap::TooltipFormat::Percentage);
            map.set_values([0.0, 1.0, 2.0, 3.0, f64::NAN]);
            win.add(map);
            win
        })
        .run()
        .unwrap();

    // every value is the same, so each cell is 100%
    let moves = [(1, 1, "100%"), (3, 1, "100% again"), (5, 1, "100% last"), (0, 0, "left")];
    let script = hover_script(
        "equal",
        &moves,
        &[0xF12D459773962D07, 0x56DEDF470943FD9B, 0x1586D1781CC58C9B, 0x81C065B6B34CAFA6, 0xF12D459773962D07],
    );
    App::new()
        .size(Size::new(40, 12))
        .debug_script(&script)
        .window(|| {
            let mut win = window!("Heat,d:f");
            let mut map = HeatMap::<i32>::new(layout!("d:f"), Size::new(3, 1), heatmap::Flags::None);
            map.set_tooltip_format(heatmap::TooltipFormat::Percentage);
            map.set_values([8, 8, 8]);
            win.add(map);
            win
        })
        .run()
        .unwrap();
}

fn paint_selection(title: &str, flags: heatmap::Flags, height: u32, y: i32, left_x: i32, right_x: i32, hashes: [u64; 3]) {
    let script = format!(
        "
        Paint.Enable(false)
        Paint('{title}')
        CheckHash(0x{idle:X})
        Mouse.Click({left_x},{y},left)
        Paint('{title} left')
        CheckHash(0x{left:X})
        Mouse.Click({right_x},{y},left)
        Paint('{title} right')
        CheckHash(0x{right:X})
        ",
        idle = hashes[0],
        left = hashes[1],
        right = hashes[2]
    );
    App::new()
        .size(Size::new(40, 12))
        .debug_script(&script)
        .window(move || {
            let mut win = window!("Select,d:f");
            let mut map = HeatMap::<i32>::new(layout!("d:f"), Size::new(3, 1), flags);
            map.set_cell_size(Size::new(2, height));
            map.set_tooltip_format(heatmap::TooltipFormat::Label);
            map.set_default_fill(heatmap::CellFill::Custom('#'));
            map.set_values([1, 2, 3]);
            map.set_selected_cell(Some((0, 1)));
            win.add(map);
            win
        })
        .run()
        .unwrap();
}

#[test]
fn check_selection_margins() {
    paint_selection("margins", heatmap::Flags::Margins, 1, 2, 2, 6, [0x41654988EF5BF0F3, 0xEA4ECAB6077C5C0C, 0x4C4E9B08D7A6E7AC]);
}

#[test]
fn check_selection_height_1() {
    paint_selection("height 1", heatmap::Flags::None, 1, 1, 1, 5, [0xFC3DF431387483FD, 0x9FC0A7F28675DC5D, 0x8146D705FDE27CB2]);
}

#[test]
fn check_selection_height_2() {
    paint_selection("height 2", heatmap::Flags::None, 2, 1, 1, 5, [0xC50939A1B5680D53, 0x1BBC297F17CB0EF3, 0x2B60F77CA9084393]);
}

#[test]
fn check_selection_height_3() {
    paint_selection("height 3", heatmap::Flags::None, 3, 1, 1, 5, [0x8609942E2E199AF3, 0x908540C6C7E0343F, 0x1A77E3465EB8881C]);
}

fn scrollable_map() -> HeatMap<i32> {
    let mut map = HeatMap::new(layout!("x:0,y:0,w:10,h:4"), Size::new(8, 8), heatmap::Flags::ScrollBars);
    map.set_cell_size(Size::new(2, 1));
    map.set_tooltip_format(heatmap::TooltipFormat::Label);
    map.fill_with(|row, column| {
        let ch = (b'A' + ((row * 8 + column) % 26) as u8) as char;
        Some(heatmap::CellBuilder::new(1).fill(heatmap::CellFill::Custom(ch)))
    });
    map
}

fn step_script(steps: &[(&str, &str)], hashes: &[u64]) -> String {
    let mut script = format!(
        "
        Paint.Enable(false)
        Paint('start')
        CheckHash(0x{idle:X})
    ",
        idle = hashes[0]
    );
    for (index, (command, name)) in steps.iter().enumerate() {
        script.push_str(&format!(
            "        {command}\n        Paint('{name}')\n        CheckHash(0x{hash:X})\n",
            hash = hashes[index + 1]
        ));
    }
    script
}

fn show_scrollable(script: &str) {
    App::new()
        .size(Size::new(40, 14))
        .debug_script(script)
        .window(|| {
            let mut win = window!("Keys,d:f");
            win.add(scrollable_map());
            win
        })
        .run()
        .unwrap();
}

#[test]
fn check_supported_keys() {
    let steps = [
        ("Key.Pressed(Right)", "Right"),
        ("Key.Pressed(Right)", "Right again"),
        ("Key.Pressed(Down)", "Down"),
        ("Key.Pressed(Left)", "Left"),
        ("Key.Pressed(Up)", "Up"),
        ("Key.Pressed(Escape)", "Escape"),
        ("Key.Pressed(Ctrl+Right)", "Ctrl+Right"),
        ("Key.Pressed(Ctrl+Down)", "Ctrl+Down"),
        ("Key.Pressed(Ctrl+Left)", "Ctrl+Left"),
        ("Key.Pressed(Ctrl+Up)", "Ctrl+Up"),
        ("Key.Pressed(Ctrl+End)", "Ctrl+End"),
        ("Key.Pressed(Ctrl+Home)", "Ctrl+Home"),
        ("Key.Pressed(Left)", "Left from none"),
    ];
    show_scrollable(&step_script(
        &steps,
        &[
            0xADB5D78E906BEC7E,
            0xCF44DEB031DFA47B,
            0xF1644017ADEC1F12,
            0xF418D948E3626D52,
            0x21AA86ED08886223,
            0xCF44DEB031DFA47B,
            0xADB5D78E906BEC7E,
            0xA671BAB0DA956306,
            0x67D631D52459D006,
            0x147A9789F577767E,
            0xADB5D78E906BEC7E,
            0x19861D819CA15036,
            0xADB5D78E906BEC7E,
            0x5BA07D297BF90853,
        ],
    ));
}

#[test]
fn check_mouse_wheel() {
    let steps = [
        ("Mouse.Wheel(2,2,right,1)", "right"),
        ("Mouse.Wheel(2,2,down,1)", "down"),
        ("Mouse.Wheel(2,2,right,2)", "right twice"),
        ("Mouse.Wheel(2,2,down,2)", "down twice"),
        ("Mouse.Wheel(2,2,left,1)", "left"),
        ("Mouse.Wheel(2,2,up,1)", "up"),
        ("Mouse.Wheel(2,2,left,10)", "left to the start"),
        ("Mouse.Wheel(2,2,up,10)", "up to the start"),
    ];
    show_scrollable(&step_script(
        &steps,
        &[
            0xADB5D78E906BEC7E,
            0xA671BAB0DA956306,
            0x67D631D52459D006,
            0x8C5D320B04733B36,
            0x5821A65D52DCE336,
            0xB2AB1EF09568B9CE,
            0x53BFDE9864E52CEE,
            0x34375DCFE0508CFE,
            0xADB5D78E906BEC7E,
        ],
    ));
}

// One cell per step from 0% to 100%. ASCII is the longest glyph list and changes every 10%.
fn paint_ramp(title: &str, hash: u64, setup: impl Fn(&mut HeatMap<i32>) + 'static) {
    let script = format!(
        "
        Paint.Enable(false)
        Paint('{title}')
        CheckHash(0x{hash:X})
    "
    );
    App::new()
        .size(Size::new(40, 8))
        .debug_script(&script)
        .window(move || {
            let mut win = window!("Fills,d:f");
            let mut map = HeatMap::<i32>::new(layout!("d:f"), Size::new(11, 1), heatmap::Flags::None);
            map.set_cell_size(Size::new(1, 1));
            map.set_values([0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
            setup(&mut map);
            win.add(map);
            win
        })
        .run()
        .unwrap();
}

fn paint_with_fill(title: &str, fill: heatmap::CellFill, hash: u64) {
    paint_ramp(title, hash, move |map| {
        map.update_with(move |_, _, cell| {
            if let Some(cell) = cell {
                cell.set_fill(fill);
            }
        });
    });
}

#[test]
fn check_paint_fill_dots() {
    // "   ··•••●●●"
    paint_with_fill("Dots", heatmap::CellFill::Dots, 0x5631EA1F90176B46);
}

#[test]
fn check_paint_fill_ascii() {
    // " .:-=+x#%@@"
    paint_with_fill("ASCII", heatmap::CellFill::ASCII, 0x1E5BE81711E0C8C5);
}

#[test]
fn check_paint_fill_circles() {
    // "  ··∘○○◎●●●"
    paint_with_fill("Circles", heatmap::CellFill::Circles, 0x36C64DCA2819DECF);
}

#[test]
fn check_paint_fill_squares() {
    // "  ▫▫□□▪▪■■■"
    paint_with_fill("Squares", heatmap::CellFill::Squares, 0xE5A03DD5DCA78CEF);
}

#[test]
fn check_paint_fill_diamonds() {
    // "   ◇◇◈◈◈◆◆◆"
    paint_with_fill("Diamonds", heatmap::CellFill::Diamonds, 0xEAE36B719EC0CDF2);
}

#[test]
fn check_paint_fill_crosses() {
    // "   ··+++✚✚✚"
    paint_with_fill("Crosses", heatmap::CellFill::Crosses, 0x7373A02153AC07E0);
}

#[test]
fn check_paint_fill_stars() {
    // "  ✧✧✦✦✶✶✹✹✹"
    paint_with_fill("Stars", heatmap::CellFill::Stars, 0xC2F8F61CCCEC8434);
}

#[test]
fn check_paint_fill_asterisks() {
    // "  ✢✢✣✣✤✤✥✥✥"
    paint_with_fill("Asterisks", heatmap::CellFill::Asterisks, 0x33713918D2DF040C);
}

#[test]
fn check_paint_fill_flowers() {
    // "   ❀❀❁❁❁✿✿✿"
    paint_with_fill("Flowers", heatmap::CellFill::Flowers, 0xDEDDE390450B3F9A);
}

#[test]
fn check_paint_fill_pluses() {
    // "   ✛✛✜✜✜✚✚✚"
    paint_with_fill("Pluses", heatmap::CellFill::Pluses, 0xE6B628826E2700CE);
}

#[test]
fn check_paint_clear_fill_uses_the_default() {
    // same picture as the Dots ramp
    paint_ramp("ClearFill", 0x5631EA1F90176B46, |map| {
        map.set_default_fill(heatmap::CellFill::Dots);
        map.update_with(|_, _, cell| {
            if let Some(cell) = cell {
                cell.set_fill(heatmap::CellFill::Stars);
                cell.clear_fill();
            }
        });
    });
}

#[test]
fn check_paint_set_attr() {
    let attr = heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black));
    paint_ramp("SetAttr", 0x7F56BA9CB9239DD1, move |map| {
        map.update_with(move |_, _, cell| {
            if let Some(cell) = cell {
                cell.set_fill(heatmap::CellFill::Dots);
                cell.set_attr(attr);
            }
        });
    });
}

#[test]
fn check_paint_clear_attr_restores_the_fill() {
    // same picture as the Dots ramp
    let attr = heatmap::CellAttribute::Fixed(CharAttribute::with_color(Color::Red, Color::Black));
    paint_ramp("ClearAttr", 0x5631EA1F90176B46, move |map| {
        map.update_with(move |_, _, cell| {
            if let Some(cell) = cell {
                cell.set_fill(heatmap::CellFill::Dots);
                cell.set_attr(attr);
                cell.clear_attr();
            }
        });
    });
}

#[test]
fn check_paint_set_label_keeps_the_fill() {
    // a label is not drawn, so the picture stays the Dots ramp
    paint_ramp("SetLabel", 0x5631EA1F90176B46, |map| {
        map.update_with(|_, _, cell| {
            if let Some(cell) = cell {
                cell.set_fill(heatmap::CellFill::Dots);
                cell.set_label("Peak");
            }
        });
    });
}

#[test]
fn check_heatmap_macro_rows() {
    in_app(|| {
        let map = heatmap!(
            "i32, size: 3x2, d:f,
            flags: Margins+DimCellsOnSelection,
            cell-size: 3x2,
            tooltip: Value,
            fill: Dots,
            attr: red,
            values: [
                [1, {10, 'Peak', fill: Stars, attr: green}, 3],
                [none, 4, _]
            ],
            selected: {0, 1}"
        );
        assert_eq!(map.heatmap_size(), Size::new(3, 2));
        assert_eq!(map.cell_size(), Size::new(3, 2));
        assert_eq!(map.tooltip_format(), heatmap::TooltipFormat::Value);
        assert_eq!(map.default_fill(), Some(heatmap::CellFill::Dots));
        assert!(matches!(map.default_attr(), Some(heatmap::CellAttribute::Fixed(_))));
        assert_eq!(map.cell(0, 0).map(|cell| cell.value()), Some(1));
        assert_eq!(map.cell(0, 0).unwrap().label(), "");
        let peak = map.cell(0, 1).unwrap();
        assert_eq!(peak.value(), 10);
        assert_eq!(peak.label(), "Peak");
        assert_eq!(peak.fill(), Some(heatmap::CellFill::Stars));
        assert!(matches!(peak.attr(), Some(heatmap::CellAttribute::Fixed(_))));
        assert_eq!(map.cell(0, 2).map(|cell| cell.value()), Some(3));
        assert!(map.cell(1, 0).is_none());
        assert_eq!(map.cell(1, 1).map(|cell| cell.value()), Some(4));
        assert!(map.cell(1, 2).is_none());
        assert_eq!(map.selected_cell(), Some((0, 1)));
    });
}

#[test]
fn check_heatmap_macro_flat_list() {
    in_app(|| {
        let map = heatmap!(
            "u32, columns: 2, rows: 2, x:0, y:0, w:20, h:8,
            fill: Custom('#'),
            color: Range(blue, red),
            data: [{7, label: Source}, 8, none, 9]"
        );
        assert_eq!(map.heatmap_size(), Size::new(2, 2));
        assert_eq!(map.cell_size(), Size::new(2, 1));
        assert_eq!(map.default_fill(), Some(heatmap::CellFill::Custom('#')));
        assert!(matches!(map.default_attr(), Some(heatmap::CellAttribute::Range(_, _))));
        let source = map.cell(0, 0).unwrap();
        assert_eq!(source.value(), 7);
        assert_eq!(source.label(), "Source");
        assert_eq!(map.cell(0, 1).map(|cell| cell.value()), Some(8));
        assert!(map.cell(1, 0).is_none());
        assert_eq!(map.cell(1, 1).map(|cell| cell.value()), Some(9));
        assert_eq!(map.selected_cell(), None);
    });
}
