use crate::backend::debug::check_clipboardtext_command::CheckClipboardTextCommand;
use crate::backend::debug::check_cursor_command::CheckCursorCommand;
use crate::backend::debug::check_hash_command::CheckHashCommand;
use crate::backend::debug::clipboard_clear_command::ClipboardClearCommand;
use crate::backend::debug::clipboard_settext_command::ClipboardSetTextCommand;
use crate::backend::debug::error_disable_command::ErrorDisableCommand;
use crate::backend::debug::keymodifier_command::KeyModifierCommand;
use crate::backend::debug::keypress_command::KeyPressedCommand;
use crate::backend::debug::keytypetext_command::KeyTypeTextCommand;
use crate::backend::debug::mouse_click_command::MouseClickCommand;
use crate::backend::debug::mouse_doubleclick_command::MouseDoubleClickCommand;
use crate::backend::debug::mouse_drag_command::MouseDragCommand;
use crate::backend::debug::mouse_hold_command::MouseHoldCommand;
use crate::backend::debug::mouse_move_command::MouseMoveCommand;
use crate::backend::debug::mouse_release_command::MouseReleaseCommand;
use crate::backend::debug::mouse_wheel_command::MouseWheelCommand;
use crate::backend::debug::paint_enable_command::PaintEnableCommand;

use super::command_parser::CommandParser;
use super::resize_command::ResizeCommand;

#[test]
fn check_command_parser_simple() {
    let cp = CommandParser::new("run(1,2)").unwrap();
    assert_eq!(cp.get_command(), "run");
    assert_eq!(cp.get_params_count(), 2);
    assert_eq!(cp.get_param(0), Some("1"));
    assert_eq!(cp.get_param(1), Some("2"));
}
#[test]
fn check_command_parser_spaced() {
    let cp = CommandParser::new("  run     (    1  ,   left  ,   -200   )    ").unwrap();
    assert_eq!(cp.get_command(), "run");
    assert_eq!(cp.get_params_count(), 3);
    assert_eq!(cp.get_param(0), Some("1"));
    assert_eq!(cp.get_param(1), Some("left"));
    assert_eq!(cp.get_param(2), Some("-200"));
}
#[test]
fn check_command_parser_string() {
    let cp =
        CommandParser::new("  run     (    'some string '  ,   left  ,   -200   )    ").unwrap();
    assert_eq!(cp.get_command(), "run");
    assert_eq!(cp.get_params_count(), 3);
    assert_eq!(cp.get_param(0), Some("some string "));
    assert_eq!(cp.get_param(1), Some("left"));
    assert_eq!(cp.get_param(2), Some("-200"));
}

#[test]
fn check_command_parser_string_with_unescape() {
    let cp = CommandParser::new("test('123\\n123','123\\t123')").unwrap();
    assert_eq!(cp.get_command(), "test");
    assert_eq!(cp.get_params_count(), 2);
    assert_eq!(cp.get_string(0), Some(String::from("123\n123")));
    assert_eq!(cp.get_string(1), Some(String::from("123\t123")));
}

#[test]
fn test_command_only() {
    let command = "help";
    let parser = CommandParser::new(command).unwrap();
    assert_eq!(parser.get_command(), "help");
    assert_eq!(parser.get_params_count(), 0);
    assert_eq!(parser.get_param(0), None);
    assert_eq!(parser.get_param(1), None);
    assert_eq!(parser.get_param(2), None);
}

#[test]
fn test_single_param() {
    let command = "set(value)";
    let parser = CommandParser::new(command).unwrap();
    assert_eq!(parser.get_command(), "set");
    assert_eq!(parser.get_params_count(), 1);
    assert_eq!(parser.get_param(0), Some("value"));
    assert_eq!(parser.get_param(1), None);
    assert_eq!(parser.get_param(2), None);
}

