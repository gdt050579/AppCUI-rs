# Absolute position

In this mode parameters `x` and `y` must be used to specify a point from where the control will be constructed. 
When using this mode, parameters `dock`, `left`, `right`, `top`, `bottom` can not be used. 
If `width` or `height` are not specified , they will be defaulted to `1 character` (unless there is a minimum width or minumum height specified for that controls - in which case that limit will be applied).
If `align` is not specified, it will be defaulted to `topleft` 

If `x`, `y`, `width` or `height` are provided using percentages, the control will automatically adjust its size if its parent size changes. 

| Layout                         | Result                                                                 |
|--------------------------------|------------------------------------------------------------------------|
| **x:5,y:5,w:10,h:10** or **x:5,y:5,w:10,h:10,a:tl**      |If no alignament is provided, top-left will be considered as a default. |
|                                | .. image:: pics/layout_xywh_tl.png                                     |
| **x:30,y:20,w:10,h:4,a:br**    | .. image:: pics/layout_xywh_br.png                                     |
| **x:50%,y:50%,w:10,h:3,a:c**   | .. image:: pics/layout_xywh_c.png                                      |
| **x:50%,y:50%,w:100%,h:3,a:c** | .. image:: pics/layout_xywh_c_full_width.png                           |
