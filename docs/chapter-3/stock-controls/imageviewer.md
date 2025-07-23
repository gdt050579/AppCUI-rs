# Image Viewer

Represents an image that is being rendered under a view-port:

<img src="img/imageviewer.png" width=300/>

To create an image viewer use `ImageViewer::new` method (with 4 parameters: an image, a layout, render options, and initialization flags). To understand more on how an image is being rendered or constructed read the [Images](../../chapter-2/images.md) chapter.

```rs
let i = ImageViewer::new(Image::from_str(...).unwrap(), 
                         layout!("x:10,y:5,w:15"),
                         image::RenderOptionsBuilder::new()
                             .scale(image::Scale::Scale50)
                             .character_set(image::CharacterSet::AsciiArt)
                             .build(),
                         imageviewer::Flags::None);
```

Or use the macro `imageviewer!`:

```rs
let i1 = imageviewer!("x:10,y:5,w:15,scale:50%,charset:AsciiArt");
let i2 = imageviewer!("image:'|R..|,|.R.|,|..R|',x:10,y:5,w:15");
```

An image viewer supports all common parameters (as they are described in [Instantiate via Macros](../instantiate_via_macros.md) section). Besides them, the following **named parameters** are also accepted:

| Parameter name                          | Type         | Positional parameter | Purpose                                                                                                                                                                                         |
| --------------------------------------- | ------------ | -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `image`                                 | String       | **No**               | A string representation of an image as described in [Images (Building from a string)](../../chapter-2/surface/images.md#building-from-a-string) chapter                                         |
| `scale`                                 | Percentage   | **No**               | The scaling percentage. Acceptable values are: `100%`, `50%`, `33%`, `25%`, `20%`, `10%` and `5%`                                                                                               |
| `charset` or `char_set`                 | Enum values  | **No**               | The character set for rendering. Can be: `SmallBlocks`, `LargeBlocks`, `DitheredShades`, `Braille`, `AsciiArt`                                                                                  |
| `color_schema` or `colorschema` or `cs` | Enum values  | **No**               | The color schema for rendering. Can be: `Auto`, `Color16`, `TrueColors`, `GrayScale4`, `GrayScaleTrueColors`, `BlackAndWhite`                                                                   |
| `luminance_threshold` or `lt`           | Percentage   | **No**               | The luminance threshold percentage (0-100) for black/white conversion                                                                                                                           |
| `flags`                                 | String       | **No**               | Image viewer initialization flags                                                                                                                                                               |
| `back` or `background`                  | char! format | **No**               | A character as described in [Macro Builds](../../chapter-2/screen.md#macro-builds) - the same as with the `char!` macro format                                                                  |
| `lsm` or `left-scroll-margin`           | Numeric      | **No**               | The left margin of the bottom scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `Scrollbars` was set up. |
| `tsm` or `top-scroll-margin`            | Numeric      | **No**               | The top margin of the right scroll bar in characters. If not provided the default value is 0. This should be a positive number and it only has an effect if the flag `Scrollbars` was set up.   |

An image viewer supports the following initialization flags:
* `imageviewer::Flags::ScrollBars` or `ScrollBars` (for macro initialization) - this enables a set of scrollbars that can be used to change the view of the inner surface, but only when the control has focus, as described in [Components](../components.md) section.

Some examples that use these parameters:

1. An image viewer with a background that consists of the character `X` with `Aqua` and `DarkBlue` colors.
    ```rs
    let img = imageviewer!("x:10,y:5,w:15,back={X,fore:aqua,back:darkblue}");
    ```
2. An image viewer with scrollbars with different margins
    ```rs
    let img = imageviewer!("x:10,y:5,w:15,flags:Scrollbars,lsm:5,tsm:1");
    ```
3. An ASCII art image with scrollbars with different margins and 50% scaling:
    ```rs
    let img = imageviewer!("image:'...',x:10,y:5,w:15,flags:Scrollbars,lsm:5,tsm:1,scale:50%,charset:AsciiArt");
    ```
4. An image viewer with custom color schema and luminance threshold:
    ```rs
    let img = imageviewer!("image:'|RGB|,|YWr|',x:10,y:5,w:15,color_schema:BlackAndWhite,luminance_threshold:30%");
    ```

## Events
An image viewer control emits no events.

## Methods

Besides the [Common methods for all Controls](../common_methods.md) an image viewer also has the following additional methods:

| Method                    | Purpose                                                            |
| ------------------------- | ------------------------------------------------------------------ |
| `set_image(...)`          | Sets a new image to be displayed in the image viewer               |
| `render_options()`        | Returns the current render options of the image viewer             |
| `set_render_options(...)` | Sets new render options for the image viewer                       |
| `set_background(...)`     | Sets the character used for background                             |
| `clear_background()`      | Remove the background character making the background transparent. |

## Key association

The following keys are processed by an image viewer control if it has focus:

| Key                                 | Purpose                                                                                                                                |
| ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `Left`,`Right`,`Up`,`Down`          | Move the view port to a specified direction by one character.                                                                          |
| `Shift+Left`                        | Moves the horizontal view port coordinate to 0                                                                                         |
| `Shift+Up`                          | Moves the vertical view port coordinate to 0                                                                                           |
| `Shift+Right`                       | Moves the horizontal view port coordinate so that the right side of the inner surface is displayed                                     |
| `Shift+Down`                        | Moves the vertical view port coordinate so that the bottom side of the inner surface is displayed                                      |
| `Ctrl`+{`Left`,`Right`,`Up`,`Down`} | Move the view port to a specified direction by a number of characters that is equal to the width for Left/Right or height for Up/Down. |
| `PageUp`, `PageDown`                | has the same effect as `Ctrl`+{`Up` or `Down`}                                                                                         |
| `Home`                              | Moves the view port to the coordinates (0,0)                                                                                           |
| `End`                               | Moves the view port so that the bottom-right part of the inner surface is visible                                                      |

## Example

The following code draws a heart with different colors using an ImageViewer:

```rs
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    let mut w = window!("Heart,a:c,w:15,h:7");
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
        layout!("d:f"),
        image::RenderOptionsBuilder::new()
            .scale(image::Scale::Scale50)
            .character_set(image::CharacterSet::AsciiArt)
            .color_schema(image::ColorSchema::Color16)
            .build(),
        imageviewer::Flags::None,
    ));
    a.add_window(w);
    a.run();
    Ok(())
}
```

