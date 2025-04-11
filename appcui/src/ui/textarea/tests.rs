use crate::prelude::*;

#[test]
fn check_move_left_right_1() {
    let script = "
        
        
        Paint.Enable(false)

        Error.Disable(true)
        
        Paint('Initial State')
        CheckHash(0xA7E1DBC20D842008)
        CheckCursor(1, 1)

        // pressed Right arrow 4 times
        Key.Pressed(Right, 4)

        Paint('Cursor on space')
        CheckHash(0xA7E1DBC20D842008)
        CheckCursor(5, 1)
    ";

    let text_print = "Unit Test 1";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 1", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();

}

#[test]
fn check_move_left_right_2() {
    let script = "
        
        Paint.Enable(false)

        // pressed Left arrow 3 times, this should not change the cursor
        Key.Pressed(Left, 3)
        
        // pressed Right arrow 4 times
        Key.Pressed(Right, 4)

        Paint('Cursor on space')
        CheckHash(0x4F5508D58B910808)
        CheckCursor(9, 1)
    ";

    let text_print = "Unit Test 2";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 2", Layout::new("d:c,w:52,h:10"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_left_right_3() {
    let script = "
        
        Paint.Enable(false)

        // pressed Left arrow 3 times, this should not change the cursor
        Key.Pressed(Left, 3)
        
        // pressed Right arrow 4 times
        Key.Pressed(Right, 4)

        // pressed Left arrow 3 times
        Key.Pressed(Left, 3)

        // pressed Right arrow 2 times
        Key.Pressed(Right, 2)

        Paint('Cursor on t')
        CheckHash(0x4114051D9BA975C9)
        CheckCursor(8,0)
    ";

    let text_print = "Unit Test 3";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 3", Layout::new("d:c,w:52,h:12"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_newline_parsing_move_right_next_line() {
    let script = "
        
        Paint.Enable(false)

        // pressed Right arrow 4 times
        Key.Pressed(Right, 12)

        Paint('Text with newlines, cursor on N')
        CheckHash(0x27515333FFC2FBA2)
        CheckCursor(5, 1)
    ";

    let text_print = "Unit Test 4\nNewline test";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 4", Layout::new("d:c,w:52,h:12"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_newline_parsing() {
    let script = "
        
        Paint.Enable(false)

        Paint('Text with newlines')
        CheckHash(0x27515333FFC2FBA2)
    ";

    let text_print = "Unit Test 4\nNewline test";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 4", Layout::new("d:c,w:52,h:12"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_newline_parsing_move_right_next_line_left_move_back() {
    let script = "
        
        Paint.Enable(false)

        // pressed Right arrow 12 times
        Key.Pressed(Right, 12)

        Key.Pressed(Left, 4)

        Paint('Text with newlines, cursor on t')
        CheckHash(0x27515333FFC2FBA2)
        CheckCursor(13, 0)
    ";

    let text_print = "Unit Test 4\nNewline test";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 4", Layout::new("d:c,w:52,h:12"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_1() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Shift+Right, 4)

        Paint('Selection 1')
        CheckHash(0x9B960662CA00857C)
        CheckCursor(5, 1)
    ";

    let text_print = "Unit Test Selection 1";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection 1", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_2() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 3)
        Key.Pressed(Shift+Right, 6)

        Key.Pressed(Shift+Left, 3)

        Paint('Selection 2')
        CheckHash(0xA0DC8380AD0B4EA3)
        CheckCursor(7, 1)
    ";

    let text_print = "Unit Test Selection 3";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection 3", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_3() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 3)
        Key.Pressed(Shift+Right, 6)

        Key.Pressed(Shift+Left, 8)

        Paint('Selection 3')
        CheckHash(0xF020E9F24E406958)
        CheckCursor(2, 1)
    ";

    let text_print = "Unit Test Selection 3";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection 3", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_4_copy() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 3)
        Key.Pressed(Shift+Right, 6)

        Key.Pressed(Shift+Left, 8)

        Paint('Selection 4')
        CheckHash(0xA72E33B3E94A4838)
        CheckCursor(2, 1)

        Key.Pressed(Ctrl+C)
        CheckClipboardText('ni')
    ";

    let text_print = "Unit Test Selection 4";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection 4", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_5_delete() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 4)
        Key.Pressed(Shift+Right, 6)

        Key.Pressed(Backspace)

        Paint('Selection 5')
        CheckHash(0x77162F75F8756D8)
        CheckCursor(5, 1)
    ";

    let text_print = "Unitzzzzzz Test Selection 5";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection 5", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_7_mouse_backspace() {
    let script = "
        
        Paint.Enable(false)

        Mouse.Drag(11,1,15,1)
        Paint('Before Backspace')

        Key.Pressed(Backspace)
        Paint('After Backspace')

        CheckHash(0x92C8C8286CABDAB0)
        CheckCursor(11, 1)
    ";

    let text_print = "Unit Test zzzzSelection 7 - Mouse";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection 7", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_8_mouse() {
    let script = "
        
        Paint.Enable(false)

        Mouse.Drag(11,1,3,2)

        Paint('Selection 8')
        CheckHash(0x2CCE8D8E09989690)
        CheckCursor(3, 2)
    ";

    let text_print = "Unit Test zz\nzzSelection 8 - Mouse";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection 8", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_shift_1() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 3)
        Key.Pressed(Shift+Down, 2)

        Key.Pressed(Shift+Left, 1)

        Paint('Selection 8')
        CheckHash(0xF993DA08BB70241D)
        CheckCursor(3, 3)
    ";

    let text_print = "Unit Test Selection 8\nMultiline Selection\nUsing Shift Key";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection 8", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_shift_2() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 3)
        Key.Pressed(Shift+Right, 3)
        Paint('Selection After Shift+Right')

        Key.Pressed(Shift+Down, 3)
        Paint('Selection After Shift+Down')


        Key.Pressed(Shift+Left, 1)
        Paint('Selection After Shift+Left')

        Key.Pressed(Shift+Up, 1)
        Paint('Selection After Shift+Up')

        CheckHash(0x67306E884E6584E6)
        CheckCursor(15, 2)
    ";

    let text_print = "Unit Test Selection Shift 2\nMultiline Selection\nUsing Shift Key";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection Shift 2", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_shift_3() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 5)
        Key.Pressed(Shift+Right, 3)
        Key.Pressed(Shift+Down, 3)

        Paint('Selection Shift 3.1')

        Key.Pressed(Shift+Left, 6)
        Key.Pressed(Shift+Up, 2)

        Paint('Selection Shift 3.2')
        CheckHash(0x2983C948E70A1252)

        Key.Pressed(Ctrl+C)
        CheckClipboardText('Test')

        CheckCursor(10, 1)
    ";

    let text_print = "Unit Test Selection Shift 3\nMultiline Selection\nUsing Shift Key";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection Shift 3", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_multi_line() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 5)
        Key.Pressed(Shift+Right, 24)

        Paint('Selection Multi-Line')
        CheckHash(0x8D7285F22873A690)
        CheckCursor(10, 2)
    ";

    let text_print = "Unit Test Selection\nMultiline Selection\nUsing Shift Key";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection Multi-Line", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_paste_1() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 3)
        Key.Pressed(Shift+Right, 6)

        Key.Pressed(Shift+Left, 8)

        Paint('Selection 4')
        CheckHash(0xA72E33B3E94A4838)
        CheckCursor(2, 1)

        Key.Pressed(Ctrl+C)
        CheckClipboardText('ni')
    ";

    let text_print = "Unit Test Selection 4";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection 4", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_paste_2() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 4)
        Paint('Before Paste')

        Clipboard.SetText(' Test\\nPaste ')
        Key.Pressed(Ctrl+V)

        Paint('Paste 2')
        CheckHash(0x122326FBF473D824)
        CheckCursor(7, 2)
    ";

    let text_print = "Unit2";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Paste 2", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_paste_3() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 4)

        Paint('Paste Test No Newline before Paste')

        Clipboard.SetText(' Test Paste')
        Key.Pressed(Ctrl+V)

        Paint('Paste Test No Newline')
        CheckHash(0x7637BD61981C32F3)
        CheckCursor(16, 1)
    ";

    let text_print = "Unit";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Paste 3", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}


#[test]
fn check_scrolling_1() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 65)

        Paint('Scrolling 1')
        CheckHash(0x895AF80E2463D0D0)
        CheckCursor(58, 1)
    ";

    let text_print = "Unit Test Scrolling, where I try to scroll as much as possible outside the displayed area\nof the text, to see test if the whole display moves";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Scrolling 1", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_scrolling_2() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 65)
        Key.Pressed(Left, 10)

        Paint('Scrolling 2')
        CheckHash(0xA26661F3D512433)
        CheckCursor(48, 1)
    ";

    let text_print = "Unit Test Scrolling, where I try to scroll as much as possible outside the displayed area\nof the text, to see test if the whole display moves";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Scrolling 2", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_scrolling_3() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 65)
        Paint('After Right Moves')

        Key.Pressed(Down, 1)
        Paint('After Down Moves')

        Key.Pressed(Left, 1)
        Paint('After Left Move')

        CheckHash(0x487B386668755E06)
        CheckCursor(43, 2)
    ";

    let text_print = "Unit Test Scrolling, where I try to scroll as much as possible outside the displayed area\nof the text, to see test if the whole display moves";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Scrolling 3", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();

    
}

