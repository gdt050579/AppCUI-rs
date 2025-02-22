# ProgressBar

Represent a progress bar that can be used to hi:

<img src="img/progressbar.png" width=300/>

To create a label use `ProgressBar::new` method (with 2 parameters: a caption and a layout).
```rs
let pg1 = ProgressBar::new(1000, Layout::new("x:10,y:5,w:15"), progressbar::Flags::None);
```
or the macro `progressbar!`
```rs
let pg1 = progressbar!("total: 1000, x:10,y:5,w:15");
let pg2 = progressbar!("count: 125 ,x:10,y:5,w:15, text: 'Copying ...'");
```