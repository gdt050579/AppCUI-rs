use appcui::prelude::*;

#[derive(ListItem)]
struct MyItem {
    #[Column(name="Text", width=100)]
    text: String,
}
impl MyItem {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Tree,d:c");
    let mut tv = treeview!("MyItem,d:c,flags: ScrollBars+SearchBar+HideHeader");
    let h1 = tv.add(MyItem::new("Root Item 1"));    
    let h2 = tv.add(MyItem::new("Root Item 2"));
    let h1_1 = tv.add_to_parent(MyItem::new("First Child of Root Item 1"), h1);
    let h1_2 = tv.add_to_parent(MyItem::new("Second Child of Root Item 1"), h1);
    let h1_3 = tv.add_to_parent(MyItem::new("Third Child of Root Item 1"), h1);
    let h1_1_1 = tv.add_to_parent(MyItem::new("First Child of First Child of Root Item 1"), h1_1);
    let h2_1 = tv.add_to_parent(MyItem::new("First Child of Root Item 1"), h2);
    let h2_2 = tv.add_to_parent(MyItem::new("Second Child of Root Item 1"), h2);

    w.add(tv);
    a.add_window(w);
    a.run();
    Ok(())
}
