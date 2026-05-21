mod folds_tests {
    use super::super::{Fold, Folds};

    const FOLDED_BIT: u32 = 0x8000_0000;

    fn ranges(folds: &Folds) -> Vec<(u32, u32)> {
        folds.folds().iter().map(|f| (f.start_line(), f.count())).collect()
    }

    fn end_line(start: u32, count: u32) -> u32 {
        start + count - 1
    }

    /// Pairs `i < j` must be disjoint (`e_i < s_j`) or strictly nested (`s_i < s_j && e_i > e_j`).
    fn assert_folds_invariant(folds: &Folds) {
        let ranges = ranges(folds);
        for w in ranges.windows(2) {
            assert!(w[0].0 < w[1].0, "folds must be sorted by start_line: {ranges:?}");
        }
        for i in 0..ranges.len() {
            let (s1, c1) = ranges[i];
            let e1 = end_line(s1, c1);
            for j in (i + 1)..ranges.len() {
                let (s2, c2) = ranges[j];
                let e2 = end_line(s2, c2);
                let disjoint = e1 < s2;
                let strictly_nested = s1 < s2 && e1 > e2;
                assert!(
                    disjoint || strictly_nested,
                    "invalid fold pair ({s1},{c1})..={e1} vs ({s2},{c2})..={e2} in {ranges:?}"
                );
            }
        }
    }

    struct AddCase {
        existing: &'static [(u32, u32)],
        new: (u32, u32),
        accept: bool,
        label: &'static str,
    }

    fn run_add_case(case: &AddCase) {
        let mut folds = Folds::new();
        for &(start, count) in case.existing {
            assert!(folds.add(start, count), "{}: setup add ({start}, {count})", case.label);
            assert_folds_invariant(&folds);
        }
        let before = ranges(&folds);
        let ok = folds.add(case.new.0, case.new.1);
        assert_eq!(
            ok, case.accept,
            "{}: existing={:?} + ({}, {})",
            case.label, case.existing, case.new.0, case.new.1
        );
        if case.accept {
            let mut expected = before;
            let pos = expected.partition_point(|&(s, _)| s < case.new.0);
            expected.insert(pos, case.new);
            assert_eq!(ranges(&folds), expected, "{}: existing={:?} + {:?}", case.label, case.existing, case.new);
            assert_folds_invariant(&folds);
        } else {
            assert_eq!(ranges(&folds), before, "{}: folds must be unchanged on reject", case.label);
        }
    }

    const ADD_CASES: &[AddCase] = &[
        // Empty / single
        AddCase {
            existing: &[],
            new: (10, 5),
            accept: true,
            label: "1 empty accept",
        },
        AddCase {
            existing: &[(10, 5)],
            new: (10, 5),
            accept: false,
            label: "2 duplicate start same count",
        },
        AddCase {
            existing: &[(10, 5)],
            new: (10, 3),
            accept: false,
            label: "3 duplicate start smaller count",
        },
        AddCase {
            existing: &[(10, 5)],
            new: (10, 10),
            accept: false,
            label: "4 duplicate start larger count",
        },
        // Disjoint
        AddCase {
            existing: &[(10, 5)],
            new: (20, 5),
            accept: true,
            label: "5 disjoint after",
        },
        AddCase {
            existing: &[(20, 5)],
            new: (10, 5),
            accept: true,
            label: "6 disjoint before",
        },
        AddCase {
            existing: &[(10, 5)],
            new: (15, 5),
            accept: true,
            label: "7 touching after",
        },
        AddCase {
            existing: &[(10, 5)],
            new: (14, 5),
            accept: false,
            label: "8 shared boundary start",
        },
        // Strict nesting (accept)
        AddCase {
            existing: &[(10, 20)],
            new: (12, 5),
            accept: true,
            label: "9 new inside existing",
        },
        AddCase {
            existing: &[(12, 5)],
            new: (10, 20),
            accept: true,
            label: "10 new contains existing",
        },
        AddCase {
            existing: &[(10, 20)],
            new: (11, 18),
            accept: true,
            label: "11 inside not sharing boundaries",
        },
        AddCase {
            existing: &[(10, 20)],
            new: (10, 18),
            accept: false,
            label: "12 shared start with outer",
        },
        AddCase {
            existing: &[(10, 20)],
            new: (12, 18),
            accept: false,
            label: "13 shared end with outer",
        },
        // Partial overlap (reject)
        AddCase {
            existing: &[(10, 10)],
            new: (15, 10),
            accept: false,
            label: "14 partial overlap forward",
        },
        AddCase {
            existing: &[(15, 10)],
            new: (10, 10),
            accept: false,
            label: "15 partial overlap backward",
        },
        AddCase {
            existing: &[(10, 10)],
            new: (5, 10),
            accept: false,
            label: "16 partial overlap before",
        },
        AddCase {
            existing: &[(10, 10)],
            new: (15, 5),
            accept: false,
            label: "17 shared end with existing",
        },
        AddCase {
            existing: &[(10, 10)],
            new: (15, 4),
            accept: true,
            label: "18 contained strict inside",
        },
        // Multiple existing, predecessor walk
        AddCase {
            existing: &[(0, 100), (50, 10)],
            new: (70, 10),
            accept: true,
            label: "19 disjoint from inner contained in outer",
        },
        AddCase {
            existing: &[(0, 100), (50, 10)],
            new: (55, 3),
            accept: true,
            label: "20 inside inner and outer",
        },
        AddCase {
            existing: &[(0, 100), (50, 10)],
            new: (55, 20),
            accept: false,
            label: "21 partial overlap with inner",
        },
        AddCase {
            existing: &[(0, 100), (50, 10)],
            new: (40, 30),
            accept: true,
            label: "22 contains inner",
        },
        AddCase {
            existing: &[(0, 100), (50, 10)],
            new: (40, 25),
            accept: true,
            label: "23 contains inner shorter",
        },
        AddCase {
            existing: &[(0, 100), (50, 10)],
            new: (55, 50),
            accept: false,
            label: "24 partial overlap with inner extended",
        },
        AddCase {
            existing: &[(0, 100), (50, 10)],
            new: (110, 5),
            accept: true,
            label: "25 disjoint from both",
        },
        // Multiple existing, forward walk
        AddCase {
            existing: &[(20, 5), (40, 5)],
            new: (10, 5),
            accept: true,
            label: "26 disjoint before both",
        },
        AddCase {
            existing: &[(20, 5), (40, 5)],
            new: (10, 50),
            accept: true,
            label: "27 contains both",
        },
        AddCase {
            existing: &[(20, 5), (40, 5)],
            new: (10, 20),
            accept: true,
            label: "28 contains first disjoint from second",
        },
        AddCase {
            existing: &[(20, 5), (40, 5)],
            new: (10, 35),
            accept: false,
            label: "29 shared end with second",
        },
        AddCase {
            existing: &[(20, 5), (40, 5)],
            new: (10, 36),
            accept: true,
            label: "30 contains both extended",
        },
        // Deep nesting
        AddCase {
            existing: &[(0, 100), (10, 80), (20, 60)],
            new: (30, 40),
            accept: true,
            label: "31 deep inside all",
        },
        AddCase {
            existing: &[(0, 100), (10, 80), (20, 60)],
            new: (15, 70),
            accept: true,
            label: "32 inside middle layer",
        },
        AddCase {
            existing: &[(0, 100), (10, 80), (20, 60)],
            new: (5, 90),
            accept: true,
            label: "33 contains middle layer",
        },
        AddCase {
            existing: &[(0, 100), (10, 80), (20, 60)],
            new: (25, 80),
            accept: false,
            label: "34 partial overlap outer end",
        },
        // Edge values (via add)
        AddCase {
            existing: &[],
            new: (0, 1),
            accept: true,
            label: "35 single line at start",
        },
        AddCase {
            existing: &[],
            new: (u32::MAX - 1, 1),
            accept: true,
            label: "36 max start valid",
        },
        AddCase {
            existing: &[],
            new: (0, 0),
            accept: false,
            label: "37 count zero",
        },
        AddCase {
            existing: &[],
            new: (u32::MAX, 1),
            accept: false,
            label: "38 overflow",
        },
        AddCase {
            existing: &[],
            new: (0, FOLDED_BIT),
            accept: false,
            label: "39 count at folded bit",
        },
    ];

    #[test]
    fn folds_add_scenarios() {
        for case in ADD_CASES {
            run_add_case(case);
        }
    }

    #[test]
    fn fold_new_validation() {
        assert!(Fold::new(0, 1, true).is_some());
        assert!(Fold::new(u32::MAX - 1, 1, true).is_some());
        assert!(Fold::new(0, 0, true).is_none());
        assert!(Fold::new(u32::MAX, 1, true).is_none());
        assert!(Fold::new(0, FOLDED_BIT, true).is_none());
        assert!(Fold::new(0, FOLDED_BIT - 1, true).is_some());
    }

    #[test]
    fn fold_end_line() {
        let f = Fold::new(10, 5, true).unwrap();
        assert_eq!(f.start_line(), 10);
        assert_eq!(f.count(), 5);
        assert_eq!(f.end_line(), 14);
        assert!(f.is_folded());
    }

    #[test]
    fn folds_build_via_add() {
        let expected = &[(0, 100), (50, 10), (55, 3)];
        let mut folds = Folds::new();
        for &(start, count) in expected {
            assert!(folds.add(start, count));
        }
        assert_eq!(ranges(&folds), expected);
        assert_folds_invariant(&folds);
    }

    #[test]
    fn folds_clear() {
        let mut folds = Folds::new();
        assert!(folds.add(10, 5));
        assert!(folds.add(20, 5));
        folds.clear();
        assert!(folds.folds().is_empty());
        assert!(folds.add(1, 2));
        assert_folds_invariant(&folds);
    }
}

