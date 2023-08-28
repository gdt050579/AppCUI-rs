# Docking

To dock a control inside its parent use ``d`` or ``dock`` key. 
When docking a control, the following key can not be used: ``align``, ``x``, ``y``, ``left``, ``right``, ``top``, ``bottom``.
`Width` and `height` should be used to specify the size of control. If not specified, they are defaulted to ``100%``.

| Layout                               | Outcome                                                        |
|--------------------------------------|----------------------------------------------------------------|
| **d:c,w:10,h:5**                     | .. image:: pics/layout_dock_c_10_5.png                         |
| **d:c,w:50%,h:75%**                  | .. image:: pics/layout_dock_c_50p_75p.png                      |
| **d:br,w:50%**                       | As ``height`` is not specified, it will be defaulted to 100%   |
|                                      |                                                                |
|                                      | .. image:: pics/layout_dock_br_50p.png                         |
|  **d:c** or **d:tl** or **d:br** ... |  As both ``width`` and ``height`` parameters are missing, they |
|                                      |  will be defaulted to **100%**. This means that curren control |
|                                      |  will ocupy its entire parent surface. This is the easyest way |
|                                      |  to make a control fill all of its parent surface.             |
|                                      |                                                                |
|                                      | .. image:: pics/layout_dock_fill.png                           |
