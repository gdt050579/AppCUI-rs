use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    let mut app = App::with_backend(appcui::backend::Type::WindowsVT).color_schema(false).build()?;
    #[cfg(not(target_os = "windows"))]
    let mut app = App::new().build()?;


    let mut w = Window::new("True Colors", Layout::new("d:c,w:70,h:19"), window::Flags::None);

    let mut c = canvas!("'68x17',a:c,w:100%,h:100%");
    c.clear_background();
    let s = c.drawing_surface_mut();
    s.write_string(1, 1, "Red pallete", CharAttribute::with_color(Color::White, Color::Black), false);
    s.write_string(1, 4, "Green pallete", CharAttribute::with_color(Color::White, Color::Black), false);
    s.write_string(1, 7, "Blue pallete", CharAttribute::with_color(Color::White, Color::Black), false);
    s.write_string(1, 10, "Gray pallete", CharAttribute::with_color(Color::White, Color::Black), false);

    for i in 0..64 {
        let v = (i * 4) as u8;
        s.write_char(1 + i, 2, Character::new(' ', Color::Black, Color::from_rgb(v, 0, 0), CharFlags::None));
        s.write_char(1 + i, 5, Character::new(' ', Color::Black, Color::from_rgb(0, v, 0), CharFlags::None));
        s.write_char(1 + i, 8, Character::new(' ', Color::Black, Color::from_rgb(0, 0, v), CharFlags::None));
        s.write_char(1 + i, 11, Character::new(' ', Color::Black, Color::from_rgb(v, v, v), CharFlags::None));
    }

    s.write_string(1, 13, "Make sure that you compile with the 'TRUE_COLORS' feature enabled.", CharAttribute::with_color(Color::Gray, Color::Black), false);
    s.write_string(1, 14, " If after this, you don't see various nounces or red, green, blue ", CharAttribute::with_color(Color::Gray, Color::Black), false);
    s.write_string(1, 15, "    and white, then your terminal does not support true colors.   ", CharAttribute::with_color(Color::Gray, Color::Black), false);
    s.write_string(38, 13, "TRUE_COLORS", CharAttribute::with_color(Color::Silver, Color::Black), false);
    

    w.add(c);
    app.add_window(w);
    app.run();
    Ok(())
}