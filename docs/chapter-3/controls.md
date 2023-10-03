# Controls

All controls from AppCUI follow a tree-like organization (a control has a `parent control` and may have multiple `children controls`).

<img src="img/controls_architecture.png" />

**Remarks**
* There is only one Desktop control. AppCUI provides a default one, but costom desktop can be created as well
* A Desktop control can have one or multiple Windows.
* All events emitted by any control are process at window level
* A control may contain other children controls