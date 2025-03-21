use appcui::prelude::*;

const LOGO: [&str; 11] = [
    "    █████████                         █████████  █████  █████ █████                               ",
    "    ███░░░░░███                       ███░░░░░███░░███  ░░███ ░░███                               ",
    "   ░███    ░███  ████████  ████████  ███     ░░░  ░███   ░███  ░███             ████████   █████  ",
    "   ░███████████ ░░███░░███░░███░░███░███          ░███   ░███  ░███  ██████████░░███░░███ ███░░   ",
    "   ░███░░░░░███  ░███ ░███ ░███ ░███░███          ░███   ░███  ░███ ░░░░░░░░░░  ░███ ░░░ ░░█████  ",
    "   ░███    ░███  ░███ ░███ ░███ ░███░░███     ███ ░███   ░███  ░███             ░███      ░░░░███ ",
    "   █████   █████ ░███████  ░███████  ░░█████████  ░░████████   █████            █████     ██████  ",
    "  ░░░░░   ░░░░░  ░███░░░   ░███░░░    ░░░░░░░░░    ░░░░░░░░   ░░░░░            ░░░░░     ░░░░░░   ",
    "                 ░███      ░███                                                                   ",
    "                 █████     █████                                                                  ",
    "                ░░░░░     ░░░░░                                                                   ",
];

#[Desktop(overwrite = OnPaint)]
struct MyDesktop {}
impl MyDesktop {
    fn new() -> Self {
        Self { base: Desktop::new() }
    }
}
impl OnPaint for MyDesktop {
    fn on_paint(&self, surface: &mut Surface, _: &Theme) {
        surface.clear(char!("' ',black,black"));
        let attr = charattr!("w,black");
        let x = ((surface.size().width as i32) / 2) - (LOGO[0].chars().count() as i32 / 2);
        let top_corner = ((surface.size().height as i32) / 2) - 5;
        let mut y = 0;
        for line in LOGO {
            for (index, c) in line.chars().enumerate() {
                if c == ' ' {
                    continue; 
                }
                if c == '█' {
                    let a = match () {
                        _ if (index >= 80) => charattr!("dr,dr"),
                        _ if y <= 3 => charattr!("w,w"),
                        _ if y <= 6 => charattr!("silver,silver"),
                        _ => charattr!("gray,gray"),
                    };
                    surface.write_char(x + index as i32, y + top_corner, Character::with_attributes(c, a));
                } else {
                    surface.write_char(x + index as i32, y + top_corner, Character::with_attributes(c, attr));
                }
            }
            y += 1;
        }
    }
}
fn main() -> Result<(), appcui::system::Error> {
    App::new().desktop(MyDesktop::new()).build()?.run();
    Ok(())
}