mod line_to_fold_tests {
    use super::super::Folds;

    fn build_folds(ranges: &[(u32, u32)]) -> Folds {
        let mut folds = Folds::new();
        for &(start, count) in ranges {
            assert!(folds.add(start, count), "setup ({start}, {count})");
        }
        folds
    }

    fn assert_line_to_fold(folds: &Folds, line: u32, expected: Option<(u32, u32)>, label: &str) {
        match folds.line_to_fold(line) {
            None => assert_eq!(expected, None, "{label}: line {line}"),
            Some(f) => {
                let (start, count) = expected.expect("{label}: expected None");
                assert_eq!(f.start_line(), start, "{label}: line {line} start");
                assert_eq!(f.count(), count, "{label}: line {line} count");
            }
        }
    }

    struct LineToFoldCase {
        folds: &'static [(u32, u32)],
        line: u32,
        expected: Option<(u32, u32)>,
        label: &'static str,
    }

    const CASES: &[LineToFoldCase] = &[
        // Empty
        LineToFoldCase {
            folds: &[],
            line: 0,
            expected: None,
            label: "empty any line",
        },
        // Single fold (10, 5) => lines 10..=14
        LineToFoldCase {
            folds: &[(10, 5)],
            line: 9,
            expected: None,
            label: "single before",
        },
        LineToFoldCase {
            folds: &[(10, 5)],
            line: 10,
            expected: Some((10, 5)),
            label: "single at start",
        },
        LineToFoldCase {
            folds: &[(10, 5)],
            line: 12,
            expected: Some((10, 5)),
            label: "single inside",
        },
        LineToFoldCase {
            folds: &[(10, 5)],
            line: 14,
            expected: Some((10, 5)),
            label: "single at end",
        },
        LineToFoldCase {
            folds: &[(10, 5)],
            line: 15,
            expected: None,
            label: "single after",
        },
        // Disjoint (10,5)=10..14, (20,5)=20..24
        LineToFoldCase {
            folds: &[(10, 5), (20, 5)],
            line: 15,
            expected: None,
            label: "disjoint gap",
        },
        LineToFoldCase {
            folds: &[(10, 5), (20, 5)],
            line: 14,
            expected: Some((10, 5)),
            label: "disjoint first end",
        },
        LineToFoldCase {
            folds: &[(10, 5), (20, 5)],
            line: 20,
            expected: Some((20, 5)),
            label: "disjoint second start",
        },
        // Touching (10,5) ends 14, (15,5) starts 15
        LineToFoldCase {
            folds: &[(10, 5), (15, 5)],
            line: 14,
            expected: Some((10, 5)),
            label: "touching first end",
        },
        LineToFoldCase {
            folds: &[(10, 5), (15, 5)],
            line: 15,
            expected: Some((15, 5)),
            label: "touching second start",
        },
        // Nested: outer (10,20)=10..29, inner (12,5)=12..16
        LineToFoldCase {
            folds: &[(10, 20), (12, 5)],
            line: 10,
            expected: Some((10, 20)),
            label: "nested outer start only",
        },
        LineToFoldCase {
            folds: &[(10, 20), (12, 5)],
            line: 11,
            expected: Some((10, 20)),
            label: "nested between outer start and inner",
        },
        LineToFoldCase {
            folds: &[(10, 20), (12, 5)],
            line: 12,
            expected: Some((12, 5)),
            label: "nested inner start",
        },
        LineToFoldCase {
            folds: &[(10, 20), (12, 5)],
            line: 16,
            expected: Some((12, 5)),
            label: "nested inner end",
        },
        LineToFoldCase {
            folds: &[(10, 20), (12, 5)],
            line: 17,
            expected: Some((10, 20)),
            label: "nested after inner before outer end",
        },
        LineToFoldCase {
            folds: &[(10, 20), (12, 5)],
            line: 29,
            expected: Some((10, 20)),
            label: "nested outer end",
        },
        LineToFoldCase {
            folds: &[(10, 20), (12, 5)],
            line: 30,
            expected: None,
            label: "nested past outer",
        },
        // Three levels: (0,100), (10,80), (20,60), (30,40)
        LineToFoldCase {
            folds: &[(0, 100), (10, 80), (20, 60), (30, 40)],
            line: 0,
            expected: Some((0, 100)),
            label: "deep outer start",
        },
        LineToFoldCase {
            folds: &[(0, 100), (10, 80), (20, 60), (30, 40)],
            line: 15,
            expected: Some((10, 80)),
            label: "deep middle layer",
        },
        LineToFoldCase {
            folds: &[(0, 100), (10, 80), (20, 60), (30, 40)],
            line: 35,
            expected: Some((30, 40)),
            label: "deep innermost",
        },
        LineToFoldCase {
            folds: &[(0, 100), (10, 80), (20, 60), (30, 40)],
            line: 69,
            expected: Some((30, 40)),
            label: "deep innermost end",
        },
        LineToFoldCase {
            folds: &[(0, 100), (10, 80), (20, 60), (30, 40)],
            line: 70,
            expected: Some((20, 60)),
            label: "deep between inner end and middle end",
        },
        LineToFoldCase {
            folds: &[(0, 100), (10, 80), (20, 60), (30, 40)],
            line: 99,
            expected: Some((0, 100)),
            label: "deep outer end",
        },
        LineToFoldCase {
            folds: &[(0, 100), (10, 80), (20, 60), (30, 40)],
            line: 100,
            expected: None,
            label: "deep past all",
        },
        // New fold contains existing: (12,5) then (10,20)
        LineToFoldCase {
            folds: &[(12, 5), (10, 20)],
            line: 14,
            expected: Some((12, 5)),
            label: "storage order inner first still innermost",
        },
        // Single-line fold (0, 1)
        LineToFoldCase {
            folds: &[(0, 1)],
            line: 0,
            expected: Some((0, 1)),
            label: "single line fold",
        },
    ];

