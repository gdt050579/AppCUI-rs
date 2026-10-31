use crate::prelude::*;

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

#[test]
fn check_macro_defaults() {
    in_app(|| {
        let c = usagechart!("d:f");
        assert!(c.segments.is_empty());
        assert_eq!(c.flags, usagechart::Flags::None);
        assert_eq!(c.sort, usagechart::SortType::None);
        assert_eq!(c.total_usage, usagechart::TotalUsage::Auto);
        assert_eq!(c.selected_segment, None);
    });
}

#[test]
fn check_macro_segments() {
    in_app(|| {
        let red = CharAttribute::with_fore_color(Color::Red);
        let c = usagechart!(
            "d:f, segments: [10, {20, 'OS'}, {v: 30, label: 'Apps', fill: Grid, attr: red}, {value: 40, text: Docs, color: {fore: green, back: black}}]"
        );
        assert_eq!(c.segments.len(), 4);
        assert_eq!(
            (c.segments[0].value, c.segments[0].label.as_str(), c.segments[0].fill_type, c.segments[0].attr),
            (10, "", usagechart::SegmentFillType::Solid, None)
        );
        assert_eq!((c.segments[1].value, c.segments[1].label.as_str(), c.segments[1].attr), (20, "OS", None));
        assert_eq!(
            (c.segments[2].value, c.segments[2].label.as_str(), c.segments[2].fill_type, c.segments[2].attr),
            (30, "Apps", usagechart::SegmentFillType::Grid, Some(red))
        );
        assert_eq!((c.segments[3].value, c.segments[3].label.as_str()), (40, "Docs"));
        assert_eq!(c.segments[3].attr, Some(CharAttribute::with_color(Color::Green, Color::Black)));
        let c = usagechart!("d:f, data: [{18446744073709551615, 'max'}]");
        assert_eq!(c.segments[0].value, u64::MAX);
        let c = usagechart!("d:f, values: [1, 2]");
        assert_eq!(c.segments.len(), 2);
        let c = usagechart!("d:f, segments: [{7, 'My OS', ft: cross-hatch}, {8, name: Cache}]");
        assert_eq!(
            (c.segments[0].label.as_str(), c.segments[0].fill_type),
            ("My OS", usagechart::SegmentFillType::CrossHatch)
        );
        assert_eq!(c.segments[1].label.as_str(), "Cache");
    });
}

#[test]
fn check_macro_fills() {
    in_app(|| {
        assert_eq!(usagechart!("d:f, segments: [{1, fill: solid}]").segments[0].fill_type, usagechart::SegmentFillType::Solid);
        assert_eq!(usagechart!("d:f, segments: [{1, fill: Shade75}]").segments[0].fill_type, usagechart::SegmentFillType::Shade75);
        assert_eq!(usagechart!("d:f, segments: [{1, fill: Shade50}]").segments[0].fill_type, usagechart::SegmentFillType::Shade50);
        assert_eq!(usagechart!("d:f, segments: [{1, fill: Shade25}]").segments[0].fill_type, usagechart::SegmentFillType::Shade25);
        assert_eq!(usagechart!("d:f, segments: [{1, fill: Braille}]").segments[0].fill_type, usagechart::SegmentFillType::Braille);
        assert_eq!(
            usagechart!("d:f, segments: [{1, fill: checkerboard}]").segments[0].fill_type,
            usagechart::SegmentFillType::Checkerboard
        );
        assert_eq!(usagechart!("d:f, segments: [{1, fill: grid}]").segments[0].fill_type, usagechart::SegmentFillType::Grid);
        assert_eq!(usagechart!("d:f, segments: [{1, fill: grid-double}]").segments[0].fill_type, usagechart::SegmentFillType::GridDouble);
        assert_eq!(usagechart!("d:f, segments: [{1, fill: cross-hatch}]").segments[0].fill_type, usagechart::SegmentFillType::CrossHatch);
        assert_eq!(usagechart!("d:f, segments: [{1, fill: Dashed}]").segments[0].fill_type, usagechart::SegmentFillType::Dashed);
        assert_eq!(usagechart!("d:f, segments: [{1, fill: DiagonalUp}]").segments[0].fill_type, usagechart::SegmentFillType::DiagonalUp);
        assert_eq!(usagechart!("d:f, segments: [{1, ft: diagonal-down}]").segments[0].fill_type, usagechart::SegmentFillType::DiagonalDown);
        assert_eq!(usagechart!("d:f, segments: [{1, fill-type: Notched}]").segments[0].fill_type, usagechart::SegmentFillType::Notched);
        assert_eq!(usagechart!("d:f, segments: [{1, fill: Custom('#')}]").segments[0].fill_type, usagechart::SegmentFillType::Custom('#'));
        assert_eq!(
            usagechart!("d:f, segments: [{1, fill: Custom(code: 0x2588)}]").segments[0].fill_type,
            usagechart::SegmentFillType::Custom('█')
        );
        assert_eq!(usagechart!("d:f, segments: [{1, fill: '*'}]").segments[0].fill_type, usagechart::SegmentFillType::Custom('*'));
        assert_eq!(usagechart!("d:f, segments: [{1, fill: x}]").segments[0].fill_type, usagechart::SegmentFillType::Custom('x'));
    });
}

