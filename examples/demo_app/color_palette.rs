use appcui::prelude::*;

#[Window()]
pub(crate) struct Win {}

impl Win {
    pub(crate) fn new() -> Self {
        let mut me = Self {
            base: window!("'Color Palette',a:c,w:48,h:11,flags:Sizeable"),
        };
        let mut canvas = canvas!("d:c,w:100%,h:100%,flags:ScrollBars,size:256x64");
        let surface = canvas.drawing_surface_mut();
        surface.clear(char!("' ',black,black"));
        for x in 0..43 {
            let c = (x * 6) as u8;
            surface.write_char(x+1, 1, Character::new(' ', Color::Black, Color::from_rgb(c, 0, 0), CharFlags::None));
            surface.write_char(x+1, 2, Character::new(' ', Color::Black, Color::from_rgb(0, c, 0), CharFlags::None));
            surface.write_char(x+1, 3, Character::new(' ', Color::Black, Color::from_rgb(0, 0, c), CharFlags::None));
            surface.write_char(x+1, 4, Character::new(' ', Color::Black, Color::from_rgb(c, c, 0), CharFlags::None));
            surface.write_char(x+1, 5, Character::new(' ', Color::Black, Color::from_rgb(c, 0, c), CharFlags::None));
            surface.write_char(x+1, 6, Character::new(' ', Color::Black, Color::from_rgb(0, c, c), CharFlags::None));
            surface.write_char(x+1, 7, Character::new(' ', Color::Black, Color::from_rgb(c, c, c), CharFlags::None));
        }
        me.add(canvas);
        me
    }
}