    #[test]
    fn line_to_fold_scenarios() {
        for case in CASES {
            let folds = build_folds(case.folds);
            assert_line_to_fold(&folds, case.line, case.expected, case.label);
        }
    }

    #[test]
    fn line_to_fold_returns_copy() {
        let folds = build_folds(&[(10, 5)]);
        let a = folds.line_to_fold(12).unwrap();
        let b = folds.line_to_fold(12).unwrap();
        assert_eq!(a.start_line(), b.start_line());
        assert_eq!(a.count(), b.count());
    }

    #[test]
    fn line_to_fold_unfolded_still_maps() {
        let mut folds = Folds::new();
        assert!(folds.add(5, 3));
        let mut f = folds.line_to_fold(6).unwrap();
        f.set_folded(false);
        // Lookup does not consult folded flag.
        let found = folds.line_to_fold(6).unwrap();
        assert_eq!(found.start_line(), 5);
        assert_eq!(found.count(), 3);
        assert!(found.is_folded());
    }
}

mod visible_lines_tests {
    use super::super::Folds;

    fn build_folds(ranges: &[(u32, u32)]) -> Folds {
        let mut folds = Folds::new();
        for &(start, count) in ranges {
            assert!(folds.add(start, count), "setup ({start}, {count})");
        }
        folds
    }

