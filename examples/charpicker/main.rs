use appcui::prelude::*;

#[Window(events = CharPickerEvents)]
struct MyWin {
    cp_ascii: Handle<CharPicker>,
    cp_vowels: Handle<CharPicker>,
    cp_symbols: Handle<CharPicker>,
    cp_all: Handle<CharPicker>,
    lb_ascii: Handle<Label>,
    lb_vowels: Handle<Label>,
    lb_symbols: Handle<Label>,
    lb_all: Handle<Label>,
}

impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: Window::new("CharPicker Example", layout!("a:c,w:58,h:15"), window::Flags::None),
            cp_ascii: Handle::None,
            cp_vowels: Handle::None,
            cp_symbols: Handle::None,
            cp_all: Handle::None,
            lb_ascii: Handle::None,
            lb_vowels: Handle::None,
            lb_symbols: Handle::None,
            lb_all: Handle::None,
        };

        // ASCII 
        win.add(Label::new("ASCII:", layout!("x:2,y:1,w:20")));
        win.cp_ascii = win.add(charpicker!("x:12,y:1,w:42,sets:[Ascii]"));
        win.lb_ascii = win.add(Label::new("No character selected", layout!("x:12,y:2,w:56")));
        win.add(hline!("x:2,y:3,w:52"));

        // Custom with a string containing vowels
        win.add(Label::new("Vowels:", layout!("x:2,y:4,w:20")));
        let mut cp_vowels = CharPicker::new(None, layout!("x:12,y:4,w:42"));
        cp_vowels.clear_sets();
        cp_vowels.add_set(charpicker::Set::new("Vowels", "AEIOUaeiou").unwrap());
        win.cp_vowels = win.add(cp_vowels);
        win.lb_vowels = win.add(Label::new("No character selected", layout!("x:12,y:5,w:56")));
        win.add(hline!("x:2,y:6,w:52"));

        // Some symbols
        win.add(Label::new("Icons:", layout!("x:2,y:7,w:20")));
        win.cp_symbols = win.add(charpicker!("x:12,y:7,w:42,sets:[Pictographs, Emoticons, Animals, Shapes, Arrows]"));
        win.lb_symbols = win.add(Label::new("No character selected", layout!("x:12,y:8,w:56")));
        win.add(hline!("x:2,y:9,w:52"));

        // all available sets
        win.add(Label::new("All Sets:", layout!("x:2,y:10,w:20")));
        win.cp_all = win.add(charpicker!("x:12,y:10,w:42,sets:[*]"));
        win.lb_all = win.add(Label::new("No character selected", layout!("x:12,y:11,w:56")));

        win
    }

    fn update_label(&mut self, char: Option<char>, label_handle: Handle<Label>) {
        let text = if let Some(ch) = char {
            format!("Char: '{}' | Dec: {} | Hex: U+{:04X}", ch, ch as u32, ch as u32)
        } else {
            "No character selected".to_string()
        };

        if let Some(label) = self.control_mut(label_handle) {
            label.set_caption(&text);
        }
    }
}

impl CharPickerEvents for MyWin {
    fn on_char_changed(&mut self, handle: Handle<CharPicker>, code: Option<char>) -> EventProcessStatus {
        let label_handle = match handle {
            _ if handle == self.cp_ascii => self.lb_ascii,
            _ if handle == self.cp_vowels => self.lb_vowels,
            _ if handle == self.cp_symbols => self.lb_symbols,
            _ if handle == self.cp_all => self.lb_all,
            _ => return EventProcessStatus::Ignored,
        };
        self.update_label(code, label_handle);
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
