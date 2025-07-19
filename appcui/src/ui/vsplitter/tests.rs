use crate::prelude::*;

#[test]
fn check_create() {
    let script = "
        Paint.Enable(false)
        // ┌─ Left ─────────────┐
        // ┌─ Right ────────────┐
        Paint('Initial state')   
        CheckHash(0xD1DEEDEB85046D1)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:51,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(0.5, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveAspectRatio);
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_create_procmacro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xD1DEEDEB85046D1)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:51,h:10,flags: Sizeable");
    let mut vs = vsplitter!("50%,a:c,w:100%,h:100%");
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_keyboard_movement() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xE2548C89F72469D9)
        Key.Pressed(Ctrl+Alt+Left)
        Paint('2. Left panel smaller by 1')   
        CheckHash(0x3A5186CCBBF6BF3D)
        Key.Pressed(Ctrl+Alt+Right)
        Paint('3. Back to original state')   
        CheckHash(0xE2548C89F72469D9)
        Key.Pressed(Ctrl+Alt+Left,100)
        Paint('4. Left most')   
        CheckHash(0xB617F953476DE6E2)
        Key.Pressed(Ctrl+Alt+Right,100)
        Paint('5. Right most')   
        CheckHash(0x1E6CE665E12C30FE)
        Key.Pressed(Ctrl+Alt+Shift+Left)
        Paint('6. Left most')   
        CheckHash(0xB617F953476DE6E2)
        Key.Pressed(Ctrl+Alt+Shift+Right)
        Paint('7. Right most')   
        CheckHash(0x1E6CE665E12C30FE)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(24, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveAspectRatio);
    vs.set_min_width(vsplitter::Panel::Left, 5);
    vs.set_min_width(vsplitter::Panel::Right, 5);
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_keyboard_movement_with_procmacro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')   
        CheckHash(0xE2548C89F72469D9)
        Key.Pressed(Ctrl+Alt+Left)
        Paint('2. Left panel smaller by 1')   
        CheckHash(0x3A5186CCBBF6BF3D)
        Key.Pressed(Ctrl+Alt+Right)
        Paint('3. Back to original state')   
        CheckHash(0xE2548C89F72469D9)
        Key.Pressed(Ctrl+Alt+Left,100)
        Paint('4. Left most')   
        CheckHash(0xB617F953476DE6E2)
        Key.Pressed(Ctrl+Alt+Right,100)
        Paint('5. Right most')   
        CheckHash(0x1E6CE665E12C30FE)
        Key.Pressed(Ctrl+Alt+Shift+Left)
        Paint('6. Left most')   
        CheckHash(0xB617F953476DE6E2)
        Key.Pressed(Ctrl+Alt+Shift+Right)
        Paint('7. Right most')   
        CheckHash(0x1E6CE665E12C30FE)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:10,flags: Sizeable");
    let mut vs = vsplitter!("pos:24,a:c,w:100%,h:100%,mlw:5,min-right-width:5");
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_buttons() {
    let script = "
        Paint.Enable(false)        
        Paint('Initial state')   
        CheckHash(0xE2548C89F72469D9)
        Mouse.Click(30,2,left)
        Paint('2. Left most')   
        CheckHash(0xB617F953476DE6E2)
        Mouse.Click(11,3,left)
        Paint('3. Right most')   
        CheckHash(0x1E6CE665E12C30FE)
        Mouse.Click(48,2,left)
        Paint('4. Left most')   
        CheckHash(0xB617F953476DE6E2)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(24, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveAspectRatio);
    vs.set_min_width(vsplitter::Panel::Left, 5);
    vs.set_min_width(vsplitter::Panel::Right, 5);
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_mouse_movement() {
    let script = "
        Paint.Enable(false)        
        Paint('Initial state')   
        CheckHash(0xE2548C89F72469D9)
        Mouse.Move(30,2)
        Paint('2. Left Button Hovered')   
        CheckHash(0x1904C6CB28C38017)
        Mouse.Move(30,3)
        Paint('3. Right Button Hovered')   
        CheckHash(0xC28A2015A90E8933)
        Mouse.Move(30,5)
        Paint('4. Splitter bar hovered')   
        CheckHash(0x16A315627F3D6011)
        Mouse.Drag(30,5,25,5)
        Paint('5. Splitter bar moved')   
        CheckHash(0xE4252AD91821669D)
        Mouse.Hold(25,5,left)
        Paint('6. Splitter bar pressed')   
        CheckHash(0xC2F66E9DABD4BB05)
        Mouse.Move(35,5)
        Paint('7. Splitter bar pressed (right)')   
        CheckHash(0xBB6A43024EE639B5)
        Mouse.Release(35,5,left)
        Paint('8. Splitter bar hovered')   
        CheckHash(0x101F59917065395D)
        Mouse.Move(30,5)
        Paint('9. Splitter bar not-hovered')   
        CheckHash(0x550B43D780CCCBDD)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(24, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveAspectRatio);
    vs.set_min_width(vsplitter::Panel::Left, 5);
    vs.set_min_width(vsplitter::Panel::Right, 5);
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_resize_preseve_aspect_ratio_with_percentage() {
    let script = "
        Paint.Enable(false)        
        Paint('1. Initial state')   
        CheckHash(0xFC29EDD7475CF785)
        Mouse.Drag(29,9,35,9)
        Paint('2. Resize - shoule be equal')   
        CheckHash(0x864A27821D05F7C1)
        Mouse.Drag(35,9,55,9)
        Paint('3. Resize - shoule be equal')   
        CheckHash(0x916704C680DF8EB9)
        Mouse.Drag(55,9,12,9)
        Paint('4. Resize - shoule be equal')   
        CheckHash(0x218360470FDB6C4B)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,x:0,y:0,w:30,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(0.5, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveAspectRatio);
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_resize_preseve_aspect_ratio_with_absolute() {
    let script = "
        Paint.Enable(false)        
        Paint('1. Initial state')   
        CheckHash(0x83B50620086A8781)
        Mouse.Drag(29,9,35,9)
        Paint('2. Resize - shoule be equal')   
        CheckHash(0x48B702D658DEFE95)
        Mouse.Drag(35,9,55,9)
        Paint('3. Resize - shoule be equal')   
        CheckHash(0x82D6CA2ABD8EF32D)
        Mouse.Drag(55,9,12,9)
        Paint('4. Resize - shoule be equal')   
        CheckHash(0x218360470FDB6C4B)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,x:0,y:0,w:30,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(14, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveAspectRatio);
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_resize_preseve_left_width_with_absolute() {
    let script = "
        Paint.Enable(false)   
        Paint('1. Initial state')   
        CheckHash(0xEA732DB4A58A4F60)
        Mouse.Drag(29,9,35,9)
        Paint('2. Resize - spliter shoudl be on y:11')   
        CheckHash(0x971EF480BA94C0E0)
        Mouse.Drag(35,9,55,9)
        Paint('3. Resize - spliter shoudl be on y:11')   
        CheckHash(0x45C466224BE23FA8)
        Mouse.Drag(55,9,12,9)
        Paint('4. Resize - spliter shoudl be on y:11')   
        CheckHash(0xE49947996A26F3D9)
        Mouse.Drag(12,9,20,9)
        Paint('5. Resize - spliter shoudl be on y:11')   
        CheckHash(0x3F47B23499481913)
        Key.Pressed(Ctrl+Alt+Right,3)
        Paint('6. Resize - spliter should be on y:14')   
        CheckHash(0xE1E8086CBBE66296)
        Mouse.Drag(20,9,40,9)
        Paint('7. Resize - spliter shoudl be on y:14')   
        CheckHash(0x8A597962CAAECEBD)
        Mouse.Drag(40,9,18,9)
        Paint('8. Resize - spliter shoudl be on y:14')   
        CheckHash(0x21CB7D3B26848632)
        Mouse.Drag(18,9,10,9)
        Paint('9. Resize - spliter shoudl be on y:10 (no space to be 14)')   
        CheckHash(0x1657F4D7630FA1E3)
        Mouse.Drag(10,9,40,9)
        Paint('10. Resize - spliter shoudl be on y:14 (again)')   
        CheckHash(0x8A597962CAAECEBD)        
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,x:0,y:0,w:30,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(10, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveLeftPanelSize);
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_resize_preseve_left_width_with_percentage() {
    let script = "
        Paint.Enable(false) 
        Paint('1. Initial state')   
        CheckHash(0xEA732DB4A58A4F60)
        Mouse.Drag(29,9,35,9)
        Paint('2. Resize - spliter shoudl be on y:11')   
        CheckHash(0x971EF480BA94C0E0)
        Mouse.Drag(35,9,55,9)
        Paint('3. Resize - spliter shoudl be on y:11')   
        CheckHash(0x45C466224BE23FA8)
        Mouse.Drag(55,9,12,9)
        Paint('4. Resize - spliter shoudl be on y:11')   
        CheckHash(0xE49947996A26F3D9)
        Mouse.Drag(12,9,20,9)
        Paint('5. Resize - spliter shoudl be on y:11')   
        CheckHash(0x3F47B23499481913)
        Key.Pressed(Ctrl+Alt+Right,3)
        Paint('6. Resize - spliter should be on y:14')   
        CheckHash(0xE1E8086CBBE66296)
        Mouse.Drag(20,9,40,9)
        Paint('7. Resize - spliter shoudl be on y:14')   
        CheckHash(0x8A597962CAAECEBD)
        Mouse.Drag(40,9,18,9)
        Paint('8. Resize - spliter shoudl be on y:14')   
        CheckHash(0x21CB7D3B26848632)
        Mouse.Drag(18,9,10,9)
        Paint('9. Resize - spliter shoudl be on y:10 (no space to be 14)')   
        CheckHash(0x1657F4D7630FA1E3)
        Mouse.Drag(10,9,40,9)
        Paint('10. Resize - spliter shoudl be on y:14 (again)')   
        CheckHash(0x8A597962CAAECEBD)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,x:0,y:0,w:30,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(0.39, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveLeftPanelSize);
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_resize_preseve_right_width_with_absolute() {
    let script = "
        Paint.Enable(false)   
        Paint('1. Initial state')   
        CheckHash(0x83B50620086A8781)
        Mouse.Drag(29,9,35,9)
        Paint('2. Resize - spliter shoudl be on y:21')   
        CheckHash(0x7234B8AA7BB968E1)
        Mouse.Drag(35,9,55,9)
        Paint('3. Resize - spliter shoudl be on y:41')   
        CheckHash(0x3D4FA1C3B7F1FA59)
        Mouse.Drag(55,9,12,9)
        Paint('4. Resize - spliter shoudl be on y:1 (no size to fit all)')   
        CheckHash(0xCF6BC7E6CC74015F)
        Mouse.Drag(12,9,20,9)
        Paint('5. Resize - spliter shoudl be on y:6 (now it fits all)')   
        CheckHash(0xEA7103FA68F5A792)
        Key.Pressed(Ctrl+Alt+Right,3)
        Paint('6. Resize - spliter should be on y:9 (text is R…)')   
        CheckHash(0xE6202A06ADD1605D)
        Mouse.Drag(20,9,40,9)
        Paint('7. Resize - spliter shoudl be on y:29 (text is R…)')   
        CheckHash(0xA14CC7CA0646F2)
        Mouse.Drag(40,9,18,9)
        Paint('8. Resize - spliter shoudl be on y:7 (text is R…)')   
        CheckHash(0xE2F190C2883A755)
        Mouse.Drag(18,9,10,9)
        Paint('9. Resize - spliter shoudl be on y:1 (┌─────┐)')   
        CheckHash(0x865FA35240029B3F)
        Mouse.Drag(10,9,40,9)
        Paint('10. Resize - spliter shoudl be on y:29 (text is R…)')   
        CheckHash(0xA14CC7CA0646F2)        
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,x:0,y:0,w:30,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(14, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveRightPanelSize);
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_resize_preseve_right_width_with_percentage() {
    let script = "
        Paint.Enable(false)   
        Paint('1. Initial state')   
        CheckHash(0x83B50620086A8781)
        Mouse.Drag(29,9,35,9)
        Paint('2. Resize - spliter shoudl be on y:21')   
        CheckHash(0x7234B8AA7BB968E1)
        Mouse.Drag(35,9,55,9)
        Paint('3. Resize - spliter shoudl be on y:41')   
        CheckHash(0x3D4FA1C3B7F1FA59)
        Mouse.Drag(55,9,12,9)
        Paint('4. Resize - spliter shoudl be on y:1 (no size to fit all)')   
        CheckHash(0xCF6BC7E6CC74015F)
        Mouse.Drag(12,9,20,9)
        Paint('5. Resize - spliter shoudl be on y:6 (now it fits all)')   
        CheckHash(0xEA7103FA68F5A792)
        Key.Pressed(Ctrl+Alt+Right,3)
        Paint('6. Resize - spliter should be on y:9 (text is R…)')   
        CheckHash(0xE6202A06ADD1605D)
        Mouse.Drag(20,9,40,9)
        Paint('7. Resize - spliter shoudl be on y:29 (text is R…)')   
        CheckHash(0xA14CC7CA0646F2)
        Mouse.Drag(40,9,18,9)
        Paint('8. Resize - spliter shoudl be on y:7 (text is R…)')   
        CheckHash(0xE2F190C2883A755)
        Mouse.Drag(18,9,10,9)
        Paint('9. Resize - spliter shoudl be on y:1 (┌─────┐)')   
        CheckHash(0x865FA35240029B3F)
        Mouse.Drag(10,9,40,9)
        Paint('10. Resize - spliter shoudl be on y:29 (text is R…)')   
        CheckHash(0xA14CC7CA0646F2)        
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,x:0,y:0,w:30,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(0.55, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveRightPanelSize);
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}

#[test]
fn check_scrollbars() {
    static CNN: &str = "
A Convolutional Neural Network (CNN) is a type of deep learning neural network commonly used for analyzing visual imagery. Here's a breakdown of how a CNN works:
1. Convolutional Layer
Convolution Operation: The primary feature of this layer is the convolution operation, which involves a filter (or kernel) sliding over the input image to produce a feature map. Each filter detects a specific feature such as edges, textures, or colors.
Activation Function: After the convolution operation, an activation function (typically ReLU) is applied to introduce non-linearity into the model.
2. Pooling Layer
Purpose: The pooling layer reduces the spatial dimensions (width and height) of the feature maps, which helps in reducing the computational complexity and also helps in making the detection invariant to small translations in the input image.
Types: The most common pooling operation is max pooling, which takes the maximum value in each patch of the feature map.
3. Stacking Layers
Multiple convolutional and pooling layers are stacked together, allowing the network to learn hierarchical features. Early layers learn low-level features (e.g., edges), while deeper layers learn high-level features (e.g., objects or faces).
    ";
    let script = "
        Paint.Enable(false)   
        Paint('1. Initial state')   
        CheckHash(0xACEBB5D97674A67B)
        Mouse.Move(12,9)
        Paint('2. Focus on horizontal scrollbar button')   
        CheckHash(0xBF5BED8F09F85A92)
        Mouse.Move(59,9)
        Paint('3. Focus on window resize grip')   
        CheckHash(0xA83215BE2822661A)
        Mouse.Move(59,8)
        Paint('4. Focus on vertical scrollbar button down')   
        CheckHash(0x484A7B8722FDBE50)
        Mouse.Click(59,8,left)
        Paint('5. Text scroll down one line')   
        CheckHash(0x2E9D6FA2E04D9AE7)
        Mouse.Click(30,9,left)
        Paint('6. Text scroll to the left')   
        CheckHash(0xD318E759299D18DA)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,x:0,y:0,w:60,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(5, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveRightPanelSize);
    let mut c = canvas!("'80x300',a:c,w:100%,h:100%,flags=ScrollBars,lsm:3,tsm:1");
    let s = c.drawing_surface_mut();
    s.write_string(0, 0, CNN, CharAttribute::with_color(Color::White, Color::Black), true);
    vs.add(vsplitter::Panel::Right, c);
    w.add(vs);
    a.add_window(w);
    a.run();
}


#[test]
fn check_mouse_wheel_and_doubleclicked() {
    let script = "
        Paint.Enable(false)
        // ┌─ Left ─────────────┐
        // ┌─ Right ────────────┐
        Paint('1. Initial state')   
        CheckHash(0xD1DEEDEB85046D1)
        Mouse.Wheel(30,8,left,1)
        Paint('2. Nothing changes')   
        CheckHash(0x7DAA05CFF05C4721)
        Mouse.DoubleClick(30,8,left)
        Paint('3. Nothing changes')   
        CheckHash(0x7DAA05CFF05C4721)
    ";
    let mut a = App::debug(60, 10, script).build().unwrap();
    let mut w = window!("Test,a:c,w:51,h:10,flags: Sizeable");
    let mut vs = VSplitter::new(0.5, Layout::new("a:c,w:100%,h:100%"), vsplitter::ResizeBehavior::PreserveAspectRatio);
    vs.add(vsplitter::Panel::Left, panel!("Left,l:1,r:1,t:1,b:1"));
    vs.add(vsplitter::Panel::Right, panel!("Right,l:1,r:1,t:1,b:1"));
    w.add(vs);
    a.add_window(w);
    a.run();
}