    fn visible_lines(folds: &Folds, from: u32, count: usize) -> Vec<u32> {
        folds.visible_lines_from(from).take(count).collect()
    }

    /// Folds `[(5, 3), (20, 4)]` (folded): headers 5 and 20 visible; 6–7 and 21–23 hidden.
    fn example_folds() -> Folds {
        build_folds(&[(5, 3), (20, 4)])
    }

    #[test]
    fn visible_lines_from_zero_two_folds() {
        let folds = example_folds();
        assert_eq!(
            visible_lines(&folds, 0, 10),
            vec![0, 1, 2, 3, 4, 5, 8, 9, 10, 11],
            "take(10): 0..4, header 5, skip 6–7, then 8.."
        );
        assert_eq!(
            visible_lines(&folds, 0, 20),
            vec![
                0, 1, 2, 3, 4, 5, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 24
            ],
            "take(20): through header 20, skip 21–23, resume at 24"
        );
    }

    #[test]
    fn visible_lines_skips_hidden_start() {
        let folds = example_folds();
        // Line 6 is inside folded (5, 3); iterator should advance to 8.
        assert_eq!(visible_lines(&folds, 6, 5), vec![8, 9, 10, 11, 12]);
        assert_eq!(visible_lines(&folds, 7, 3), vec![8, 9, 10]);
        assert_eq!(visible_lines(&folds, 21, 4), vec![24, 25, 26, 27]);
    }

