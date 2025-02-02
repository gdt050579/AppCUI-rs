# Images

While AppCUI is developed for CLI usage, it can still use images to some degree, meaning that it can store an images as an array of pixels and it has various methods to represent it using characters and combination of existing colors.

To create an image, use the class `Image` with the following construction methods:
1. `Image::new(width,height)` creates an image with a specific size. That image will be filled with a transparent pixel that you can later change
2. `Image::with_str(...)` creates a 16 color image based on a string representation.


## Methods

Once an image is create you can use the following methods to manipulate it:

| Method             | Purpose                                                                                                                                                          |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `clear(...)`       | Fills the entire image with a specific pixel                                                                                                                     |
| `pixel(...)`       | Provide the pixel from a specific coordonate in the image or None otherwise                                                                                      |
| `set_pixel(...)`   | Sets the pixel from a specific coordonate from the image                                                                                                         |
| `width()`          | The width of the image in pixels                                                                                                                                 |
| `height()`         | The height of the image in pixels                                                                                                                                |
| `size()`           | The size (width and height) of te image in pixels                                                                                                                |
| `render_size(...)` | The size (in characters) needed for a surface object to allow the entire image to be painted. It requires both a **scale** and a **rendering method** to compute |

## Pixel

A pixel is a simple structure defined as follows:

```rs
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
```

You can create a pixel in the following way:
1. using direct construction:
    ```rs
    let px = Pixel { red:10, green: 20, blue:30, alpha: 255 };
    ```
2. using the `.new(...)` constructor:
    ```rs
    let px = Pixel::new(10, 20, 30, 255);
    ```
3. using the `.with_rgb(...)` constructor:
    ```rs
    let px = Pixel::with_rgb(10, 20, 30);
    ```
4. using the `.with_color(...)` constructor:
    ```rs
    let px = Pixel::with_color(Color::Aqua);
    ```
5. using the **From** implementation from an u32 (in an ARGB format - `A`lpha `R`ed `G`reen `B`lue).
    ```rs
    let px = Pixel::from(0xFF005090u32);
    ```
6. using the **Default** implementation (this will create a trnsparent pixel where `Red=0`, `Green=0`, `Blue=0` and `Alpha=0`)
    ```rs
    let px = Pixel::default();
    ```

## Usage

A typical way to create an image is as follows:
1. create a new `Image` object
2. optionally, fill the entire image with a differnt pixel than the default one
3. use `.set_pixel(...)` method to fill the image. At this point aditional crates that can load an image from a file can be used to transfer the content of that image into this object.

The following example draws a horizontal `Red` line on a `Blue` background image of size `32x32`:

```rs
let mut img = Image::new(32,32);
img.clear(Pixel::with_color(Color::Blue));
for i in 5..30 {
    img.set_pixel(i,10,Pixel::with_color(Color::Red));
}
```

## Building from a string

A more common usage is to build a small image from a string that specifies colors for each pixel. The format in this case is as follows:
* each line is enclosed between two characters `|`
* outside of these characters any other character is being ignored (usually you add spaces or new lines to align the text)
* each line must have the same with (in terms of the number of characters that are located between `|` characters )

for example, a `5x5` image will be represented as follows:
```rs
let string_representation = r#"
      |.....|
      |.....|
      |.....|
      |.....|
      |.....|
"#;
```

Within the space between the characters `|` the following characters have the a color association:

