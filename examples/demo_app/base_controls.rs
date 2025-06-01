use appcui::prelude::*;

#[Window()]
pub(crate) struct Win {
}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("'Controls Demo',d:c,w:50,h:14,flags:Sizeable"),
        };

        // Create tab control
        let mut tabs = tab!("l:1,t:1,r:1,b:3,tabs:['Checkboxes','Radioboxes','Texts']");
        
        // Add controls to checkboxes tab
        tabs.add(0, checkbox!("'Option 1',l:1,t:1,w:20"));
        tabs.add(0, checkbox!("'Option 2',l:1,t:2,w:20"));
        tabs.add(0, checkbox!("'Option 3',l:1,t:3,w:20"));

        // Add controls to radioboxes tab
        tabs.add(1, radiobox!("'Choice 1',l:1,t:1,w:20,select:true"));
        tabs.add(1, radiobox!("'Choice 2',l:1,t:2,w:20"));
        tabs.add(1, radiobox!("'Choice 3',l:1,t:3,w:20"));

        // Add controls to texts tab
        tabs.add(2, textfield!("l:1,t:1,w:30"));
        tabs.add(2, combobox!("l:1,t:3,w:30,items:['Item 1','Item 2','Item 3']"));

        me.add(tabs);
        
        // Add a button docked at the bottom center of the window
        me.add(button!("'Click Me!',d:b,w:20"));
        
        me
    }
}

