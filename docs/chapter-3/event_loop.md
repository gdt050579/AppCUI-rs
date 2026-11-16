# Event loop

`AppCUI` is an event-driven framework, meaning that each control can emit events to reflect various actions or changes that occur. For example, whenever you push a button, an event will be raised. All events are processed at window level by implementing various traits. A window that supports event handling is usually declared with the `#[Window]` procedural macro:

```rs
#[Window(events=..., )]
struct MyWindow {
    // specific fields
}
```

where the attribute `events` has the following form:
* `events=EventTrait-1 + EventTrait-2 + EventTrait-3 + ... EventTrait-n`

An event trait is the trait of a child control, for example:
* ButtonEvents
* CheckBoxEvents
* RadioBoxEvents
* WindowEvents
* MenuEvents
* CommandBarEvents
* ToolBarEvents
* ColorPickerEvents
* ThreeStateBoxEvents
* PasswordEvents
* KeySelectorEvents
* TextFieldEvents
* ... and more (every control event trait is an event trait)

Any other `*Events` trait documented on a [stock control](stock_controls.md) page (for example `ListViewEvents<T>` or `DatePickerEvents`) is written in `events` the same way. The short name without the `Events` suffix is also accepted (`Button` and `ButtonEvents` select the same trait).

These traits can be implemented to receive notifications about various actions that child controls are performing.

## What the `#[Window]` attribute does

`#[Window(...)]` is a procedural macro brought into scope by `use appcui::prelude::*;`. It rewrites the struct so the application can register it as a window, and so the event handlers you care about are the only ones you have to write.

The attribute does three things:

1. It inserts a field named `base`, of type [`Window`](event-loop/window.md), as the **first** field of the struct, and marks the struct `#[repr(C)]`. The runtime stores the object and reads its window state from the start of that allocation, so `base` has to stay first.
2. It implements [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) and [`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) with `Target = Window`. Calls such as `my_win.add(...)` or `my_win.set_title(...)` are forwarded to `self.base`.
3. It implements the traits [`App::window`](../chapter-2/multi_window.md) requires: `Control`, `WindowControl`, and `NotModalWindow`. `Control` itself requires every input trait (`OnPaint`, `OnKeyPressed`, `OnMouseEvent`, and the rest) and every control event trait. Paint, keyboard, mouse, resize, focus, the default action, and window registration are forwarded to `self.base`, so the window still draws its frame and handles input. Every event trait that is **not** named in `events` receives an empty implementation; the trait's default methods ignore the event. Every event trait that **is** named in `events` is left out, and you implement it yourself.

This declaration:

```rs
#[Window(events = ButtonEvents)]
struct MyWin {
    x: i32,
}
```

is expanded along these lines (the generated code is longer; the shape is what matters):

```rs
#[repr(C)]
struct MyWin {
    base: Window, // inserted by the macro; stays the first field
    x: i32,
}

impl std::ops::Deref for MyWin {
    type Target = Window;
    fn deref(&self) -> &Window {
        &self.base
    }
}
impl std::ops::DerefMut for MyWin {
    fn deref_mut(&mut self) -> &mut Window {
        &mut self.base
    }
}

impl Control for MyWin {}
impl WindowControl for MyWin {}
impl NotModalWindow for MyWin {}

impl OnPaint for MyWin {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        self.base.on_paint(surface, theme);
    }
}
// OnKeyPressed, OnMouseEvent, OnResize, OnFocus, OnDefaultAction,
// and OnWindowRegistered are forwarded to `base` in the same way.

impl CheckBoxEvents for MyWin {}
impl RadioBoxEvents for MyWin {}
impl WindowEvents for MyWin {}
// ...an empty impl for every other event trait required by `Control`
```

`impl ButtonEvents for MyWin` is not generated. Naming `ButtonEvents` in `events` is the request to write that implementation yourself:

```rs
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        // react to the button
        EventProcessStatus::Processed
    }
}
```

Because the macro adds `base`, every constructor initializes it:

```rs
MyWin {
    base: Window::new(title, layout, flags),
    x: 0,
}
```

`window!(...)` can stand in for `Window::new(...)`. The value is still stored in `base`.

`#[ModalWindow(...)]` rewrites a struct the same way. Its `base` field has type `ModalWindow<Response>` instead of `Window`. See [Modal Window](event-loop/modal_window.md).