#[test]
fn check_scrolling_4() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 75)
        Key.Pressed(Left, 70)

        Paint('Scrolling 4')
        CheckHash(0x50C85D3044D20B5C)
        CheckCursor(1, 1)
    ";

    let text_print = "Unit Test Scrolling, where I try to scroll as much as possible outside the displayed area\nof the text, to see test if the whole display moves";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Scrolling 4", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_scrolling_enter_1() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 10)
        Key.Pressed(Enter, 1)

        Paint('Unit Test Enter Key')
        CheckHash(0x15DF1159B8CBC33C)
        CheckCursor(1, 2)
    ";

    let text_print = "Unit Test Enter Key 1";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Enter Key 1", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_scrolling_backspace_1() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 10)

        Key.Pressed(Backspace, 1)

        Paint('Unit Test Enter Key')
        CheckHash(0x8B7F8B1872D06ADC)
        CheckCursor(10, 1)
    ";

    let text_print = "Unit Test\n Enter Key 2";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Enter Key 2", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_scrolling_backspace_2() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 11)

        Key.Pressed(Backspace, 2)

        Paint('Unit Test Enter Key')
        CheckHash(0x8B7F8B1872D06ADC)
        CheckCursor(10, 1)
    ";

    let text_print = "Unit Test \n Enter Key 2";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Enter Key 2", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_scrolling_delete_1() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 9)
        Paint('Unit Test before Delete newline')
        Key.Pressed(Delete, 2)
        Paint('Unit Test Delete newline')
        
        CheckHash(0x2BFA0D26576AE2D4)
        CheckCursor(10, 1)
    ";

    let text_print = "Unit Test \n Delete Key";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Delete", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_scrolling_delete_2() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 9)
        Paint('Unit Test before Delete newline')
        Key.Pressed(Delete, 2)
        Paint('Unit Test Delete newline')
        
        CheckHash(0x53FFA5BFCB0C00F0)
        CheckCursor(10, 1)
    ";

    let text_print = "Unit Test \n Delete Key\n\n\n\n\n\n\n\nTest 2";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Delete", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}