#[test]
fn test_multiple_params() {
    let command = "create(user,\"John ,Smith\",30)";
    let parser = CommandParser::new(command).unwrap();
    assert_eq!(parser.get_command(), "create");
    assert_eq!(parser.get_params_count(), 3);
    assert_eq!(parser.get_param(0), Some("user"));
    assert_eq!(parser.get_param(1), Some("John ,Smith"));
    assert_eq!(parser.get_param(2), Some("30"));
}

#[test]
fn test_too_many_params() {
    let command = "update(id,name,age,location,size)";
    let parser = CommandParser::new(command);
    assert!(parser.is_err());
    assert_eq!(
        parser.unwrap_err().get_error(),
        "Too many parameters (max allowed is 4)"
    );
}

#[test]
fn test_missing_command() {
    let command = "";
    let parser = CommandParser::new(command);
    assert!(parser.is_err());
    assert_eq!(parser.unwrap_err().get_error(), "Expecting a valid command (not an empty line)");
}

#[test]
fn test_missing_param() {
    let command = "create(user)";
    let parser = CommandParser::new(command).unwrap();
    assert_eq!(parser.get_command(), "create");
    assert_eq!(parser.get_params_count(), 1);
    assert_eq!(parser.get_param(0), Some("user"));
    assert_eq!(parser.get_param(1), None);
    assert_eq!(parser.get_param(2), None);
}

#[test]
fn test_invalid_syntax() {
    let command = "update(, id)";
    let parser = CommandParser::new(command);
    assert!(parser.is_err());
    assert_eq!(
        parser.unwrap_err().get_error(),
        "Expecting a word but found ',' separator !"
    );
}

#[test]
fn test_invalid_character() {
    let command = "create( #user)";
    let parser = CommandParser::new(command);
    assert!(parser.is_err());
    assert_eq!(parser.unwrap_err().get_error(), "Invalid character (expecting a word)");
}

#[test]
fn test_invalid_string() {
    let command = "create (user, \"John Smith)";
    let parser = CommandParser::new(command);
    assert!(parser.is_err());
    assert_eq!(
        parser.unwrap_err().get_error(),
        "Invalid string (no ending '\"' character found)"
    );
}


#[test]
fn check_command_parser_bool_params() {
    let cp = CommandParser::new("  validate(1,true,false)").unwrap();
    assert_eq!(cp.get_command(), "validate");
    assert_eq!(cp.get_params_count(), 3);
    assert_eq!(cp.get_bool(0), None);
    assert_eq!(cp.get_bool(1), Some(true));
    assert_eq!(cp.get_bool(2), Some(false));
}

#[test]
fn check_command_parser_i32_params() {
    let cp = CommandParser::new("  validate(123,-1276,false)").unwrap();
    assert_eq!(cp.get_command(), "validate");
    assert_eq!(cp.get_params_count(), 3);
    assert_eq!(cp.get_i32(0), Some(123));
    assert_eq!(cp.get_i32(1), Some(-1276));
    assert_eq!(cp.get_i32(2), None);
}

#[test]
fn check_resize_errors() {
    // invalid number of parameters
    assert!(ResizeCommand::new(&CommandParser::new("resize(1)").unwrap()).is_err());
    assert!(ResizeCommand::new(&CommandParser::new("resize(1,2,3)").unwrap()).is_err());
    assert!(ResizeCommand::new(&CommandParser::new("resize()").unwrap()).is_err());
    // invalid numeric values
    assert!(ResizeCommand::new(&CommandParser::new("resize(w,10)").unwrap()).is_err());
    assert!(ResizeCommand::new(&CommandParser::new("resize(10,h)").unwrap()).is_err());
    assert!(ResizeCommand::new(&CommandParser::new("resize(w,h)").unwrap()).is_err());
    //  invalid ranges
    assert!(ResizeCommand::new(&CommandParser::new("resize(3,10)").unwrap()).is_err());
    assert!(ResizeCommand::new(&CommandParser::new("resize(4,10)").unwrap()).is_err());
    assert!(ResizeCommand::new(&CommandParser::new("resize(10001,10)").unwrap()).is_err());
    assert!(ResizeCommand::new(&CommandParser::new("resize(10,3)").unwrap()).is_err());
    assert!(ResizeCommand::new(&CommandParser::new("resize(10,4)").unwrap()).is_err());
    assert!(ResizeCommand::new(&CommandParser::new("resize(10,10001)").unwrap()).is_err());
}


