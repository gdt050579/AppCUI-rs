use appcui::prelude::*;
// use appcui::controls::common::*;
// use appcui::controls::layout::Layout;
// use appcui::graphics::*;
// use appcui::input::*;
// use appcui::system::*;
// use AppCUIProcMacro::*;

// use appcui::controls::menu::*;
// use appcui::controls::*;
// use appcui::input::{Key, KeyCode, KeyModifier};

// static keywords: [&str; 26] = [
//     "if",
//     "do",
//     "while",
//     "until",
//     "case",
//     "return",
//     "local",
//     "global",
//     "for",
//     "foreach",
//     "error",
//     "exit",
//     "repeat",
//     "class",
//     "struct",
//     "union",
//     "auto",
//     "try",
//     "catch",
//     "hrow",
//     "finally",
//     "extends",
//     "implements",
//     "public",
//     "private",
//     "protected",
// ];

// fn main_2() -> Result<(), appcui::system::Error> {
//     // let script = "
//     //     Key.Pressed(Tab)
//     //     Paint('all checkboxes are checked')
//     //     //CheckHash(0xECEDF66E8A6A588C)
//     //     Key.Pressed(Enter)
//     //     Resize(50,16)
//     //     //Paint('Checkbox is not checked')
//     //     //CheckHash(0x9FADB347088D23A5)
//     //     Mouse.Move(16,0)
//     //     //Paint('After mouse move')
//     //     //CheckHash(0x68FB99AFFF8F9CF5)
//     //     Mouse.Click(16,0,left)
//     //     Paint('After mouse click')
//     //     Mouse.Move(16,2);
//     //     Paint('After mouse move')
//     //     Mouse.Click(16,2,left)
//     //     Paint('Show sub-menu')
//     // ";
//     let script = "
//         Key.Pressed(Alt+S)
//         Key.Pressed(Down,2)
//         Paint('Show sub-menu')
//         Key.Pressed(Enter)
//         Key.Pressed(Down,4)
//         Paint('Show sub-menu (2)')
//         Key.Pressed(Enter)
//         Paint('Show sub-menu (3)')
//     ";
//     //let mut a = App::debug(60, 20, InitializationFlags::Menu, script)?;
//     //let mut a = App::default()?;
//     // let mut a = App::new(TerminalType::Default, None, InitializationFlags::Menu)?;
//     // let mut w = Window::new(
//     //     "Simple window",
//     //     Layout::new("d:c,w:40,h:10"),
//     //     WindowFlags::Sizeable,
//     // );
//     // w.add(CheckBox::new(
//     //     "This is a checkbox that &enables a certain property that is required by this program",
//     //     Layout::new("x:1,y:1,w:35,h:3"),
//     //     true,
//     // ));
//     // w.add(CheckBox::new(
//     //     "&Second check box",
//     //     Layout::new("x:1,y:4,w:35"),
//     //     true,
//     // ));
//     // w.add(CheckBox::new(
//     //     "&Third check box",
//     //     Layout::new("x:1,y:5,w:35"),
//     //     true,
//     // ));
//     // w.add(Button::new(
//     //     "&Press me",
//     //     Layout::new("x:1,y:7,w:30"),
//     //     ButtonFlags::None,
//     // ));

//     // let mut m_file = Menu::new("&File");
//     // m_file.add_command("&New", key!("Ctrl+N"), 100);
//     // m_file.add_command("&Open", Key::new(KeyCode::O, KeyModifier::Ctrl), 101);
//     // m_file.add_command("&Save", Key::new(KeyCode::S, KeyModifier::Ctrl), 102);
//     // m_file.add_command("Save &as ...", Key::default(), 103);
//     // m_file.add_separator();
//     // m_file.add_checkbox("Option &1", Key::default(), 100, true);
//     // m_file.add_checkbox(
//     //     "Option &2",
//     //     Key::new(KeyCode::F10, KeyModifier::None),
//     //     101,
//     //     false,
//     // );
//     // m_file.add_separator();
//     // m_file.add_command("E&xit", Key::new(KeyCode::F4, KeyModifier::Alt), 100);
//     // a.add_menu(m_file, "&File");

