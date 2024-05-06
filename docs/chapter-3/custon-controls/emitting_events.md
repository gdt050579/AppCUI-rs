# Emitting custom events

There are scenarios where a custom control needs to emit some events that will be used in the event loop window. For this the following steps need to be performed:

1. Create a list of events in the custom control using `emit` attribute. For example:
    ```rs
    #[CustomControl(emit:Event1+Event2+Event3)]
    pub struct MyCustomControl {
        // data members
    }
    ```

    Make sure that the custom control is defined with appropriate visibility as you will need to use it in another structure (e.g. where you define the event loop via `#[Window(...)]` or `#[ModalWindow(...)]`).

2. When you create an event loop via `#[Window(...)]` add the following attribute `custom_events` that has as value a list of all custom controls from where you want to intercept custom events. For example:
    ```rs
    #[Window(custom_events: CustomControl1Events+CustomControl2Events+...)]
    struct MyWin {
        // data members
    }
    ```
    **Remarks**: The name of the custom event **MUST** be in the following format: *CustomControlName* followed by *Events*.

3. Implement the custom event trait in the following way:
    ```rs
    impl MyCustomControlEvents for MyWin {
        fn on_event(&mut self, handle: Handle<CustomControl>, 
                               event: customcontrl::Events) -> EventProcessStatus 
        {
            // add your code here
        }
    }
    ```
    **Remarks**: Returning `EventProcessStatus::Processed` will repaint the entire custom control.

4. Make sure that you import the custom control in you window file. The `#[CustomControl(...)]` creates an internal module with the same name (but with lowercase) as the custom control where an enum (named `Events`) will store all defined custom events. As such, the file where you define the window (event loop) should have the following:
    ```rs
    use <path to custom control>::MyCustomControl;
    use <path to custom control>::mycustomcontrol::*;
    ```

## Example

Let's consider the following example:
* we want to create a custom control for a Chess game. That control will display all pieces, will allow movement and will notify the main window when the game is over.
* we will also need a main window from where we can start the game, make configuration changes, etc.

Let's start by designing the Chess custom control. We will create a separate file (`chess.rs`) where we will define the control in the following way:

```rs
use appcui::prelude::*;

#[CustomControl(overwrite: OnPaint+OnKeyPressed+OnMouseEvent,
                emit     : DrawGame+Player1Wins+Player2Wins)]
pub struct Chess {
    // private data
}
impl OnPaint for Chess { ... }
impl OnMouseEvent for Chess { ... }
impl OnKeyPressed for Chess { ...}
```

This code will also create an inner module that contains an `Events` enum with the following variants:

```rs
pub mod chess {
    #[repr(u32)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Events {
        DrawGame,
        Player1Wins,
        Player2Wins
    }
}
```
Notice that we define the structure `Chess` as public. The visibility attribute is copied into the inner module and as such, the module `chess` is public as well.

Also, a method `raise_event(event: chess::Events)` was added to the `Chess` struct.

Now lets see how we can define the event loop component (the window where the custom control will be added):

```rs
use appcui::prelude::*;
use <path_to_cheese_file>::Chess;
use <path_to_cheese_file>::chess::*;

#[Window(custom_event: ChessEvents)]
struct MyWin {
    // data members
    table: Handle<Chess>
}

impl ChessEvents for MyWin {
        fn on_event(&mut self, handle: Handle<Chess>, 
                               event: board::Events) -> EventProcessStatus 
        {
            match event {
                chess::Events::DrawGame    => { /* in case of a draw game */ }
                chess::Events::Player1Wins => { /* Player one has won     */ }
                chess::Events::Player2Wins => { /* Player two has won     */ }
            }
        }
    }
}