#[test]
fn check_line_number_1() {
    let script = "
        
        Paint.Enable(false)

        Paint('Text with newlines and line numbers, cursor on U')
        CheckHash(0x30F87B74D74AE698)
        CheckCursor(3, 1)
    ";

    let text_print = "Unit Test 5\nLineNumber test";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::ShowLineNumber);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 5", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_up_down_1() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Down, 10)

        Paint('Cursor on Scrolling')
        CheckHash(0xE697965D60DC9F74)
        CheckCursor(1, 9)
    ";

    let text_print = "Unit\n...\n...\n...\n...\nTest\n...\n...\n...\n...\nScrolling\n...\n...\n...\n...\nDown";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Scrolling", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_up_down_2() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Down, 10)
        Key.Pressed(Up, 5)

        Paint('Cursor on Test')
        CheckHash(0xE697965D60DC9F74)
        CheckCursor(1, 4)
    ";

    let text_print = "Unit\n...\n...\n...\n...\nTest\n...\n...\n...\n...\nScrolling\n...\n...\n...\n...\nDown";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Scrolling", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_up_down_3() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Down, 15)
        Key.Pressed(Up, 10)

        Paint('Cursor on Test')
        CheckHash(0x26DEF31B780D274C)
        CheckCursor(1, 1)
    ";

    let text_print = "Unit\n...\n...\n...\n...\nTest\n...\n...\n...\n...\nScrolling\n...\n...\n...\n...\nDown";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Scrolling", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_up_down_4() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Down, 2)
        Key.Pressed(PageDown, 1)

        Paint('Cursor on Scrolling')
        CheckHash(0xE697965D60DC9F74)
        CheckCursor(1, 9)
    ";

    let text_print = "Unit\n...\n...\n...\n...\nTest\n...\n...\n...\n...\nScrolling\n...\n...\n...\n...\nDown";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Scrolling", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_up_down_5() {
    let script = "
        
        //Paint.Enable(false)

        Key.Pressed(PageDown, 2)

        Paint('Cursor on Down')
        CheckHash(0x2AFAEA882ADB0DD0)
        CheckCursor(1, 9)
    ";

    let text_print = "Unit\n...\n...\n...\n...\nTest\n...\n...\n...\n...\nScrolling\n...\n...\n...\n...\n...\nDown";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Scrolling", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_up_down_6() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(PageDown, 2)

        Paint('Cursor on Down')
        CheckHash(0xF931EEE2A34103F8)

        Key.Pressed(Up, 2)
        Key.Pressed(PageUp, 1)

        Paint('Cursor on Test')
        CheckHash(0x26DEF31B780D274C)

        CheckCursor(1, 1)
    ";

    let text_print = "Unit\n...\n...\n...\n...\n...\nTest\n...\n...\n...\n...\nScrolling\n...\n...\n...\n...\nDown";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Scrolling", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_up_down_7() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Down, 2)
        Key.Pressed(PageDown, 3)

        Paint('Cursor on Down')
        CheckHash(0xF931EEE2A34103F8)

        Key.Pressed(Up, 2)
        Key.Pressed(PageUp, 3)

        Paint('Cursor on Unit')
        CheckHash(0x4453FC08BE22CFE7)

        CheckCursor(1, 1)
    ";

    let text_print = "Unit\n...\n...\n...\n...\nTest\n...\n...\n...\n...\nScrolling\n...\n...\n...\n...\nDown";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Scrolling", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_write_1() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 5)

        Key.Pressed(T)
        Key.Pressed(E)
        Key.Pressed(S)
        Key.Pressed(T)
        Key.Pressed(Enter)
        Key.Pressed(W)
        Key.Pressed(R)
        Key.Pressed(I)
        Key.Pressed(T)
        Key.Pressed(E)

        Paint('Text Write')
        CheckHash(0x27E9105FE0C5638)
        CheckCursor(6, 2)
    ";

    let text_print = "Unit ";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Paste 2", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_write_2() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Right, 4)

        Key.TypeText(' Test')
        Key.Pressed(Enter)
        Key.TypeText('Write')

        Paint('Text Write')
        CheckHash(0xC40F747070F53B2A)
        CheckCursor(6, 2)
    ";

    let text_print = "Unit 2";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Paste 2", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_drag_1() {
    let script = "
        
        // Paint.Enable(false)

        Mouse.Hold(9,1,left)
        Mouse.Move(3,2)

        Paint('Mouse Drag 1')

        Mouse.Move(6,1)

        Paint('Mouse Drag 2')

        Mouse.Release(6,1,left)

        Key.Pressed(Ctrl+C)
        CheckClipboardText('Tes')

        CheckHash(0xD5F8DC8568E16C25)
        CheckCursor(6, 1)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}


