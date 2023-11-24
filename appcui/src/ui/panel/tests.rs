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