| Character                       | Enum variant       | RGB                                      | Color                                                                                                        |
| ------------------------------- | ------------------ | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `0`<br>` `(space)<br>`.`(point) | `Color::Black`     | Red=**0**, Green=**0**, Blue=**0**       | <div style="width: 24px; height: 24px; background-color: rgb(0, 0, 0);border: 2px solid white;"></div>       |
| `1`<br>`B`(capital **B**)       | `Color::DarkBlue`  | Red=**0**, Green=**0**, Blue=**128**     | <div style="width: 24px; height: 24px; background-color: rgb(0, 0, 128);border: 2px solid white;"></div>     |
| `2`<br>`G`(capital **G**)       | `Color::DarkGreen` | Red=**0**, Green=**128**, Blue=**0**     | <div style="width: 24px; height: 24px; background-color: rgb(0, 128, 0);border: 2px solid white;"></div>     |
| `3`<br>`T`(capital **T**)       | `Color::Teal`      | Red=**0**, Green=**128**, Blue=**128**   | <div style="width: 24px; height: 24px; background-color: rgb(0, 128, 128);border: 2px solid white;"></div>   |
| `4`<br>`R`(capital **R**)       | `Color::DarkRed`   | Red=**128**, Green=**0**, Blue=**0**     | <div style="width: 24px; height: 24px; background-color: rgb(128, 0, 0);border: 2px solid white;"></div>     |
| `5`<br>`M` or `m`               | `Color::Magenta`   | Red=**128**, Green=**0**, Blue=**128**   | <div style="width: 24px; height: 24px; background-color: rgb(128, 0, 128);border: 2px solid white;"></div>   |
| `6`<br>`O` or `o`               | `Color::Olive`     | Red=**128**, Green=**128**, Blue=**0**   | <div style="width: 24px; height: 24px; background-color: rgb(128, 128, 0);border: 2px solid white;"></div>   |
| `7`<br>`S`(capital **S**)       | `Color::Silver`    | Red=**192**, Green=**192**, Blue=**192** | <div style="width: 24px; height: 24px; background-color: rgb(192, 192, 192);border: 2px solid white;"></div> |
| `8`<br>`s`(lower **s**)         | `Color::Gray`      | Red=**128**, Green=**128**, Blue=**128** | <div style="width: 24px; height: 24px; background-color: rgb(128, 128, 128);border: 2px solid white;"></div> |
| `9`<br>`b`(lower **b**)         | `Color::Blue`      | Red=**0**, Green=**0**, Blue=**255**     | <div style="width: 24px; height: 24px; background-color: rgb(0, 0, 255);border: 2px solid white;"></div>     |
| `g`(lower **g**)                | `Color::Green`     | Red=**0**, Green=**255**, Blue=**0**     | <div style="width: 24px; height: 24px; background-color: rgb(0, 255, 0);border: 2px solid white;"></div>     |
| `r`(lower **r**)                | `Color::Red`       | Red=**255**, Green=**0**, Blue=**0**     | <div style="width: 24px; height: 24px; background-color: rgb(255, 0, 0);border: 2px solid white;"></div>     |
| `A` or `a`<br>`t`(lower **t**)  | `Color::Aqua`      | Red=**0**, Green=**255**, Blue=**255**   | <div style="width: 24px; height: 24px; background-color: rgb(0, 255, 255);border: 2px solid white;"></div>   |
| `P` or `p`                      | `Color::Pink`      | Red=**255**, Green=**0**, Blue=**255**   | <div style="width: 24px; height: 24px; background-color: rgb(255, 0, 255);border: 2px solid white;"></div>   |
| `Y` or `y`                      | `Color::Yellow`    | Red=**255**, Green=**255**, Blue=**0**   | <div style="width: 24px; height: 24px; background-color: rgb(255, 255, 0);border: 2px solid white;"></div>   |
| `W` or `w`                      | `Color::White`     | Red=**255**, Green=**255**, Blue=**255** | <div style="width: 24px; height: 24px; background-color: rgb(255, 255, 255);border: 2px solid white;"></div> |

So ... to create an image of a red heart &#9829; you will need to create the folowing string:
```rs
let heart = r#"
    |..rr.rr..|
    |.rrrrrrr.|
    |.rrrrrrr.|
    |..rrrrr..|
    |...rrr...|
    |....r....|
"#;
let img = Image::with_str(heart);
```

## Rendering images

AppCUI framework relies on characters. As such, an image can not be displayes **as it is**. However, there is one method in the [Surface](surface.md) object that be used to aproximate an image:

```rs
impl Surface {
    // other methods
    pub fn draw_image(&mut self, x: i32, 
                                y: i32, 
                                image: &Image, 
                                rendering_method: image::RendererType, 
                                scale_method: image::Scale
                    ) { ... }
}
```
This method attempts to draw an image using characters and the available colors.
The following rendering methods are available:
* SmallBlocks
* LargeBlocks64Colors
* GrayScale
* AsciiArt

Let's consider an image of [Cuddly Ferris](https://www.rustacean.net/assets/cuddlyferris.svg) and see how it will be displayed using different rendering methods:

<img src="img/cuddlyferris.png" width=120/>

| Methods             | Result                                                      |
| ------------------- | ----------------------------------------------------------- |
| SmallBlocks         | <img src="img/cuddlyferris_small_blocks.png" width=400/>    |
| LargeBlocks64Colors | <img src="img/cuddlyferris_large_blocks_64.png" width=400/> |
| GrayScale           | <img src="img/cuddlyferris_grayscale.png" width=400/>       |
| AsciiArt            | <img src="img/cuddlyferris_ascii_art.png" width=400/>       |

The supported scales (from the enume `image::Scale`):
* `Scale::NoScale` => 100%
* `Scale::Scale50` => 50%
* `Scale::Scale33` => 33%
* `Scale::Scale25` => 25%
* `Scale::Scale20` => 20%
* `Scale::Scale10` => 10%
* `Scale::Scale5` => 5%
