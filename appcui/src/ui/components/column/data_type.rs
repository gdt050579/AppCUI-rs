#[derive(Copy, Clone, Eq, PartialEq)]
/// How a boolean column value is shown in a list header.
///
/// Used by [`Type`] when a column stores booleans, choosing labels such as True/False
/// or a check-mark glyph.
pub enum BoolFormat {
    /// The words `True` / `False`.
    TrueFalse,
    /// The words `Yes` / `No`.
    YesNo,
    /// The words `On` / `Off`.
    OnOff,
    /// A checkbox glyph, for example `☑` / `☐`.
    CheckBox,
    /// A bare check mark, for example `√` / empty.
    CheckMark
}

#[derive(Copy, Clone, Eq, PartialEq)]
/// The value type stored in a list column, used for sorting and display.
///
/// A column is either a string or a boolean with a display format such as True/False or a check mark.
pub enum Type {
    /// A text column.
    String,
    /// A boolean column using the given display format.
    Bool(BoolFormat),
}