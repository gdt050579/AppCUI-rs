# Layout

AppCUI supports the following layout modes:
* **Absolute** - The control is positioned using an explicit top-left coordinate (x, y) and a fixed size (width, height).
(Example: `x = 10, y = 5, width = 20, height = 3`)

* **Pivot** - The control is positioned relative to a reference point (x, y) which acts as a pivot.
The pivot determines how the control’s rectangle is placed around that point (e.g., TopLeft, Center, BottomRight).
(Example: `pivot = Center means (x, y) is the center of the control`)

* **Dock** - The control is attached to one of its parent’s edges (Left, Right, Top, Bottom) or fills the remaining space (Fill).
The control stretches along the opposite axis to fully occupy the available space.
(Example: `Dock::Top → control fills width and stays at the top of the parent`)

* **Aligned** - The control is aligned relative to its parent’s bounding box, using one of nine alignment positions:
TopLeft, TopCenter, TopRight, CenterLeft, Center, CenterRight, BottomLeft, BottomCenter, BottomRight.
(Example: `Align::BottomRight → control is placed at the bottom-right corner of the parent with its own size`)

* **Anchored** - The control is attached to one or more of the parent’s margins (left, right, top, bottom).
Anchors determine how the control resizes when the parent changes size:
    - If both left and right anchors are set → control’s width is adjusted dynamically.
    - If both top and bottom anchors are set → control’s height is adjusted dynamically.


## Creating a layout

You can create a layout using the following 3 methods:

### 1. Using the `Layout` class
This is the simplest and fastest way to create a layout, useful for scenarios where you need a basic configuration:

```rs
// An absolute layout with:
// - x = 10
// - y = 20
// - width = 30
// - height = 40
let l = Layout::absolute(10, 20, 30, 40);
```

### 2. Using the `LayoutBuilder` class
LayoutBuilder gives you full access to the layout system, including support for percentages and combining various layout parameters:

```rs
// A pivot layout with:
// - x = 10
// - y position at 50% of the parent's height
// - width = 12 characters
// - height = 25% of the parent's height
// - pivot set to Center (the control is centered around the (x, y) point)
let l = LayoutBuilder::new()
    .x(10)
    .y(0.5)          // 50% of parent height
    .width(12)       // absolute width
    .height(0.25)    // 25% of parent height
    .pivot(Pivot::Center)
    .build();
```
### 3. Using the `layout!` procedural macro

The `layout!` macro provides the same capabilities as LayoutBuilder in a more concise and readable way:

```rs
let l = layout!("x:10, y:50%, w:12, h:25%, p:center");
```

## old
Each control in AppCUI is created based on a layout rule that can be described as an ascii string that respects the following format:

```rs
"key:value , key:value , ... key:value"
```

Where key can be one of the following:

| Key     | Alias<br>(short)| Value type               | Description                                              |
|---------|------|--------------------------|----------------------------------------------------------|
| x       |      | numerical or percentage  | "X" coordonate                                           |
| y       |      | numerical or percentage  | "Y" coordonate                                           |
| left    | l    | numerical or percentage  | left anchor for the control<br>(the space between parent left margin and control)|                              
| right   | r    | numerical or percentage  | right anchor for the control<br>(the space between parent right margin and control)|
| top     | t    | numerical or percentage  | top anchor for the control<br>(the space between parent top margin and control)|
| bottom  | b    | numerical or percentage  | bottom anchor for the control<br>(the space between parent bottom margin and control)|
| width   | w    | numerical or percentage  | the width of the control                                 |
| height  | h    | numerical or percentage  | the height of the control                                |
| dock    | d    | docking value            | the way the entire control is docked on its parent       |
| align   | a    | alignment value         | the way the entire control is aligne against a fix point |

**Remarks**
* Key aliases can be use to provide a shorter format for a layout. In other words, the following two formats are identical: ``x:10,y:10,width:30,height:30`` and ``x:10,y:10,w:30,h:30``
* A numerical value is represented by an integer (positive and negative) number between **-30000** and **30000**. Example: ``x:100`` --> X will be 100. Using a value outside accepted interval (**[-30000..30000]**) will reject the layout.
* A percentage value is represented by a floating value (positive and negative) succeded by the character ``%`` between **-300%** and **300%**. Example: ``x:12.75%`` --> X will be converted to a numerical value that is equal to the width of its parent multiplied by ``0.1275``. Using a value outside accepted interval (**[-300%..300%]**) will reject the layout. Percentage values can be use to ensure that if a parent size is changed, its children change their size with it.
* All layout keys are case insensitive (meaning that 'left=10' and 'LEFT=10' have the same meaning)

Dock values can be one of the following:                              
<table>
<tr>
<td style="width:50%; vertical-align:top;">

![Alt text for image](layout/img/layout_dock.png)

</td>

<td style="width:50%; vertical-align:center;">

| Value       | Alias               | 
|-------------|---------------------|
| topleft     | lefttop, tl, lt     |
| top         | t                   |
| topright    | righttop, tr, rt    |
| right       | r                   |                                 
| bottomright | rightbottom, br, rb |                                 
| bottom      | b                   |                                 
| bottomleft  | leftbottom, lb, bl  |                                 
| left        | l                   |                                 
| center      | c                   | 

</td>

</tr>
</table>



**Remarks**: 
* Dock value aliases can be use to provide a shorter format for a layout. In other words: ``dock:topleft`` is the same with ``dock:tl`` or ``dock:lt`` or ``d:tl``

Align values have the same name as the docking ones, but they refer to the direction of width and height from a specific point (denoted by "X" and "Y" keys). Align parameter is used to compute top-left and bottom-right corner of a control that is described using a (X,Y) coordonate. The following table ilustrate how this values are computed:

| Value       | Alias               | Top-Left corner        | Bottom-Right corner    |
|-------------|---------------------|------------------------|------------------------|
| topleft     | lefttop, tl, lt     | (x,y)                  | (x+width,y+height)     |
| top         | t                   | (x-width/2,y)          | (x+width/2,y+height)   |
| topright    | righttop, tr, rt    | (x-width,y)            | (x,y+height)           |
| right       | r                   | (x-width,y-height/2)   | (x,y+height/2)         |
| bottomright | rightbottom, br, rb | (x-width,y-height)     | (x,y)                  |
| bottom      | b                   | (x-width/2,y-height)   | (x+width/2,y)          |
| bottomleft  | leftbottom, lb, bl  | (x,y-height)           | (x+width,y)            |
| left        | l                   | (x,y-height/2)         | (x+width,y+height/2)   |
| center      | c                   | (x-width/2,y-height/2) | (x+width/2,y+height/2) |

**Remarks**: 
* Align value aliases can be use to provide a shorter format for a layout. In other words: ``align:center`` is the same with ``align:c`` or  ``a:c``


