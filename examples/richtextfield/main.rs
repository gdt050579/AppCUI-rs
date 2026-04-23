use appcui::prelude::*;

/// Toggle modes, no heap allocation: each `` ` `` toggles code (aqua), each `**` toggles bold (yellow), each `*` toggles italic (green). Delimiters are skipped for coloring.
fn markdown_like_colors(t: &mut richtextfield::AttributeText) {
    let n = t.char_count();
    let mut ticks = false;
    let mut bold = false;
    let mut italic = false;
    let mut i = 0;
    while i < n {
        let c = t.char_at(i);
        if i + 1 < n && c == '*' && t.char_at(i + 1) == '*' {
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
            t.set_color(i, Color::Aqua, Color::Transparent);
            t.set_flags(i, CharFlags::None);
        } else if bold {
            t.set_color(i, Color::Yellow, Color::Transparent);
            t.set_flags(i, CharFlags::Bold);
        } else if italic {
            t.set_color(i, Color::Green, Color::Transparent);
            t.set_flags(i, CharFlags::Italic);
        }
        i += 1;
    }
}

#[Window()]
struct MyWin {}

impl MyWin {
    fn new() -> Self {
        let initial = concat!(
            "`code` **bold** *italic* — toggles only; pair each delimiter again to exit.\n",
            "Mix `x` with **y** and *z*; `**` still toggles bold (two stars first).",
        );
        let mut win = MyWin {
            base: Window::new(
                "RichTextField — ` ** * toggles (no alloc)",
                layout!("a:c,w:80,h:20"),
                window::Flags::None,
            ),
        };
        win.add(label!(
            "'` = aqua   ** = yellow+bold   * = green+italic — same stack idea, no Vec.',l:1,t:1,r:1,h:2"
        ));
        win.add(RichTextField::with_on_color(
            initial,
            layout!("l:1,t:3,r:1,h:4"),
            richtextfield::Flags::None,
            markdown_like_colors,
        ));
        win
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
