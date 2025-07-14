# Image Viewer

Represent a image that is being rendered under a view-port:

<img src="img/imageviewer.png" width=300/>

To create a image viewer use `ImageViewer::new` method (with 5 parameters: an image, a layout, a rendering method, scaling and initialization flags). To undestand more on how an image is being renedered or constructed read the [Images](../../chapter-2/images.md) chapter.
```rs
let i = ImageViewer::new(Image::from_str(...).unwrap(), 
                         Layout::new("x:10,y:5,w:15"),
                         image::RendererType::SmallBlocks, 
                         image::Scale::NoScale, 
                         imageviewer::Flags::None);
```
or the macro `imageviewer!`
```rs
let i1 = imageviewer!("x:10,y:5,w:15,scale:50%,render:AsciiArt");
let i2 = imageviewer!("image:'|R..|,|.R.|,|..R|',x:10,y:5,w:15");
```

A image viewer supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                     | Type         | Positional parameter | Purpose                                                                                                                                                                                         |
| ---------------------------------- | ------------ | -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `image`                            | String       | **No**               | A string representation of an image as described in [Images (Building from a string)](../../chapter-2/surface/images.md#building-from-a-string) chapter                                                 |
| `scale`                            | Percentage   | **No**               | The scaling percentage. Acceptable values are: `100%`, `50%`, `33%`, `25%`, `20%`, `10%` and `5%`                                                                                               |
| `render` or `RendererType` or `rm` | Enum values  | **No**               | The rendering method as described in [Images (Rendering)](../../chapter-2/surface/images.md#rendering-images) chapter                                                                                   |
| `flags`                            | String       | **No**               | image viewer initialization flags                                                                                                                                                               |
| `back` or `backgroud`              | char! format | **No**               | A character as describes in [Macro Builds](../../chapter-2/screen.md#macro-builds) - the same as with the  `char!` macro format                                                                 |
| `lsm` or `left-scroll-margin`      | Numeric      | **No**               | The left margin of the bottom scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `Scrollbars` was set up. |
| `tsm` or `top-scroll-margin`       | Numeric      | **No**               | The top margin of the right scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `Scrollbars` was set up.   |

A image viewer supports the following initialization flags:
* `image viewer::Flags::ScrollBars` or `ScrollBars` (for macro initialization) - thils enable a set of scrollbars that can be used to change the view of the inner surface, but only when the control has focus, as described in [Components](../components.md) section.

Some examples that uses these paramateres:

1. A image viewer with a backgroud that consists in the character `X` in with `Aqua` and `DarkBlue` colors.
    ```rs
    let img = imageviewer!("x:10,y:5,w:15,back={X,fore:aqua,back:darkblue}");
    ```
2. A image viewer with scrollbars with different margins
    ```rs
    let img = imageviewer!("x:10,y:5,w:15,flags:Scrollbars,lsm:5,tsm:1");
    ```
3. Am ascii art image with scrollbars with different margins and 50% scaling:
    ```rs
    let img = imageviewer!("image:'...',x:10,y:5,w:15,flags:Scrollbars,lsm:5,tsm:1,scale:50%,render:AsciArt");
    ```

## Events
An image viewer control emits no events.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) a image viewer also has the following aditional methods:

| Method                   | Purpose                                                                                              |
| ------------------------ | ---------------------------------------------------------------------------------------------------- |
| `set_image(...)`         | Sets a new image to be displayed in the image viewer                                                 |
| `set_scale(...)`         | Sets the new scale for the current image                                                             |
| `scale()`                | Returns the current scale of the current image                                                       |
| `set_render_method(...)` | Sets the new render_method of the current image                                                      |
| `render_method()`        | Returns the render method (SmallBlocks, AsciiArt, ...) used to described the paint the current image |
| `set_backgound(...)`     | Sets the character used for background                                                               |
| `clear_background()`     | Remove the background character making the background transparent.                                   |

## Key association

The following keys are processed by a image viewer control if it has focus:

| Key                                 | Purpose                                                                                                                                |
| ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `Left`,`Right`,`Up`,`Down`          | Move the view port to a specified direction by one character.                                                                          |
| `Shift+Left`                        | Moves the horizontal view port coordonate to 0                                                                                         |
| `Shift+Up`                          | Moves the vertical view port coordonate to 0                                                                                           |
| `Shift+Right`                       | Moves the horizontal view port coordonate so that the right side of the inner surface is displayed                                     |
| `Shift+Down`                        | Moves the vertical view port coordonate so that the bottom side of the inner surface is displayed                                      |
| `Ctrl`+{`Left`,`Right`,`Up`,`Down`} | Move the view port to a specified direction by a number of characters that is equal to the width for Left/Right or height for Up/Down. |
| `PageUp`, `PageDown`                | has the same effect as `Ctrl`+{`Up` or `Down`}                                                                                         |
| `Home`                              | Moves the view port to the coordonates (0,0)                                                                                           |
| `End`                               | Moves the view port so that the bottom-right part of the inner surface is visible                                                      |

## Example

The following code draws a heart with differemt colors using an ImageView:

```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Heart,d:c,w:15,h:7");
    let heart = Image::from_str(r#"
        |.............|
        |...rr...rr...|
        |..rrrr.rrrr..|
        |.rrrrrrrrrrr.|
        |.raaaaaaaaar.|
        |..ryyyyyyyr..|
        |   rwwwwwr   |
        |....rwwwr....|
        |.....rwr.....|
        |......r......|
    "#).unwrap();
    w.add(ImageViewer::new(
        heart,
        Layout::new("d:c"),
        image::RendererType::SmallBlocks,
        image::Scale::NoScale,
        imageviewer::Flags::None,
    ));
    a.add_window(w);
    a.run();
    Ok(())
}
```

