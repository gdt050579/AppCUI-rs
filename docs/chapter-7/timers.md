# Timers

Timers are internal AppCUI mechanisms that allow each control to receive a signal at a specified interval. This signal can be used to update the control's content, to animate it, or to perform other actions. 

Each timer has its own thread that sends a signal to the control at a specified interval. Because of this, the total number of timers that can be used in an application is limited. By default, an application can use up to 4 timers. This number can be increased by using the `.timers_count(count)` method from the `App` builder, but it can not be more than 255. 

When a control is destroyes, if it has a timer associated with it, that timer will also be closed and the slot associated with it released.

To use a timer, you will need the following things:
1. access the control timer via the `.timer()` method
2. implement `TimerEvents` trait for the control to get notification when the timer signal is received

The timer events is define as follows:
```rust
pub trait TimerEvents {
    fn on_start(&mut self) -> EventProcessStatus {
        // called when the timer is started
        EventProcessStatus::Ignored
    }
    fn on_resume(&mut self, ticks: u64) -> EventProcessStatus {
        // called when the timer is resumed
        EventProcessStatus::Ignored
    }
    fn on_pause(&mut self, ticks: u64) -> EventProcessStatus {

        EventProcessStatus::Ignored
    }    
    fn on_update(&mut self, ticks: u64) -> EventProcessStatus {
        // called when the timer updates itself
        // (e.g. the interval of the timer has passed)
        EventProcessStatus::Ignored
    }
}
```

The `ticks` variable represents the number of times the timer has been triggered. Whenever the timer starts this variable is set to 0, and them it is incremented each time the timer is triggered.

**Remarks**: It is important to note that if you must return `EventProcessStatus::Process` from these methods if you want AppCUI framework to redraw itself (this is usually the case if you are updating a control context in the timer event).

## Timer object

The timer object is created by calling the `.timer()` method from the control. This method returns an `Option<&mut Timer>` object that can be used to start, stop, pause, or resume the timer. If a slot (from the list of maximum number of timers) is present, the method will return a `Some(&mut Timer)` object. If no slot is available, the method will return `None`. Once a slot is available, the timer will be created (e.g. the slot will be occupied) but no threads will be created until the `.start()` method is called.

The following methods are available for a timer:

| Method              | Description                                                                                                                                                                                                                                   |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `start(...)`        | Starts the timer with the specified interval and reset the internal **tick count** to 0. If this is the first time that timer is started, a thread will also be created (otherwise the existing thread associated with the timer will be used |
| `pause()`           | Pauses the timer. The timer thread will also be paused (but not terminated)                                                                                                                                                                   |
| `resume()`          | Resumes the timer after it was paused                                                                                                                                                                                                         |
| `stop()`            | Stops the timer. The timer thread will be terminated and the slot will be freed (meaning that other object can use that slot for its own timer)                                                                                               |
| `set_interval(...)` | Sets the interval for the timer. If the timer is already started, the new interval is applied imediately. Otherwise, the new interval will be use the moment that timer is being started or resumed                                           |
| `is_paused()`       | Returns `true` if the timer is paused, `false` otherwise                                                                                                                                                                                      |
| `is_running()`      | Returns `true` if the timer is started, `false` otherwise                                                                                                                                                                                     |

Typical a timer is being used like this:
```rust
// assuming that we run in a control context (e.g. self refers to a control)
if let Some(timer) = self.timer() {
    timer.start(Duration::with_seconds(1)); // start the timer with 1 second interval
}
```

## Example

The following example starts a 1 second timer that updates a label control with the current time:

```rust
use std::time::Duration;
use appcui::prelude::*;

#[Window(events = TimerEvents)]
struct MyWin {
    lb: Handle<Label>,
}
impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("'Timer Example', d:c, w:30, h:5"),
            lb: Handle::None,
        };
        w.lb = w.add(label!("'',x:1,y:1,w:28"));
        if let Some(timer) = w.timer() {
            timer.start(Duration::from_secs(1));
        }
        w
    }
}
impl TimerEvents for MyWin {
    fn on_update(&mut self, ticks: u64) -> EventProcessStatus {
        let text = format!("Ticks: {}", ticks);
        let h = self.lb;
        if let Some(lb) = self.control_mut(h) {
            lb.set_caption(&text);
        }
        // return EventProcessStatus::Process to repaint controls
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())
}
```