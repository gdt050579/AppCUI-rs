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
        map.clear(0, 1);
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
        map.clear(0, 1);
        map.clear(1, 0);
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
        map.clear(0, 1);
        assert_eq!(map.selected_cell(), None);
        map.set_selected_cell(Some((0, 0)));
        map.clear_all();
        assert_eq!(map.selected_cell(), None);
        assert!(map.cell(0, 0).is_none());
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
