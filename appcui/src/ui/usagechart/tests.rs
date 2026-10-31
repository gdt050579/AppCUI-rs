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
fn check_add_segments() {
    in_app(|| {
        let mut c = UsageChart::new(layout!("d:f"), usagechart::Flags::None);
        c.add_segments([10u64, 30, 20]);
        c.add_segments([usagechart::SegmentBuilder::new().value(5).label("cache").build()]);
        assert_eq!(c.segments.iter().map(|s| s.value).collect::<Vec<_>>(), vec![10, 30, 20, 5]);
        assert_eq!(c.segments[3].label.as_str(), "cache");
        assert_eq!(c.segments.iter().map(|s| s.order).collect::<Vec<_>>(), vec![0, 1, 2, 3]);
    });
}

#[test]
fn check_add_segment_from_u8() {
    in_app(|| {
        let mut c = UsageChart::new(layout!("d:f"), usagechart::Flags::None);
        c.add_segment(10u8);
        c.add_segment(20u8);
        c.add_segment(u8::MAX);
        assert_eq!(c.segments.iter().map(|s| s.value).collect::<Vec<_>>(), vec![10, 20, u8::MAX as u64]);
        assert!(c.segments.iter().all(|s| s.label.is_empty() && s.fill_type == usagechart::SegmentFillType::Solid && s.attr.is_none()));
        assert_eq!(c.segments.iter().map(|s| s.order).collect::<Vec<_>>(), vec![0, 1, 2]);
    });
}

#[test]
fn check_add_segment_from_u16() {
    in_app(|| {
        let mut c = UsageChart::new(layout!("d:f"), usagechart::Flags::None);
        c.add_segment(10u16);
        c.add_segment(20u16);
        c.add_segment(u16::MAX);
        assert_eq!(c.segments.iter().map(|s| s.value).collect::<Vec<_>>(), vec![10, 20, u16::MAX as u64]);
        assert!(c.segments.iter().all(|s| s.label.is_empty() && s.fill_type == usagechart::SegmentFillType::Solid && s.attr.is_none()));
        assert_eq!(c.segments.iter().map(|s| s.order).collect::<Vec<_>>(), vec![0, 1, 2]);
    });
}

#[test]
fn check_add_segment_from_u32() {
    in_app(|| {
        let mut c = UsageChart::new(layout!("d:f"), usagechart::Flags::None);
        c.add_segment(10u32);
        c.add_segment(20u32);
        c.add_segment(u32::MAX);
        assert_eq!(c.segments.iter().map(|s| s.value).collect::<Vec<_>>(), vec![10, 20, u32::MAX as u64]);
        assert!(c.segments.iter().all(|s| s.label.is_empty() && s.fill_type == usagechart::SegmentFillType::Solid && s.attr.is_none()));
        assert_eq!(c.segments.iter().map(|s| s.order).collect::<Vec<_>>(), vec![0, 1, 2]);
    });
}

#[test]
fn check_clear() {
    in_app(|| {
        let mut c = UsageChart::new(layout!("d:f"), usagechart::Flags::Margins);
        c.set_label_content(usagechart::LabelContent::Percentage);
        c.set_sort_type(usagechart::SortType::HighestToLowest);
        c.set_total_usage(usagechart::TotalUsage::Fixed(100));
        c.add_segments([10u64, 30, 20]);
        c.set_selected_segment(Some(0));
        assert_eq!(c.segments.len(), 3);
        assert_eq!(c.selected_segment, Some(0));
        c.clear();
        assert!(c.segments.is_empty());
        assert_eq!(c.selected_segment, None);
        assert_eq!(c.flags, usagechart::Flags::Margins);
        assert_eq!(c.label_content(), usagechart::LabelContent::Percentage);
        assert_eq!(c.sort, usagechart::SortType::HighestToLowest);
        assert_eq!(c.total_usage, usagechart::TotalUsage::Fixed(100));
        c.add_segment(7u64);
        assert_eq!((c.segments[0].value, c.segments[0].order), (7, 0));
    });
}

