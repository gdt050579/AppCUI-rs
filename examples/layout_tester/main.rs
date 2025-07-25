use appcui::prelude::*;
use std::fmt::Write;

mod child_control;
mod parent_control;

use child_control::ChildControl;
use parent_control::ParentControl;

#[Window(events = TextFieldEvents + SelectorEvents<Alignment> + SelectorEvents<Dock> + SelectorEvents<Pivot>)]
struct LayoutTesterWindow {
    // Layout parameter controls
    x_field: Handle<TextField>,
    y_field: Handle<TextField>,
    width_field: Handle<TextField>,
    height_field: Handle<TextField>,
    left_anchor_field: Handle<TextField>,
    right_anchor_field: Handle<TextField>,
    top_anchor_field: Handle<TextField>,
    bottom_anchor_field: Handle<TextField>,
    align_selector: Handle<Selector<Alignment>>,
    dock_selector: Handle<Selector<Dock>>,
    pivot_selector: Handle<Selector<Pivot>>,

    // Display controls
    parent_control: Handle<ParentControl>,
    child_control: Handle<ChildControl>,
}

impl LayoutTesterWindow {
    fn new() -> Self {
        let mut win = Self {
            base: window!("title:'Layout Tester',d:f"),
            x_field: Handle::None,
            y_field: Handle::None,
            width_field: Handle::None,
            height_field: Handle::None,
            left_anchor_field: Handle::None,
            right_anchor_field: Handle::None,
            top_anchor_field: Handle::None,
            bottom_anchor_field: Handle::None,
            align_selector: Handle::None,
            dock_selector: Handle::None,
            pivot_selector: Handle::None,
            parent_control: Handle::None,
            child_control: Handle::None,
        };

        // Position parameters
        win.add(label!("'X:',x:1,y:1,w:8,h:1"));
        win.x_field = win.add(textfield!("'5',x:9,y:1,w:10,h:1"));

        win.add(label!("'Y:',x:21,y:1,w:8,h:1"));
        win.y_field = win.add(textfield!("'5',x:29,y:1,w:10,h:1"));

        win.add(label!("'Width:',x:1,y:3,w:8,h:1"));
        win.width_field = win.add(textfield!("'20',x:9,y:3,w:10,h:1"));

        win.add(label!("'Height:',x:21,y:3,w:8,h:1"));
        win.height_field = win.add(textfield!("'10',x:29,y:3,w:10,h:1"));

        // Anchor parameters
        win.add(label!("'Left:',x:1,y:5,w:8,h:1"));
        win.left_anchor_field = win.add(textfield!("'',x:9,y:5,w:10,h:1"));

        win.add(label!("'Right:',x:21,y:5,w:8,h:1"));
        win.right_anchor_field = win.add(textfield!("'',x:29,y:5,w:10,h:1"));

        win.add(label!("'Top:',x:1,y:7,w:8,h:1"));
        win.top_anchor_field = win.add(textfield!("'',x:9,y:7,w:10,h:1"));

        win.add(label!("'Bottom:',x:21,y:7,w:8,h:1"));
        win.bottom_anchor_field = win.add(textfield!("'',x:29,y:7,w:10,h:1"));

        // Enum selectors
        win.add(label!("'Alignment:',x:1,y:9,w:10,h:1"));
        win.align_selector = win.add(selector!("enum: Alignment,x:12,y:9,w:27,flags:AllowNoneVariant"));

        win.add(label!("'Dock:',x:1,y:11,w:10,h:1"));
        win.dock_selector = win.add(selector!("enum: Dock,x:12,y:11,w:27,flags:AllowNoneVariant"));

        win.add(label!("'Pivot:',x:1,y:13,w:10,h:1"));
        win.pivot_selector = win.add(selector!("enum: Pivot,x:12,y:13,w:27,flags:AllowNoneVariant"));

        // Create right panel for display
        win.add(label!("'Layout Preview',x:40,y:1,w:35,h:1"));
        let mut p = ParentControl::new(layout!("x:40,y:3,w:55,h:25"));
        win.child_control = p.add(ChildControl::new(layout!("x:5,y:5,w:20,h:10")));
        win.parent_control = win.add(p);

        win
    }

