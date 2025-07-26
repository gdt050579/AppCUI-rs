# The `Layout` Structure

The `Layout` struct is the primary type used to define how controls are positioned and sized within their parent containers in AppCUI. It serves as a wrapper around layout format information and provides the main interface for specifying control layouts.

The `Layout` struct provides several methods that can be used to quickly create a layout (for the most common use cases).
- **absolute(...)**  - The control is positioned using explicit coordinates (`x`, `y`) for the top-left corner and fixed dimensions (`width`, `height`).  
    ```rust
    use appcui::prelude::*;

    // Creates a control positioned at (8, 5) with a size of 33x6 characters.
    let layout = Layout::absolute(8, 5, 33, 6);
    ```

- **pivot(...)**  - The control is positioned relative to a reference point (`x`, `y`) and a [`Pivot`] value, which determines which part of the control attaches to that point (e.g., top-left, center, bottom-right).  
    ```rust
    use appcui::prelude::*;

    // Creates a control centered around the point (10, 10) 
    // with a size of 6x6 characters. The control will start from
    // the point (7,7), where 7 = 10 - 6/2, and will expand until
    // the point (13,13), where 13 = 10 + 6/2.
    let layout = Layout::pivot(10, 10, 6, 6, Pivot::Center);
    ```

- **fill()**  
   The control automatically resizes to completely fill its parent container.  
   ```rust
   use appcui::prelude::*;

   // Creates a layout that occupies the entire parent container.
   let layout = Layout::fill();
   ```

- **aligned(...)**  
   The control is aligned relative to the parent using an [`Alignment`] value (e.g., top-left, center, bottom-right). Its size remains fixed.  
   ```rust
   use appcui::prelude::*;

   // Creates a control aligned to the bottom-right corner of 
   // the parent with a fixed size of 20x5 characters.
   let layout = Layout::aligned(Alignment::BottomRight, 20, 5);
   ```

## Remarks:
- while the Layout can work with percentage values, the methods from the Layout struct only support absolute values. If percentages are needed use [LayoutBuilder](layout_builder.md) instead.
- not all possible layouts can be created with the methods from the Layout struct. For example, using various anchors or different dock method are not possible. In this case, use [LayoutBuilder](layout_builder.md) or the [layout!](layout_procmacro.md) proc-macro instead.


