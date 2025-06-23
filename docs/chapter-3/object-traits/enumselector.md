# EnumSelector
The `EnumSelector` trait is a trait design to provide controls like [Selector](../stock-controls/selector.md). The trait has to be implemented by the enum types you wish to associate with a selection mechanism.

```rs
pub trait EnumSelector {
    const COUNT: u32;
    fn from_index(index: u32) -> Option<Self> where Self: Sized;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str {
        ""
    }
}
```
These methods have the following purpose:

* `COUNT`   
  This constant defines the total number of variants in the enum. It is used to assist in selection logic and must be set appropriately.

* `from_index(index)`
  This method is responsible for mapping a provided index to a specific variant of the enum. It should return `Some(variant)` if the index is valid, and `None` if the index is out of bounds.

* `name()`  
  This method returns the name of the enum variant as a static string. It can be used to display the name of the variant in user interfaces or for documentation purposes.

* `description()`
  This method provides a description of the enum variant. By default, it returns an empty string, but it can be overridden to provide more detailed information about the variant.

  **Example**

Lets consider the following enum: `Shape` with the following structure:
```rs
enum Shape {
    Square,
    Rectangle,
    Triangle,
    Circle,
}
```
In order to use this enum in a EnumSelector, the minimum implementation of the `EnumSelector` trait would be:

```rs
impl EnumSelector for Shape {
    const COUNT: u32 = 4;

    fn from_index(index: u32) -> Option<Self> where Self: Sized {
        match index {
            0 => Some(Shape::Square),
            1 => Some(Shape::Rectangle),
            2 => Some(Shape::Triangle),
            3 => Some(Shape::Circle),
            _ => None
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Shape::Square => "Square",
            Shape::Rectangle => "Rectangle",
            Shape::Triangle => "Triangle",
            Shape::Circle => "Circle",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Shape::Square => "a red square",
            Shape::Rectangle => "a green rectangle",
            Shape::Triangle => "a blue triangle",
            Shape::Circle => "a white circle",
        }        
    }
}
```

Alternatively, you can use the `EnumSelector` derive macro to automatically implement the `EnumSelector` trait for a enum
This means that the previous `Shape` enum can be rewritten as follows:

```rs
#[derive(EnumSelector, Eq, PartialEq, Copy, Clone)]
enum Shape {
    #[VariantInfo(name = "Square", description = "a red square")]
    Square,

    #[VariantInfo(name = "Rectangle", description = "a green rectangle")]
    Rectangle,

    #[VariantInfo(name = "Triangle", description = "a blue triangle")]
    Triangle,

    #[VariantInfo(name = "Circle", description = "a white circle")]
    Circle,
}
```

Make sure that you also add the following derives: `Eq`, `PartialEq`, `Copy` and `Clone` to the enum. This is required for the `EnumSelector` derive macro to work properly.