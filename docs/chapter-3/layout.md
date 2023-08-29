# Layout

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
| align   | a    | alignament value         | the way the entire control is aligne against a fix point |

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


