use appcui::prelude::*;
use appcui::ui::numericslider::*;
use appcui::ui::common::number::*;



fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Test,d:c,w:300,h:11,flags: Sizeable");
    //let mut p = panel!("Test,l:10,t:1,b:1,r:1");
    //let l = listbox!("d:c,w:100%,h:100%,flags: ScrollBars+HighlightSelectedItemWhenInactive, lsm:2, items=[1,2,3,4,5,6,7,8,9,10]");
    let num_slider = NumericSlider::new(100, 1000, 50, 550, Format::Decimal, Layout::new("x:1,y:2,w:100%"), Flags::SingleLine | Flags::HorizontalSlider);

    //p.add(num_slider);
    w.add(num_slider);
    //w.add(button!("Add,x:1,y:1,w:7,type:flat"));
    a.add_window(w);
    a.run();
    Ok(())
}