#[test]
fn check_mouse_drag_2() {
    let script = "
        
        // Paint.Enable(false)

        Mouse.Drag(3,2,9,1)

        Paint('Mouse Drag')

        // Key.Pressed(Ctrl+C)
        // CheckClipboardText('t\\nMou')

        CheckHash(0xA02D97F65F1989C9)
        CheckCursor(9, 1)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_drag_3() {
    let script = "
        
        // Paint.Enable(false)

        Mouse.Drag(1,1,7,1)
        Paint('Mouse Drag')

        Key.Pressed(Delete)
        Paint('After Delete')

        CheckHash(0xAB1C34EA8C6AA8F8)
        CheckCursor(1, 1)
    ";

    let text_print = " Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}


#[test]
fn check_selection_down_left_1() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 4)        
        Key.Pressed(Down, 3)
        Paint('After Before Shift+Left')

        Key.Pressed(Shift+Left, 20)        
        Paint('Before Shift+Down')

        Key.Pressed(Shift+Down, 3)        
        Paint('After Shift+Down')

        CheckHash(0x8637EADB203A8AFA)
        CheckCursor(8, 5)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_down_left_2() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 4)        
        Key.Pressed(Down, 3)
        Paint('After Before Shift+Left')

        Key.Pressed(Shift+Left, 25)        
        Paint('Before Shift+Down')

        Key.Pressed(Shift+Down, 2)        
        Paint('After Shift+Down')

        Key.Pressed(Ctrl+C)
        CheckClipboardText('ud')

        CheckHash(0x7EAD0B31C0DDA26E)
        CheckCursor(3, 4)
    ";
  
    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_down_left_3() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 5)        
        Key.Pressed(Down, 4)      
        Paint('Before Shift+Down')

        Key.Pressed(Shift+Down, 2)        
        Paint('After Shift+Down')

        Key.Pressed(Ctrl+C)
        CheckClipboardText('Cargo et Rust')

        CheckHash(0xA3B9684803AAD769)
        CheckCursor(19, 5)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn test_scrollbar_1() {
    let script = "
        
        // Paint.Enable(false)
   
        Paint('Initial View')

        Mouse.Click(59, 9, left)

        Paint('View After 1 Click Down')

        CheckHash(0x7B432D113EA7EFF4)
        CheckCursor(1, 1)

        Mouse.Click(59, 1, left)

        Paint('View After 1 Click Up')

        CheckHash(0xA83691DB58C957EC)
        CheckCursor(1, 2)
    ";

    let text_print = "Laus Cargo et Rust\n tipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\nUt enim ad minim veniam, quis nostrud exercitation ullamco laboris.\nNisi ut aliquip ex ea commodo consequat. Duis aute irure dolor.\nIn reprehenderit in voluptate velit esse cillum dolore eu fugiat.\nNulla pariatur. Excepteur sint occaecat cupidatat non proident.\nSunt in culpa qui officia deserunt mollit anim id est laborum.\nPraesent malesuada eros ut felis efficitur, vitae tincidunt velit.\nCurabitur nec nisl a odio euismod fringilla non nec risus.\nMorbi sit amet nulla ac nisi faucibus tempus sit amet a justo.";
    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::ScrollBars);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag Scrollbar", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn test_scrollbar_2() {
    let script = "
        
        // Paint.Enable(false)
   
        Paint('Initial View')

        Mouse.Click(57, 10, left)
        Mouse.Click(57, 10, left)
        Mouse.Click(57, 10, left)

        Paint('View After 3 Clicks Right')

        CheckHash(0x41F9155618775A5B)
        CheckCursor(1, 1)

        Mouse.Click(1, 10, left)
        Mouse.Click(1, 10, left)

        Paint('View After 2 Clicks Left')

        CheckHash(0x97613F6F9E73A50)
        CheckCursor(3, 1)
    ";

    let text_print = "Laus Cargo et Rust\n tipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\nUt enim ad minim veniam, quis nostrud exercitation ullamco laboris.\nNisi ut aliquip ex ea commodo consequat. Duis aute irure dolor.\nIn reprehenderit in voluptate velit esse cillum dolore eu fugiat.\nNulla pariatur. Excepteur sint occaecat cupidatat non proident.\nSunt in culpa qui officia deserunt mollit anim id est laborum.\nPraesent malesuada eros ut felis efficitur, vitae tincidunt velit.\nCurabitur nec nisl a odio euismod fringilla non nec risus.\nMorbi sit amet nulla ac nisi faucibus tempus sit amet a justo.";
    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::ScrollBars);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag Scrollbar", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn test_scrollbar_3() {
    let script = "
        
        // Paint.Enable(false)
   
        Paint('Initial View')

        Key.Pressed(Down)
        Key.Pressed(Right, 70)

        Paint('View After Some Right Moves')

        Mouse.Click(57, 10, left)
        Mouse.Click(57, 10, left)
        Mouse.Click(57, 10, left)
        Mouse.Click(57, 10, left)

        Paint('View After 4 Click Rights')

        CheckHash(0xDB65790755BAFE5)
        CheckCursor(54, 2)

        Mouse.Click(1, 10, left)
        Mouse.Click(1, 10, left)

        Paint('View After 2 Clicks Left')

        CheckHash(0xE1C77CA3A506A0EA)
        CheckCursor(56, 2)
    ";

    let text_print = "Laus Cargo et Rust\n tipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\nUt enim ad minim veniam, quis nostrud exercitation ullamco laboris.\nNisi ut aliquip ex ea commodo consequat. Duis aute irure dolor.\nIn reprehenderit in voluptate velit esse cillum dolore eu fugiat.\nNulla pariatur. Excepteur sint occaecat cupidatat non proident.\nSunt in culpa qui officia deserunt mollit anim id est laborum.\nPraesent malesuada eros ut felis efficitur, vitae tincidunt velit.\nCurabitur nec nisl a odio euismod fringilla non nec risus.\nMorbi sit amet nulla ac nisi faucibus tempus sit amet a justo.";
    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::ScrollBars);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag Scrollbar", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_drag_delete() {
    let script = "
        
        // Paint.Enable(false)

        Mouse.Drag(6,2,5,1)

        Paint('Mouse Drag')

        Key.Pressed(Backspace)

        Paint('After Backspace')

        CheckHash(0xF670818DB593C749)
        CheckCursor(5, 1)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_drag_4() {
    let script = "
        
        // Paint.Enable(false)

        Mouse.Hold(6, 2, left)
        Mouse.Move(19, 5)

        Paint('Mouse Drag 1')

        Mouse.Move(4, 5)
        Mouse.Move(4, 2)

        Mouse.Release(4, 2, left)

        Paint('Mouse Drag')

        Key.Pressed(Backspace)

        Paint('After Backspace')

        CheckHash(0xCDA423FD5EBA7168)
        CheckCursor(4, 2)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_copy_unicode() {
    let script = "
        
        Paint.Enable(false)

        Key.Pressed(Down, 1)
        Key.Pressed(Right, 2)
        Key.Pressed(Shift+Right, 8)

        Paint('Selection Unicode')
        CheckHash(0x4CFC4AF356572440)
        CheckCursor(11, 2)

        Key.Pressed(Ctrl+C)
        CheckClipboardText('♡ love ♡')
    ";

    let text_print = "Unit Test Selection.\nI ♡ love ♡ Unicode";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection 4", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn check_selection_7_mouse_backspace_2() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Left, 3)

        Mouse.Drag(13,1,17,1)
        Paint('Before Backspace')

        Key.Pressed(Backspace)
        Paint('After Backspace')

        CheckHash(0xEE8927570D0DE206)
        CheckCursor(13, 1)
    ";

    let text_print = "Unit Test zzzzSelection 7 - Mouse";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::ShowLineNumber);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Selection 7", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn show_line_number_increase() {
    let script = "
        // Paint.Enable(false)

        Paint('Initial State')
        CheckHash(0x4E9A5E71C0DB16C1)
        CheckCursor(3, 1)

        Key.Pressed(Right, 4)

        Paint('Cursor on space')
        CheckCursor(7, 1)

        Key.Pressed(Enter)

        Paint('Cursor on space, new line')
        CheckHash(0xA96EF9168B8DFBB7)
        CheckCursor(4, 2)
    ";

    let text_print = "Unit Test\n\n\n\n\n\n\n\nShowLineNumber";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::ShowLineNumber);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 1", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn show_line_number_decrease() {
    let script = "
        // Paint.Enable(false)

        Paint('Initial State')
        CheckHash(0x601AF9C95E6A93B7)
        CheckCursor(4, 1)

        Key.Pressed(Down, 1)

        CheckCursor(4, 2)

        Key.Pressed(Backspace)

        Paint('Cursor at the end of the line')
        CheckHash(0x4E9A5E71C0DB16C1)
        CheckCursor(12, 1)
    ";

    let text_print = "Unit Test\n\n\n\n\n\n\n\n\nShowLineNumber";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::ShowLineNumber);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test 1", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_right() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Right, 3)

        Paint('After Move to end of Test')

        CheckHash(0x57840FC2539DCB6E)
        CheckCursor(10, 1)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_right_left() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Right, 3)
        Key.Pressed(Ctrl+Left, 1)

        Paint('After Move to beginning of Test')

        CheckHash(0x57840FC2539DCB6E)
        CheckCursor(6, 1)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_shift_right() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Shift+Right, 3)

        Paint('After Move to end of Test')

        CheckHash(0xC20834940D3C8385)
        CheckCursor(10, 1)

        Key.Pressed(Ctrl+C)
        CheckClipboardText('Unit Test')
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_shift_right_shift_left() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Shift+Right, 3)
        Key.Pressed(Ctrl+Shift+Left, 1)

        Paint('After Move to beginning of Test')

        CheckHash(0xA4F9F46A799AAF21)
        CheckCursor(6, 1)

        Key.Pressed(Ctrl+C)
        CheckClipboardText('Unit ')
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_right_shift_left() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Right, 3)
        Key.Pressed(Ctrl+Shift+Left, 1)

        Paint('After Move to beginning of Test')

        CheckHash(0x5567B86FCDC7C972)
        CheckCursor(6, 1)

        Key.Pressed(Ctrl+C)
        CheckClipboardText('Test')
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}