#[test]
fn check_paint_enable_errors() {
    // invalid bool constanta
    assert!(PaintEnableCommand::new(&CommandParser::new("Paint.Enable(blablabla)").unwrap()).is_err());
}

#[test]
fn check_mouse_wheel_errors() {
    // invalid number of parameters
    assert!(MouseWheelCommand::new(&CommandParser::new("Mouse.Wheel(1)").unwrap()).is_err());
    assert!(MouseWheelCommand::new(&CommandParser::new("Mouse.Wheel(1,2)").unwrap()).is_err());
    assert!(MouseWheelCommand::new(&CommandParser::new("Mouse.Wheel(1,2,3)").unwrap()).is_err());
    assert!(MouseWheelCommand::new(&CommandParser::new("Mouse.Wheel()").unwrap()).is_err());
    // invalid numeric values
    assert!(MouseWheelCommand::new(&CommandParser::new("Mouse.Wheel(x,0,left,1)").unwrap()).is_err());
    assert!(MouseWheelCommand::new(&CommandParser::new("Mouse.Wheel(0,y,up,1)").unwrap()).is_err());
    // invalid direction
    assert!(MouseWheelCommand::new(&CommandParser::new("Mouse.Wheel(0,0,blablbbla,1)").unwrap()).is_err());
    //  invalid time value
    assert!(MouseWheelCommand::new(&CommandParser::new("Mouse.Wheel(0,0,down,blablabla)").unwrap()).is_err());
    assert!(MouseWheelCommand::new(&CommandParser::new("Mouse.Wheel(-10,-10,right,-5)").unwrap()).is_err());
    assert!(MouseWheelCommand::new(&CommandParser::new("Mouse.Wheel(10,10,left,0)").unwrap()).is_err());
}

#[test]
fn check_mouse_release_errors() {
    // invalid number of parameters
    assert!(MouseReleaseCommand::new(&CommandParser::new("Mouse.Release    (1)").unwrap()).is_err());
    assert!(MouseReleaseCommand::new(&CommandParser::new("Mouse.Release(1,2)").unwrap()).is_err());
    assert!(MouseReleaseCommand::new(&CommandParser::new("Mouse.Release(1,2,3,4)").unwrap()).is_err());
    assert!(MouseReleaseCommand::new(&CommandParser::new("Mouse.Release()").unwrap()).is_err());
    // invalid numeric values
    assert!(MouseReleaseCommand::new(&CommandParser::new("Mouse.Release(x,0,left)").unwrap()).is_err());
    assert!(MouseReleaseCommand::new(&CommandParser::new("Mouse.Release(0,y,right)").unwrap()).is_err());
    assert!(MouseReleaseCommand::new(&CommandParser::new("Mouse.Release(x,y,center)").unwrap()).is_err());
    // invalid button
    assert!(MouseReleaseCommand::new(&CommandParser::new("Mouse.Release(0,0,blablbbla)").unwrap()).is_err());
}

#[test]
fn check_mouse_move_errors() {
    // invalid number of parameters
    assert!(MouseMoveCommand::new(&CommandParser::new("Mouse.Move(1)").unwrap()).is_err());
    assert!(MouseMoveCommand::new(&CommandParser::new("Mouse.Move(1,2,3)").unwrap()).is_err());
    assert!(MouseMoveCommand::new(&CommandParser::new("Mouse.Move(1,2,3,4)").unwrap()).is_err());
    assert!(MouseMoveCommand::new(&CommandParser::new("Mouse.Move()").unwrap()).is_err());
    // invalid numeric values
    assert!(MouseMoveCommand::new(&CommandParser::new("Mouse.Move(x,0)").unwrap()).is_err());
    assert!(MouseMoveCommand::new(&CommandParser::new("Mouse.Move(0,y)").unwrap()).is_err());
}


