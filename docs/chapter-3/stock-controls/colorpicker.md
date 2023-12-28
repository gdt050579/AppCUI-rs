# ColorPicker

Represent a control from where you can choose a color:

<img src="img/colorpicker.png" width=300/>

To create a button use `ColorPicker::new` method (with 2 parameters: a color and a layout).
```rs
let c = ColorPicker::new(Color::Green, Layout::new("x:10,y:5,w:15"));
```
or the macro `colorpicker!`
```rs
let c1 = colorpicker!("color=Red,x:10,y:5,w:15");
let c2 = colorpicker!("Darkgreen,x:10,y:5,w:15");
let c3 = colorpicker!("Yellow,x:10,y:5,w:15,visible:false");
```