use crate::prelude::*;
use crate::ui::markdown_composer::{Flags, ListFlags, MarkdownComposer};

#[test]
fn check_creation() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('empty composer, plain text, text with markdown')
        CheckHash(0x7EA296B6B9140808)
    ";
    App::new()
        .size(Size::new(60, 14))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:0,y:0,w:58,h:13");
            w.add(MarkdownComposer::new(layout!("x:1,y:0,w:54,h:3"), Flags::None));
            w.add(MarkdownComposer::from("plain text", layout!("x:1,y:3,w:54,h:3"), Flags::None));
            w.add(MarkdownComposer::from(
                "**bold** _italic_ `code`\n- item\n> quote",
                layout!("x:1,y:6,w:54,h:4"),
                Flags::None,
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_creation_procmacro() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('controls built through the markdown_composer! macro')
        CheckHash(0x9ED51285EAB3B66)
    ";
    App::new()
        .size(Size::new(60, 14))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:0,y:0,w:58,h:13");
            w.add(markdown_composer!("x:1,y:0,w:54,h:2"));
            w.add(markdown_composer!("'**first**',x:1,y:2,w:54,h:2"));
            w.add(markdown_composer!("text:'**second**',x:1,y:4,w:54,h:2,flags:ShowMarkers"));
            w.add(markdown_composer!("content:'third',x:1,y:6,w:54,h:2,flags:ReadOnly|Emoticons"));
            w.add(markdown_composer!(
                "'with lists',x:1,y:8,w:54,h:2,emoji:':',
             lists:[{'@',items:['Ana','Bob'],flags:RemoveTrigger},{trigger:'#',values:[{bug,'🐛'},{todo,'📝'}]}]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_setters_and_getters() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('after the setters ran')
        CheckHash(0x8BE2225B43F34BF2)
    ";
    App::new()
        .size(Size::new(60, 12))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:0,y:0,w:58,h:11");

            let mut mc = MarkdownComposer::new(layout!("x:1,y:0,w:54,h:3"), Flags::None);
            assert_eq!(mc.text(), "");
            assert!(mc.flags() == Flags::None);
            assert!(!mc.show_markers());
            assert!(!mc.is_read_only());
            assert!(!mc.emoticons_enabled());
            assert_eq!(mc.cursor_offset(), 0);
            assert_eq!(mc.selected_text(), None);

            mc.set_text("**Hi** there\r\nsecond\rthird");
            assert_eq!(mc.text(), "**Hi** there\nsecond\nthird");
            assert_eq!(mc.cursor_offset(), 0);

            mc.set_show_markers(true);
            assert!(mc.show_markers());
            mc.set_show_markers(true);
            assert!(mc.show_markers());
            mc.set_read_only(true);
            assert!(mc.is_read_only());
            mc.set_emoticons_enabled(true);
            assert!(mc.emoticons_enabled());
            assert!(mc.flags() == Flags::ShowMarkers | Flags::ReadOnly | Flags::Emoticons);

            mc.set_show_markers(false);
            mc.set_read_only(false);
            mc.set_emoticons_enabled(false);
            assert!(mc.flags() == Flags::None);
            w.add(mc);

            let mut mc = MarkdownComposer::from("ab ăî 😀", layout!("x:1,y:3,w:54,h:2"), Flags::ShowMarkers | Flags::ReadOnly);
            assert!(mc.flags() == Flags::ShowMarkers | Flags::ReadOnly);
            mc.set_cursor_offset(1000);
            assert_eq!(mc.cursor_offset(), 12);
            mc.set_cursor_offset(4);
            assert_eq!(mc.cursor_offset(), 3);
            mc.set_cursor_offset(6);
            assert_eq!(mc.cursor_offset(), 5);
            mc.set_cursor_offset(9);
            assert_eq!(mc.cursor_offset(), 8);

            mc.select_all();
            assert_eq!(mc.selected_text(), Some("ab ăî 😀"));
            assert_eq!(mc.cursor_offset(), 12);
            mc.clear_selection();
            assert_eq!(mc.selected_text(), None);
            assert_eq!(mc.cursor_offset(), 12);
            w.add(mc);

            let mut mc = MarkdownComposer::new(layout!("x:1,y:5,w:54,h:2"), Flags::None);
            mc.select_all();
            assert_eq!(mc.selected_text(), None);
            assert!(mc.list('@').is_none());
            mc.add_list('@', &["Ana", "Bob"], ListFlags::None);
            assert_eq!(mc.list('@').unwrap().items(), ["Ana".to_string(), "Bob".to_string()]);
            mc.add_list('@', &["Carl"], ListFlags::RemoveTrigger);
            assert_eq!(mc.list('@').unwrap().items(), ["Carl".to_string()]);
            mc.add_list_with_values('#', &[("bug", "🐛")], ListFlags::None);
            assert_eq!(mc.list('#').unwrap().items(), ["bug".to_string()]);
            mc.add_emoji_list(':');
            assert!(mc.list(':').map(|list| list.len()).unwrap_or(0) > 100);
            assert!(mc.remove_list('#'));
            assert!(!mc.remove_list('#'));
            assert!(mc.list('#').is_none());
            w.add(mc);

            let mc = MarkdownComposer::new(layout!("x:1,y:7,w:54,h:2"), Flags::None)
                .with_list('@', &["Ana"], ListFlags::None)
                .with_list_values('#', &[("todo", "📝")], ListFlags::RemoveTrigger)
                .with_emoji_list(':');
            assert_eq!(mc.list('@').unwrap().items(), ["Ana".to_string()]);
            assert_eq!(mc.list('#').unwrap().items(), ["todo".to_string()]);
            assert!(mc.list(':').is_some());
            w.add(mc);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_write_text() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('empty composer, cursor on the first cell')
        CheckHash(0x87E151555690DAF0)
        CheckCursor(2,2)
        Key.TypeText('Hello world')
        Paint('Hello world')
        CheckHash(0x319FCD6A037F39D0)
        CheckCursor(13,2)
        Key.Pressed(Enter)
        Key.TypeText('second')
        Paint('second line')
        CheckHash(0x9A5CD32DFED9620)
        CheckCursor(8,3)
        Key.TypeText('ăî 😀')
        Paint('unicode text, the emoji takes two columns')
        CheckHash(0xF5023CD7ED43E365)
        CheckCursor(13,3)
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:8");
            w.add(MarkdownComposer::new(layout!("x:0,y:0,w:100%,h:100%"), Flags::None));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_write_markdown() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.TypeText('**bold** and _italic_')
        Paint('markers are hidden while typing')
        CheckHash(0x1FD3B6D8FE065F74)
        CheckCursor(17,2)
        Key.Pressed(Enter)
        Key.TypeText('- item')
        Paint('bullet on the second line')
        CheckHash(0xEF65AD253966A7BD)
        CheckCursor(8,3)
        Key.Pressed(Enter)
        Key.TypeText('> quote')
        Paint('quote on the third line')
        CheckHash(0xA38BB7106AE370AD)
        CheckCursor(9,4)
        Key.Pressed(Enter)
        Key.TypeText('`code` https://a.ro')
        Paint('inline code and a link')
        CheckHash(0xDA051C0F15DA1B0F)
        CheckCursor(19,5)
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:8");
            w.add(MarkdownComposer::new(layout!("x:0,y:0,w:100%,h:100%"), Flags::None));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_backspace_and_delete() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('Hello big world, cursor at the start')
        CheckHash(0x2AB3839C07A3B9DC)
        CheckCursor(2,2)
        Key.Pressed(Delete)
        Paint('ello big world')
        CheckHash(0xD3C96691683C0664)
        CheckCursor(2,2)
        Key.Pressed(Backspace)
        Paint('backspace at the start, nothing changes')
        CheckHash(0xD3C96691683C0664)
        CheckCursor(2,2)
        Key.Pressed(End)
        Key.Pressed(Backspace)
        Paint('ello big worl')
        CheckHash(0x5AAD8CE5969C7D70)
        CheckCursor(15,2)
        Key.Pressed(Delete)
        Paint('delete at the end, nothing changes')
        CheckHash(0x5AAD8CE5969C7D70)
        CheckCursor(15,2)
        Key.Pressed(Home)
        Key.Pressed(Shift+Right,4)
        Key.Pressed(Delete)
        Paint('ello selected and deleted, the space stays: _big worl')
        CheckHash(0xDA9AD7D9BC6DE136)
        CheckCursor(2,2)
        Key.Pressed(Shift+End)
        Key.Pressed(Backspace)
        Paint('everything deleted')
        CheckHash(0x87E151555690DAF0)
        CheckCursor(2,2)
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:8");
            w.add(MarkdownComposer::from("Hello big world", layout!("x:0,y:0,w:100%,h:100%"), Flags::None));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_delete_words() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('unu doi trei patru')
        CheckHash(0xC4858DF21FA02624)
        CheckCursor(2,2)
        Key.Pressed(Ctrl+Delete)
        Paint('first word deleted: doi trei patru')
        CheckHash(0x5777FEA525C5EF36)
        CheckCursor(2,2)
        Key.Pressed(End)
        Key.Pressed(Ctrl+Backspace)
        Paint('last word deleted: doi trei')
        CheckHash(0x70C7C3832ABC4488)
        CheckCursor(11,2)
        Key.Pressed(Ctrl+Backspace)
        Paint('trei deleted: doi')
        CheckHash(0xA298BD983E24589E)
        CheckCursor(6,2)
        Key.TypeText('abc def')
        Key.Pressed(Shift+Left,3)
        Key.Pressed(Ctrl+Backspace)
        Paint('selection deleted instead of the word: doi abc')
        CheckHash(0xBA188AAC7BB3525E)
        CheckCursor(10,2)
        Key.Pressed(Home)
        Key.Pressed(Ctrl+Delete,5)
        Paint('everything deleted, extra presses do nothing')
        CheckHash(0x87E151555690DAF0)
        CheckCursor(2,2)
        Key.Pressed(Ctrl+Backspace)
        Paint('still empty')
        CheckHash(0x87E151555690DAF0)
        CheckCursor(2,2)
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:8");
            w.add(MarkdownComposer::from(
                "unu doi trei patru",
                layout!("x:0,y:0,w:100%,h:100%"),
                Flags::None,
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_move_left_right_home_end() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('cursor at the start')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(2,2)
        Key.Pressed(Right)
        Paint('after a, the hidden markers are skipped')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(3,2)
        Key.Pressed(Right,2)
        Paint('after the space')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(5,2)
        Key.Pressed(End)
        Paint('end of the first line')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(7,2)
        Key.Pressed(Right)
        Paint('start of the second line')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(2,3)
        Key.Pressed(Right,2)
        Paint('after the emoji')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(5,3)
        Key.Pressed(Left)
        Paint('before the emoji')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(3,3)
        Key.Pressed(Home)
        Key.Pressed(Left)
        Paint('back at the end of the first line')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(7,2)
        Key.Pressed(Home)
        Key.Pressed(Left)
        Paint('left at the start, nothing changes')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(2,2)
        Key.Pressed(Ctrl+End)
        Paint('end of the text')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(6,3)
        Key.Pressed(Right)
        Paint('right at the end, nothing changes')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(6,3)
        Key.Pressed(Ctrl+Home)
        Paint('start of the text')
        CheckHash(0xAEEE82EA505AF9EA)
        CheckCursor(2,2)
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:8");
            w.add(MarkdownComposer::from("**ab** cd\nx😀y", layout!("x:0,y:0,w:100%,h:100%"), Flags::None));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_move_up_down() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.Pressed(End)
        Paint('end of the long first line')
        CheckHash(0x3747D5EE36A2FD11)
        CheckCursor(15,2)
        Key.Pressed(Down)
        Paint('the short second line, cursor at its end')
        CheckHash(0x3747D5EE36A2FD11)
        CheckCursor(4,3)
        Key.Pressed(Down)
        Paint('third line, back on the remembered column')
        CheckHash(0x3747D5EE36A2FD11)
        CheckCursor(15,4)
        Key.Pressed(Down)
        Paint('down on the last line')
        CheckHash(0x3747D5EE36A2FD11)
        CheckCursor(16,4)
        Key.Pressed(Up,2)
        Paint('back on the first line, on the remembered column')
        CheckHash(0x3747D5EE36A2FD11)
        CheckCursor(15,2)
        Key.Pressed(Up)
        Paint('up on the first line, nothing changes')
        CheckHash(0x3747D5EE36A2FD11)
        CheckCursor(15,2)
        Key.Pressed(Left,4)
        Key.Pressed(Down,2)
        Paint('moving left forgets the old column, the new one is used')
        CheckHash(0x3747D5EE36A2FD11)
        CheckCursor(11,4)
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:8");
            w.add(MarkdownComposer::from(
                "long line one\nab\nlast line here",
                layout!("x:0,y:0,w:100%,h:100%"),
                Flags::None,
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_move_words() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.Pressed(Ctrl+Right)
        Paint('before doi')
        CheckHash(0x2AE1FA805DA4B8DC)
        CheckCursor(6,2)
        Key.Pressed(Ctrl+Right)
        Paint('before trei, both spaces skipped')
        CheckHash(0x2AE1FA805DA4B8DC)
        CheckCursor(11,2)
        Key.Pressed(Ctrl+Right)
        Paint('end of the first line, stops before the new line')
        CheckHash(0x2AE1FA805DA4B8DC)
        CheckCursor(15,2)
        Key.Pressed(Ctrl+Right)
        Paint('start of the second line')
        CheckHash(0x2AE1FA805DA4B8DC)
        CheckCursor(2,3)
        Key.Pressed(Ctrl+Right,2)
        Paint('end of the text, extra press does nothing')
        CheckHash(0x2AE1FA805DA4B8DC)
        CheckCursor(7,3)
        Key.Pressed(Ctrl+Left)
        Paint('before patru')
        CheckHash(0x2AE1FA805DA4B8DC)
        CheckCursor(2,3)
        Key.Pressed(Ctrl+Left)
        Paint('before trei')
        CheckHash(0x2AE1FA805DA4B8DC)
        CheckCursor(11,2)
        Key.Pressed(Ctrl+Left,3)
        Paint('start of the text, extra press does nothing')
        CheckHash(0x2AE1FA805DA4B8DC)
        CheckCursor(2,2)
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:8");
            w.add(MarkdownComposer::from(
                "unu doi  trei\npatru",
                layout!("x:0,y:0,w:100%,h:100%"),
                Flags::None,
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_select_and_copy() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.Pressed(Shift+Right,5)
        Key.Pressed(Ctrl+C)
        Paint('Hello selected')
        CheckHash(0x403049493280A08B)
        CheckCursor(7,2)
        CheckClipboardText('Hello')
        Key.Pressed(Ctrl+Shift+Right,2)
        Key.Pressed(Ctrl+Insert)
        Paint('selection extended by two words')
        CheckHash(0xD8236D55ECA0CAAC)
        CheckCursor(12,2)
        CheckClipboardText('Hello big ')
        Key.Pressed(Shift+End)
        Key.Pressed(Ctrl+C)
        Paint('whole line selected')
        CheckHash(0x65D1DBA45DA36B5B)
        CheckCursor(17,2)
        CheckClipboardText('Hello big world')
        Key.Pressed(Ctrl+Shift+Left)
        Key.Pressed(Ctrl+C)
        Paint('selection shrunk back before world')
        CheckHash(0xD8236D55ECA0CAAC)
        CheckCursor(12,2)
        CheckClipboardText('Hello big ')
        Key.Pressed(End)
        Key.Pressed(Ctrl+Shift+Left)
        Key.Pressed(Ctrl+C)
        Paint('only world selected')
        CheckHash(0x157DA7E4532C6E0B)
        CheckCursor(12,2)
        CheckClipboardText('world')
        Key.Pressed(Right)
        Key.Pressed(Ctrl+C)
        Paint('right with a selection goes to its end, the clipboard keeps the old text')
        CheckHash(0x2AB3839C07A3B9DC)
        CheckCursor(17,2)
        CheckClipboardText('world')
        Key.Pressed(Ctrl+A)
        Key.Pressed(Ctrl+C)
        Paint('everything selected')
        CheckHash(0x65D1DBA45DA36B5B)
        CheckCursor(17,2)
        CheckClipboardText('Hello big world')
        Key.Pressed(Left)
        Paint('left with a selection goes to its start')
        CheckHash(0x2AB3839C07A3B9DC)
        CheckCursor(2,2)
        Key.Pressed(Shift+Right,5)
        Key.Pressed(Shift+Left)
        Key.Pressed(Ctrl+C)
        Paint('shift keeps selecting, Hell selected')
        CheckHash(0xB049F13739DA74F8)
        CheckCursor(6,2)
        CheckClipboardText('Hell')
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:8");
            w.add(MarkdownComposer::from("Hello big world", layout!("x:0,y:0,w:100%,h:100%"), Flags::None));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_cut_and_paste() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.Pressed(Shift+Right,6)
        Key.Pressed(Ctrl+X)
        Paint('Hello cut: big world')
        CheckHash(0xC79CFDEDD311EDA)
        CheckCursor(2,2)
        CheckClipboardText('Hello ')
        Key.Pressed(End)
        Key.Pressed(Ctrl+V)
        Paint('pasted at the end: big worldHello')
        CheckHash(0x32B633C5C4D3DDDC)
        CheckCursor(17,2)
        Clipboard.SetText('X')
        Key.Pressed(Shift+Insert)
        Paint('pasted again: big worldHello X')
        CheckHash(0x3347C2A509A0E74)
        CheckCursor(18,2)
        Key.Pressed(Home)
        Key.Pressed(Shift+Right,3)
        Key.Pressed(Ctrl+V)
        Paint('paste replaces the selection: X worldHello X')
        CheckHash(0xCA110D0684019230)
        CheckCursor(3,2)
        Key.Pressed(Ctrl+X)
        Paint('cut without selection does nothing')
        CheckHash(0xCA110D0684019230)
        CheckCursor(3,2)
        CheckClipboardText('X')
        Key.Pressed(Ctrl+A)
        Key.Pressed(Shift+Delete)
        Paint('everything cut')
        CheckHash(0x87E151555690DAF0)
        CheckCursor(2,2)
        CheckClipboardText('X worldHello X')
        Clipboard.Clear()
        Key.Pressed(Ctrl+V)
        Paint('empty clipboard, nothing pasted')
        CheckHash(0x87E151555690DAF0)
        CheckCursor(2,2)
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:8");
            w.add(MarkdownComposer::from("Hello big world", layout!("x:0,y:0,w:100%,h:100%"), Flags::None));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_scroll_page_and_wrap() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('the first line wraps on two rows, rows 1 to 7 visible')
        CheckHash(0x7FB3869A519C63FB)
        CheckCursor(1,1)
        Key.Pressed(PageDown)
        Paint('cursor moved one page down, the view follows it')
        CheckHash(0x28F6D13A7B084A0)
        CheckCursor(1,8)
        Key.Pressed(PageDown)
        Paint('second page down')
        CheckHash(0x72C28475C2FB90F3)
        CheckCursor(1,8)
        Key.Pressed(PageUp)
        Paint('one page up')
        CheckHash(0xD42ADA9DFB4E01C4)
        CheckCursor(1,1)
        Key.Pressed(Ctrl+Down,2)
        Paint('view scrolled two rows, the cursor stays on its line')
        CheckHash(0x29837D56D3B5841C)
        CheckCursor(hidden)
        Key.Pressed(Ctrl+Up)
        Paint('view scrolled back one row')
        CheckHash(0x72C28475C2FB90F3)
        CheckCursor(hidden)
        Key.Pressed(Ctrl+PageDown)
        Paint('view scrolled one page down')
        CheckHash(0xDC9EE3814C90CFDD)
        CheckCursor(hidden)
        Key.Pressed(Ctrl+PageUp)
        Paint('view scrolled one page up')
        CheckHash(0x72C28475C2FB90F3)
        CheckCursor(hidden)
        Mouse.Wheel(10,4,down,1)
        Paint('wheel down scrolls three rows')
        CheckHash(0xFE22B3923D1003EC)
        CheckCursor(hidden)
        Mouse.Wheel(10,4,up,1)
        Paint('wheel up scrolls back')
        CheckHash(0x72C28475C2FB90F3)
        CheckCursor(hidden)
        Key.Pressed(Ctrl+End)
        Paint('end of the text, last rows visible')
        CheckHash(0x859FDD8AE2866487)
        CheckCursor(8,8)
        Mouse.Wheel(10,4,down,5)
        Paint('wheel past the end, nothing more to scroll')
        CheckHash(0x859FDD8AE2866487)
        CheckCursor(8,8)
        Key.Pressed(Ctrl+Home)
        Paint('back at the start')
        CheckHash(0x7FB3869A519C63FB)
        CheckCursor(1,1)
        Mouse.Wheel(10,4,up,5)
        Paint('wheel past the start, nothing more to scroll')
        CheckHash(0x7FB3869A519C63FB)
        CheckCursor(1,1)
        Resize(24,10)
        Paint('narrower terminal, the text wraps on more rows')
        CheckHash(0xC1F90A8018AC7712)
        CheckCursor(1,1)
        Resize(60,10)
        Paint('wider terminal, the first line fits on one row')
        CheckHash(0xAC89505108110035)
        CheckCursor(1,1)
    ";
    App::new().size(Size::new(40, 10)).debug_script(script).window(|| {
        let mut w = window!("Title,x:0,y:0,w:100%,h:100%");
        w.add(MarkdownComposer::from(
            "one two three four five six seven eight nine\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9\nline 10\nline 11\nline 12\nline 13\nline 14\nline 15\nline 16\nline 17\nline 18\nline 19\nline 20\nline 21\nline 22\nline 23\nline 24\nline 25\nline 26\nline 27\nline 28\nline 29\nline 30",
            layout!("x:0,y:0,w:100%,h:100%"),
            Flags::None,
        ));
        w
    }).run().unwrap();
}

#[test]
fn check_mouse() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Mouse.Click(4,1,left)
        Paint('click inside the bold word, cursor between the two l letters')
        CheckHash(0x8DBD670253641493)
        CheckCursor(4,1)
        Mouse.Click(30,2,left)
        Paint('click after the end of the second line, cursor at its end')
        CheckHash(0x8DBD670253641493)
        CheckCursor(17,2)
        Mouse.DoubleClick(8,1,left)
        Key.Pressed(Ctrl+C)
        Paint('double click selects the word big')
        CheckHash(0x808897BB11B32738)
        CheckClipboardText('big')
        Mouse.Drag(1,1,6,1)
        Key.Pressed(Ctrl+C)
        Paint('drag selects Hello, the copy gets both bold markers')
        CheckHash(0x38E1A15E24DF487C)
        CheckClipboardText('**Hello**')
        Mouse.Drag(2,1,4,1)
        Key.Pressed(Ctrl+C)
        Paint('part of the bold word selected, the copy is still valid markdown')
        CheckHash(0x79B5D845D0A5DB43)
        CheckClipboardText('**el**')
        Mouse.Drag(1,2,1,9)
        Paint('drag below the control scrolls the text')
        CheckHash(0xD081B9D01D790D1C)
        Key.Pressed(Ctrl+C)
        Mouse.Click(2,1,left)
        Paint('a click clears the selection')
        CheckHash(0x64DF1C8D0ACD44F4)
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:0,y:0,w:100%,h:100%");
            w.add(MarkdownComposer::from(
                "**Hello** big world\nsecond line here\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9\nline 10\nline 11\nline 12",
                layout!("x:0,y:0,w:100%,h:100%"),
                Flags::None,
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_selection_with_hidden_markers() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Mouse.Drag(1,1,6,1)
        Key.Pressed(Ctrl+X)
        Paint('Hello cut, no bold marker is left behind')
        CheckHash(0xE67596E0BFD04112)
        CheckCursor(1,1)
        CheckClipboardText('**Hello**')
        Key.Pressed(Ctrl+A)
        Key.Pressed(Ctrl+C)
        CheckClipboardText(' big **world**')
        Key.Pressed(End)
        Key.Pressed(Shift+Left,2)
        Key.Pressed(Backspace)
        Paint('ld deleted from the bold word, the word stays bold')
        CheckHash(0xEA9DB96236E8B82A)
        CheckCursor(9,1)
        Key.Pressed(Ctrl+A)
        Key.Pressed(Ctrl+C)
        CheckClipboardText(' big **wor**')
        Key.Pressed(End)
        Key.Pressed(Shift+Left,2)
        Key.TypeText('X')
        Paint('typing over or keeps the new letter bold')
        CheckHash(0xA311CB9B3C9E6867)
        CheckCursor(8,1)
        Key.Pressed(Ctrl+A)
        Key.Pressed(Ctrl+C)
        CheckClipboardText(' big **wX**')
        Key.Pressed(Home)
        Key.Pressed(Right)
        Key.Pressed(Shift+End)
        Key.Pressed(Delete)
        Paint('selection from normal text into the bold word deleted')
        CheckHash(0xE9F1EE53356CB4CC)
        CheckCursor(2,1)
        Key.Pressed(Ctrl+A)
        Key.Pressed(Ctrl+C)
        CheckClipboardText(' ')
    ";
    App::new()
        .size(Size::new(40, 6))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:0,y:0,w:100%,h:100%");
            w.add(MarkdownComposer::from(
                "**Hello** big **world**",
                layout!("x:0,y:0,w:100%,h:100%"),
                Flags::None,
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_code_blocks() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('code block framed on the left, raw markers on the right')
        CheckHash(0x279BA1291D8550A8)
        CheckCursor(2,2)
        Key.Pressed(Down,2)
        Paint('cursor inside the code block')
        CheckHash(0x279BA1291D8550A8)
        CheckCursor(3,4)
        Key.Pressed(End)
        Key.TypeText(' // ok')
        Paint('typing inside the code block, the frame grows')
        CheckHash(0x4BFBEF3737A07B64)
        CheckCursor(19,4)
    ";
    App::new()
        .size(Size::new(60, 12))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:58,h:10");
            w.add(MarkdownComposer::from(
                "text `inline` end\n```\nlet x = 1;\nlet y;\n```\nafter",
                layout!("x:29,y:0,w:27,h:8"),
                Flags::ShowMarkers | Flags::ReadOnly,
            ));
            w.add(MarkdownComposer::from(
                "text `inline` end\n```\nlet x = 1;\nlet y;\n```\nafter",
                layout!("x:0,y:0,w:28,h:8"),
                Flags::None,
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_emoticons() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.TypeText('a :B): b ::D: c :(: d')
        Paint('emoticons replaced while typing, the emoji list closed each time')
        CheckHash(0xC0A2530EDC58C984)
        CheckCursor(18,2)
        Key.TypeText(' x:P: :nothing: `:P:`')
        Paint('glued to a letter, unknown code and inside code stay as text')
        CheckHash(0xE1B57E9ABEAB053D)
        Key.TypeText(' :rocket: :SMILE:')
        Paint('emoji names between colons are replaced too, whatever the case')
        CheckHash(0xBE2FE09C571AD4D)
        Key.Pressed(Ctrl+A)
        Key.Pressed(Ctrl+C)
        CheckClipboardText('a 😎 b 😀 c 🙁 d x:P: :nothing: `:P:` 🚀 😀')
        Key.Pressed(Tab)
        Key.TypeText(':B):')
        Paint('the second composer has no Emoticons flag, nothing is replaced')
        CheckHash(0xEAACC768B9238252)
        CheckCursor(6,5)
    ";
    App::new()
        .size(Size::new(60, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:58,h:8");
            w.add(MarkdownComposer::new(layout!("x:0,y:3,w:56,h:2"), Flags::None));
            w.add(MarkdownComposer::new(layout!("x:0,y:0,w:56,h:3"), Flags::Emoticons).with_emoji_list(':'));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_read_only() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('read only text')
        CheckHash(0xE2AC0047CBFD3D13)
        CheckCursor(2,2)
        Key.TypeText('xyz @')
        Key.Pressed(Enter)
        Key.Pressed(Delete)
        Key.Pressed(Ctrl+Delete)
        Clipboard.SetText('pasted')
        Key.Pressed(Ctrl+V)
        Key.Pressed(End)
        Key.Pressed(Backspace)
        Key.Pressed(Ctrl+Backspace)
        Paint('nothing was written, deleted or pasted, no list opened')
        CheckHash(0xE2AC0047CBFD3D13)
        CheckCursor(16,2)
        Key.Pressed(Home)
        Key.Pressed(Ctrl+Right)
        Key.Pressed(Shift+End)
        Key.Pressed(Ctrl+X)
        Key.Pressed(Ctrl+C)
        Paint('moving, selecting and copying still work, cut does not delete')
        CheckHash(0x10A3DB08305E39EC)
        CheckCursor(16,2)
        CheckClipboardText('**only** text')
        Key.Pressed(Ctrl+A)
        Key.Pressed(Ctrl+C)
        CheckClipboardText('read **only** text')
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:8");
            w.add(
                MarkdownComposer::from("read **only** text", layout!("x:0,y:0,w:100%,h:100%"), Flags::ReadOnly).with_list(
                    '@',
                    &["Ana"],
                    ListFlags::None,
                ),
            );
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_events() {
    #[Window(events = MarkdownComposerEvents, internal = true)]
    struct MyWin {
        changes: u32,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title,x:1,y:1,w:38,h:6"),
                changes: 0,
            };
            w.add(MarkdownComposer::new(layout!("x:0,y:0,w:100%,h:100%"), Flags::Emoticons));
            w
        }
    }
    impl MarkdownComposerEvents for MyWin {
        fn on_text_changed(&mut self, handle: Handle<MarkdownComposer>) -> EventProcessStatus {
            self.changes += 1;
            let len = self.control(handle).map(|mc| mc.text().len()).unwrap_or(0);
            let title = format!("changes={} len={}", self.changes, len);
            self.base.set_title(&title);
            EventProcessStatus::Processed
        }
        fn on_validate(&mut self, _handle: Handle<MarkdownComposer>, text: &str) -> EventProcessStatus {
            let title = format!("valid:{}", text.replace('\n', "|"));
            self.base.set_title(&title);
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('no event yet')
        CheckHash(0xDCC73EE08F41D1F8)
        Key.TypeText('ab')
        Paint('title reads changes=2 len=2')
        CheckHash(0x817959E49C1ECBE1)
        Key.Pressed(Left)
        Key.Pressed(Right)
        Key.Pressed(Home)
        Paint('moving the cursor raises no event, title unchanged')
        CheckHash(0x817959E49C1ECBE1)
        Key.Pressed(Delete)
        Key.Pressed(Enter)
        Paint('title reads changes=4 len=2')
        CheckHash(0xF36D7741ED582232)
        CheckCursor(2,3)
        Key.Pressed(Ctrl+End)
        Key.Pressed(Delete)
        Key.Pressed(Ctrl+Home)
        Key.Pressed(Backspace)
        Paint('delete at the end and backspace at the start raise nothing')
        CheckHash(0xF36D7741ED582232)
        CheckCursor(2,2)
        Clipboard.SetText('xy')
        Key.Pressed(Ctrl+V)
        Paint('paste raises one event, title reads changes=5 len=4')
        CheckHash(0x623D7E0F82607B50)
        CheckCursor(4,2)
        Key.Pressed(Ctrl+Enter)
        Paint('title reads valid:xy|b')
        CheckHash(0xF6D2A75B3D2E52F7)
        Key.Pressed(Ctrl+End)
        Key.TypeText(' :B):')
        Paint('five keys and the emoticon replacement, title reads changes=11 len=9')
        CheckHash(0x55781D5504CEE58E)
        CheckCursor(6,3)
    ";
    App::new().size(Size::new(40, 8)).debug_script(script).window(MyWin::new).run().unwrap();
}

#[test]
fn check_list_popup_keyboard() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.TypeText('hi @')
        Paint('list opened under the trigger, first item selected')
        CheckHash(0x97579348C1560750)
        CheckCursor(6,2)
        Key.Pressed(Down)
        Paint('Bogdan selected')
        CheckHash(0x12FB9F24F6AFBA00)
        Key.Pressed(Down,5)
        Paint('last item selected, the list scrolled')
        CheckHash(0xEB0485BA132F1E37)
        Key.Pressed(PageUp,2)
        Paint('first item selected again')
        CheckHash(0x97579348C1560750)
        Key.Pressed(PageDown)
        Paint('one page down, Elena selected')
        CheckHash(0x2397595B42C307A9)
        Key.Pressed(PageUp)
        Key.Pressed(Up)
        Paint('back on Ana, up on the first item does nothing')
        CheckHash(0x97579348C1560750)
        Key.TypeText('ri')
        Paint('filtered by ri, only Cristina, Florin and Gabriel left')
        CheckHash(0xBA5840F04565F41)
        CheckCursor(8,2)
        Key.Pressed(Backspace,2)
        Key.TypeText('aN')
        Paint('filter ignores the case, Ana, Bogdan and Dan left')
        CheckHash(0x85C4A3AD8355F89E)
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('Enter inserted @Bogdan, the list is closed')
        CheckHash(0x5557EFF21F25C78)
        CheckCursor(12,2)
        Key.TypeText(' @b')
        Key.Pressed(Tab)
        Paint('Tab inserts as well')
        CheckHash(0xAA7D899FB2DAE555)
        CheckCursor(20,2)
        Key.TypeText(' @c')
        Key.Pressed(Home)
        Paint('Home closes the list and moves the cursor, the typed text stays')
        CheckHash(0x26D9C492F1A09B36)
        CheckCursor(2,2)
        Key.Pressed(End)
        CheckCursor(23,2)
        Key.TypeText(' @d')
        Key.Pressed(Right)
        Paint('Right closes the list without inserting anything')
        CheckHash(0x3E9D6A881C067E7A)
        CheckCursor(26,2)
        Key.TypeText(' @e')
        Key.Pressed(Escape)
        Paint('Escape closes the list and keeps the typed text')
        CheckHash(0x523B171681F590A3)
        Key.TypeText(' @zz')
        Paint('no match, the list closes by itself')
        CheckHash(0x952411808273E9A3)
        Key.TypeText(' @d')
        Key.Pressed(Backspace)
        Paint('the filter is empty again, the whole list is back')
        CheckHash(0xD538C6BAF0B18DE6)
        Key.Pressed(Backspace)
        Paint('the trigger was deleted, the list is closed')
        CheckHash(0x952411808273E9A3)
        Key.TypeText('a@')
        Paint('a trigger glued to a letter does not open the list')
        CheckHash(0xFDF20C4445C9DF56)
        Key.TypeText(' #r')
        Key.Pressed(Enter)
        Paint('the # list removes its trigger, only red is inserted')
        CheckHash(0x1655148424E69105)
        Key.Pressed(Ctrl+A)
        Key.Pressed(Ctrl+C)
        CheckClipboardText('hi @Bogdan @Bogdan @c @d @e @zz a@ red')
    ";
    App::new()
        .size(Size::new(60, 12))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:58,h:10");
            w.add(
                MarkdownComposer::new(layout!("x:0,y:0,w:100%,h:100%"), Flags::None)
                    .with_list('@', &["Ana", "Bogdan", "Cristina", "Dan", "Elena", "Florin", "Gabriel"], ListFlags::None)
                    .with_list('#', &["red", "green", "blue"], ListFlags::RemoveTrigger),
            );
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_list_popup_values_emoji_and_procmacro() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.TypeText('#')
        Paint('values list, each value shown before its name')
        CheckHash(0x97F590CAF1597FB9)
        Key.TypeText('t')
        Key.Pressed(Enter)
        Paint('the value of todo was inserted, the trigger is kept')
        CheckHash(0x5D383BD6FEF7E3EB)
        CheckCursor(5,2)
        Key.TypeText(' :pizz')
        Paint('emoji list filtered by pizz')
        CheckHash(0x7DDF83320CC1671D)
        Key.Pressed(Enter)
        Key.TypeText(' @b')
        Key.Pressed(Enter)
        Paint('pizza emoji and Bob inserted, both triggers removed')
        CheckHash(0x3189E2F6FD959939)
        CheckCursor(12,2)
        Key.Pressed(Ctrl+A)
        Key.Pressed(Ctrl+C)
        CheckClipboardText('#📝 🍕 Bob')
    ";
    App::new()
        .size(Size::new(40, 12))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:10");
            w.add(markdown_composer!(
                "x:0,y:0,w:100%,h:100%,emoji:':',
             lists:[{'@',items:['Ana','Bob'],flags:RemoveTrigger},{trigger:'#',values:[{bug,'🐛'},{todo,'📝'},{done,'✅'}]}]"
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_list_popup_mouse() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.TypeText('@')
        Paint('list opened, Ana selected')
        CheckHash(0xDAB133B7F100944D)
        Mouse.Move(5,6)
        Paint('hovering Cristina selects it')
        CheckHash(0x6F2C2F989D24129D)
        Mouse.Wheel(5,6,down,1)
        Paint('wheel down selects Dan')
        CheckHash(0x1CED26CEA2A684CD)
        Mouse.Wheel(5,6,up,2)
        Paint('wheel up twice selects Bogdan')
        CheckHash(0xE89B884FCF409A2D)
        Key.Pressed(Enter)
        Paint('Enter inserts the item chosen with the mouse, Bogdan')
        CheckHash(0x5400D7C553020D55)
        CheckCursor(9,2)
        Key.TypeText(' @')
        Mouse.Click(12,4,left)
        Paint('clicking Ana inserts it')
        CheckHash(0x6BD7F55B5FF71BEF)
        CheckCursor(14,2)
        Key.TypeText(' @')
        Mouse.Click(40,3,left)
        Paint('a click in the text outside the list closes it and moves the cursor')
        CheckHash(0x46F8F4A53D43A28F)
        CheckCursor(16,2)
        Key.TypeText(' @')
        Mouse.Click(5,9,left)
        Paint('focus moved to the second composer, the list is closed')
        CheckHash(0x95B5C0908F1AA7AF)
        CheckCursor(2,9)
        Mouse.Click(5,2,left)
        Key.Pressed(Ctrl+A)
        Key.Pressed(Ctrl+C)
        CheckClipboardText('@Bogdan @Ana @ @')
    ";
    App::new()
        .size(Size::new(60, 12))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:58,h:10");
            w.add(MarkdownComposer::new(layout!("x:0,y:7,w:56,h:1"), Flags::None));
            w.add(MarkdownComposer::new(layout!("x:0,y:0,w:56,h:7"), Flags::None).with_list(
                '@',
                &["Ana", "Bogdan", "Cristina", "Dan", "Elena"],
                ListFlags::None,
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_list_popup_outside_the_window_below() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.TypeText('one')
        Key.Pressed(Enter)
        Key.TypeText('two @')
        Paint('the composer is taller than the window, the list opens below the window')
        CheckHash(0x42D160E3AAD38289)
        CheckCursor(7,3)
        Key.TypeText('a')
        Paint('still outside the window while filtering')
        CheckHash(0x4AD5F9F96DF44C94)
        Key.Pressed(Enter)
        Paint('Ana inserted, the list is gone and nothing is left drawn under the window')
        CheckHash(0x95308DF8D60CE4BA)
        CheckCursor(10,3)
    ";
    App::new()
        .size(Size::new(40, 14))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:30,h:4");
            w.add(MarkdownComposer::new(layout!("x:0,y:0,w:100%,h:10"), Flags::None).with_list(
                '@',
                &["Ana", "Bogdan", "Cristina", "Dan", "Elena"],
                ListFlags::None,
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_list_popup_outside_the_window_above() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.TypeText('one')
        Key.Pressed(Enter)
        Key.TypeText('two')
        Key.Pressed(Enter)
        Key.TypeText('x @')
        Paint('no room below the window, the list opens above it and the text stays visible')
        CheckHash(0xE31635D78D6FFB15)
        CheckCursor(5,8)
        Key.Pressed(Down)
        Key.Pressed(Enter)
        Paint('Bogdan inserted, the window looks as before')
        CheckHash(0xB4D1411C3FF0C851)
        CheckCursor(11,8)
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:5,w:30,h:5");
            w.add(MarkdownComposer::new(layout!("x:0,y:0,w:100%,h:100%"), Flags::None).with_list(
                '@',
                &["Ana", "Bogdan", "Cristina", "Dan", "Elena"],
                ListFlags::None,
            ));
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_list_editing() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.TypeText('@')
        Paint('the list was edited, Ana is gone and eu was added')
        CheckHash(0x665766097969D2A4)
        Key.Pressed(Down,2)
        Key.Pressed(Enter)
        Paint('the value of eu was inserted and the trigger removed by the new flags')
        CheckHash(0x5527B2EA5838B500)
        CheckCursor(5,2)
        Key.TypeText(' #')
        Paint('the emptied list does not open')
        CheckHash(0x377922A0EA3B130B)
        CheckCursor(7,2)
    ";
    App::new()
        .size(Size::new(40, 10))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:8");
            let mut mc = MarkdownComposer::new(layout!("x:0,y:0,w:100%,h:100%"), Flags::None);
            mc.add_list('@', &["Ana", "Bogdan"], ListFlags::None);
            mc.add_list_with_values('#', &[("bug", "🐛"), ("todo", "📝")], ListFlags::None);
            assert!(mc.list_mut('x').is_none());

            let list = mc.list_mut('@').unwrap();
            assert_eq!(list.trigger(), '@');
            assert!(list.flags() == ListFlags::None);
            assert_eq!(list.len(), 2);
            assert!(!list.is_empty());
            list.add("Cristina");
            list.add_value("eu", "@EU");
            list.remove(0);
            list.remove(100);
            list.set_flags(ListFlags::RemoveTrigger);
            assert_eq!(list.len(), 3);
            assert_eq!(list.items(), ["Bogdan".to_string(), "Cristina".to_string(), "eu".to_string()]);
            assert_eq!(list.item(0), Some("Bogdan"));
            assert_eq!(list.item(2), Some("eu"));
            assert_eq!(list.item(3), None);
            assert_eq!(list.value(0), Some("Bogdan"));
            assert_eq!(list.value(2), Some("@EU"));
            assert_eq!(list.value(3), None);
            assert!(list.flags() == ListFlags::RemoveTrigger);

            let list = mc.list_mut('#').unwrap();
            assert_eq!(list.trigger(), '#');
            assert_eq!(list.value(1), Some("📝"));
            list.clear();
            assert!(list.is_empty());
            assert_eq!(list.len(), 0);
            assert_eq!(list.item(0), None);
            assert_eq!(list.value(0), None);

            assert!(mc.list('#').unwrap().is_empty());
            assert_eq!(mc.list('@').unwrap().len(), 3);
            w.add(mc);
            w
        })
        .run()
        .unwrap();
}

#[test]
fn check_validate_event() {
    #[Window(events = MarkdownComposerEvents, internal = true)]
    struct MyWin {
        editable: Handle<MarkdownComposer>,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title,x:1,y:1,w:38,h:8"),
                editable: Handle::None,
            };
            w.add(MarkdownComposer::from("read only", layout!("x:0,y:4,w:100%,h:2"), Flags::ReadOnly));
            w.editable = w.add(MarkdownComposer::new(layout!("x:0,y:0,w:100%,h:4"), Flags::None).with_list('@', &["Ana", "Bogdan"], ListFlags::None));
            w
        }
    }
    impl MarkdownComposerEvents for MyWin {
        fn on_validate(&mut self, handle: Handle<MarkdownComposer>, text: &str) -> EventProcessStatus {
            let title = format!("valid:{}", text.replace('\n', "|"));
            self.base.set_title(&title);
            if handle == self.editable {
                let handle = self.editable;
                if let Some(mc) = self.control_mut(handle) {
                    mc.set_text("");
                }
            }
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.TypeText('hi')
        Key.Pressed(Ctrl+Enter)
        Paint('title reads valid:hi and the handler emptied the composer')
        CheckHash(0xED22832F965DFCC8)
        CheckCursor(2,2)
        Key.TypeText('a @')
        Key.Pressed(Ctrl+Enter)
        Paint('the list is closed by the validation, nothing is left drawn')
        CheckHash(0xE0D0AF19F1E39A27)
        CheckCursor(2,2)
        Key.Pressed(Ctrl+Enter)
        Paint('an empty text is validated as well, title reads valid:')
        CheckHash(0x68F2464C26D3F989)
        Key.Pressed(Tab)
        Key.Pressed(Ctrl+Enter)
        Paint('a read only composer validates its own text')
        CheckHash(0xD03199055ABB88E4)
        CheckCursor(2,6)
    ";
    App::new().size(Size::new(40, 10)).debug_script(script).window(MyWin::new).run().unwrap();
}

#[test]
fn check_events_default_handlers() {
    #[Window(events = MarkdownComposerEvents, internal = true)]
    struct MyWin {
        changes: u32,
    }
    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Title,x:1,y:1,w:38,h:6"),
                changes: 0,
            };
            w.add(MarkdownComposer::new(layout!("x:0,y:0,w:100%,h:100%"), Flags::None));
            w
        }
    }
    impl MarkdownComposerEvents for MyWin {
        fn on_text_changed(&mut self, _handle: Handle<MarkdownComposer>) -> EventProcessStatus {
            self.changes += 1;
            let title = format!("changes={}", self.changes);
            self.base.set_title(&title);
            EventProcessStatus::Processed
        }
    }
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Key.TypeText('ab')
        Key.Pressed(Ctrl+Enter)
        Paint('the window does not handle the validation, the title still reads changes=2')
        CheckHash(0xC92CF0BE6D632FB1)
        CheckCursor(4,2)
    ";
    App::new().size(Size::new(40, 8)).debug_script(script).window(MyWin::new).run().unwrap();
}

#[test]
fn check_code_block_edge_cases() {
    let script = "
        Paint.Enable(false)
        //Error.Disable(true)
        Paint('fences on the same line are inline code, glued blocks get one frame each')
        CheckHash(0x140629E714C6AF94)
        CheckCursor(2,10)
        Key.Pressed(Tab)
        Key.Pressed(Ctrl+A)
        Key.TypeText('z')
        Paint('the whole text is gone, no markers and no fences are left behind')
        CheckHash(0x6949F3228AABD77A)
        CheckCursor(3,2)
        Key.Pressed(Ctrl+A)
        Key.Pressed(Ctrl+C)
        CheckClipboardText('z')
        Key.Pressed(Tab)
        Key.Pressed(End)
        CheckCursor(7,7)
    ";
    App::new()
        .size(Size::new(40, 18))
        .debug_script(script)
        .window(|| {
            let mut w = window!("Title,x:1,y:1,w:38,h:16");
            w.add(MarkdownComposer::from(
                "a **b** `c`\n```\nx\n```\nd",
                layout!("x:0,y:0,w:36,h:5"),
                Flags::None,
            ));
            w.add(MarkdownComposer::from(
                "``` one ```\n``` two ```\nend",
                layout!("x:0,y:5,w:36,h:3"),
                Flags::None,
            ));
            w.add(MarkdownComposer::from(
                "```\na\n```\n```\nb\n```",
                layout!("x:0,y:8,w:36,h:6"),
                Flags::None,
            ));
            w
        })
        .run()
        .unwrap();
}
