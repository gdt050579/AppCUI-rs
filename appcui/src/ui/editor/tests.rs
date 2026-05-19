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