//     // let mut m_opt = Menu::new("&Radio");
//     // m_opt.add_radiobox("Radio &1", Key::default(), 1000, false);
//     // m_opt.add_radiobox("Radio &2", Key::default(), 1000, false);
//     // m_opt.add_radiobox("Radio &3", Key::default(), 1000, true);
//     // m_opt.add_radiobox("Radio &4", Key::default(), 1000, false);
//     // m_opt.add_radiobox("Radio &5", Key::default(), 1000, false);
//     // a.add_menu(m_opt, "&Radio");

//     // let mut m_sm = Menu::new("&Submenus");
//     // let mut m_colors = Menu::new("&Colors");
//     // m_colors.add_command("Red", Key::None, 103);
//     // m_colors.add_command("Green", Key::None, 103);
//     // m_colors.add_command("Blue", Key::None, 103);
//     // m_colors.add_command("White", Key::None, 103);
//     // m_colors.add_command("Teak", Key::None, 103);
//     // m_sm.add_submenu(m_colors);
//     // let mut m_size = Menu::new("&Size in ...");
//     // m_size.add_radiobox("Km", KeyCode::F1, 123, false);
//     // m_size.add_radiobox("Cm", Key::from(KeyCode::F2), 123, false);
//     // m_size.add_radiobox("Mm", Key::from(KeyCode::F3), 123, true);
//     // m_size.add_separator();
//     // let mut m_keywords = Menu::new("&Keywords");
//     // for k in &keywods {
//     //     m_keywords.add_command(k, Key::None, 1234);
//     // }
//     // m_keywords.add_separator();
//     // for k in &keywods {
//     //     m_keywords.add_command(k, Key::None, 1234);
//     // }
//     // m_size.add_submenu(m_keywords);

//     // m_sm.add_submenu(m_size);
//     // a.add_menu(m_sm, "&Submenus");

//     // a.add(w);
//     // a.run();
//     Ok(())
// }

// #[AppCUIWindow(overwrite=OnEvent+CommandBarEvents)]
// struct MyWindow {
//     h_button: ControlHandle<Button>,
//     h_label: ControlHandle<Label>,
// }
// impl MyWindow {
//     fn new() -> Self {
//         let mut w = MyWindow {
//             base: Window::new(
//                 "My Windows",
//                 Layout::new("d:c,w:40,h:10"),
//                 WindowFlags::None,
//             ),
//             h_button: ControlHandle::None,
//             h_label: ControlHandle::None,
//         };
//         w.h_label = w.add(Label::new("", Layout::new("x:1,y:1,w: 30")));
//         let b = Button::new("&Press me", Layout::new("x:1,y:4,w:12"), ButtonFlags::None);
//         w.h_button = w.add(b);
//         w
//     }
// }
// impl OnEvent for MyWindow {
//     fn on_event(&mut self, event: Event) -> EventProcessStatus {
//         match event {
//             Event::ButtonClicked(evnt) => {
//                 if self.h_button == evnt.handle {
//                     let h_label = self.h_label;
//                     if let Some(label) = self.get_control_mut(h_label) {
//                         label.set_text("Button pressed");
//                     }
//                     return EventProcessStatus::Processed;
//                 }
//             }
//             _ => {}
//         }
//         EventProcessStatus::Ignored
//     }
// }
// impl CommandBarEvents for MyWindow {
//     fn on_update_commandbar(&self, c: &mut CommandBar) {
//         c.set(key!("F2"), "Save", 100);
//         c.set(KeyCode::F3, "Load", 101);
//         c.set(Key::new(KeyCode::F1,KeyModifier::Alt), "New", 102);
//     }

//     fn on_event(&mut self, command_id: u32) {
//         match command_id {
//             100 => {},
//             101 => {},
//             _ => {}
//         }
//     }
// }

// fn main() -> Result<(), appcui::system::Error> {
//     let mut a = App::debug(60, 20, InitializationFlags::CommandBar, Desktop::new(), "
//         Paint('print')
//         Mouse.Move(12,10)
//         Paint('after move')
//     ")?;
//     //let mut a = App::default()?;
//     a.add_window(MyWindow::new());
//     a.run();
//     Ok(())
// }