#[test]
fn ctrl_shift_left_shift_right() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Right, 3)
        Paint('After Move to end of Test')
        Key.Pressed(Ctrl+Shift+Left, 2)
        Paint('After Move to end of Unit')
        Key.Pressed(Ctrl+Shift+Right, 1)

        Paint('After Move to beginning of Test')

        CheckHash(0x5567B86FCDC7C972)
        CheckCursor(6, 1)

        Key.Pressed(Ctrl+C)
        CheckClipboardText('Test')
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_delete() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Delete, 2)

        Paint('After Delete')

        CheckHash(0xAB1C34EA8C6AA8F8)
        CheckCursor(1, 1)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_right_ctrl_delete_1() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Shift+Right, 1)
        Key.Pressed(Ctrl+Delete, 2)

        Paint('After Delete')

        CheckHash(0xAB1C34EA8C6AA8F8)
        CheckCursor(1, 1)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_right_ctrl_delete_2() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Shift+Right, 2)
        Key.Pressed(Ctrl+Delete, 1)

        Paint('After Delete')

        CheckHash(0xAB1C34EA8C6AA8F8)
        CheckCursor(1, 1)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_backspace() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Right, 2)
        Key.Pressed(Ctrl+Backspace, 2)

        Paint('After Delete')

        CheckHash(0xAB1C34EA8C6AA8F8)
        CheckCursor(1, 1)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_right_ctrl_backspace_1() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Shift+Right, 2)
        Paint('After Delete')

        Key.Pressed(Ctrl+Backspace, 1)

        Paint('After Delete')

        CheckHash(0xAB1C34EA8C6AA8F8)
        CheckCursor(1, 1)
    ";

    let text_print = "Unit Test\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_selection_unicode_1() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Right, 2)
        Key.Pressed(Ctrl+Shift+Right, 1)

        Paint('Before Delete')
        CheckHash(0x7CEB5463C45970B4)

        Key.Pressed(Backspace, 1)

        Paint('After Delete')

        CheckHash(0x2ECEFCF1AAE2C608)
        CheckCursor(6, 1)
    ";

    let text_print = "Unit Tȅst\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_selection_unicode_2() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Right, 3)
        Key.Pressed(Ctrl+Shift+Left, 1)

        Paint('Before Delete')
        CheckHash(0x7CEB5463C45970B4)

        Key.Pressed(Backspace, 1)

        Paint('After Delete')

        CheckHash(0x2ECEFCF1AAE2C608)
        CheckCursor(6, 1)
    ";

    let text_print = "Unit Tȅst\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_selection_unicode_3() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Right, 2)
        Key.Pressed(Ctrl+Shift+Right, 1)

        Paint('Before Delete')
        CheckHash(0xD8BDA97726B8B7A7)

        Key.Pressed(Backspace, 1)

        Paint('After Delete')

        CheckHash(0x2ECEFCF1AAE2C608)
        CheckCursor(6, 1)
    ";

    let text_print = "Unit Ꭲȅst\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_selection_unicode_4() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Right, 3)
        Key.Pressed(Ctrl+Shift+Left, 1)

        Paint('Before Delete')
        CheckHash(0xD8BDA97726B8B7A7)

        Key.Pressed(Backspace, 1)

        Paint('After Delete')

        CheckHash(0x2ECEFCF1AAE2C608)
        CheckCursor(6, 1)
    ";

    let text_print = "Unit Ꭲȅst\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_selection_unicode_5() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Right, 2)
        Key.Pressed(Ctrl+Shift+Right, 1)

        Paint('Before Delete')
        CheckHash(0x14830EC1D7487311)

        Key.Pressed(Backspace, 1)

        Paint('After Delete')

        CheckHash(0x2ECEFCF1AAE2C608)
        CheckCursor(6, 1)
    ";

    let text_print = "Unit Ꭲȅst𓀀\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_selection_unicode_6() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Ctrl+Right, 3)
        Key.Pressed(Ctrl+Shift+Left, 1)

        Paint('Before Delete')
        CheckHash(0x14830EC1D7487311)

        Key.Pressed(Backspace, 1)

        Paint('After Delete')

        CheckHash(0x2ECEFCF1AAE2C608)
        CheckCursor(6, 1)
    ";

    let text_print = "Unit Ꭲȅst𓀀\nMouse Drag\nLorem Ipsum\nLaudate Solem\nLaus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn ctrl_right_last_word() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 15)

        Paint('Before Ctrl+Right')
        CheckHash(0x5E5CD4BA23ACAD98)
        CheckCursor(16, 1)

        Key.Pressed(Ctrl+Right, 3)

        Paint('After Ctrl+Right')
        CheckHash(0x5E5CD4BA23ACAD98)
        CheckCursor(19, 1)
    ";

    let text_print = "Laus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn move_up_first_row() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 15)

        Paint('Before Up')
        CheckHash(0x5E5CD4BA23ACAD98)
        CheckCursor(16, 1)

        Key.Pressed(Up)

        Paint('After Up')
        CheckHash(0x5E5CD4BA23ACAD98)
        CheckCursor(1, 1)
    ";

    let text_print = "Laus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn move_down_last_row() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 15)

        Paint('Before Down')
        CheckHash(0x5E5CD4BA23ACAD98)
        CheckCursor(16, 1)

        Key.Pressed(Down)

        Paint('After Down')
        CheckHash(0x5E5CD4BA23ACAD98)
        CheckCursor(19, 1)
    ";

    let text_print = "Laus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn move_right_new_row_1() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Right, 18)

        Paint('Before Right')
        CheckHash(0x167D2194D362A3F4)
        CheckCursor(18, 1)

        Key.Pressed(Right, 2)

        Paint('After Right')
        CheckHash(0xC9B3FE9E35DFC75A)
        CheckCursor(1, 2)
    ";

    let text_print = "Laus Cargo et Rust,\nLingua fortis, codi fust,\nMem'riam sacram curat bene,\nNulla bug nec error plene.\n\nFerrum solidum, verbum clarum,\nNulla segfault, nil amarum,\nTypi fortes, vita laeta,\nNulla poena, nulla peta.\n\nConcurrentia sine metu,\nOwnership est sacrum fretu,\nIterum atque iterum scriptum,\nCompile-time servat victum.\n\nSic ad astra, sic ad gloriam,\nRust regnat in memoriam! 🚀🦀";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(20, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn move_right_new_row_2() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Down, 8)
        Key.Pressed(Right, 18)

        Paint('Before Right')
        CheckHash(0x167D2194D362A3F4)
        CheckCursor(18, 9)

        Key.Pressed(Right, 7)

        Paint('After Right')
        CheckHash(0x6C1FC2EDFEA2CCAA)
        CheckCursor(1, 9)
    ";

    let text_print = "Laus Cargo et Rust,\nLingua fortis, codi fust,\nMem'riam sacram curat bene,\nNulla bug nec error plene.\n\nFerrum solidum, verbum clarum,\nNulla segfault, nil amarum,\nTypi fortes, vita laeta,\nNulla poena, nulla peta.\nConcurrentia sine metu,\nOwnership est sacrum fretu,\nIterum atque iterum scriptum,\nCompile-time servat victum.\n\nSic ad astra, sic ad gloriam,\nRust regnat in memoriam! 🚀🦀";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(20, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn enter_text_longer_line() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Down)

        Key.Pressed(Enter)

        Key.TypeText('Lingua fortis, codi fust,')

        Key.Pressed(Enter)

        Key.TypeText('Memriam sacram curat bene,')

        Key.Pressed(Enter)

        Key.TypeText('Nulla bug nec error plene.')

        Paint('After Write')
        CheckHash(0xA21DBA7AFD486D8A)
        CheckCursor(27, 4)
    ";

    let text_print = "Laus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn delete_last_char() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Down)

        Key.Pressed(Delete, 10)

        Paint('After Delete')
        CheckHash(0x5E5CD4BA23ACAD98)
        CheckCursor(19, 1)
    ";

    let text_print = "Laus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}

#[test]
fn backspace_first_char() {
    let script = "
        
        // Paint.Enable(false)

        Key.Pressed(Backspace, 10)

        Paint('After Delete')
        CheckHash(0x5E5CD4BA23ACAD98)
        CheckCursor(1, 1)
    ";

    let text_print = "Laus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Unit Test Mouse Drag", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}


#[test]
fn out_ouf_bounds_mouse_click() {
    let script = "
        
        // Paint.Enable(false)

        Mouse.Click(5, 5, left)

        Paint('After Click')
        CheckHash(0xAFD43010ED21E34F)
        CheckCursor(19, 1)
    ";

    let text_print = "Laus Cargo et Rust";    
    let textarea = TextArea::new(text_print, Layout::new("d:c,h:100%,"), textarea::Flags::None);
    
    let mut a = App::debug(60, 11, script).build().unwrap();
    let mut w = Window::new("Issue #80 Mouse Click", Layout::new("d:c,w:100%,h:100%"), window::Flags::None);
    
    w.add(textarea);
    a.add_window(w);
    a.run();
}