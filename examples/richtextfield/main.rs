use appcui::prelude::*;

/// Toggle modes, no heap allocation: each `` ` `` toggles code (aqua), each `**` toggles bold (yellow), each `*` toggles italic (green). Delimiters are skipped for coloring.
fn markdown_like_colors(t: &mut richtextfield::AttributeText, _theme: &Theme) {
    let n = t.count();
    let mut ticks = false;
    let mut bold = false;
    let mut italic = false;
    let mut i = 0;
    while i < n {
        let c = t.char(i).unwrap_or('\0');
        if i + 1 < n && c == '*' && t.char(i + 1).unwrap_or('\0') == '*' {
            bold = !bold;
            i += 2;
            continue;
        }
        if c == '`' {
            ticks = !ticks;
            i += 1;
            continue;
        }
        if c == '*' {
            italic = !italic;
            i += 1;
            continue;
        }

        if ticks {
            t.set_attr(i, CharAttribute::new(Color::Aqua, Color::Transparent, CharFlags::None));
        } else if bold {
            t.set_attr(i, CharAttribute::new(Color::Yellow, Color::Transparent, CharFlags::Bold));
        } else if italic {
            t.set_attr(i, CharAttribute::new(Color::Green, Color::Transparent, CharFlags::Italic));
        }
        i += 1;
    }
}

fn all_capitals(t: &mut richtextfield::AttributeText, theme: &Theme) {
    let n = t.count();
    let mut i = 0;
    while i < n {
        let c = t.char(i).unwrap_or('\0');
        if c.is_ascii_lowercase() {
            t.set_char(i, c.to_ascii_uppercase());
            t.set_attr(i, theme.text.normal);
        }
        i += 1;
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    let mut w = window!("'RichTextField example',a:c,w:80,h:20");
    w.add(label!("'Use **, * and ` to toggle colors',l:1,t:1,r:1,h:1"));
    w.add(richtextfield!("'',l:1,t:2,r:1,h:4,parser:markdown_like_colors"));
    w.add(label!("'Auto capitalize text',l:1,t:6,r:1,h:1"));
    w.add(richtextfield!("'',l:1,t:7,r:1,h:4,parser:all_capitals"));
    w.add(label!("'Parser applied after each edit',l:1,t:13,r:1,h:1"));
    w.add(richtextfield!("'**Plan**: Run `code`',l:1,t:14,r:1,h:1,parser:markdown_like_colors"));

    app.add_window(w);
    app.run();
    Ok(())
}