    fn update_child_layout(&mut self) {
        let mut layout_builder = LayoutBuilder::new();
        let mut has_error = false;
        let mut error_msg = String::new();

        // Parse position values
        if let Some(tf) = self.control(self.x_field) {
            let text = tf.text().trim();
            if !text.is_empty() {
                match text.parse::<i32>() {
                    Ok(value) => {
                        layout_builder = layout_builder.x(value);
                    }
                    Err(_) => {
                        has_error = true;
                        write!(&mut error_msg, "Invalid X value: {}. ", text).unwrap();
                    }
                }
            }
        }

        if let Some(tf) = self.control(self.y_field) {
            let text = tf.text().trim();
            if !text.is_empty() {
                match text.parse::<i32>() {
                    Ok(value) => {
                        layout_builder = layout_builder.y(value);
                    }
                    Err(_) => {
                        has_error = true;
                        write!(&mut error_msg, "Invalid Y value: {}. ", text).unwrap();
                    }
                }
            }
        }

        if let Some(tf) = self.control(self.width_field) {
            let text = tf.text().trim();
            if !text.is_empty() {
                match text.parse::<u32>() {
                    Ok(value) => {
                        layout_builder = layout_builder.width(value);
                    }
                    Err(_) => {
                        has_error = true;
                        write!(&mut error_msg, "Invalid Width value: {}. ", text).unwrap();
                    }
                }
            }
        }

        if let Some(tf) = self.control(self.height_field) {
            let text = tf.text().trim();
            if !text.is_empty() {
                match text.parse::<u32>() {
                    Ok(value) => {
                        layout_builder = layout_builder.height(value);
                    }
                    Err(_) => {
                        has_error = true;
                        write!(&mut error_msg, "Invalid Height value: {}. ", text).unwrap();
                    }
                }
            }
        }

        // Parse anchor values
        if let Some(tf) = self.control(self.left_anchor_field) {
            let text = tf.text().trim();
            if !text.is_empty() {
                match text.parse::<i32>() {
                    Ok(value) => {
                        layout_builder = layout_builder.left_anchor(value);
                    }
                    Err(_) => {
                        has_error = true;
                        write!(&mut error_msg, "Invalid Left Anchor value: {}. ", text).unwrap();
                    }
                }
            }
        }

        if let Some(tf) = self.control(self.right_anchor_field) {
            let text = tf.text().trim();
            if !text.is_empty() {
                match text.parse::<i32>() {
                    Ok(value) => {
                        layout_builder = layout_builder.right_anchor(value);
                    }
                    Err(_) => {
                        has_error = true;
                        write!(&mut error_msg, "Invalid Right Anchor value: {}. ", text).unwrap();
                    }
                }
            }
        }

        if let Some(tf) = self.control(self.top_anchor_field) {
            let text = tf.text().trim();
            if !text.is_empty() {
                match text.parse::<i32>() {
                    Ok(value) => {
                        layout_builder = layout_builder.top_anchor(value);
                    }
                    Err(_) => {
                        has_error = true;
                        write!(&mut error_msg, "Invalid Top Anchor value: {}. ", text).unwrap();
                    }
                }
            }
        }

        if let Some(tf) = self.control(self.bottom_anchor_field) {
            let text = tf.text().trim();
            if !text.is_empty() {
                match text.parse::<i32>() {
                    Ok(value) => {
                        layout_builder = layout_builder.bottom_anchor(value);
                    }
                    Err(_) => {
                        has_error = true;
                        write!(&mut error_msg, "Invalid Bottom Anchor value: {}. ", text).unwrap();
                    }
                }
            }
        }

        // Handle enum selectors
        if let Some(selector) = self.control(self.align_selector) {
            if let Some(align) = selector.try_value() {
                layout_builder = layout_builder.alignment(align);
            }
        }

        if let Some(selector) = self.control(self.dock_selector) {
            if let Some(dock) = selector.try_value() {
                layout_builder = layout_builder.dock(dock);
            }
        }

        if let Some(selector) = self.control(self.pivot_selector) {
            if let Some(pivot) = selector.try_value() {
                layout_builder = layout_builder.pivot(pivot);
            }
        }

        // Update parent control with error state
        let parent_handle = self.parent_control;
        let child_handle = self.child_control;

        if has_error {
            if let Some(parent) = self.control_mut(parent_handle) {
                parent.set_error_message(error_msg);
                parent.hide_child();
            }
        } else {
            match layout_builder.try_build() {
                Ok(l) => {
                    if let Some(parent) = self.control_mut(parent_handle) {
                        parent.clear_error();
                        parent.show_child();
                    }
                    if let Some(child) = self.control_mut(child_handle) {
                        child.update_layout(l);
                    }
                },
                Err(e) => {
                    let _ = write!(&mut error_msg, "{e}");
                    if let Some(parent) = self.control_mut(parent_handle) {
                        parent.set_error_message(error_msg);
                        parent.hide_child();
                    }                    
                }
            }

        }
    }
}

impl TextFieldEvents for LayoutTesterWindow {
    fn on_text_changed(&mut self, _handle: Handle<TextField>) -> EventProcessStatus {
        self.update_child_layout();
        EventProcessStatus::Processed
    }
}

impl SelectorEvents<Alignment> for LayoutTesterWindow {
    fn on_selection_changed(&mut self, _: Handle<Selector<Alignment>>, _value: Option<Alignment>) -> EventProcessStatus {
        self.update_child_layout();
        EventProcessStatus::Processed
    }
}

impl SelectorEvents<Dock> for LayoutTesterWindow {
    fn on_selection_changed(&mut self, _: Handle<Selector<Dock>>, _value: Option<Dock>) -> EventProcessStatus {
        self.update_child_layout();
        EventProcessStatus::Processed
    }
}

impl SelectorEvents<Pivot> for LayoutTesterWindow {
    fn on_selection_changed(&mut self, _: Handle<Selector<Pivot>>, _value: Option<Pivot>) -> EventProcessStatus {
        self.update_child_layout();
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().size(Size::new(100, 30)).single_window().build()?;
    app.add_window(LayoutTesterWindow::new());
    app.run();
    Ok(())
}
