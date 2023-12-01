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