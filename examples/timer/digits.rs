static DIGIT_0: &str = "
┌───┐
│   │
│   │
│   │
└───┘
";

static DIGIT_1: &str = "
    ┐
    │
    │
    │
    ┘
";

static DIGIT_2: &str = "
┌───┐
    │
┌───┘
│    
└───┘
";

static DIGIT_3: &str = "
┌───┐
    │
 ───┤
    │
└───┘
";

static DIGIT_4: &str = "
┌   ┐
│   │
└───┤
    │
    ┘
";

static DIGIT_5: &str = "
┌───┐
│
└───┐
    │
└───┘
";

static DIGIT_6: &str = "
┌───┐
│
├───┐
│   │
└───┘
";

static DIGIT_7: &str = "
┌───┐
    │
    │
    │
    ┘
";

static DIGIT_8: &str = "
┌───┐
│   │
├───┤
│   │
└───┘
";

static DIGIT_9: &str = "
┌───┐
│   │
└───┤
    │
└───┘
";

pub(crate) fn digit_to_text(digit: u8) -> &'static str {
    match digit {
        0 => &DIGIT_0,
        1 => &DIGIT_1,
        2 => &DIGIT_2,
        3 => &DIGIT_3,
        4 => &DIGIT_4,
        5 => &DIGIT_5,
        6 => &DIGIT_6,
        7 => &DIGIT_7,
        8 => &DIGIT_8,
        9 => &DIGIT_9,
        _ => ""
    }
}
