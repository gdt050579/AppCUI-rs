# Menu Items

Each menu is form out of menu items. AppCUI supports the following menu items:
- [Command]() : a command (clicking this item will send a command)
- [CheckBox]() : a item that has two states (`checked` or `unchecked`). Clicking on this item will change the state (from `checked` to `unchecked` and vice-versa) and will send a command.
- [SingleChoice]() : a item that is part of a group of items from which only one can be selected at a moment of time. Clicking on this item will select it (an implicelely unselect any other selected SingleChoice item from the group) and send a command.
- [SubMenu]() : a item that contains another menu. Clicking on this item will open the sub-menu.
- [Separator](): a item that has no input and is represented by a horizontal line that separates groups or commands withing a menu.