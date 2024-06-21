use crate::prelude::*;

#[test]
fn check_panel_with_border() {
    let script = "
        Paint.Enable(false)
        // we should see something like: ┌─ Options ────┐┌─ Optio… ─┐┌─ Options ─┐┌─ Opt… ─┐┌─ O… ─┐┌─────┐┌──────┐
        //                               ┌─ Inactive panel ───────────────────────────────────────────────────────┐
        Paint('Border panel')   
        CheckHash(0xDE639472091C422)   
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,d:c,w:78,h:11");
    w.add(Panel::new("Options", Layout::new("x:1,y:1,w:16,h:4"), panel::Type::Border));
    w.add(Panel::new("Options", Layout::new("x:17,y:1,w:12,h:4"), panel::Type::Border));
    w.add(Panel::new("Options", Layout::new("x:29,y:1,w:13,h:4"), panel::Type::Border));
    w.add(Panel::new("Options", Layout::new("x:42,y:1,w:10,h:4"), panel::Type::Border));
    w.add(Panel::new("Options", Layout::new("x:52,y:1,w:8,h:4"), panel::Type::Border));
    w.add(Panel::new("Options", Layout::new("x:60,y:1,w:7,h:4"), panel::Type::Border));
    w.add(Panel::new("", Layout::new("x:67,y:1,w:8,h:4"), panel::Type::Border));
    let mut inactive_panel = Panel::new("Inactive panel",Layout::new("x:1,y:5,w:74,h:4"),panel::Type::Border);
    inactive_panel.set_enabled(false);
    w.add(inactive_panel);
    a.add_window(w);
    a.run();
}
#[test]
fn check_panel_with_macro() {
    let script = "
        //Paint.Enable(false)
        Paint('Border panel')   
        //CheckHash(0xDE639472091C422)   
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,d:c,w:78,h:11");
    w.add(panel!("Options,x:1,y:1,w:16,h:4"));
    w.add(panel!("caption:'Inactive panel',x:1,y:5,w:74,h:4,type:Border,enabled:false"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_panel_with_macro_no_title() {
    let script = "
        Paint.Enable(false)
        Paint('No title panel')   
        CheckHash(0x4718D94609DBF8B4)   
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,d:c,w:78,h:11");
    w.add(panel!("l:1,r:1,t:1,b:1"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_panel_with_page() {
    let script = "
        Paint.Enable(false)
        Paint('Page panel')   
        CheckHash(0xBF28919C860D0024)   
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,d:c,w:78,h:11");
    w.add(Panel::new("Options", Layout::new("x:1,y:1,w:16,h:4"), panel::Type::Page));
    w.add(Panel::new("Options", Layout::new("x:18,y:1,w:12,h:4"), panel::Type::Page));
    w.add(Panel::new("Options", Layout::new("x:31,y:1,w:13,h:4"), panel::Type::Page));
    w.add(Panel::new("Options", Layout::new("x:50,y:1,w:10,h:4"), panel::Type::Page));
    let mut inactive_panel = Panel::new("Inactive panel",Layout::new("x:1,y:5,w:74,h:4"),panel::Type::Page);
    inactive_panel.set_enabled(false);
    w.add(inactive_panel);
    a.add_window(w);
    a.run();
}
#[test]
fn check_panel_with_window() {
    let script = "
        Paint.Enable(false)
        // we should see something like: ┌─── Options ──┐┌─ Optio… ─┐┌─ Options ─┐┌─ Opt… ─┐┌─ O… ─┐┌─────┐┌──────┐
        //                               ┌──────────────────────────── Inactive panel ────────────────────────────┐ 
        Paint('window panel')   
        CheckHash(0x48A952908E22B5CA)   
    ";
    let mut a = App::debug(80, 13, script).build().unwrap();
    let mut w = window!("Title,d:c,w:78,h:11");
    w.add(Panel::new("Options", Layout::new("x:1,y:1,w:16,h:4"), panel::Type::Window));
    w.add(Panel::new("Options", Layout::new("x:17,y:1,w:12,h:4"), panel::Type::Window));
    w.add(Panel::new("Options", Layout::new("x:29,y:1,w:13,h:4"), panel::Type::Window));
    w.add(Panel::new("Options", Layout::new("x:42,y:1,w:10,h:4"), panel::Type::Window));
    w.add(Panel::new("Options", Layout::new("x:52,y:1,w:8,h:4"), panel::Type::Window));
    w.add(Panel::new("Options", Layout::new("x:60,y:1,w:7,h:4"), panel::Type::Window));
    w.add(Panel::new("", Layout::new("x:67,y:1,w:8,h:4"), panel::Type::Window));
    let mut inactive_panel = Panel::new("Inactive panel",Layout::new("x:1,y:5,w:74,h:4"),panel::Type::Window);
    inactive_panel.set_enabled(false);
    w.add(inactive_panel);
    a.add_window(w);
    a.run();
}

#[test]
fn check_panel_with_topbar() {
    let script = "
        Paint.Enable(false)
        Paint('TopBar panel')   
        CheckHash(0x77A2D12C49F335CE)   
    ";
    let mut a = App::debug(90, 13, script).build().unwrap();
    let mut w = window!("Title,d:c,w:88,h:11");
    w.add(Panel::new("Options", Layout::new("x:1,y:1,w:16,h:4"), panel::Type::TopBar));
    w.add(Panel::new("Options", Layout::new("x:18,y:1,w:12,h:4"), panel::Type::TopBar));
    w.add(Panel::new("Options", Layout::new("x:31,y:1,w:13,h:4"), panel::Type::TopBar));
    w.add(Panel::new("Options", Layout::new("x:45,y:1,w:10,h:4"), panel::Type::TopBar));
    w.add(Panel::new("Options", Layout::new("x:56,y:1,w:8,h:4"), panel::Type::TopBar));
    w.add(Panel::new("Options", Layout::new("x:65,y:1,w:7,h:4"), panel::Type::TopBar));
    w.add(Panel::new("", Layout::new("x:73,y:1,w:8,h:4"), panel::Type::TopBar));
    let mut inactive_panel = Panel::new("Inactive panel",Layout::new("x:1,y:5,w:84,h:4"),panel::Type::TopBar);
    inactive_panel.set_enabled(false);
    w.add(inactive_panel);
    a.add_window(w);
    a.run();
}

#[test]
fn check_panel_add_controls() {
    let script = "
        Paint.Enable(false)
        Paint('focus on checkbox')   
        CheckHash(0xD719107CC4836D31)
        Key.Pressed(Tab)
        Paint('focus on but-1')   
        CheckHash(0x333B7140835D2CDD)
        Key.Pressed(Tab)
        Paint('focus on <some option> checkbox')   
        CheckHash(0xE8B901C118B2EF1)
        Key.Pressed(Tab)
        Paint('focus on but-2')   
        CheckHash(0xA2FEDAC449ABC938)
        Key.Pressed(Tab)
        Paint('focus on checkbox (again)')   
        CheckHash(0xD719107CC4836D31)
        Key.Pressed(Shift+Tab)
        Paint('Backwards: focus on but-2')   
        CheckHash(0xA2FEDAC449ABC938)
        Key.Pressed(Shift+Tab)
        Paint('Backwards: focus on <some option> checkbox')   
        CheckHash(0xE8B901C118B2EF1)
        Key.Pressed(Shift+Tab)
        Paint('Backwards: focus on but-1')   
        CheckHash(0x333B7140835D2CDD)
        Key.Pressed(Shift+Tab)
        Paint('Back to start')   
        CheckHash(0xD719107CC4836D31)
    ";
    let mut a = App::debug(60, 13, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:11");
    let mut p = panel!("Controls,l:1,t:1,r:1,h:6");
    p.add(button!("but-1,x:1,y:1,w:10,type:flat"));
    p.add(checkbox!("'Some option',x:1,y:2,w:15"));
    p.add(button!("but-2,x:20,y:1,w:10"));
    w.add(p);
    w.add(checkbox!("'option outside panel',x:1,y:8,w:35,checked:true"));
    a.add_window(w);
    a.run();
}

#[test]
fn check_panel_navigate() {
    let script = "
        Paint.Enable(false)
        Paint('Focus: button (11) (initial state)')   
        CheckHash(0x250A86FF5B2DE3B5)
        Key.Pressed(Tab)
        Paint('Focus: button (12)')   
        CheckHash(0x6EFE19BE6FB9DAB9)
        Key.Pressed(Tab)
        // we will skip buttons 13,14 and 15 that are inactive
        Paint('Focus: button (16)')   
        CheckHash(0x40081ECFB94F6B1)
        Key.Pressed(Tab)
        Paint('Focus: button (17)')   
        CheckHash(0x838352A7081D9CD1)
        Key.Pressed(Tab)
        Paint('Focus: button (1)')   
        CheckHash(0xA6EEFE23A11FAE35)
        Key.Pressed(Tab)
        Paint('Focus: button (2)')   
        CheckHash(0x14E5F43F047C86B1)
        Key.Pressed(Tab)
        Paint('Focus: button (3)')   
        CheckHash(0xB1058BEE8BCB0171)
        Key.Pressed(Tab)
        Paint('Focus: button (4)')   
        CheckHash(0xA7AC355CF0DF9B75)
        Key.Pressed(Tab)
        Paint('Focus: button (5)')   
        CheckHash(0x832C9AB6555DB9D5)
        Key.Pressed(Tab)
        Paint('Focus: button (6)')   
        CheckHash(0xA33577C3EBE688B1)
        Key.Pressed(Tab)
        Paint('Focus: button (7)')   
        CheckHash(0xAF8A6C6B0936C4B9)
        Key.Pressed(Tab)
        Paint('Focus: button (8)')   
        CheckHash(0x631DA185E82FF475)
        Key.Pressed(Tab)
        Paint('Focus: button (9)')   
        CheckHash(0xFCDDDE9C513587A5)
        Key.Pressed(Tab)
        Paint('Focus: button (10)')   
        CheckHash(0xBE87E6DF332D1125)
        Key.Pressed(Tab)
        Paint('Focus: button (11) - back to start')   
        CheckHash(0x250A86FF5B2DE3B5)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (10)')   
        CheckHash(0xBE87E6DF332D1125)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (9)')   
        CheckHash(0xFCDDDE9C513587A5)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (8)')   
        CheckHash(0x631DA185E82FF475)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (7)')   
        CheckHash(0xAF8A6C6B0936C4B9)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (6)')   
        CheckHash(0xA33577C3EBE688B1)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (5)')   
        CheckHash(0x832C9AB6555DB9D5)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (4)')   
        CheckHash(0xA7AC355CF0DF9B75)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (3)')   
        CheckHash(0xB1058BEE8BCB0171)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (2)')   
        CheckHash(0x14E5F43F047C86B1)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (1)')   
        CheckHash(0xA6EEFE23A11FAE35)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (17)')   
        CheckHash(0x838352A7081D9CD1)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (16)')   
        CheckHash(0x40081ECFB94F6B1)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (12)')   
        CheckHash(0x6EFE19BE6FB9DAB9)
        Key.Pressed(Shift+Tab)
        Paint('Backwards -> Focus: button (11) (initial state)')   
        CheckHash(0x250A86FF5B2DE3B5)
    ";
    let mut a = App::debug(80, 22, script).build().unwrap();
    let mut w = window!("Title,d:c,w:70,h:20");
    let mut p1 = panel!("Controls,l:1,t:1,r:1,h:8");
    let mut p2 = panel!("Layer-2,l:1,t:0,r:30,b:0");
    let mut p3 = panel!("Layer-3,l:46,t:0,r:1,b:0");
    p2.add(button!("1,x:1,y:1,w:10,type:flat"));
    p2.add(button!("2,x:1,y:3,w:10,type:flat"));
    p2.add(button!("3,x:12,y:1,w:10,type:flat"));
    p2.add(button!("4,x:12,y:3,w:10,type:flat"));
    p1.add(p2);
    p1.add(button!("5,x:35,y:1,w:10,type:flat"));
    p1.add(button!("6,x:35,y:3,w:10,type:flat"));
    p3.add(button!("7,x:1,y:1,w:10,type:flat"));
    p3.add(button!("8,x:1,y:3,w:10,type:flat"));
    p1.add(p3);
    w.add(p1);
    w.add(button!("9,x:1,y:10,w:10,type:flat"));
    w.add(button!("10,x:1,y:12,w:10,type:flat"));
    w.add(button!("11,x:1,y:14,w:10,type:flat"));
    let mut p4 = panel!("Layer-4,l:12,t:10,r:1,b:0");
    let mut p5 = panel!("Layer-5,l:1,t:0,w:14,b:0");
    p5.add(button!("12,x:1,y:1,w:10,type:flat"));
    p5.add(button!("13,x:1,y:3,w:10,type:flat,enabled:false"));   
    p4.add(p5);
    let mut p6 = panel!("Inactives,l:15,t:0,w:14,b:0");
    p6.add(button!("14,x:1,y:1,w:10,type:flat,enabled:false"));
    p6.add(button!("15,x:1,y:3,w:10,type:flat,enabled:false"));   
    p4.add(p6);
    p4.add(button!("16,x:32,y:1,w:10,type:flat"));
    p4.add(button!("17,x:32,y:3,w:10,type:flat"));   
    w.add(p4);
    a.add_window(w);
    a.run();    
}

#[test]
fn check_panel_navigate_keys() {
    let script = "
        Paint.Enable(false)
        Paint('Focus: button (11) (initial state)')   
        CheckHash(0x250A86FF5B2DE3B5)
        Key.Pressed(Up)
        Paint('Focus: button (10)')   
        CheckHash(0xBE87E6DF332D1125)
        Key.Pressed(Ctrl+Up)
        Paint('Focus: button (9)')   
        CheckHash(0xFCDDDE9C513587A5)
        Key.Pressed(Alt+Up)
        Paint('Focus: button (2)')   
        CheckHash(0x14E5F43F047C86B1)
        Key.Pressed(Right)
        Paint('Focus: button (4)')   
        CheckHash(0xA7AC355CF0DF9B75)
        Key.Pressed(Ctrl+Right)
        Paint('Focus: button (6)')   
        CheckHash(0xA33577C3EBE688B1)
        Key.Pressed(Alt+Right)
        Paint('Focus: button (7)')   
        CheckHash(0xAF8A6C6B0936C4B9)
        Key.Pressed(Down)
        Paint('Focus: button (8)')   
        CheckHash(0x631DA185E82FF475)
        Key.Pressed(Ctrl+Down)
        Paint('Focus: button (16)')   
        CheckHash(0x40081ECFB94F6B1)
        Key.Pressed(Alt+Down)
        Paint('Focus: button (17)')   
        CheckHash(0x838352A7081D9CD1)
        Key.Pressed(Left)
        Paint('Focus: button (12)')   
        CheckHash(0x6EFE19BE6FB9DAB9)
        Key.Pressed(Left)
        Paint('Focus: button (10)')   
        CheckHash(0xBE87E6DF332D1125)
    ";
    let mut a = App::debug(80, 22, script).build().unwrap();
    let mut w = window!("Title,d:c,w:70,h:20");
    let mut p1 = panel!("Controls,l:1,t:1,r:1,h:8");
    let mut p2 = panel!("Layer-2,l:1,t:0,r:30,b:0");
    let mut p3 = panel!("Layer-3,l:46,t:0,r:1,b:0");
    p2.add(button!("1,x:1,y:1,w:10,type:flat"));
    p2.add(button!("2,x:1,y:3,w:10,type:flat"));
    p2.add(button!("3,x:12,y:1,w:10,type:flat"));
    p2.add(button!("4,x:12,y:3,w:10,type:flat"));
    p1.add(p2);
    p1.add(button!("5,x:35,y:1,w:10,type:flat"));
    p1.add(button!("6,x:35,y:3,w:10,type:flat"));
    p3.add(button!("7,x:1,y:1,w:10,type:flat"));
    p3.add(button!("8,x:1,y:3,w:10,type:flat"));
    p1.add(p3);
    w.add(p1);
    w.add(button!("9,x:1,y:10,w:10,type:flat"));
    w.add(button!("10,x:1,y:12,w:10,type:flat"));
    w.add(button!("11,x:1,y:14,w:10,type:flat"));
    let mut p4 = panel!("Layer-4,l:12,t:10,r:1,b:0");
    let mut p5 = panel!("Layer-5,l:1,t:0,w:14,b:0");
    p5.add(button!("12,x:1,y:1,w:10,type:flat"));
    p5.add(button!("13,x:1,y:3,w:10,type:flat,enabled:false"));   
    p4.add(p5);
    let mut p6 = panel!("Inactives,l:15,t:0,w:14,b:0");
    p6.add(button!("14,x:1,y:1,w:10,type:flat,enabled:false"));
    p6.add(button!("15,x:1,y:3,w:10,type:flat,enabled:false"));   
    p4.add(p6);
    p4.add(button!("16,x:32,y:1,w:10,type:flat"));
    p4.add(button!("17,x:32,y:3,w:10,type:flat"));   
    w.add(p4);
    a.add_window(w);
    a.run();    
}