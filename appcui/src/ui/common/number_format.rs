#[derive(Clone, Copy)]
/// How a numeric value is displayed in controls such as [`crate::ui::NumericSelector`].
///
/// Choose plain decimal, a percentage, thousands grouping, hexadecimal, or a
/// human-readable size.
pub enum NumberFormat {
    /// Plain decimal, for example `1234`.
    Decimal,
    /// A percentage with a `%` suffix, for example `12%`.
    Percentage,
    /// Decimal with thousands separators, for example `1,234`.
    DigitGrouping,
    /// Hexadecimal, for example `0x4D2`.
    Hex,
    /// Human-readable size, for example `1 KB`.
    Size, 
}