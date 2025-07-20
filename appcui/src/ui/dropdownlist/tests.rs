use crate::prelude::*;

#[derive(DropDownListType)]
enum MathOp {
    #[VariantInfo(description = "(Add multiple numbers)", symbol = "∑")]
    Sum,

    #[VariantInfo(description = "(Multiply multiple numbers)", symbol = "∏")]
    Product,

    #[VariantInfo(description = "(Calculate the integral of a function)", symbol = "∫")]
    Integral,

    #[VariantInfo(description = "(Calculate the radical of a number)", symbol = "√")]
    Radical,

    #[VariantInfo(description = "(Check if all elements from a set are different)", symbol = "≠")]
    Different,
}

#[test]
fn check_symbol_paint() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x2A53E5A0DF2E4049) 
        Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0x40CE2546F1948D43) 
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('Sum selected')   
        CheckHash(0xFE59E5136C4A00BD) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:56,h:7");
    let mut db = DropDownList::<MathOp>::with_symbol(1, layout!("x:1,y:1,w:50"), dropdownlist::Flags::ShowDescription);
    db.add(MathOp::Sum);
    db.add(MathOp::Product);
    db.add(MathOp::Integral);
    db.add(MathOp::Radical);
    db.add(MathOp::Different);
    w.add(db);
    a.add_window(w);
    a.run();
}

#[test]
fn check_create_with_macro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x2A53E5A0DF2E4049) 
        Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0x40CE2546F1948D43) 
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('Sum selected')   
        CheckHash(0xFE59E5136C4A00BD) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:56,h:7");
    let mut db = dropdownlist!("class:MathOp,x:1,y:1,w:50,flags:ShowDescription,symbolsize:1");
    db.add(MathOp::Sum);
    db.add(MathOp::Product);
    db.add(MathOp::Integral);
    db.add(MathOp::Radical);
    db.add(MathOp::Different);
    w.add(db);
    a.add_window(w);
    a.run();
}

#[test]
fn check_no_symbols() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x2A53E5A0DF2E4049) 
        Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0x1CFE19F59CB0D41A) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:56,h:7");
    let mut db = dropdownlist!("class:MathOp,x:1,y:1,w:50,flags:ShowDescription");
    db.add(MathOp::Sum);
    db.add(MathOp::Product);
    db.add(MathOp::Integral);
    db.add(MathOp::Radical);
    db.add(MathOp::Different);
    w.add(db);
    a.add_window(w);
    a.run();
}

#[test]
fn check_allow_none() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (should be None)')   
        CheckHash(0xC716BFA675C55B4B) 
        Key.Pressed(Space)
        Paint('Opened - None should be selected')   
        CheckHash(0xAA1D368EE131B77B) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:56,h:7");
    let mut db = dropdownlist!("class:MathOp,x:1,y:1,w:50,flags:AllowNoneSelection");
    db.add(MathOp::Sum);
    db.add(MathOp::Product);
    db.add(MathOp::Integral);
    w.add(db);
    a.add_window(w);
    a.run();
}

#[test]
fn check_none_with_different_name() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state (should be Select Something)')   
        CheckHash(0x6A430F7BD6933C21) 
        Key.Pressed(Space)
        Paint('Opened - Select Something should be selected')   
        CheckHash(0xF30519060AA60813) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    let mut w = window!("Title,x:0,y:0,w:56,h:7");
    let mut db = dropdownlist!("class:MathOp,x:1,y:1,w:50,flags:AllowNoneSelection,none:'Select something'");
    db.add(MathOp::Sum);
    db.add(MathOp::Product);
    db.add(MathOp::Integral);
    w.add(db);
    a.add_window(w);
    a.run();
}