#[test]
fn check_mouse_hold_errors() {
    // invalid number of parameters
    assert!(MouseHoldCommand::new(&CommandParser::new("Mouse.Hold(1)").unwrap()).is_err());
    assert!(MouseHoldCommand::new(&CommandParser::new("Mouse.Hold(1,2)").unwrap()).is_err());
    assert!(MouseHoldCommand::new(&CommandParser::new("Mouse.Hold(1,2,3,4)").unwrap()).is_err());
    assert!(MouseHoldCommand::new(&CommandParser::new("Mouse.Hold()").unwrap()).is_err());
    // invalid numeric values
    assert!(MouseHoldCommand::new(&CommandParser::new("Mouse.Hold(x,0,left)").unwrap()).is_err());
    assert!(MouseHoldCommand::new(&CommandParser::new("Mouse.Hold(0,y,right)").unwrap()).is_err());
    assert!(MouseHoldCommand::new(&CommandParser::new("Mouse.Hold(x,y,center)").unwrap()).is_err());
    // invalid button
    assert!(MouseHoldCommand::new(&CommandParser::new("Mouse.Hold(0,0,blablbbla)").unwrap()).is_err());
}

#[test]
fn check_mouse_doubleclicked_errors() {
    // invalid number of parameters
    assert!(MouseDoubleClickCommand::new(&CommandParser::new("Mouse.DoubleClick(1)").unwrap()).is_err());
    assert!(MouseDoubleClickCommand::new(&CommandParser::new("Mouse.DoubleClick(1,2)").unwrap()).is_err());
    assert!(MouseDoubleClickCommand::new(&CommandParser::new("Mouse.DoubleClick(1,2,3,4)").unwrap()).is_err());
    assert!(MouseDoubleClickCommand::new(&CommandParser::new("Mouse.DoubleClick()").unwrap()).is_err());
    // invalid numeric values
    assert!(MouseDoubleClickCommand::new(&CommandParser::new("Mouse.DoubleClick(x,0,left)").unwrap()).is_err());
    assert!(MouseDoubleClickCommand::new(&CommandParser::new("Mouse.DoubleClick(0,y,right)").unwrap()).is_err());
    assert!(MouseDoubleClickCommand::new(&CommandParser::new("Mouse.DoubleClick(x,y,center)").unwrap()).is_err());
    // invalid button
    assert!(MouseDoubleClickCommand::new(&CommandParser::new("Mouse.DoubleClick(0,0,blablbbla)").unwrap()).is_err());
}

#[test]
fn check_mouse_click_errors() {
    // invalid number of parameters
    assert!(MouseClickCommand::new(&CommandParser::new("Mouse.Clck(1)").unwrap()).is_err());
    assert!(MouseClickCommand::new(&CommandParser::new("Mouse.Clck(1,2)").unwrap()).is_err());
    assert!(MouseClickCommand::new(&CommandParser::new("Mouse.Clck(1,2,3,4)").unwrap()).is_err());
    assert!(MouseClickCommand::new(&CommandParser::new("Mouse.Clck()").unwrap()).is_err());
    // invalid numeric values
    assert!(MouseClickCommand::new(&CommandParser::new("Mouse.Clck(x,0,left)").unwrap()).is_err());
    assert!(MouseClickCommand::new(&CommandParser::new("Mouse.Clck(0,y,right)").unwrap()).is_err());
    assert!(MouseClickCommand::new(&CommandParser::new("Mouse.Clck(x,y,center)").unwrap()).is_err());
    // invalid button
    assert!(MouseClickCommand::new(&CommandParser::new("Mouse.Clck(0,0,blablbbla)").unwrap()).is_err());
}

