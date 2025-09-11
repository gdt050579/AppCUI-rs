use appcui::prelude::*;

struct MyObject {
    name: String,
    description: String,
    symbol: String,
}

impl MyObject {
    fn new(name: &str, description: &str, symbol: &str) -> MyObject {
        MyObject {
            name: name.to_string(),
            description: description.to_string(),
            symbol: symbol.to_string(),
        }
    }
}

impl DropDownListType for MyObject {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn symbol(&self) -> &str {
        &self.symbol
    }
}

#[derive(EnumSelector, Eq, PartialEq, Copy, Clone)]
enum Shape {
    #[VariantInfo(name = "Square", description = "a red square")]
    Square,

    #[VariantInfo(name = "Rectangle", description = "a green rectangle")]
    Rectangle,

    #[VariantInfo(name = "Triangle", description = "a blue triangle")]
    Triangle,

    #[VariantInfo(name = "Circle", description = "a white circle")]
    Circle,
}

fn add_description(tab: &mut Tab, index: u32, desc: &str) {
    tab.add(index, vline!("r:30,t:0,b:0"));
    tab.add(index, Label::new(desc, layout!("x:99%,y:1,w:27,h:100%,p:tr")));
}

fn add_buttons(tab: &mut Tab, index: u32) {
    add_description(tab, index, "Buttons are controls that can be used to trigger a command when:\n- the user clicks on them\n- you have focus and you press Enter of Space\n- there is a hot key associated and the user pressed Alt+<HotKey>");
    tab.add(index, button!("'&Normal button',x: 1,y:1, w: 18"));
    tab.add(index, button!("'Inactive button',x: 1,y:4, w: 18, enable: false"));
    tab.add(index, button!("'&Flat button',x: 1,y:7, w: 18, type: Flat"));
}

fn add_checkboxes(tab: &mut Tab, index: u32) {
    macro_rules! add {
        ($x: expr, $y: expr, $type: expr, $name: expr) => {
            let mut p = Panel::new($name, LayoutBuilder::new().x($x).y($y).width(20).height(9).build(), panel::Type::Window);
            p.add(CheckBox::with_type("Checked", LayoutBuilder::new().x(1).y(1).width(18).build(), true, $type));
            p.add(CheckBox::with_type("Un-Checked", LayoutBuilder::new().x(1).y(2).width(18).build(), false, $type));
            let mut c = CheckBox::with_type("Inactive", LayoutBuilder::new().x(1).y(3).width(18).build(), false, $type);
            c.set_enabled(false);
            p.add(c);
            p.add(CheckBox::with_type("Multi-Line\nText", LayoutBuilder::new().x(1).y(4).width(18).height(2).build(), false, $type));
            tab.add(index, p);
        };
    }
    add_description(tab, index, "Checkboxes are controls that have two states (checked or unchecked) - a reflection of a boolean value.");
    add!(1,1, checkbox::Type::Standard, "Standard");
    add!(22,1, checkbox::Type::CheckBox, "CheckBoxes");
    add!(43,1, checkbox::Type::CheckMark, "CheckMark");
    add!(1,11, checkbox::Type::PlusMinus, "Plus/Minus");
    let mut p = panel!("'Tri-State checkboxes',x:22,y:11,w:42,h:9, type: Window");
    p.add(threestatebox!("'Checked',x:1,y:1,w:18,state: checked"));
    p.add(threestatebox!("'Not-Checked',x:1,y:2,w:18,state: unchecked"));
    p.add(threestatebox!("'Unknown/Undefined state',x:1,y:3,w:18,state: unknown"));
    tab.add(index, p);
}

fn add_radioboxes(tab: &mut Tab, index: u32) {
    macro_rules! add {
        ($x: expr, $y: expr, $type: expr, $name: expr) => {
            let mut p = Panel::new($name, LayoutBuilder::new().x($x).y($y).width(20).height(9).build(), panel::Type::Window);
            p.add(RadioBox::with_type("Checked", LayoutBuilder::new().x(1).y(1).width(18).build(), true, $type));
            p.add(RadioBox::with_type("Un-Checked", LayoutBuilder::new().x(1).y(2).width(18).build(), false, $type));
            let mut c = RadioBox::with_type("Inactive", LayoutBuilder::new().x(1).y(3).width(18).build(), false, $type);
            c.set_enabled(false);
            p.add(c);
            p.add(RadioBox::with_type("Multi-Line\nText", LayoutBuilder::new().x(1).y(4).width(18).height(2).build(), false, $type));
            tab.add(index, p);
        };
    }
    add_description(tab, index, "Radioboxes are controls that have two states (checked or unchecked) but unlike a checkbox they are used in groups where only one can be checked at a time.");
    add!(1,1, radiobox::Type::Standard, "Standard");
    add!(22,1, radiobox::Type::Circle, "Circular");
    add!(43,1, radiobox::Type::Diamond, "Diamond");
}

fn add_selectors(tab: &mut Tab, index: u32) {
    add_description(tab, index, "Selctors are component that allows you to chose from multiple variants (such as colors, characters, list of options or numbers).");
    tab.add(index, label!("'Combobox (drop-down)',x:1,y:1, w: 20"));
    let mut c = ComboBox::new(layout!("x:25,y:1,w:30"), combobox::Flags::ShowDescription);
    // data from https://en.wikipedia.org/wiki/Fastest_animals
    c.add_item(combobox::Item::new("Cheetah","(120 km/h)"));
    c.add_item(combobox::Item::new("Swordfish","(97 km/h)"));
    c.add_item(combobox::Item::new("Iguana","(35 km/h)"));
    c.add_item(combobox::Item::new("Gazelle","(81 km/h)"));
    c.add_item(combobox::Item::new("Lion","(80 km/h)"));
    c.add_item(combobox::Item::new("Dog","(60 km/h)"));
    c.add_item(combobox::Item::new("Zebra","(56 km/h)"));  
    tab.add(index, c);  

    tab.add(index, label!("'Drop down list',x:1,y:3, w: 20"));
    let mut db = DropDownList::<MyObject>::with_symbol(1, layout!("x:25,y:3,w:30"), dropdownlist::Flags::ShowDescription);
    db.add(MyObject::new("Heart", "(symbol of love)", "♥"));
    db.add(MyObject::new("Spade", "(used in a deck of cards)", "♠"));
    tab.add(index, db); 

    tab.add(index, label!("'Enum selector',x:1,y:5, w: 20"));
    tab.add(index, selector!("enum: Shape,x:25,y:5,w:30, flags: AllowNoneVariant"));    

    tab.add(index, label!("'Color picker',x:1,y:7, w: 20"));
    tab.add(index, colorpicker!("Yellow,x:25,y:7,w:30")); 

    tab.add(index, label!("'Date selector',x:1,y:9, w: 20"));
    tab.add(index, datepicker!("date:2025-09-09,x:25,y:9,w:30"));    

    tab.add(index, label!("'Character picker',x:1,y:11, w: 20"));
    tab.add(index, charpicker!("code=65,x:25,y:11,w:30,sets=[Ascii,Arrows,Emoticons,Shapes]"));    

}



fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(120, 30)).single_window().build()?;
    let mut w = window!("'Basic Controls',dock:fill");
    let mut t = tab!("d:f, tabs:[Buttons,CheckBoxes,RadioBoxes,Selectors], type: OnLeft");
    add_buttons(&mut t, 0);
    add_checkboxes(&mut t, 1);
    add_radioboxes(&mut t, 2);
    add_selectors(&mut t, 3);
    w.add(t);
    a.add_window(w);
    a.run();
    Ok(())
}