#[test]
fn check_macro_sort() {
    in_app(|| {
        let c = usagechart!("d:f, values: [{2, 'b'}, {1, 'a'}, {3, 'c'}]");
        assert_eq!(c.sort, usagechart::SortType::None);
        assert_eq!(c.segments.iter().map(|s| s.value).collect::<Vec<_>>(), vec![2, 1, 3]);

        let c = usagechart!("d:f, sort: HighestToLowest, values: [1, 50, 10]");
        assert_eq!(c.sort, usagechart::SortType::HighestToLowest);
        assert_eq!(c.segments.iter().map(|s| s.value).collect::<Vec<_>>(), vec![50, 10, 1]);

        let c = usagechart!("d:f, sort: lowest-to-highest, values: [1, 50, 10]");
        assert_eq!(c.sort, usagechart::SortType::LowestToHighest);
        assert_eq!(c.segments.iter().map(|s| s.value).collect::<Vec<_>>(), vec![1, 10, 50]);

        let c = usagechart!("d:f, sort: alpha, values: [{2, 'b'}, {1, 'a'}, {3, 'c'}]");
        assert_eq!(c.sort, usagechart::SortType::Alphabetical);
        assert_eq!(c.segments.iter().map(|s| s.label.as_str()).collect::<Vec<_>>(), vec!["a", "b", "c"]);

        let c = usagechart!("d:f, order: reverse-alphabetical, values: [{2, 'b'}, {1, 'a'}, {3, 'c'}]");
        assert_eq!(c.sort, usagechart::SortType::ReverseAlphabetical);
        assert_eq!(c.segments.iter().map(|s| s.label.as_str()).collect::<Vec<_>>(), vec!["c", "b", "a"]);
    });
}

#[test]
fn check_macro_total() {
    in_app(|| {
        assert_eq!(usagechart!("d:f, total: Auto").total_usage, usagechart::TotalUsage::Auto);
        assert_eq!(usagechart!("d:f, total: auto").total_usage, usagechart::TotalUsage::Auto);
        assert_eq!(usagechart!("d:f, total: 256").total_usage, usagechart::TotalUsage::Fixed(256));
        assert_eq!(usagechart!("d:f, total: Fixed(256)").total_usage, usagechart::TotalUsage::Fixed(256));
        assert_eq!(usagechart!("d:f, total: fix(100)").total_usage, usagechart::TotalUsage::Fixed(100));
        assert_eq!(usagechart!("d:f, total-usage: fixed(0)").total_usage, usagechart::TotalUsage::Fixed(0));
        assert_eq!(
            usagechart!("d:f, totalusage: 18446744073709551615").total_usage,
            usagechart::TotalUsage::Fixed(u64::MAX)
        );
    });
}

#[test]
fn check_macro_flags_and_selection() {
    in_app(|| {
        let c = usagechart!("d:f, flags: Margins+DimSegmentsOnSelection, selected: 1, sort: HighestToLowest, values: [1, 50, 10]");
        assert_eq!(c.flags, usagechart::Flags::Margins | usagechart::Flags::DimSegmentsOnSelection);
        assert_eq!(c.sort, usagechart::SortType::HighestToLowest);
        assert_eq!(c.selected_segment, Some(1));
        assert_eq!(usagechart!("d:f, select: 0, values: [1, 2, 3]").selected_segment, Some(0));
        assert_eq!(usagechart!("d:f, selected-segment: 2, values: [1, 2, 3]").selected_segment, Some(2));
        assert_eq!(
            usagechart!("d:f, flags: [Margins, DimSegmentsOnSelection]").flags,
            usagechart::Flags::Margins | usagechart::Flags::DimSegmentsOnSelection
        );
    });
}

#[test]
fn check_macro_layout_and_state() {
    in_app(|| {
        let c = usagechart!("x:2, y:3, w:30, h:3, enabled: false, visible: false, segments: [1]");
        assert!(!c.is_enabled());
        assert!(!c.is_visible());
        assert_eq!(c.segments.len(), 1);
    });
}