To print the code the compiler actually compiles, install [`cargo-expand`](https://github.com/dtolnay/cargo-expand) and run `cargo expand` on the module that contains the struct. The output is this expansion, with every generated impl written out.

### Compiler errors for `#[Window]`

The messages around a `#[Window]` struct follow from the expansion above.

| Message | Cause and fix |
| --- | --- |
| `missing field 'base' in initializer` | The macro added `base: Window`. Initialize it in every struct literal with `Window::new(...)` or `window!(...)`. |
| `conflicting implementations of trait 'ButtonEvents' for type 'MyWin'` | `impl ButtonEvents for MyWin` is present, and the macro generated an empty one as well. The trait is absent from `events`. Add it: `#[Window(events = ButtonEvents)]`. The same message appears for any other event trait written twice. |
| `the trait bound 'MyWin: Control' is not satisfied` (often naming `ButtonEvents`) | The trait is listed in `events`, so the macro skipped it, and `impl ButtonEvents for MyWin` is still missing. `Control` requires that trait, so the error is reported at `.window(...)`, where the window is registered. |
| `Unknown trait to allow overwriting: 'ButtonEvens'` | The name in `events` is not an event trait. A typo is the usual cause (`ButtonEvens` instead of `ButtonEvents`). The macro stops while expanding the attribute. |
| `Trait 'OnPaint' can not be used with the 'overwrite' attribute` | A trait that is not a control event (such as `OnPaint` or `OnKeyPressed`) was written in `events`. The panic text says `overwrite` even in that case. Paint and keyboard handling are already forwarded to `base`. `events` accepts control event traits (`ButtonEvents`, `WindowEvents`, and the other `*Events` traits). |
| `Trait 'DesktopEvents' can not be overwritten (for proc macro: Window)` | That event trait is implemented by the macro with an empty body and cannot be listed in `events`. `DesktopEvents` is the one in this group. |

`CommandBarEvents` and `MenuEvents` also expect a `commands` list (`#[Window(events = MenuEvents, commands = Save+Quit)]`). The macro builds a `Commands` enum from those names and passes it to the event methods. Expanding either trait without `commands` stops with a message that the `commands` attribute is required.

For a generic control, implement the typed trait the macro creates, such as `ListViewEvents<MyItem>`. The macro itself implements `GenericListViewEvents` and forwards to that typed trait. The same split is used for `TreeViewEvents<T>`, `DropDownListEvents<T>`, `SelectorEvents<T>`, and the other generic controls. Each stock-control page lists the methods of the typed trait.

### Creating a window without using `#[Window]` attribute

The attribute is a shorthand for code you can write yourself. A type passed to `App::window` or `App::single_window` implements `Control + WindowControl + NotModalWindow`. `Control` requires the input traits and every control event trait, so a hand-written window implements each of them. Empty impls are valid: the event traits provide default methods that ignore the event.

`#[repr(C)]` and a leading `base: Window` field are part of that contract. The runtime reads the window through the first field of the value.

```rs
use appcui::prelude::*;

#[repr(C)]
struct MyWin {
    base: Window, // first field
    x: i32,
}

impl std::ops::Deref for MyWin {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl std::ops::DerefMut for MyWin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Control for MyWin {}
impl WindowControl for MyWin {}
impl NotModalWindow for MyWin {}

impl OnPaint for MyWin {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        self.base.on_paint(surface, theme);
    }
}
impl OnKeyPressed for MyWin {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        self.base.on_key_pressed(key, character)
    }
}
impl OnMouseEvent for MyWin {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        self.base.on_mouse_event(event)
    }
}
impl OnResize for MyWin {
    fn on_resize(&mut self, old: Size, new: Size) {
        self.base.on_resize(old, new);
    }
}
impl OnFocus for MyWin {
    fn on_focus(&mut self) {
        self.base.on_focus();
    }
    fn on_lose_focus(&mut self) {
        self.base.on_lose_focus();
    }
}
impl OnDefaultAction for MyWin {
    fn on_default_action(&mut self) {
        self.base.on_default_action();
    }
}
impl OnWindowRegistered for MyWin {
    fn on_registered(&mut self) {
        self.base.on_registered();
    }
}

// These three have default methods. An empty impl keeps the defaults.
impl OnExpand for MyWin {}
impl OnSiblingSelected for MyWin {}
impl OnThemeChanged for MyWin {}

// The one event this window handles.
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        self.x += 1;
        EventProcessStatus::Processed
    }
}

// Every other event trait required by `Control`. One empty impl each:
impl CheckBoxEvents for MyWin {}
impl RadioBoxEvents for MyWin {}
impl ToggleButtonEvents for MyWin {}
impl PasswordEvents for MyWin {}
impl ThreeStateBoxEvents for MyWin {}
impl ColorPickerEvents for MyWin {}
impl KeySelectorEvents for MyWin {}
impl TextFieldEvents for MyWin {}
impl MarkdownComposerEvents for MyWin {}
impl RichTextFieldEvents for MyWin {}
impl GenericCommandBarEvents for MyWin {}
impl WindowEvents for MyWin {}
impl ToolBarEvents for MyWin {}
impl DesktopEvents for MyWin {}
impl GenericMenuEvents for MyWin {}
impl CustomEvents for MyWin {}
impl GenericSelectorEvents for MyWin {}
impl GenericDropDownListEvents for MyWin {}
impl ComboBoxEvents for MyWin {}
impl GenericNumericSelectorEvents for MyWin {}
impl DatePickerEvents for MyWin {}
impl ListBoxEvents for MyWin {}
impl GenericListViewEvents for MyWin {}
impl GenericBufferViewEvents for MyWin {}
impl PathFinderEvents for MyWin {}
impl TimerEvents for MyWin {}
impl GenericTreeViewEvents for MyWin {}
impl MarkdownEvents for MyWin {}
impl GenericBackgroundTaskEvents for MyWin {}
impl AccordionEvents for MyWin {}
impl TabEvents for MyWin {}
impl CharPickerEvents for MyWin {}
impl GenericGraphViewEvents for MyWin {}
impl AppBarEvents for MyWin {}
impl TimePickerEvents for MyWin {}
impl HyperLinkEvents for MyWin {}
impl GenericHSliderEvents for MyWin {}
impl GenericVBarChartEvents for MyWin {}
impl GenericHBarChartEvents for MyWin {}
impl PieChartEvents for MyWin {}
impl GenericLineChartEvents for MyWin {}
impl UsageChartEvents for MyWin {}
impl GenericHeatMapEvents for MyWin {}
```

Leaving `OnPaint` empty draws nothing: the title bar and the children are painted by `base`. Forward the input traits the same way the sample forwards `OnKeyPressed` and `OnMouseEvent`.

That is the whole attribute. It keeps `base` first, forwards window behavior to it, and fills in the event traits you are not handling. A name in `events` is the one impl reserved for your code.

When creating a window that supports the event loop in this manner, you will need to instantiate it. A common approach is the following:
```rs
#[Window(events=..., )]
struct MyWindow {
    // specific fields
}
impl MyWindow {
    fn new(/* extra parameters */) -> Self {
        let mut obj = MyWindow {
            base: Window::new(title, layout, flags),
            // initialize other fields from MyWindow here
        };
        // other initialization (such as creating children)
        return obj;
    }
}
```

The field `base: Window::new(title, layout, flags)` is the `Window` the attribute inserted; see [What the `#[Window]` attribute does](#what-the-window-attribute-does). As for the `title`, `layout`, and `flags`, you can provide them as parameters in the **new** method, or you can infer or hardcode them in another way. Read more about how a window can be created on the [Window](event-loop/window.md) page.

Once you create such a window, register it with the application using the `.window(...)` builder method. The factory can be a **closure** (when `new` takes arguments, or you build the window inline) or a **function** / constructor with no arguments (`MyWindow::new`):
```rs
fn main() -> Result<(), appcui::system::Error> {
    App::new().window(|| MyWindow::new(/* parameters */)).run()
    // or, if `new` takes no arguments:
    // App::new().window(MyWindow::new).run()
}
```

## A simple example

Let's start with a simple example that creates a window with a fixed size of `40x20` characters and two internal `i32` values.

```rs
use appcui::prelude::*;

#[Window()]
struct MyWindow {
    value1: i32,
    value2: i32
}
impl MyWindow {
    fn new(title: &str) -> Self {
        MyWindow {
            base: Window::new(title, layout!("a:c,w:40,h:20"), window::Flags::None),
            value1: 0,
            value2: 1,
        }
    }
}
fn main() -> Result<(), appcui::system::Error> {
    App::new().window(|| MyWindow::new("Some title")).run()
}
```

## Intercepting events from a child control

Usually, a window that processes events maintains handles to various controls and enables event processing in the `#[Window(...)]` declaration.

```rs
use appcui::prelude::*;

#[Window(events = /*Events specific to a control */)]
struct MyWindow {
    value1: i32,
    control: Handle</*control type*/>
}
impl MyWindow {
    fn new(/* parameters */) -> Self {
        let mut mywin = MyWindow {
            base: Window::new(/*...*/),
            control: Handle::None,
        };
        // now we create the control
        mywin.control =  mywin.add(/* Code that creates a control */);

        return mywin;
    }
}
impl /*Control event*/ for MyWindow {
    // add logic for event
}
fn main() -> Result<(), appcui::system::Error> {
    App::new().window(|| MyWindow::new(/* parameters */)).run()
}
```

For every control described in [Stock Controls](stock_controls.md), an example of how that control can be used with the event loop and the type of events it emits is presented.
