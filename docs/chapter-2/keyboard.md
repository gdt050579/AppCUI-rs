# Keyboard

Keyboard events are received through the trait `OnKeyPressed` defined as follows:

```rs
pub trait OnKeyPressed {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        // do something depending on the key pressed
    }
}
```

This method has two parameters:
1. the `key` parameter (that provides information about the code of the key that was pressed and its modifiers)
2. the `character` (when this is the case). This is usually when you want insert text intro a control (for example in case of a [TextField](../chapter-3/stock-controls/textfield.md))


## Key

A key in AppCUI is defined as follows:

```rs
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Key {
    pub code: KeyCode,
    pub modifier: KeyModifier,
}
```

where:
* `code` is an enum that indicates a code for the key that was pressed and it includes:
  * F-commands (`F1` to `F12`)
  * Letters (`A` to `Z`) - with apper case
  * Numbers (`0` to `9`)
  * Arrows (`Up`, `Down`, `Left`, `Right`)
  * Navigation keys (`PageUp`, `PageDown`, `Home`, `End`)
  * Deletion and Insertions (`Delete` , `Backspace`, `Insert`)
  * White-spaces (`Space`, `Tab`)
  * Other (`Enter`, `Escape`)
* `modifier` can be one of the following (including combination between them):
  * Shift
  * Ctrl
  * Alt


The crete a key use:
1. `Key::new(code, modifier)`  - for example:
    ```rs
    let k = Key::new(KeyCode::F1,KeyModifier::Alt | KeyModifier::Ctrl);
    let k2 = Key::new(KeyCode::Enter, KeyModifier::None);
    ```
2. using `From` implementation:
   ```rs
   let k = Key::from(KeyCode::F1);
   // this is equivalent to Key::new(KeyCode::F1, KeyModifier::None);
   ```
3. `key!` macro - this can be used to create a key:
    ```rs
    let k1 = key!("F2");
    let k2 = key!("Enter")
    let k3 = key!("Alt+F4")
    let k4 = key!("Ctrl+Alt+F")
    let k5 = key!("Ctrl+Shift+Alt+Tab")
    ```

## Usage

The usual usage is via `OnKeyPressed::on_key_pressed(...)` method as follows:

```rs
impl OnKeyPressed for <MyControl> {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        // check the key
        match key.value() {
            // check various key combinations
        }
        // check the character
        match character {
            // do something with the character
        }
        
        EventProcessStatus::Ignored
    }
}
```

The following example checks the arrow keys for movement and ignores the rest of the keys:

```rs
impl OnKeyPressed for <MyControl> {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        match key.value() {
            key!("Left") => { 
                /* move left */ 
                return EventProcessStatus::Processed;
            }
            key!("Right") => { 
                /* move right */ 
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Left") => { 
                /* Move to begining */ 
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Right") => { 
                /* Move to end */ 
                return EventProcessStatus::Processed;
            }
            _ => {
                return EventProcessStatus::Ignored;
            }
        }
    }
}
```