// // fn main() -> Result<(), appcui::system::Error> {
// //     //let mut a = App::debug(60, 20, InitializationFlags::None, "Paint('print')")?;
// //     let mut a = App::default()?;
// //     let mut w = Window::new("MyWin", Layout::new("d:c,w:60,h:10"), WindowFlags::None);
// //     let l_handle = w.add(Label::new("A label", Layout::new("x:1,y:1,w:40")));
// //     let mut b = Button::new("Press me", Layout::new("x:1,y:3,w:15"), ButtonFlags::None);
// //     b.set_handler(move |handle| {
// //         if l_handle == handle {
// //             println!("label");
// //         }
// //     });
// //     w.add(b);
// //     a.add(w);
// //     a.run();
// //     Ok(())
// // }use appcui::prelude::*;

// #[CustomControl(overwrite=OnPaint+OnKeyPressed)]
// struct MyControl {}
// impl MyControl {
//     fn new()->Self { MyControl{base: ControlBase::new("d:c,w:10,h:1", StatusFlags::)}}
// }

static text: &str = r"--- From Wiki ----
Rust is a multi-paradigm, general-purpose 
programming language that emphasizes performance, 
type safety, and concurrency. It enforces memory 
safety—meaning that all references point to valid 
memory—without a garbage collector. To 
simultaneously enforce memory safety and prevent 
data races, its 'borrow checker' tracks the object 
lifetime of all references in a program during 
compilation. Rust was influenced by ideas from 
functional programming, including immutability, 
higher-order functions, and algebraic data types. 
It is popular for systems programming.

From: https://en.wikipedia.org/wiki/Rust_(programming_language)
";

#[Window(events = ButtonEvents, commands=Save+Open+New)]
struct MyWin {
    viewer: Handle<Canvas>,
}
impl MyWin {
    fn new() -> Self {
        let mut win = MyWin {
            base: window!("'My Win',d:c,w:40,h:16"),
            viewer: Handle::None,
        };
        let mut c = Canvas::new(Size::new(6, 3), Layout::new("l:15,t:0,b:0,r:0"), canvas::Flags::ScrollBars);
        let s = c.get_drawing_surface();
        s.write_string(0, 0, text, CharAttribute::with_color(Color::White, Color::Black), true);
        win.viewer = win.add(c);
        win.add(button!("Test,l:1,t:1,a:tl,w:10"));     
                           
        win
    }
}
impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        // first run the modal window
        let h = self.viewer;
        if let Some(canvas) = self.get_control_mut(h) {
            canvas.resize_surface(Size::new(60, 15));
            canvas
                .get_drawing_surface()
                .write_string(0, 0, text, CharAttribute::with_color(Color::White, Color::Black), true);
        }
        EventProcessStatus::Processed
    }
}
fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().size(Size::new(60, 20)).build()?;
    a.add_window(MyWin::new());
    a.run();
    Ok(())

    // //let mut a = App::new().size(Size::new(80,25)).build()?;
    // // let script = "
    // //     Paint()
    // //     Key.Pressed(Up)
    // //     Paint()
    // // ";
    // // let mut a = App::debug(80, 20, script).build()?;

    // let mut w = window!("Title,d:c,w:60,h:14,flags:Sizeable");
    // let mut c = Canvas::new(Size::new(80,40),Layout::new("l:15,t:0,b:0,r:0"),canvas::Flags::ScrollBars);
    // let s = c.get_drawing_surface();
    // s.clear(Character::with_color(Color::White, Color::Black));
    // //s.draw_rect(Rect::new(0, 0, 39, 39), LineType::Single, CharAttribute::with_color(Color::Aqua, Color::Black));
    // for x in 1..39 {
    //     for y in 1..39 {
    //         let c = (((x+y)+30) as u8) as char;
    //         s.write_char(x, y, Character::with_char(c));
    //     }
    // }
    // w.add(c);
    // w.add(button!("Test,l:1,t:1,a:tl    ,w:10"));
    // // let g = w.get_toolbar().create_group(toolbar::GroupPosition::BottomLeft);
    // // w.add(ColorPicker::new(Color::DarkGreen,Layout::new("x:1,y:1,w:32")));
    // // w.add(colorpicker!("Red,x:1,y:3,w:7"));
    // // w.add(colorpicker!("color:green,x:20,y:3,w:7"));
    // // w.add(ColorPicker::new(Color::Red,Layout::new("x:1,y:3,w:7")));
    // // w.add(ColorPicker::new(Color::Green,Layout::new("x:20,y:3,w:7")));
    // a.add_window(w);
    // //a.add_window(MyWin::new());
    // a.run();
    // Ok(())
}