#[test]
fn check_mouse_drag_errors() {
    // invalid number of parameters
    assert!(MouseDragCommand::new(&CommandParser::new("Mouse.Drag(1)").unwrap()).is_err());
    assert!(MouseDragCommand::new(&CommandParser::new("Mouse.Drag(1,2)").unwrap()).is_err());
    assert!(MouseDragCommand::new(&CommandParser::new("Mouse.Drag(1,2,3)").unwrap()).is_err());
    assert!(MouseDragCommand::new(&CommandParser::new("Mouse.Drag()").unwrap()).is_err());
    // invalid numeric values
    assert!(MouseDragCommand::new(&CommandParser::new("Mouse.Drag(x1,0,0,0)").unwrap()).is_err());
    assert!(MouseDragCommand::new(&CommandParser::new("Mouse.Drag(0,y1,0,0)").unwrap()).is_err());
    assert!(MouseDragCommand::new(&CommandParser::new("Mouse.Drag(0,0,x2,0)").unwrap()).is_err());
    assert!(MouseDragCommand::new(&CommandParser::new("Mouse.Drag(0,0,0,y2)").unwrap()).is_err());
}

#[test]
fn check_key_type_text_errors() {
    // invalid number of parameters
    assert!(KeyTypeTextCommand::new(&CommandParser::new("Key.TypeText(1,2,3,4)").unwrap()).is_err());
    assert!(KeyTypeTextCommand::new(&CommandParser::new("Key.TypeText(1,2)").unwrap()).is_err());
    assert!(KeyTypeTextCommand::new(&CommandParser::new("Key.TypeText(1,2,3)").unwrap()).is_err());
    assert!(KeyTypeTextCommand::new(&CommandParser::new("Key.TypeText()").unwrap()).is_err());
}

#[test]
fn check_key_pressed_errors() {
    // invalid number of parameters
    assert!(KeyPressedCommand::new(&CommandParser::new("Key.Pressed(1,2,3,4)").unwrap()).is_err());
    assert!(KeyPressedCommand::new(&CommandParser::new("Key.Pressed(1,2,3)").unwrap()).is_err());
    assert!(KeyPressedCommand::new(&CommandParser::new("Key.Pressed()").unwrap()).is_err());
    // ivalid key
    assert!(KeyPressedCommand::new(&CommandParser::new("Key.Pressed(blablabla)").unwrap()).is_err());
    // invalid number of times
    assert!(KeyPressedCommand::new(&CommandParser::new("Key.Pressed(Enter,blablabla)").unwrap()).is_err());
    assert!(KeyPressedCommand::new(&CommandParser::new("Key.Pressed(Enter,0)").unwrap()).is_err());
}

#[test]
fn check_key_modifier_errors() {
    // invalid number of parameters
    assert!(KeyModifierCommand::new(&CommandParser::new("Key.Modifier(1,2,3,4)").unwrap()).is_err());
    assert!(KeyModifierCommand::new(&CommandParser::new("Key.Modifier(1,2,3)").unwrap()).is_err());
    assert!(KeyModifierCommand::new(&CommandParser::new("Key.Modifier(1,2)").unwrap()).is_err());
    assert!(KeyModifierCommand::new(&CommandParser::new("Key.Modifier()").unwrap()).is_err());
    // ivalid modifiers
    assert!(KeyModifierCommand::new(&CommandParser::new("Key.Modifier(blablabla)").unwrap()).is_err());
}

#[test]
fn check_error_disable_errors() {
    // invalid number of parameters
    assert!(ErrorDisableCommand::new(&CommandParser::new("Error.Disable(1,2,3,4)").unwrap()).is_err());
    assert!(ErrorDisableCommand::new(&CommandParser::new("Error.Disable(1,2,3)").unwrap()).is_err());
    assert!(ErrorDisableCommand::new(&CommandParser::new("Error.Disable(1,2)").unwrap()).is_err());
    assert!(ErrorDisableCommand::new(&CommandParser::new("Error.Disable()").unwrap()).is_err());
    // ivalid bool value
    assert!(ErrorDisableCommand::new(&CommandParser::new("Error.Disable(blablabla)").unwrap()).is_err());
}

