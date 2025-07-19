use appcui::prelude::*;

#[Window(events = ButtonEvents)]
struct EmoticonsWindow {
    canvas: Handle<Canvas>,
    next_button: Handle<Button>,
    prev_button: Handle<Button>,
    current_index: usize,
}

static EMOTICONS: &[(&str, &str)] = &[
    // Smileys and emotions
    ("😀", "Grinning Face"),
    ("😃", "Grinning Face with Big Eyes"),
    ("😄", "Grinning Face with Smiling Eyes"),
    ("😁", "Beaming Face with Smiling Eyes"),
    ("😅", "Grinning Face with Sweat"),
    ("😂", "Face with Tears of Joy"),
    ("🤣", "Rolling on the Floor Laughing"),
    ("😊", "Smiling Face with Smiling Eyes"),
    ("😇", "Smiling Face with Halo"),
    ("🙂", "Slightly Smiling Face"),
    ("🙃", "Upside-Down Face"),
    ("😉", "Winking Face"),
    ("😌", "Relieved Face"),
    ("😍", "Smiling Face with Heart-Eyes"),
    ("🥰", "Smiling Face with Hearts"),
    ("😘", "Face Blowing a Kiss"),
    ("😗", "Kissing Face"),
    ("😙", "Kissing Face with Smiling Eyes"),
    ("😚", "Kissing Face with Closed Eyes"),
    ("😋", "Face Savoring Food"),
    ("😛", "Face with Tongue"),
    ("😝", "Squinting Face with Tongue"),
    ("😜", "Winking Face with Tongue"),
    ("🤪", "Zany Face"),
    ("🤨", "Face with Raised Eyebrow"),
    ("🧐", "Face with Monocle"),
    ("🤓", "Nerd Face"),
    ("😎", "Smiling Face with Sunglasses"),
    ("🤩", "Star-Struck"),
    ("🥳", "Partying Face"),
    ("😏", "Smirking Face"),
    ("😒", "Unamused Face"),
    ("😞", "Disappointed Face"),
    ("😔", "Pensive Face"),
    ("😟", "Worried Face"),
    ("😕", "Confused Face"),
    ("🙁", "Slightly Frowning Face"),
    ("☹️", "Frowning Face"),
    ("😣", "Persevering Face"),
    ("😖", "Confounded Face"),
    ("😫", "Tired Face"),
    ("😩", "Weary Face"),
    ("🥺", "Pleading Face"),
    ("😢", "Crying Face"),
    ("😭", "Loudly Crying Face"),
    ("😤", "Face with Steam from Nose"),
    ("😠", "Angry Face"),
    ("😡", "Pouting Face"),
    ("🤬", "Face with Symbols on Mouth"),
    ("🤯", "Exploding Head"),
    ("😳", "Flushed Face"),
    ("🥵", "Hot Face"),
    ("🥶", "Cold Face"),
    ("😱", "Face Screaming in Fear"),
    ("😨", "Fearful Face"),
    ("😰", "Anxious Face with Sweat"),
    ("😥", "Sad but Relieved Face"),
    ("😓", "Downcast Face with Sweat"),
    ("🤗", "Hugging Face"),
    ("🤔", "Thinking Face"),
    ("🤭", "Face with Hand Over Mouth"),
    ("🤫", "Shushing Face"),
    ("🤥", "Lying Face"),
    ("😶", "Face Without Mouth"),
    ("😐", "Neutral Face"),
    ("😑", "Expressionless Face"),
    ("😯", "Hushed Face"),
    ("😦", "Frowning Face with Open Mouth"),
    ("😧", "Anguished Face"),
    ("😮", "Face with Open Mouth"),
    ("😲", "Astonished Face"),
    ("😴", "Sleeping Face"),
    ("🤤", "Drooling Face"),
    ("😪", "Sleepy Face"),
    ("😵", "Dizzy Face"),
    ("🤐", "Zipper-Mouth Face"),
    ("🥴", "Woozy Face"),
    ("😷", "Face with Medical Mask"),
    ("🤒", "Face with Thermometer"),
    ("🤕", "Face with Head-Bandage"),
    ("🤢", "Nauseated Face"),
    ("🤮", "Face Vomiting"),
    ("🤧", "Sneezing Face"),
    ("😈", "Smiling Face with Horns"),
    ("👿", "Angry Face with Horns"),
    ("👹", "Ogre"),
    ("👺", "Goblin"),
    ("💀", "Skull"),
    ("☠️", "Skull and Crossbones"),
    ("👻", "Ghost"),
    ("👽", "Alien"),
    ("👾", "Alien Monster"),
    ("🤖", "Robot Face"),
    ("😺", "Grinning Cat Face"),
    ("😸", "Grinning Cat Face with Smiling Eyes"),
    ("😹", "Cat Face with Tears of Joy"),
    ("😻", "Smiling Cat Face with Heart-Eyes"),
    ("😼", "Cat Face with Wry Smile"),
    ("😽", "Kissing Cat Face"),
    ("🙀", "Weary Cat Face"),
    ("😿", "Crying Cat Face"),
    ("😾", "Pouting Cat Face"),
];

impl EmoticonsWindow {
    fn new() -> Self {
        let mut win = Self {
            base: window!("'Emoticons',a:c,w:50,h:12"),
            canvas: Handle::None,
            next_button: Handle::None,
            prev_button: Handle::None,
            current_index: 0,
        };

        win.canvas = win.add(canvas!("l:0,t:0,r:0,b:3,size:78x20,back:{' ',white,black}"));
        win.prev_button = win.add(button!("'&Previous',l:5,b:0,w:15"));
        win.next_button = win.add(button!("'&Next',l:30,b:0,w:15"));
        win.repaint_emoticons();
        win
    }

    fn repaint_emoticons(&mut self) {
        let c_index = self.current_index;
        let (emoji, name) = EMOTICONS[c_index];
        let index_text = format!("{}/{}", c_index + 1, EMOTICONS.len());
        let h = self.canvas;
        let x_poz = 25 - (name.len() as i32) / 2;
        let first_char = emoji.chars().next().unwrap_or('?');
        let unicode_text = format!("U+{:04X}", first_char as u32);

        if let Some(canvas) = self.control_mut(h) {
            let s = canvas.drawing_surface_mut();
            s.clear(Character::new(' ', Color::White, Color::Black, CharFlags::None));
            s.fill_horizontal_line(5, 1, 42, Character::new(' ', Color::Black, Color::White, CharFlags::None));
            s.write_string(24, 1, emoji, CharAttribute::with_color(Color::Black, Color::White), false);
            s.write_string(x_poz, 3, name, CharAttribute::with_color(Color::White, Color::Black), false);
            s.write_string(x_poz, 4, &index_text, CharAttribute::with_color(Color::Yellow, Color::Black), false);
            s.write_string(x_poz, 5, &unicode_text, CharAttribute::with_color(Color::Aqua, Color::Black), false);
        }
    }
}

impl ButtonEvents for EmoticonsWindow {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.prev_button {
            if self.current_index > 0 {
                self.current_index -= 1;
            } else {
                self.current_index = EMOTICONS.len() - 1;
            }
            self.repaint_emoticons();
            EventProcessStatus::Processed
        } else if handle == self.next_button {
            self.current_index = (self.current_index + 1) % EMOTICONS.len();
            self.repaint_emoticons();
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(EmoticonsWindow::new());
    app.run();
    Ok(())
}
