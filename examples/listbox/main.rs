use appcui::prelude::*;

#[Window(events=ListBoxEvents)]
struct MyWin {
    lbox: Handle<ListBox>,
    log: Handle<ListBox>,    
}

impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("Title:'Colors',d:c,w:80,h:20,flags:Sizeable"),
            lbox: Handle::None,
            log: Handle::None,
        };
        let mut vs = vsplitter!("50%,d:c,w:100%,h:100%");
        w.lbox = vs.add(
            vsplitter::Panel::Left,
            listbox!("d:c,w:100%,h:100%,flags: ScrollBars+CheckBoxes+SearchBar,items:['Red','Green','Blue','Yellow','Black','White'],tsm:4,lsm:1"),
        );
        let mut p = panel!("caption:'Event logs',d:c,w:100%,h:100%,type: TopBar");
        w.log = p.add(listbox!("d:c,w:100%,h:100%,flags: ScrollBars+SearchBar+AutoScroll, lsm:1"));
        vs.add(vsplitter::Panel::Right, p);
        w.add(vs);
        w
    }
}
impl ListBoxEvents for MyWin {
    fn on_current_item_changed(&mut self, handle: Handle<ListBox>, index: usize) -> EventProcessStatus {
        if self.lbox == handle {
            let h = self.log;
            if let Some(log) = self.control_mut(h) {
                let idx = log.count()+1;
                log.add(&format!("{} => Current item changed to index: {}", idx, index));
            }
        }
        EventProcessStatus::Processed
    }

    fn on_item_checked(&mut self, _handle: Handle<ListBox>, _index: usize, _checked: bool) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
