use appcui::prelude::*;
use std::fmt::Write;

mod child_control;
mod parent_control;
mod value;


use child_control::ChildControl;
use parent_control::ParentControl;
use value::Value;


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
    min_width: Handle<NumericSelector<u16>>,
    max_width: Handle<NumericSelector<u16>>,
    min_height: Handle<NumericSelector<u16>>,
    max_height: Handle<NumericSelector<u16>>,
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
            min_height: Handle::None,
            max_height: Handle::None,
            min_width: Handle::None,
            max_width: Handle::None,
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

        win.add(hline!("x:0,y:14,w:40"));
        win.add(label!("'Min width:',x:1,y:15,w:10"));
        win.min_width = win.add(numericselector!("u16,x:12,y:15,w:27,value:1, min:1, max:65535"));
        win.add(label!("'Max width:',x:1,y:17,w:10"));
        win.max_width = win.add(numericselector!("u16,x:12,y:17,w:27,value:65535, min:1, max:65535"));
        win.add(label!("'Min height:',x:1,y:19,w:11"));
        win.min_height = win.add(numericselector!("u16,x:12,y:19,w:27,value:1, min:1, max:65535"));
        win.add(label!("'Max height:',x:1,y:21,w:11"));
        win.max_height = win.add(numericselector!("u16,x:12,y:21,w:27,value:65535, min:1, max:65535"));

        // Create right panel for display
        let mut p = ParentControl::new(layout!("l:40,t:1,r:1,b:1"));
        win.child_control = p.add(ChildControl::new(layout!("x:5,y:5,w:20,h:10")));
        win.parent_control = win.add(p);

        win
    }

    fn set_error_message(&mut self, err: &str) {
        let h = self.parent_control;
        if let Some(p) = self.control_mut(h) {
            p.set_error_message(err);
        }
        let h = self.child_control;
        if let Some(c) = self.control_mut(h) {
            c.set_visible(false);
        }
    }
    fn set_child_layout(&mut self, layout: Layout) {
        let h = self.parent_control;
        if let Some(p) = self.control_mut(h) {
            p.clear_error();
        }
        let min_w = self.control(self.min_width).unwrap().value();
        let max_w = self.control(self.max_width).unwrap().value();
        let min_h = self.control(self.min_height).unwrap().value();
        let max_h = self.control(self.max_height).unwrap().value();
        let h = self.child_control;
        if let Some(c) = self.control_mut(h) {
            c.set_visible(true);
            c.update_layout(layout);
            c.set_size_bounds(min_w,min_h, max_w, max_h);
        }
    }


    fn update_child_layout(&mut self) {
        let mut layout_builder = LayoutBuilder::new();
        let mut error_msg = String::new();

        macro_rules! parse_value {
            ($field:expr, $field_name:literal, $should_be_positive:expr, $method:ident) => {
                match Value::new(self.control($field).unwrap().text(), $field_name, $should_be_positive) {
                    Value::None => {}
                    Value::Percent(percent) => {
                        layout_builder = layout_builder.$method(percent);
                    }
                    Value::Integer(integer) => {
                        layout_builder = layout_builder.$method(integer);
                    }
                    Value::Error(err) => {
                        write!(&mut error_msg, "{}", err).unwrap();
                    }
                }
            };
        }


        parse_value!(self.x_field, "X", false, x);
        parse_value!(self.y_field, "Y", false, y);
        parse_value!(self.width_field, "Width", true, width);
        parse_value!(self.height_field, "Height", true, height);
        parse_value!(self.left_anchor_field, "Left Anchor", false, left_anchor);
        parse_value!(self.right_anchor_field, "Right Anchor", false, right_anchor);
        parse_value!(self.top_anchor_field, "Top Anchor", false, top_anchor);
        parse_value!(self.bottom_anchor_field, "Bottom Anchor", false, bottom_anchor);

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

        if !error_msg.is_empty() {
            self.set_error_message(&error_msg);
        } else {
            match layout_builder.try_build() {
                Ok(l) => self.set_child_layout(l),
                Err(e) => {
                    let _ = write!(&mut error_msg, "{e}");
                    self.set_error_message(&error_msg);
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
