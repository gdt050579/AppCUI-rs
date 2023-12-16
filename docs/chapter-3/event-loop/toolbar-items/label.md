# Label toolbar item

A toolbar label is a text that can be written on the top or bottom part of a windows (like in the following image).

<img src="img/label.png" width=300/>

To create a label toolbar use the `toolbar::Label::new(...)` method:

```rust
let toolbar_label = toolbar::Label::new("content");
```

or the `toolbaritem!` macro:

```rust
let toolbar_label_1 = toolbaritem!("content,type=label");
let toolbal_label_2 = toolbaritem!("content='label text',type:label");
```