#[test]
fn check_events() {
    #[Window(events=DropDownListEvents<MathOp>,internal: true)]
    struct MyWindow {}
    impl MyWindow {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title,x:0,y:0,w:56,h:7"),
            };
            let mut db = DropDownList::<MathOp>::with_symbol(1, layout!("x:1,y:1,w:50"), dropdownlist::Flags::ShowDescription);
            db.add(MathOp::Sum);
            db.add(MathOp::Product);
            db.add(MathOp::Integral);
            db.add(MathOp::Radical);
            db.add(MathOp::Different);
            w.add(db);
            w
        }
    }
    impl DropDownListEvents<MathOp> for MyWindow {
        fn on_selection_changed(&mut self, handle: Handle<DropDownList<MathOp>>) -> EventProcessStatus {
            let s = self
                .control(handle)
                .and_then(|p| p.selected_item())
                .map(|p| p.name())
                .unwrap_or("Invalid")
                .to_string();
            self.set_title(&s);
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        Paint('Initial state (closed)')   
        CheckHash(0x2A53E5A0DF2E4049) 
        Key.Pressed(Space)
        Paint('Opened')   
        CheckHash(0x40CE2546F1948D43) 
        Key.Pressed(Down)
        Paint('Sum selected (on title window)')   
        CheckHash(0x277171A9375683CA) 
        Key.Pressed(Down)
        Paint('Product selected (on title window)')   
        CheckHash(0x7035C003A73ED604) 
    ";
    let mut a = App::debug(60, 12, script).build().unwrap();
    a.add_window(MyWindow::new());
    a.run();
}

#[test]
fn check_selected_item_and_index_methods() {
    struct TestItem {
        name: String,
        description: String,
        modified: bool,
    }

    impl TestItem {
        fn new(name: &str, description: &str) -> Self {
            Self {
                name: name.to_string(),
                description: description.to_string(),
                modified: false,
            }
        }

        fn set_modified(&mut self, modified: bool) {
            self.modified = modified;
        }

        fn is_modified(&self) -> bool {
            self.modified
        }
    }

    impl DropDownListType for TestItem {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            &self.description
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state - no selection')   
        CheckHash(0xAC537C8AB81C7B1A)
        Key.Pressed(Down)
        Paint('First item selected')   
        CheckHash(0xB70E672AA50C7A89)
        Key.Pressed(F1)
        Paint('First item modified')   
        CheckHash(0x2DBD0CE963E6B2A)
        Key.Pressed(Space)
        Key.Pressed(Down,2)
        Key.Pressed(Enter)
        Paint('Third item selected')   
        CheckHash(0xEA821AAC0F94116A)
        Key.Pressed(F2)
        Paint('Set back to first item')   
        CheckHash(0x2DBD0CE963E6B2A)
        Key.Pressed(F3)
        Paint('No items, no selection')   
        CheckHash(0xAC537C8AB81C7B1A)
    ";

    #[Window(events=CommandBarEvents+DropDownListEvents<TestItem>,commands:A+B+C, internal:true)]
    struct MyWin {
        dropdownlist_handle: Handle<DropDownList<TestItem>>,
        info_handle: Handle<Label>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,x:1,y:1,w:78,h:12"),
                dropdownlist_handle: Handle::None,
                info_handle: Handle::None,
            };

            let mut ddl = DropDownList::<TestItem>::new(layout!("x:1,y:1,w:60"), dropdownlist::Flags::ShowDescription);

            // Add test items
            ddl.add(TestItem::new("Item One", "First test item"));
            ddl.add(TestItem::new("Item Two", "Second test item"));
            ddl.add(TestItem::new("Item Three", "Third test item"));
            ddl.add(TestItem::new("Item Four", "Fourth test item"));

            // Create a label to display information about selected item and index
            let l = Label::new("", layout!("x:1,y:3,w:70,h:7"));

            w.dropdownlist_handle = w.add(ddl);
            w.info_handle = w.add(l);

            // Update the info label with initial state
            w.update_info_label();

            w
        }

        fn update_info_label(&mut self) {
            let h = self.dropdownlist_handle;
            let info_text = if let Some(ddl) = self.control(h) {
                // Get index information
                let index_info = match ddl.index() {
                    Some(idx) => format!("index(): Some({idx})"),
                    None => "index(): None".to_string(),
                };

                // Get selected item information
                let selected_item_info = match ddl.selected_item() {
                    Some(item) => {
                        format!(
                            "selected_item(): Some(\"{}\")\nDescription: \"{}\"\nModified: {}",
                            item.name(),
                            item.description(),
                            item.is_modified()
                        )
                    }
                    None => "selected_item(): None".to_string(),
                };

                format!(
                    "{index_info}\n{selected_item_info}\n\nselected_item_mut() can be used to modify the selected item"
                )
            } else {
                "Error: DropDownList not found".to_string()
            };

            // Update the label
            let h = self.info_handle;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(&info_text);
            }
        }

        // Modify the selected item using selected_item_mut method
        fn modify_selected_item(&mut self) {
            let h = self.dropdownlist_handle;
            if let Some(ddl) = self.control_mut(h) {
                if let Some(item) = ddl.selected_item_mut() {
                    // Modify the item
                    item.set_modified(true);

                    // Append "MODIFIED" to the name
                    if !item.name.contains("MODIFIED") {
                        item.name = format!("{} (MODIFIED)", item.name);
                    }
                }
            }
            self.update_info_label();
        }

        // Set the index to 0 (first item) using set_index method
        fn set_index_to_first(&mut self) {
            let h = self.dropdownlist_handle;
            if let Some(ddl) = self.control_mut(h) {
                ddl.set_index(0);
            }
            self.update_info_label();
        }

        // Clear all items from the dropdown list
        fn clear_items(&mut self) {
            let h = self.dropdownlist_handle;
            if let Some(ddl) = self.control_mut(h) {
                ddl.clear();
            }
            self.update_info_label();
        }
    }

    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Modify Item", mywin::Commands::A);
            commandbar.set(key!("F2"), "Set to First", mywin::Commands::B);
            commandbar.set(key!("F3"), "Clear Items", mywin::Commands::C);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            match command_id {
                mywin::Commands::A => self.modify_selected_item(),
                mywin::Commands::B => self.set_index_to_first(),
                mywin::Commands::C => self.clear_items(),
            }
        }
    }

    impl DropDownListEvents<TestItem> for MyWin {
        fn on_selection_changed(&mut self, _handle: Handle<DropDownList<TestItem>>) -> EventProcessStatus {
            self.update_info_label();
            EventProcessStatus::Processed
        }
    }

    let mut a = App::debug(80, 16, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_has_selection_and_count_with_mouse() {
    #[derive(Clone)]
    struct CountItem {
        id: u32,
        name: String,
    }

    impl CountItem {
        fn new(id: u32, name: &str) -> Self {
            Self { id, name: name.to_string() }
        }
    }

    impl DropDownListType for CountItem {
        fn name(&self) -> &str {
            &self.name
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state - no selection')   
        CheckHash(0xE5D46AEA90B3EC9F)
        Mouse.Click(30,3,left)
        Paint('Dropdown expanded')   
        CheckHash(0xACCC1F715B85846D)
        Mouse.Click(10,5,left)
        Paint('Item 1 selected')   
        CheckHash(0x7E0444B15FFEC84E)
        Key.Pressed(F1)
        Paint('Added new item (Item One remains selected)')   
        CheckHash(0xCBC37D4886B91F11)
        Key.Pressed(F2)
        Paint('Removed one item (two remains - no selection)')   
        CheckHash(0xEE6F6E2D1DA04DA)
        Mouse.Click(30,3,left)
        Paint('Dropdown expanded with fewer items')   
        CheckHash(0xEB2C3C4A89F5E259)
        Mouse.Click(10,6,left)
        Paint('Selected the last item (Item Three)')   
        CheckHash(0xCA37DC16E4F0F96D)
        Key.Pressed(F3)
        Paint('Cleared all items - no selection')   
        CheckHash(0x72980CD7C579ADDC)
    ";

    #[Window(events=CommandBarEvents+DropDownListEvents<CountItem>,commands:A+B+C, internal:true)]
    struct MyWin {
        dropdownlist_handle: Handle<DropDownList<CountItem>>,
        info_handle: Handle<Label>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,x:1,y:1,w:78,h:12"),
                dropdownlist_handle: Handle::None,
                info_handle: Handle::None,
            };

            let mut ddl = DropDownList::<CountItem>::new(layout!("x:1,y:1,w:60"), dropdownlist::Flags::None);

            // Add initial items
            ddl.add(CountItem::new(1, "Item One"));
            ddl.add(CountItem::new(2, "Item Two"));
            ddl.add(CountItem::new(3, "Item Three"));

            // Create a label to display information about has_selection and count
            let l = Label::new("", layout!("x:1,y:3,w:70,h:7"));

            w.dropdownlist_handle = w.add(ddl);
            w.info_handle = w.add(l);

            // Update the info label with initial state
            w.update_info_label();

            w
        }

        fn update_info_label(&mut self) {
            let h = self.dropdownlist_handle;
            let info_text = if let Some(ddl) = self.control(h) {
                // Get selection information
                let has_selection = ddl.has_selection();

                // Get count information
                let count = ddl.count();

                // Get selected item information for additional context
                let selected_item_info = match ddl.selected_item() {
                    Some(item) => format!("Selected: {} (ID: {})", item.name(), item.id),
                    None => "No item selected".to_string(),
                };

                format!("has_selection(): {has_selection}\ncount(): {count}\n\n{selected_item_info}")
            } else {
                "Error: DropDownList not found".to_string()
            };

            // Update the label
            let h = self.info_handle;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(&info_text);
            }
        }

        // Add a new item to the dropdown list
        fn add_new_item(&mut self) {
            let h = self.dropdownlist_handle;
            if let Some(ddl) = self.control_mut(h) {
                let new_id = ddl.count() + 1;
                ddl.add(CountItem::new(new_id, &format!("New Item {new_id}")));
            }
            self.update_info_label();
        }

        // Remove some items from the dropdown list
        fn remove_items(&mut self) {
            let h = self.dropdownlist_handle;
            if let Some(ddl) = self.control_mut(h) {
                let v = vec![
                    ddl.item(0).unwrap().clone(),
                    ddl.item(2).unwrap().clone(),
                ];
                ddl.clear();
                for item in v {
                    ddl.add(item);
                }            
            }
            self.update_info_label();
        }

        // Clear all items from the dropdown list
        fn clear_items(&mut self) {
            let h = self.dropdownlist_handle;
            if let Some(ddl) = self.control_mut(h) {
                ddl.clear();
            }
            self.update_info_label();
        }
    }

    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Add Item", mywin::Commands::A);
            commandbar.set(key!("F2"), "Remove Items", mywin::Commands::B);
            commandbar.set(key!("F3"), "Clear All", mywin::Commands::C);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            match command_id {
                mywin::Commands::A => self.add_new_item(),
                mywin::Commands::B => self.remove_items(),
                mywin::Commands::C => self.clear_items(),
            }
        }
    }

    impl DropDownListEvents<CountItem> for MyWin {
        fn on_selection_changed(&mut self, _handle: Handle<DropDownList<CountItem>>) -> EventProcessStatus {
            self.update_info_label();
            EventProcessStatus::Processed
        }
    }

    let mut a = App::debug(80, 16, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_item_and_item_mut_methods() {
    #[derive(Clone)]
    struct ItemTest {
        id: u32,
        name: String,
        value: f32,
    }

    impl ItemTest {
        fn new(id: u32, name: &str, value: f32) -> Self {
            Self {
                id,
                name: name.to_string(),
                value,
            }
        }
        
        fn update_value(&mut self, new_value: f32) {
            self.value = new_value;
        }
    }

    impl DropDownListType for ItemTest {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn description(&self) -> &str {
            ""
        }
    }

    let script = "
        Paint.Enable(false)
        Paint('Initial state with 3 items')   
        CheckHash(0x601857CCB7559B9)
        Key.Pressed(F1)
        Paint('First item modified directly (no items selected)')   
        CheckHash(0xB911AC31F37A6CB)
        Key.Pressed(F2)
        Paint('Items examined by index')   
        CheckHash(0xC6E3DA636E6EDD9D)
        Key.Pressed(F3)
        Paint('Access to out-of-bounds items attempted')   
        CheckHash(0x125A2A04C6663246)
        Key.Pressed(F4)
        Paint('Clear all items')   
        CheckHash(0x45F14B7717A22EF8)
        Key.Pressed(F5)
        Paint('Try to access items from empty list')   
        CheckHash(0x6D4AE24F419E4FE1)
    ";
    
    #[Window(events=CommandBarEvents,commands:A+B+C+D+E, internal:true)]
    struct MyWin {
        dropdownlist_handle: Handle<DropDownList<ItemTest>>,
        output_handle: Handle<Label>,
    }
    
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,x:1,y:1,w:78,h:15"),
                dropdownlist_handle: Handle::None,
                output_handle: Handle::None,
            };
            
            let mut ddl = DropDownList::<ItemTest>::new(layout!("x:1,y:1,w:60"), dropdownlist::Flags::None);
            
            // Add initial items
            ddl.add(ItemTest::new(1, "Item One", 10.5));
            ddl.add(ItemTest::new(2, "Item Two", 20.7));
            ddl.add(ItemTest::new(3, "Item Three", 30.2));
            
            // Create a label to display results
            let l = Label::new("", layout!("x:1,y:3,w:76,h:10"));
            
            w.dropdownlist_handle = w.add(ddl);
            w.output_handle = w.add(l);
            
            // Update the info label with initial values
            w.update_info();
            
            w
        }
        
        fn update_info(&mut self) {
            let h = self.dropdownlist_handle;
            let mut info_text = String::new();
            
            if let Some(ddl) = self.control(h) {
                // Get count information
                let count = ddl.count();
                info_text.push_str(&format!("Total items: {count}\n\n"));
                
                // Show all items if any
                if count > 0 {
                    info_text.push_str("Items:\n");
                    for i in 0..count {
                        if let Some(item) = ddl.item(i) {
                            info_text.push_str(&format!("  [{}] {} (value: {:.2})\n", 
                                item.id, item.name, item.value));
                        }
                    }
                } else {
                    info_text.push_str("No items in the list.\n");
                }
                
                // Get selection information
                if ddl.has_selection() {
                    if let Some(selected) = ddl.selected_item() {
                        info_text.push_str(&format!("\nSelected: {} (ID: {})", 
                            selected.name, selected.id));
                    }
                } else {
                    info_text.push_str("\nNo item is selected.");
                }
            } else {
                info_text = "Error: DropDownList not found".to_string();
            }
            
            // Update the label
            let h = self.output_handle;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(&info_text);
            }
        }
        
        // Modify first item using item_mut
        fn modify_first_item(&mut self) {
            let h = self.dropdownlist_handle;
            if let Some(ddl) = self.control_mut(h) {
                if let Some(item) = ddl.item_mut(0) {
                    // Modify the item
                    item.update_value(99.9);
                    item.name = format!("{} (Modified)", item.name);
                }
            }
            self.update_info();
        }
        
        // Examine all items by index and show detailed info
        fn examine_items_by_index(&mut self) {
            let h = self.dropdownlist_handle;
            
            let mut output = String::new();
            output.push_str("Examining items by index:\n");
            
            if let Some(ddl) = self.control(h) {
                let count = ddl.count();
                
                for i in 0..count {
                    if let Some(item) = ddl.item(i) {
                        output.push_str(&format!("Item at index {}: {} (ID: {}, Value: {:.2})\n", 
                            i, item.name, item.id, item.value));
                    } else {
                        output.push_str(&format!("Item at index {i}: None (should not happen)\n"));
                    }
                }
            }
            
            let h = self.output_handle;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(&output);
            }
        }
        
        // Try to access out-of-bounds items
        fn test_out_of_bounds_access(&mut self) {
            let h = self.dropdownlist_handle;
            
            let mut output = String::new();
            output.push_str("Testing out-of-bounds access:\n\n");
            
            if let Some(ddl) = self.control_mut(h) {
                let count = ddl.count();
                
                // Try to access at valid index
                match ddl.item(0) {
                    Some(item) => output.push_str(&format!("item(0): Some({}) - Valid\n", item.name)),
                    None => output.push_str("item(0): None - Invalid\n"),
                }
                
                // Try to access at out-of-bounds index
                match ddl.item(count) {
                    Some(_) => output.push_str(&format!("item({count}): Some - Invalid\n")),
                    None => output.push_str(&format!("item({count}): None - Correctly returns None\n")),
                }
                
                // Try to access at another out-of-bounds index
                match ddl.item(99) {
                    Some(_) => output.push_str("item(99): Some - Invalid\n"),
                    None => output.push_str("item(99): None - Correctly returns None\n"),
                }
                
                // Same with item_mut (we can't easily display the result directly)
                output.push_str(&format!("\nitem_mut(0) exists: {}\n", ddl.item_mut(0).is_some()));
                output.push_str(&format!("item_mut({}) exists: {}\n", count, ddl.item_mut(count).is_some()));
                output.push_str(&format!("item_mut(99) exists: {}\n", ddl.item_mut(99).is_some()));
            }
            
            let h = self.output_handle;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(&output);
            }
        }
        
        // Clear all items
        fn clear_items(&mut self) {
            let h = self.dropdownlist_handle;
            if let Some(ddl) = self.control_mut(h) {
                ddl.clear();
            }
            self.update_info();
        }
        
        // Try to access items from an empty list
        fn test_empty_list_access(&mut self) {
            let h = self.dropdownlist_handle;
            
            let mut output = String::new();
            output.push_str("Testing empty list access:\n\n");
            
            if let Some(ddl) = self.control_mut(h) {
                output.push_str(&format!("List is empty: {}\n", ddl.count() == 0));
                
                // Test selected_item on empty list
                match ddl.selected_item() {
                    Some(_) => output.push_str("selected_item(): Some - Invalid\n"),
                    None => output.push_str("selected_item(): None - Correctly returns None\n"),
                }
                
                // Test selected_item_mut on empty list
                match ddl.selected_item_mut() {
                    Some(_) => output.push_str("selected_item_mut(): Some - Invalid\n"),
                    None => output.push_str("selected_item_mut(): None - Correctly returns None\n"),
                }
                
                // Test item on empty list
                match ddl.item(0) {
                    Some(_) => output.push_str("item(0): Some - Invalid\n"),
                    None => output.push_str("item(0): None - Correctly returns None\n"),
                }
                
                // Test item_mut on empty list
                match ddl.item_mut(0) {
                    Some(_) => output.push_str("item_mut(0): Some - Invalid\n"),
                    None => output.push_str("item_mut(0): None - Correctly returns None\n"),
                }
                
                // Test has_selection on empty list
                output.push_str(&format!("\nhas_selection(): {} - Should be false", ddl.has_selection()));
            }
            
            let h = self.output_handle;
            if let Some(label) = self.control_mut(h) {
                label.set_caption(&output);
            }
        }
    }
    
    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Modify First", mywin::Commands::A);
            commandbar.set(key!("F2"), "Examine Items", mywin::Commands::B);
            commandbar.set(key!("F3"), "Test OOB", mywin::Commands::C);
            commandbar.set(key!("F4"), "Clear Items", mywin::Commands::D);
            commandbar.set(key!("F5"), "Test Empty", mywin::Commands::E);
        }
        
        fn on_event(&mut self, command_id: mywin::Commands) {
            match command_id {
                mywin::Commands::A => self.modify_first_item(),
                mywin::Commands::B => self.examine_items_by_index(),
                mywin::Commands::C => self.test_out_of_bounds_access(),
                mywin::Commands::D => self.clear_items(),
                mywin::Commands::E => self.test_empty_list_access(),
            }
        }
    }
    
    let mut a = App::debug(80, 18, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
