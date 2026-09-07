#[derive(Clone, Copy)]
/// How a numeric value is displayed in controls such as [`crate::ui::NumericSelector`].
///
/// Choose plain decimal, a percentage, thousands grouping, hexadecimal, or a
/// human-readable size.
pub enum NumberFormat {
    Decimal,
    Percentage,
    DigitGrouping,
    Hex,
    Size, 
}