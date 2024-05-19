# Selector

A selector is a templetize (generics based) control that allows you to select a variant of an enum. 

<img src="img/selector.png" width=300/>

It can be create using `Selector::new(...)` or the `selector!` macro. Using `Selector::new(...)` can be done in two ways:
1. by specifying the type for a variable:
    ```rs
    let s: Selector<T> = Selector::new(...);
    ```

2. by using turbo-fish notation (usually when you don't want to create a separate variable for the control):
    ```rs
    let s = Selector::<T>::new(...);
    ```
**Remarks**: It is important to notice that the `T` type must implement a special trait `EnumSelector` as well as `Copy`, `Clone`, `Eq` and `PartialEq`. The `EnumSelector` trait is defined as follows:

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

where:
* `COUNT` must contain thet number of variants
* `from_index(...)` is a method that converts an index (from `0` to `COUNT`) to a variant
* `name()` is a method that provides a string representation (name) for a specific variant
* `description()` is a method that provides a detailed description for a specific variant

**Remarks**: While a `Selector` should normally be used with an enum, it can be used with any type that implements `Copy`, `Clone`, `Eq`, `PartialEq` and `EnumSelector`.

The proc macro should be used in the following way: `selector!(enum=T,...)`

## Examples

Assuming we have the following enum: `Animal` thet implements the required traits as follows:

```rs
#[derive(Copy,Clone,PartialEq,Eq)]
enum Animal { Cat,Mouse,Dog }

impl EnumSelector for Animal { ... }
```

then we can create a selector object based on this type as follows:

```rs
let s1: Selector<Animal> = Selector::new(Some(Animal::Dog),Layout::new("..."),selector::Flags::None);
let s2: Selector<Animal> = Selector::new(None,Layout::new("..."),selector::Flags::AllowNoneVariant);
let s3 = selector!("Animal,value:Dog,x:1,y:1,w:20");
let s4 = selector!("enum: Animal,x:1,y:1,w:20, flags: AllowNoneVariant");
```

A selector supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name   | Type   | Positional parameter                | Purpose                                                                                                                                                                                                                       |
| ---------------- | ------ | ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enum` or `type` | String | **Yes** (first postional parameter) | The name of a templetized type to be used when creating the selector                                                                                                                                                          |
| `flags`          | String | **No**                              | Selector initialization flags                                                                                                                                                                                                 |
| `value`          | String | **No**                              | The initial value of the selector (should be one of the variants of the enum). If not specified, `None` is assume (and you **MUST** also set the `AllowNoneVariant` flag on initalization). If you don't a panic will occur ! |

A selector supports the following initialization flags:
* `selector::Flags::AllowNoneVariant` or `AllowNoneVariant` (for macro initialization) - thils will allow a selector to hold a `None` value as well. If it is not specified, the value from a selector will always be one of the variants from the templetized type.

## Events

To intercept events from a selector, the following trait has to be implemented to the Window that processes the event loop:
```rs
pub trait SelectorEvents<T> {
    fn on_selection_changed(&mut self, handle: Handle<Selector<T>>, value: Option<T>) -> EventProcessStatus {...}
}
```

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a selector also has the following aditional methods:

| Method           | Purpose                                                                                                                                 |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `set_value(...)` | Sets the new value associated with the selector. The value will be of type `T` (that was used to templetized the selector control)      |
| `clear_value()`  | Sets the new value of the control to `None`. If the flag `AllowNoneVariant` was not set when the control was create, a panic will occur |
| `value()`        | Returns the current value of the control. If the current value is `None` a panic will occur                                             |
| `try_value()`    | Returns an `Option<T>` containint the current value of the control.                                                                     |

**Remarks**: If the flag `AllowNoneVariant` was set, it is recommended to use `try_value()` method. If not, you can **safely** use the `value()` method.

## Key association

The following keys are processed by a `Selector` control if it has focus:

| Key                            | Purpose                                                                                                                                    |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `Space` or `Enter`             | Expands or packs (collapses) the Selector control.                                                                                         |
| `Up`, `Down`, `Left`,  `Right` | Changes the current selected color from the Selector.                                                                                      |
| `PageUp`, `PageDown`           | Navigates through the list of variants page by page. If the control is not expanded, their behavior is similar to the keys `Up` and `Down` |
| `Home`                         | Move the selection to the first variant                                                                                                    |
| `End`                          | Move the selection to the last variant or to `None` if `AllowNoneVariant` flag was set upon initialization                                 |

Besides this using any one of the following keys: `A` to `Z` and/or `0` to `9` will move the selection to the fist variant that starts with that letter (case is ignored). The search starts from the next variant after the current one. This means that if you have multiple variants that starts with letter `G`, pressing `G` multiple times will efectively switch between all of the variants that starts with letter `G`.

When the selector is expanded the following additional keys can be used:

| Key           | Purpose                                                                                                                                                                           |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl`+`Up`   | Scroll the view to top. If the new view can not show the current selection, move the selection to the previous value so that it would be visible                                  |
| `Ctrl`+`Down` | Scroll the view to bottom. If the new view can not show the current selection, move the selection to the next value so that it would be visible                                   |
| `Escape`      | Collapses the control. If the Selector is already colapsed, this key will not be captured (meaning that one of the Selector ancestors will be responsable with treating this key) |

## Example

The following example creates a Window with a Selector that can chose between 4 animals: a cat, a dog, a horse and a mouse. When an animal is being selected the title of the window changes to reflect this.
```rs
use appcui::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Animals {
    Cat,
    Dog,
    Horse,
    Mouse,
}
impl EnumSelector for Animals {
    const COUNT: u32 = 4;

    fn from_index(index: u32) -> Option<Self>
    where
        Self: Sized,
    {
        match index {
            0 => Some(Animals::Cat),
            1 => Some(Animals::Dog),
            2 => Some(Animals::Horse),
            3 => Some(Animals::Mouse),
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Animals::Cat => "Cat",
            Animals::Dog => "Dog",
            Animals::Horse => "Horse",
            Animals::Mouse => "Mouse",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Animals::Cat => "A cat is a ...",
            Animals::Dog => "A dog is a ...",
            Animals::Horse => "A horse is a ...",
            Animals::Mouse => "A mouse is a ...",
        }
    }
}

#[Window(events = SelectorEvents<Animals>)]
struct MyWin {}
impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("x:1,y:1,w:30,h:8,caption:Win"),
        };
        w.add(selector!("Animals,value:Cat,x:1,y:1,w:26"));
        w
    }
}
impl SelectorEvents<Animals> for MyWin {
    fn on_selection_changed(&mut self, _handle: Handle<Selector<Animals>>, 
                                       value: Option<Animals>) -> EventProcessStatus 
    {
        self.set_title(value.unwrap().name());
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
```