    #[test]
    fn visible_lines_fold_header_stays_visible() {
        let folds = example_folds();
        assert_eq!(visible_lines(&folds, 5, 4), vec![5, 8, 9, 10]);
        assert_eq!(visible_lines(&folds, 20, 4), vec![20, 24, 25, 26]);
    }

    #[test]
    fn visible_lines_no_folds_is_sequential() {
        let folds = Folds::new();
        assert_eq!(visible_lines(&folds, 0, 5), vec![0, 1, 2, 3, 4]);
        assert_eq!(visible_lines(&folds, 3, 4), vec![3, 4, 5, 6]);
    }

    #[test]
    fn visible_lines_never_yields_hidden_body() {
        let folds = example_folds();
        let lines: Vec<u32> = folds.visible_lines_from(0).take(50).collect();
        for hidden in [6, 7, 21, 22, 23] {
            assert!(!lines.contains(&hidden), "hidden line {hidden} must not appear");
        }
        assert!(lines.contains(&5));
        assert!(lines.contains(&20));
    }

    #[test]
    fn visible_lines_single_fold() {
        // (10, 5): lines 10..14, body 11..14 hidden
        let folds = build_folds(&[(10, 5)]);
        assert_eq!(visible_lines(&folds, 0, 12), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15]);
        assert_eq!(visible_lines(&folds, 12, 4), vec![15, 16, 17, 18]);
    }

    #[test]
    fn visible_lines_before_first_fold() {
        let folds = example_folds();
        assert_eq!(visible_lines(&folds, 3, 5), vec![3, 4, 5, 8, 9]);
    }

    #[test]
    fn visible_lines_between_folds() {
        let folds = example_folds();
        assert_eq!(visible_lines(&folds, 16, 6), vec![16, 17, 18, 19, 20, 24]);
    }
}
