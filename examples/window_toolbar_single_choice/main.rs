use appcui::prelude::*;

#[Window(events = ToolBarEvents)]
struct SingleChoiceWindow {
    option_one: Handle<toolbar::SingleChoice>,
    option_two: Handle<toolbar::SingleChoice>,
    status_label: Handle<Label>,
}

impl SingleChoiceWindow {
    fn new() -> Self {
        let mut win = SingleChoiceWindow {
            base: window!("'Single Choice Demo',d:c,w:40,h:6"),
            option_one: Handle::None,
            option_two: Handle::None,
            status_label: Handle::None,
        };

        // Create a toolbar group at the bottom left of the window
        let group = win.toolbar().create_group(toolbar::GroupPosition::BottomLeft);

        // Add single choice items to the toolbar group
        let mut opt1 = toolbar::SingleChoice::new("Option 1");
        opt1.set_tooltip("First option");
        win.option_one = win.toolbar().add(group, opt1);

        let mut opt2 = toolbar::SingleChoice::new("Option 2");
        opt2.set_tooltip("Second option");
        win.option_two = win.toolbar().add(group, opt2);

        // Select the first option by default
        let h = win.option_one;
        if let Some(choice) = win.toolbar().get_mut(h) {
            choice.select();
        }

        // Add a label to display the selection state
        win.status_label = win.add(label!("'Option 1 is selected',d:c,w:30,h:1"));

        win
    }
}

impl ToolBarEvents for SingleChoiceWindow {
    fn on_choice_selected(&mut self, handle: Handle<toolbar::SingleChoice>) -> EventProcessStatus {
        let message = if handle == self.option_one {
            "Option 1 is selected"
        } else if handle == self.option_two {
            "Option 2 is selected"
        } else {
            return EventProcessStatus::Ignored;
        };

        let h = self.status_label;
        if let Some(label) = self.control_mut(h) {
            label.set_caption(message);
        }
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(SingleChoiceWindow::new());
    app.run();
    Ok(())
}
