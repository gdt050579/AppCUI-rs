# DropDownListType

The `DropDownListType` trait is designed to provide a structured way of associating an enum with a dropdown selection mechanism.

```rs
pub trait DropDownListType {
    fn name(&self) -> &str;
    fn description(&self) -> &str {
        ""
    }
    fn symbol(&self) -> &str {
        ""
    }
}
```

### Methods

* `name()`  
  Returns the display name of the enum variant. This is typically used for identifying the variant in UI dropdowns and selection lists.

* `description()`  
  Provides additional context or details about the enum variant. By default, it returns an empty string, but it can be overridden to give more meaningful descriptions.

* `symbol()`
  Returns a symbolic representation of the variant, which can be useful for visual representation in dropdowns.

---

## Example Usage

Consider the following `MathOp` enum:

```rs
enum MathOp {
    Sum,
    Product,
    Integral,
    Radical,
    Different,
}
```
## Implementing `DropDownListType`

To integrate this enum with a dropdown selection mechanism, you can implement `DropDownListType` manually:

```rs
impl DropDownListType for MathOp {
    fn name(&self) -> &str {
        match self {
            MathOp::Sum => "Sum",
            MathOp::Product => "Product",
            MathOp::Integral => "Integral",
            MathOp::Radical => "Radical",
            MathOp::Different => "Different",
        }
    }

    fn description(&self) -> &str {
        match self {
            MathOp::Sum => "Add multiple numbers",
            MathOp::Product => "Multiply multiple numbers",
            MathOp::Integral => "Calculate the integral of a function",
            MathOp::Radical => "Calculate the radical of a number",
            MathOp::Different => "Check if all elements from a set are different",
        }
    }

    fn symbol(&self) -> &str {
        match self {
            MathOp::Sum => "∑",
            MathOp::Product => "∏",
            MathOp::Integral => "∫",
            MathOp::Radical => "√",
            MathOp::Different => "≠",
        }
    }
}
```
---

## Using the `DropDownListType` Derive Macro

Instead of manually implementing the `DropDownListType` trait, you can use the derive macro to automatically generate the implementation.

### Example:

```rs
#[derive(DropDownListType)]
enum MathOp {
    #[VariantInfo(name = "Sum", description = "Add multiple numbers", symbol = "∑")]
    Sum,

    #[VariantInfo(name = "Product", description = "Multiply multiple numbers", symbol = "∏")]
    Product,

    #[VariantInfo(name = "Integral", description = "Calculate the integral of a function", symbol = "∫")]
    Integral,

    #[VariantInfo(name = "Radical", description = "Calculate the radical of a number", symbol = "√")]
    Radical,

    #[VariantInfo(name = "Different", description = "Check if all elements from a set are different", symbol = "≠")]
    Different,
}
```