#[test]
fn check_set_flags() {
    in_app(|| {
        let mut c = UsageChart::new(layout!("d:f"), usagechart::Flags::None);
        assert_eq!(c.flags, usagechart::Flags::None);
        c.set_flags(usagechart::Flags::Margins);
        assert_eq!(c.flags, usagechart::Flags::Margins);
        c.set_flags(usagechart::Flags::Margins | usagechart::Flags::DimSegmentsOnSelection);
        assert_eq!(c.flags, usagechart::Flags::Margins | usagechart::Flags::DimSegmentsOnSelection);
        c.set_flags(usagechart::Flags::Margins | usagechart::Flags::DimSegmentsOnSelection);
        assert_eq!(c.flags, usagechart::Flags::Margins | usagechart::Flags::DimSegmentsOnSelection);
        c.set_flags(usagechart::Flags::DimSegmentsOnSelection);
        assert_eq!(c.flags, usagechart::Flags::DimSegmentsOnSelection);
        c.set_flags(usagechart::Flags::None);
        assert_eq!(c.flags, usagechart::Flags::None);
    });
}

#[test]
fn check_label_content() {
    in_app(|| {
        let mut c = UsageChart::new(layout!("d:f"), usagechart::Flags::None);
        assert_eq!(c.label_content(), usagechart::LabelContent::LabelAndValue);
        c.set_label_content(usagechart::LabelContent::Value);
        assert_eq!(c.label_content(), usagechart::LabelContent::Value);
        c.set_label_content(usagechart::LabelContent::Label);
        assert_eq!(c.label_content(), usagechart::LabelContent::Label);
        c.set_label_content(usagechart::LabelContent::Percentage);
        assert_eq!(c.label_content(), usagechart::LabelContent::Percentage);
        c.set_label_content(usagechart::LabelContent::LabelAndPercentage);
        assert_eq!(c.label_content(), usagechart::LabelContent::LabelAndPercentage);
        c.set_label_content(usagechart::LabelContent::LabelAndValue);
        assert_eq!(c.label_content(), usagechart::LabelContent::LabelAndValue);
        assert_eq!(usagechart!("d:f").label_content(), usagechart::LabelContent::LabelAndValue);
        assert_eq!(usagechart!("d:f, lc: percentage").label_content(), usagechart::LabelContent::Percentage);
    });
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
fn check_example_chart_hover_and_keyboard() {
    // the 256 GB drive from the usage chart example, on a 70x10 screen
    // client area is 68x8, so the bar (with margins) is 66 cells wide:
    // OS 12, Apps 9, Documents 5, Media 5, Cache 2, and the rest stays free
    let script = "
        Paint.Enable(false)
        Paint('0. nothing selected')
        CheckHash(0x9B1E31EC5AC910D1)
        Mouse.Move(8,5)
        Paint('1. hover OS')
        CheckHash(0x689E405BE1B2D465)
        Mouse.Move(10,5)
        Paint('2. hover OS again, tooltip stays on the segment')
        CheckHash(0x689E405BE1B2D465)
        Mouse.Move(18,5)
        Paint('3. hover Apps')
        CheckHash(0x97F7C322A50EE01B)
        Mouse.Move(25,5)
        Paint('4. hover Documents')
        CheckHash(0x85407C160A33B511)
        Mouse.Move(30,5)
        Paint('5. hover Media')
        CheckHash(0x67472A703AA1602E)
        Mouse.Move(34,5)
        Paint('6. hover Cache')
        CheckHash(0x2980EEA1D939AA67)
        Mouse.Move(52,5)
        Paint('7. hover the free space')
        CheckHash(0x9B1E31EC5AC910D1)
        Mouse.Move(0,0)
        Paint('8. mouse left the chart')
        CheckHash(0x9B1E31EC5AC910D1)
        Key.Pressed(Right)
        Paint('9. Right: OS selected')
        CheckHash(0x9932E532AA19AF11)
        Key.Pressed(Right)
        Paint('10. Right: Apps selected')
        CheckHash(0x25E88D42D2D89BCD)
        Key.Pressed(Right)
        Paint('11. Right: Documents selected')
        CheckHash(0x89BCB7CFB9736615)
        Key.Pressed(Right)
        Paint('12. Right: Media selected')
        CheckHash(0xBCFAAE3207F0E189)
        Key.Pressed(Right)
        Paint('13. Right: Cache selected')
        CheckHash(0xA30E6474A4321251)
        Key.Pressed(Right)
        Paint('14. Right: OS selected again')
        CheckHash(0x9932E532AA19AF11)
        Key.Pressed(Left)
        Paint('15. Left: Cache selected')
        CheckHash(0xA30E6474A4321251)
        Key.Pressed(Escape)
        Paint('16. Escape: nothing selected')
        CheckHash(0x9B1E31EC5AC910D1)
    ";
    App::new()
        .size(Size::new(70, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("'Usage chart',d:f");
            w.add(usagechart!(
                "d:f, flags: Margins+DimSegmentsOnSelection, total: Fixed(256), nf: {dec, suffix: ' GB'}, lc: LabelAndPercentage, segments: [
                    {48, label: 'OS', fill: Solid},
                    {36, label: 'Apps', fill: CrossHatch},
                    {22, label: 'Documents', fill: Grid},
                    {22, label: 'Media', fill: DiagonalDown},
                    {8, label: 'Cache', fill: GridDouble}
                ]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_example_chart_hover_and_click() {
    // same drive as `check_example_chart_hover_and_keyboard`, without dimming,
    // and with the value (not the percentage) in the tooltip
    let script = "
        Paint.Enable(false)
        Paint('0. nothing selected')
        CheckHash(0x9B1E31EC5AC910D1)
        Mouse.Move(8,5)
        Paint('1. hover OS')
        CheckHash(0xBD9CC0412FE580E9)
        Mouse.Move(10,5)
        Paint('2. hover OS again, tooltip stays on the segment')
        CheckHash(0xBD9CC0412FE580E9)
        Mouse.Move(18,5)
        Paint('3. hover Apps')
        CheckHash(0xA5E45A258027D982)
        Mouse.Move(25,5)
        Paint('4. hover Documents')
        CheckHash(0x7E2CC8FA01E6ECBB)
        Mouse.Move(30,5)
        Paint('5. hover Media')
        CheckHash(0x3B11A06F2AAFEA98)
        Mouse.Move(34,5)
        Paint('6. hover Cache')
        CheckHash(0xEB3FEF435E65256C)
        Mouse.Move(52,5)
        Paint('7. hover the free space')
        CheckHash(0x9B1E31EC5AC910D1)
        Mouse.Move(0,0)
        Paint('8. mouse left the chart')
        CheckHash(0x9B1E31EC5AC910D1)
        Mouse.Click(8,5,left)
        Paint('9. click OS')
        CheckHash(0xE96C2BA3C23F7801)
        Mouse.Click(18,5,left)
        Paint('10. click Apps')
        CheckHash(0x6E847829C7CF5645)
        Mouse.Click(25,5,left)
        Paint('11. click Documents')
        CheckHash(0xC11A7174A94DD4C5)
        Mouse.Click(30,5,left)
        Paint('12. click Media')
        CheckHash(0x3AF710B09668B545)
        Mouse.Click(34,5,left)
        Paint('13. click Cache')
        CheckHash(0xFD2B9FE8FA59B819)
        Mouse.Click(8,5,left)
        Paint('14. click OS again')
        CheckHash(0xE96C2BA3C23F7801)
        Mouse.Click(52,5,left)
        Paint('15. click the free space: nothing selected')
        CheckHash(0x9B1E31EC5AC910D1)
    ";
    App::new()
        .size(Size::new(70, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("'Usage chart',d:f");
            w.add(usagechart!(
                "d:f, flags: Margins, total: Fixed(256), nf: {dec, suffix: ' GB'}, lc: LabelAndValue, segments: [
                    {48, label: 'OS', fill: Solid},
                    {36, label: 'Apps', fill: CrossHatch},
                    {22, label: 'Documents', fill: Grid},
                    {22, label: 'Media', fill: DiagonalDown},
                    {8, label: 'Cache', fill: GridDouble}
                ]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_example_chart_hover_and_click_auto_total() {
    // same drive as `check_example_chart_hover_and_click`, with no margins.
    // the total is the sum of the segments (136), so the bar is full:
    // OS 24, Apps 18, Documents 11, Media 11, Cache 4
    let script = "
        Paint.Enable(false)
        Paint('0. nothing selected')
        CheckHash(0xF3D8679DB5E8339)
        Mouse.Move(13,5)
        Paint('1. hover OS')
        CheckHash(0x3291F3BA3A72DEE9)
        Mouse.Move(5,5)
        Paint('2. hover OS again, tooltip stays on the segment')
        CheckHash(0x3291F3BA3A72DEE9)
        Mouse.Move(34,5)
        Paint('3. hover Apps')
        CheckHash(0xE6B2BD490958C37E)
        Mouse.Move(48,5)
        Paint('4. hover Documents')
        CheckHash(0x78432465CAA8450D)
        Mouse.Move(59,5)
        Paint('5. hover Media')
        CheckHash(0xC25CBFBD88DEF244)
        Mouse.Move(67,5)
        Paint('6. hover Cache')
        CheckHash(0x55577FA84A0F74DF)
        Mouse.Move(0,0)
        Paint('7. mouse left the chart')
        CheckHash(0xF3D8679DB5E8339)
        Mouse.Click(13,5,left)
        Paint('8. click OS')
        CheckHash(0x5A2F818E695BFE9)
        Mouse.Click(34,5,left)
        Paint('9. click Apps')
        CheckHash(0xD9096E128E9CD221)
        Mouse.Click(48,5,left)
        Paint('10. click Documents')
        CheckHash(0xCDC695EF023D1DB1)
        Mouse.Click(59,5,left)
        Paint('11. click Media')
        CheckHash(0xE6FD286D7E1F1639)
        Mouse.Click(67,5,left)
        Paint('12. click Cache')
        CheckHash(0xBF7D463741BED5A9)
        Mouse.Click(13,5,left)
        Paint('13. click OS again')
        CheckHash(0x5A2F818E695BFE9)
        Mouse.Move(0,0)
        Key.Pressed(Escape)
        Paint('14. Escape: nothing selected')
        CheckHash(0xF3D8679DB5E8339)
    ";
    App::new()
        .size(Size::new(70, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("'Usage chart',d:f");
            w.add(usagechart!(
                "d:f, total: Auto, nf: {dec, suffix: ' GB'}, lc: LabelAndValue, segments: [
                    {48, label: 'OS', fill: Solid},
                    {36, label: 'Apps', fill: CrossHatch},
                    {22, label: 'Documents', fill: Grid},
                    {22, label: 'Media', fill: DiagonalDown},
                    {8, label: 'Cache', fill: GridDouble}
                ]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_example_chart_hover_and_click_auto_total_one_row() {
    // same bar as `check_example_chart_hover_and_click_auto_total`, one cell high,
    // with the other segments dimmed while one is selected
    let script = "
        Paint.Enable(false)
        Paint('0. nothing selected')
        CheckHash(0x1690CEED7E9787C)
        Mouse.Move(13,1)
        Paint('1. hover OS')
        CheckHash(0x630D0B6EC27D8502)
        Mouse.Move(5,1)
        Paint('2. hover OS again, tooltip stays on the segment')
        CheckHash(0x630D0B6EC27D8502)
        Mouse.Move(34,1)
        Paint('3. hover Apps')
        CheckHash(0x78C251C7CEA3ABA1)
        Mouse.Move(48,1)
        Paint('4. hover Documents')
        CheckHash(0xE1CA1770037A4324)
        Mouse.Move(59,1)
        Paint('5. hover Media')
        CheckHash(0xD6CF0C1DD5A751C0)
        Mouse.Move(67,1)
        Paint('6. hover Cache')
        CheckHash(0xB7AB9E2F0F76DF3F)
        Mouse.Move(0,0)
        Paint('7. mouse left the chart')
        CheckHash(0x1690CEED7E9787C)
        Mouse.Click(13,1,left)
        Paint('8. click OS')
        CheckHash(0xE52EF5CE344AEAA1)
        Mouse.Click(34,1,left)
        Paint('9. click Apps')
        CheckHash(0x1B166FC6A60A211D)
        Mouse.Click(48,1,left)
        Paint('10. click Documents')
        CheckHash(0x37EC2FBB68048F33)
        Mouse.Click(59,1,left)
        Paint('11. click Media')
        CheckHash(0xEA0768D9147AF491)
        Mouse.Click(67,1,left)
        Paint('12. click Cache')
        CheckHash(0xB5858F9FE8D7C534)
        Mouse.Click(13,1,left)
        Paint('13. click OS again')
        CheckHash(0xE52EF5CE344AEAA1)
        Mouse.Move(0,0)
        Key.Pressed(Escape)
        Paint('14. Escape: nothing selected')
        CheckHash(0x1690CEED7E9787C)
    ";
    App::new()
        .size(Size::new(70, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("'Usage chart',d:f");
            w.add(usagechart!(
                "d:t, h:1, flags: DimSegmentsOnSelection, total: Auto, nf: {dec, suffix: ' GB'}, lc: LabelAndValue, segments: [
                    {48, label: 'OS', fill: Solid},
                    {36, label: 'Apps', fill: CrossHatch},
                    {22, label: 'Documents', fill: Grid},
                    {22, label: 'Media', fill: DiagonalDown},
                    {8, label: 'Cache', fill: GridDouble}
                ]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_example_chart_hover_and_click_auto_total_two_rows() {
    // same bar as `check_example_chart_hover_and_click_auto_total_one_row`, two cells high.
    // the selection is marked with corners instead of a full border
    let script = "
        Paint.Enable(false)
        Paint('0. nothing selected')
        CheckHash(0x66295BCE5DDF2741)
        Mouse.Move(13,2)
        Paint('1. hover OS')
        CheckHash(0xC5DFFD53D459AA6)
        Mouse.Move(5,2)
        Paint('2. hover OS again, tooltip stays on the segment')
        CheckHash(0xC5DFFD53D459AA6)
        Mouse.Move(34,2)
        Paint('3. hover Apps')
        CheckHash(0xE29E451A0FFF41E2)
        Mouse.Move(48,2)
        Paint('4. hover Documents')
        CheckHash(0xE6475D175B1FA303)
        Mouse.Move(59,2)
        Paint('5. hover Media')
        CheckHash(0xBC213A2CCAD86685)
        Mouse.Move(67,2)
        Paint('6. hover Cache')
        CheckHash(0xE487F891BCA887BF)
        Mouse.Move(0,0)
        Paint('7. mouse left the chart')
        CheckHash(0x66295BCE5DDF2741)
        Mouse.Click(13,2,left)
        Paint('8. click OS')
        CheckHash(0x96239F0E6B879A01)
        Mouse.Click(34,2,left)
        Paint('9. click Apps')
        CheckHash(0xAF3276EF82635391)
        Mouse.Click(48,2,left)
        Paint('10. click Documents')
        CheckHash(0x1AE6E731837FA2B1)
        Mouse.Click(59,2,left)
        Paint('11. click Media')
        CheckHash(0xDBA937C058A77639)
        Mouse.Click(67,2,left)
        Paint('12. click Cache')
        CheckHash(0x3B9E3EB1A192A591)
        Mouse.Click(13,2,left)
        Paint('13. click OS again')
        CheckHash(0x96239F0E6B879A01)
        Mouse.Move(0,0)
        Key.Pressed(Escape)
        Paint('14. Escape: nothing selected')
        CheckHash(0x66295BCE5DDF2741)
    ";
    App::new()
        .size(Size::new(70, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("'Usage chart',d:f");
            w.add(usagechart!(
                "d:t, h:2, flags: DimSegmentsOnSelection, total: Auto, nf: {dec, suffix: ' GB'}, lc: LabelAndValue, segments: [
                    {48, label: 'OS', fill: Solid},
                    {36, label: 'Apps', fill: CrossHatch},
                    {22, label: 'Documents', fill: Grid},
                    {22, label: 'Media', fill: DiagonalDown},
                    {8, label: 'Cache', fill: GridDouble}
                ]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_example_chart_hover_and_click_auto_total_three_rows() {
    // same bar as `check_example_chart_hover_and_click_auto_total_two_rows`, three cells high.
    // the selection is a full border
    let script = "
        Paint.Enable(false)
        Paint('0. nothing selected')
        CheckHash(0x9B335645E68166C4)
        Mouse.Move(13,2)
        Paint('1. hover OS')
        CheckHash(0xAC6971B823816C53)
        Mouse.Move(5,2)
        Paint('2. hover OS again, tooltip stays on the segment')
        CheckHash(0xAC6971B823816C53)
        Mouse.Move(34,2)
        Paint('3. hover Apps')
        CheckHash(0x256831011CF2C48F)
        Mouse.Move(48,2)
        Paint('4. hover Documents')
        CheckHash(0x3CDD3FB1F9E45D26)
        Mouse.Move(59,2)
        Paint('5. hover Media')
        CheckHash(0x7AA59BB764DB0828)
        Mouse.Move(67,2)
        Paint('6. hover Cache')
        CheckHash(0xBC610DF1F8C3ABA2)
        Mouse.Move(0,0)
        Paint('7. mouse left the chart')
        CheckHash(0x9B335645E68166C4)
        Mouse.Click(13,2,left)
        Paint('8. click OS')
        CheckHash(0x4E8CF27F1A5A12BF)
        Mouse.Click(34,2,left)
        Paint('9. click Apps')
        CheckHash(0xD0ED9D3C108E6F2F)
        Mouse.Click(48,2,left)
        Paint('10. click Documents')
        CheckHash(0xBBB0A1AABBA65799)
        Mouse.Click(59,2,left)
        Paint('11. click Media')
        CheckHash(0x67E543F4F1D2AB83)
        Mouse.Click(67,2,left)
        Paint('12. click Cache')
        CheckHash(0xF85AEDE841576C9E)
        Mouse.Click(13,2,left)
        Paint('13. click OS again')
        CheckHash(0x4E8CF27F1A5A12BF)
        Mouse.Move(0,0)
        Key.Pressed(Escape)
        Paint('14. Escape: nothing selected')
        CheckHash(0x9B335645E68166C4)
    ";
    App::new()
        .size(Size::new(70, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("'Usage chart',d:f");
            w.add(usagechart!(
                "d:t, h:3, flags: DimSegmentsOnSelection, total: Auto, nf: {dec, suffix: ' GB'}, lc: LabelAndValue, segments: [
                    {48, label: 'OS', fill: Solid},
                    {36, label: 'Apps', fill: CrossHatch},
                    {22, label: 'Documents', fill: Grid},
                    {22, label: 'Media', fill: DiagonalDown},
                    {8, label: 'Cache', fill: GridDouble}
                ]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_example_chart_disabled() {
    // same drive as `check_example_chart_hover_and_keyboard`, with the chart disabled.
    // the bar is painted with the inactive color, and a selected segment draws no border
    let script = "
        Paint.Enable(false)
        Paint('disabled chart')
        CheckHash(0xCB9C88EFBC32B2F5)
        Mouse.Move(8,5)
        Paint('hover does not change a disabled chart')
        CheckHash(0xCB9C88EFBC32B2F5)
        Mouse.Click(8,5,left)
        Paint('click does not change a disabled chart')
        CheckHash(0xCB9C88EFBC32B2F5)
    ";
    App::new()
        .size(Size::new(70, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("'Usage chart',d:f");
            w.add(usagechart!(
                "d:f, enabled: false, selected: 0, flags: Margins+DimSegmentsOnSelection, total: Fixed(256), nf: {dec, suffix: ' GB'}, lc: LabelAndPercentage, segments: [
                    {48, label: 'OS', fill: Solid},
                    {36, label: 'Apps', fill: CrossHatch},
                    {22, label: 'Documents', fill: Grid},
                    {22, label: 'Media', fill: DiagonalDown},
                    {8, label: 'Cache', fill: GridDouble}
                ]"
            ));
            w
        })
        .run()
        .unwrap();
}

fn example_drive_chart(content: usagechart::LabelContent) -> UsageChart {
    let mut c = UsageChart::new(layout!("d:f"), usagechart::Flags::Margins | usagechart::Flags::DimSegmentsOnSelection);
    c.set_total_usage(usagechart::TotalUsage::Fixed(256));
    c.set_number_format(FormatNumber::new(10).suffix(" GB"));
    c.set_label_content(content);
    c.add_segments([
        usagechart::SegmentBuilder::new().value(48).label("OS").fill(usagechart::SegmentFillType::Solid).build(),
        usagechart::SegmentBuilder::new().value(36).label("Apps").fill(usagechart::SegmentFillType::CrossHatch).build(),
        usagechart::SegmentBuilder::new().value(22).label("Documents").fill(usagechart::SegmentFillType::Grid).build(),
        usagechart::SegmentBuilder::new().value(22).label("Media").fill(usagechart::SegmentFillType::DiagonalDown).build(),
        usagechart::SegmentBuilder::new().value(8).label("Cache").fill(usagechart::SegmentFillType::GridDouble).build(),
    ]);
    c
}

fn run_drive_hover(content: usagechart::LabelContent, script: &str) {
    App::new()
        .size(Size::new(70, 10))
        .debug_script(script)
        .window(move || {
            let mut w = window!("'Usage chart',d:f");
            w.add(example_drive_chart(content));
            w
        })
        .run()
        .unwrap();
}

fn check_label_content_hover(content: usagechart::LabelContent, os: u64, apps: u64, documents: u64, media: u64, cache: u64) {
    // the bar itself does not depend on the tooltip, so the resting hashes are shared
    let script = format!(
        "
        Paint.Enable(false)
        Paint('0. nothing hovered')
        CheckHash(0x9B1E31EC5AC910D1)
        Mouse.Move(8,5)
        Paint('1. hover OS')
        CheckHash(0x{os:X})
        Mouse.Move(18,5)
        Paint('2. hover Apps')
        CheckHash(0x{apps:X})
        Mouse.Move(25,5)
        Paint('3. hover Documents')
        CheckHash(0x{documents:X})
        Mouse.Move(30,5)
        Paint('4. hover Media')
        CheckHash(0x{media:X})
        Mouse.Move(34,5)
        Paint('5. hover Cache')
        CheckHash(0x{cache:X})
        Mouse.Move(52,5)
        Paint('6. hover the free space')
        CheckHash(0x9B1E31EC5AC910D1)
        Mouse.Move(0,0)
        Paint('7. mouse left the chart')
        CheckHash(0x9B1E31EC5AC910D1)
        "
    );
    run_drive_hover(content, &script);
}

#[test]
fn check_hover_label_content_value() {
    // tooltip is the value: "48 GB", "36 GB", "22 GB", "22 GB", "8 GB"
    check_label_content_hover(
        usagechart::LabelContent::Value,
        0x1DD8018C79FD3C1F,
        0x182337E3066C9942,
        0xEEB8810F13F3D7C8,
        0x6AD82145BC7D7E93,
        0x1EC4B7E9BEE3D71C,
    );
}

#[test]
fn check_hover_label_content_label() {
    // tooltip is the label: "OS", "Apps", "Documents", "Media", "Cache"
    check_label_content_hover(
        usagechart::LabelContent::Label,
        0x43BC9D6ECBB66B2E,
        0x2739B2E3964DFE60,
        0x2FEB7670F488FFDD,
        0x4C83076B31E3CE2E,
        0x8032FD47AE067889,
    );
}

#[test]
fn check_hover_label_content_percentage() {
    // tooltip is the share of 256 GB: "18.75%", "14.06%", "8.59%", "8.59%", "3.13%"
    check_label_content_hover(
        usagechart::LabelContent::Percentage,
        0xDF0AFBD3A778F59F,
        0x47D3483A76FE470D,
        0xCB5EA809FB090C36,
        0x67002B23677C9661,
        0x8C7D3CFDE44A74DE,
    );
}

#[test]
fn check_hover_label_content_label_and_value() {
    // tooltip is "OS: 48 GB", "Apps: 36 GB", "Documents: 22 GB", "Media: 22 GB", "Cache: 8 GB"
    check_label_content_hover(
        usagechart::LabelContent::LabelAndValue,
        0xBD9CC0412FE580E9,
        0xA5E45A258027D982,
        0x7E2CC8FA01E6ECBB,
        0x3B11A06F2AAFEA98,
        0xEB3FEF435E65256C,
    );
}

#[test]
fn check_hover_label_content_label_and_percentage() {
    // tooltip is "OS: 18.75%", "Apps: 14.06%", "Documents: 8.59%", "Media: 8.59%", "Cache: 3.13%"
    check_label_content_hover(
        usagechart::LabelContent::LabelAndPercentage,
        0x689E405BE1B2D465,
        0x97F7C322A50EE01B,
        0x85407C160A33B511,
        0x67472A703AA1602E,
        0x2980EEA1D939AA67,
    );
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
