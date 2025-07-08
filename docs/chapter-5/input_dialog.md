# Input Dialog

An input dialog is a dialog that allows the user to enter a value of a generic type T and returns the value as an optional.

To create an input dialog, use the `dialogs::input` function, defined as follows:

```rust
pub fn input<T>(title: &str, text: &str, value: Option<T>, validation: Option<fn(&T) -> Result<(), String>>) -> Option<T>
where
    T: FromStr + Sized + std::fmt::Display + 'static,
{
    ...
}
```

The function takes the following parameters:

- `title`: The title of the dialog.
- `text`: The text to display in the dialog.
- `value`: An optional value to pre-fill the input field with.
- `validation`: An optional validation function that can be used to validate the input value.

The function returns an optional value of type T.

It is important that type T implements the following traits:
* `FromStr` - to be able to parse the input value from a string.
* `Sized` - to be able to store the value in a variable.
* `std::fmt::Display` - to be able to display the value in the dialog.

If the validation function is provided, it will be called with the input value and should return a `Result` with an error message if the value is invalid.

If the validation function returns an error, the dialog will show the error message and the user will be able to try again.