#[test]
fn check_clipboard_settext_errors() {
    // invalid number of parameters
    assert!(ClipboardSetTextCommand::new(&CommandParser::new("Clipboard.SetText(1,2,3,4)").unwrap()).is_err());
    assert!(ClipboardSetTextCommand::new(&CommandParser::new("Clipboard.SetText(1,2,3)").unwrap()).is_err());
    assert!(ClipboardSetTextCommand::new(&CommandParser::new("Clipboard.SetText(1,2)").unwrap()).is_err());
    assert!(ClipboardSetTextCommand::new(&CommandParser::new("Clipboard.SetText()").unwrap()).is_err());
}

#[test]
fn check_clipboard_clear_errors() {
    // invalid number of parameters
    assert!(ClipboardClearCommand::new(&CommandParser::new("Clipboard.Clear(1,2,3,4)").unwrap()).is_err());
    assert!(ClipboardClearCommand::new(&CommandParser::new("Clipboard.Clear(1,2,3)").unwrap()).is_err());
    assert!(ClipboardClearCommand::new(&CommandParser::new("Clipboard.Clear(1,2)").unwrap()).is_err());
    assert!(ClipboardClearCommand::new(&CommandParser::new("Clipboard.Clear(1)").unwrap()).is_err());
    assert!(ClipboardClearCommand::new(&CommandParser::new("Clipboard.Clear()").unwrap()).is_ok());
}

#[test]
fn check_checkclipboardtext_errors() {
    // invalid number of parameters
    assert!(CheckClipboardTextCommand::new(&CommandParser::new("CheckClipboatfText(1,2,3,4)").unwrap()).is_err());
    assert!(CheckClipboardTextCommand::new(&CommandParser::new("CheckClipboatfText(1,2,3)").unwrap()).is_err());
    assert!(CheckClipboardTextCommand::new(&CommandParser::new("CheckClipboatfText(1,2)").unwrap()).is_err());
    assert!(CheckClipboardTextCommand::new(&CommandParser::new("CheckClipboatfText()").unwrap()).is_err());
}

#[test]
fn check_checkhash_errors() {
    // invalid number of parameters
    assert!(CheckHashCommand::new(&CommandParser::new("CheckHash(1,2,3,4)").unwrap()).is_err());
    assert!(CheckHashCommand::new(&CommandParser::new("CheckHash(1,2,3)").unwrap()).is_err());
    assert!(CheckHashCommand::new(&CommandParser::new("CheckHash(1,2)").unwrap()).is_err());
    assert!(CheckHashCommand::new(&CommandParser::new("CheckHash()").unwrap()).is_err());
    // ivalid hash value
    assert!(CheckHashCommand::new(&CommandParser::new("CheckHash(blablabla)").unwrap()).is_err());
}

#[test]
fn check_checkcursor_errors() {
    // invalid number of parameters
    assert!(CheckCursorCommand::new(&CommandParser::new("CheckCursor(1,2,3,4)").unwrap()).is_err());
    assert!(CheckCursorCommand::new(&CommandParser::new("CheckCursor(1,2,3)").unwrap()).is_err());
    assert!(CheckCursorCommand::new(&CommandParser::new("CheckCursor()").unwrap()).is_err());
    // invalid one-parameter value (should be hidden or false)
    assert!(CheckCursorCommand::new(&CommandParser::new("CheckCursor(blablabla)").unwrap()).is_err());
    // invalid position values (integers)
    assert!(CheckCursorCommand::new(&CommandParser::new("CheckCursor(x,0)").unwrap()).is_err());
    assert!(CheckCursorCommand::new(&CommandParser::new("CheckCursor(0,y)").unwrap()).is_err());
}