use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    let mut app = App::with_backend(appcui::backend::Type::WindowsVT).color_schema(true).build()?;
    #[cfg(not(target_os = "windows"))]
    let mut app = App::new().build()?;


    let mut win = window!("Title:'Character Formatting Demo',d:c,w:80,h:20,flags:Sizeable");
    let mut c = canvas!("'80x20',d:c,w:100%,h:100%,flags=ScrollBars,lsm:3,tsm:1");
    let s = c.drawing_surface_mut();
    
    let normal_attr = charattr!("white,black");
    let bold_attr = charattr!("white,black,attr:Bold");
    let italic_attr = charattr!("white,black,attr:Italic");
    let underline_attr = charattr!("white,black,attr:Underline");
    let bold_italic_attr = charattr!("white,black,attr:Bold+Italic");
    let bold_underline_attr = charattr!("white,black,attr:Bold+Underline");
    let italic_underline_attr = charattr!("white,black,attr:Italic+Underline");
    let all_formats_attr = charattr!("white,black,attr:Bold+Italic+Underline");
    
    s.write_string(3, 2, "This is normal text", normal_attr, false);    
    s.write_string(3, 4, "This is bold text", bold_attr, false);    
    s.write_string(3, 6, "This is italic text", italic_attr, false);    
    s.write_string(3, 8, "This is underlined text", underline_attr, false);    
    s.write_string(3, 10, "Bold + Italic", bold_italic_attr, false);
    s.write_string(3, 12, "Bold + Underline", bold_underline_attr, false);
    s.write_string(3, 14, "Italic + Underline", italic_underline_attr, false);
    s.write_string(3, 16, "All three formats (Bold + Italic + Underline)", all_formats_attr, false);

    s.write_string(40, 2, "Strike through text", charattr!("white,black,attr:StrikeThrough"), false);
    s.write_string(40, 4, "Double underlined text", charattr!("white,black,attr:Doubleunderline"), false);
    s.write_string(40, 6, "Curly underline text", charattr!("white,black,attr:CurlyUnderline"), false);
    s.write_string(40, 8, "Text with dotted line", charattr!("white,black,attr:DottedUnderline"), false);
    
    win.add(c);
    app.add_window(win);
    app.run();
    
    Ok(())
} 