use crate::prelude::*;

#[test]
fn check_move_left_right_1() {
    let script = "
        
        // Paint.Enable(false)
        
        // pressed Right arrow 4 times
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)

        Paint('Cursor on space')
        CheckHash(0xBBC20CDC60026848)
        CheckCursor(9,0)
    ";

    let text_print = "Unit Test 1";    
    let textarea = TextArea::new(&text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 1", Layout::new("d:c,w:52,h:12"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();

}

#[test]
fn check_move_left_right_2() {
    let script = "
        
        // Paint.Enable(false)

        // pressed Left arrow 3 times, this should not change the cursor
        Key.Pressed(Left)
        Key.Pressed(Left)
        Key.Pressed(Left)
        
        // pressed Right arrow 4 times
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)

        Paint('Cursor on space')
        CheckHash(0xC011384B78F5493B)
        CheckCursor(9,0)
    ";

    let text_print = "Unit Test 2";    
    let textarea = TextArea::new(&text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 2", Layout::new("d:c,w:52,h:12"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_left_right_3() {
    let script = "
        
        // Paint.Enable(false)

        // pressed Left arrow 3 times, this should not change the cursor
        Key.Pressed(Left)
        Key.Pressed(Left)
        Key.Pressed(Left)
        
        // pressed Right arrow 4 times
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)

        // pressed Left arrow 3 times
        Key.Pressed(Left)
        Key.Pressed(Left)
        Key.Pressed(Left)

        // pressed Right arrow 2 times
        Key.Pressed(Right)
        Key.Pressed(Right)

        Paint('Cursor on t')
        CheckHash(0xA646FF019E663E2A)
        CheckCursor(8,0)
    ";

    let text_print = "Unit Test 3";    
    let textarea = TextArea::new(&text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 3", Layout::new("d:c,w:52,h:12"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_newline_parsing_move_right_next_line() {
    let script = "
        
        // Paint.Enable(false)

        // pressed Right arrow 4 times
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)
        Key.Pressed(Right)

        Paint('Text with newlines, cursor on N')
        CheckHash(0x109F989BAC9496A5)
        CheckCursor(5, 1)
    ";

    let text_print = "Unit Test 4\nNewline test";    
    let textarea = TextArea::new(&text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 4", Layout::new("d:c,w:52,h:12"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_newline_parsing() {
    let script = "
        
        // Paint.Enable(false)

        Paint('Text with newlines')
        CheckHash(0x109F989BAC9496A5)
    ";

    let text_print = "Unit Test 4\nNewline test";    
    let textarea = TextArea::new(&text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 4", Layout::new("d:c,w:52,h:12"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}