# ListItem

The `ListItem` trait is a trait design to provide controls like [ListView](../stock-controls/listview.md) or [TreeView](../stock-controls/treeview.md) a way to undestand how a structure should be represented. The trait has to be implemented by the item type that is displayed in the listview. This trait has the following methods that have to be implemented:

```rs
pub trait ListItem {
    fn columns_count() -> u16 { 0 }
    fn column(index: u16) -> Column { 
        Column::new("", 10, TextAlignment::Left) 
    }
    fn paint(&self, column_index: u32, width: u16, surface: &mut Surface, theme: &Theme, attr: Option<CharAttribute>) {
        // paint the item in the surface
    }
    fn render_method(&self, column_index: u16) -> Option<RenderMethod>;
    fn compare(&self, other: &Self, column_index: u16) -> Ordering {
        Ordering::Equal
    }
    fn matches(&self, text: &str) -> bool {
        true
    }
}
```
These methods have the following purpose:
* `columns_count()` - the number of columns that are displayed in the listview. If let unspecfied, the default value is 0. Adding new columns to the listview will not be affected by this value (all of the new columns will be added after the last column defined by the item type).
* `column(index)` - returns the column definition for the column with the specified index. This method has to be implemented by the item type. The column definition contains the name of the column, the width of the column, and the alignment of the column. This method is called once, when the listview is created, for indexes from 0 to `columns_count()-1`.
* `paint(column_index, width, surface, theme, attr)` - paints the item in the surface. This method has to be implemented by the item type. This method is only called if the `render_method(...)` returns the value `RenderMethod::Custom`.
* `render_method(column_index)` - returns the render method for the column with the specified index. This method has to be implemented by the item type. 
* `compare(other, column_index)` - compares the item with another item based on the column index. This method has to be implemented by the item type. This method is used to sort the items in the listview.
* `matches(text)` - returns true if the item matches the text. This method needs to be implemented only if the flag `CustomFilter` is set. This method is used to filter the items in the listview based on the search text and a custom algorithm that interprets the search test and filters based on it.

The RenderMethod enum is defined as follows:

```rs
pub enum RenderMethod<'a> {
    Text(&'a str),
    Ascii(&'a str),
    DateTime(NaiveDateTime, DateTimeFormat),
    Time(NaiveTime, TimeFormat),
    Date(NaiveDate, DateFormat),
    Duration(Duration, DurationFormat),
    Int64(i64, NumericFormat),
    UInt64(u64, NumericFormat),
    Bool(bool, BoolFormat),
    Size(u64, SizeFormat),
    Percentage(f64, PercentageFormat),
    Float(f64, FloatFormat),
    Status(Status, StatusFormat),
    Temperature(f64, TemperatureFormat),
    Area(u64, AreaFormat),
    Rating(u32, RatingFormat),
    Currency(f64, CurrencyFormat),
    Distance(u64, DistanceFormat),
    Volume(u64, VolumeFormat),
    Weight(u64, WeightFormat),
    Speed(u64, SpeedFormat),
    Custom,
}
```
with the following meanings:

| RenderMethod | Format variants                                                                                                                                                                                                 | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Text         | N/A                                                                                                                                                                                                             | Renders the text as it is                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Ascii        | N/A                                                                                                                                                                                                             | Renders the text as ASCII (this is usefull if you know the text is in Ascii format as some thins can be computed faster)                                                                                                                                                                                                                                                                                                                                                                                         |
| DateTime     | `Full`<br>`Normal`<br>`Short`                                                                                                                                                                                   | Renders a date and time value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Time         | `Short`<br>`AMPM`<br>`Normal`                                                                                                                                                                                   | Renders a time value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Date         | `Full`<br>`YearMonthDay`<br>`DayMonthYear`                                                                                                                                                                      | Renders a date value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Duration     | `Auto`<br>`Seconds`<br>`Details`                                                                                                                                                                                | Renders a duration value. The `Auto` value will attempt to find the best representation (e.g. `1:20` instead of 80 seconds)                                                                                                                                                                                                                                                                                                                                                                                      |
| Int64        | `Normal`<br>`Separator`<br>`Hex`<br>`Hex16`<br>`Hex32`<br>`Hex64`                                                                                                                                               | Renders an integer value. Example: <br>- `Normal` -> 12345<br>- `Separator` -> 12,345<br>- `Hex` and derivate will format a number into various hex representations                                                                                                                                                                                                                                                                                                                                              |
| UInt64       | `Normal`<br>`Separator`<br>`Hex`<br>`Hex16`<br>`Hex32`<br>`Hex64`                                                                                                                                               | Renders an unsigned integer value. The format is simialr to the one from `Int64` variant                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Bool         | `YesNo`<br>`TrueFalse`<br>`XMinus` <br> `CheckmarkMinus`                                                                                                                                                        | Renders a boolean value. Example: <br>- `YesNo` -> Yes<br>- `TrueFalse` -> True<br>                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Size         | `Auto`<br>`AutoWithDecimals`<br>`Bytes`<br>`KiloBytes`<br>`MegaBytes`<br>`GigaBytes`<br>`TeraBytes`<br>`KiloBytesWithDecimals`<br>`MegaBytesWithDecimals`<br>`GigaBytesWithDecimals`<br>`TeraBytesWithDecimals` | Renders a size value. The `Auto` and `AutoWithDecimals` variants will attempt to find the best representation (e.g. `1.20 MB` instead of 1234567 bytes)                                                                                                                                                                                                                                                                                                                                                          |
| Percentage   | `Normal`<br>`Decimals`                                                                                                                                                                                          | Renders a percentage value. The `Normal` variant will display the percentage without any decimals, while the `Decimals` variant will display the percentage with two decimals. For example: `PercentageFormat::Normal(0.5)` will display `50%`, while `PercentageFormat::Decimals(0.525)` will display `52.50%`                                                                                                                                                                                                  |
| Float        | `Normal`<br>`TwoDigits`<br>`ThreeDigits`<br>`FourDigits`                                                                                                                                                        | Renders a float value. The `Normal` variant will display the float without any decimals, while the other ones will add 2,3 or 4 digits to the representation                                                                                                                                                                                                                                                                                                                                                     |
| Status       | `Hashtag`<br>`Graphical`<br>`Arrow`<br>`Block`                                                                                                                                                                             | Renders a a value of type `listview::Status` with th following potential variants: `Running`, `Queued`,`Paused`, `Stopped`, `Error` and `Completed`. For the variant `Running` a progress bar is drawn. For the rest of th possible Status valuesa strng is shown                                                                                                                                                                                                                                                |
| Temperature  | `Celsius`<br>`Fahrenheit`<br>`Kelvin`                                                                                                                                                                           | Renders a temperature value. For example: `TemperatureFormat::Celsius(20.5)` will display `20.5°C`, while `TemperatureFormat::Fahrenheit(20.5)` will display `20.5°F`                                                                                                                                                                                                                                                                                                                                            |
| Area         | `SquaredMillimeters`<br>`SquaredCentimeters`<br>`SquaredMeters`<br>`SquaredKilometers`<br>`Hectares`<br>`Ares`<br>`SquareFeet`<br>`SquareInches`<br>`SquareYards`<br>`SquareMiles`                              | Renders an area value.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Rating       | `Numerical`<br>`Stars`<br>`Circles`<br>`Asterix`                                                                                                                                                                | Renders a rating value. The `Numerical` variant will display the rating as a report (e.g. `3/4`) while the other variants will use a star based representation (for example: `★★★☆☆` )                                                                                                                                                                                                                                                                                                                           |
| Currency     | `USD`<br>`USDSymbol`<br>`EUR`<br>`EURSymbol`<br>`GBP`<br>`GBPSymbol`<br>`YEN`<br>`YENSymbol`<br>`Bitcoin`<br>`BitcoinSymbol`<br>`RON`                                                                           | Renders a currency value. The `USD` and `EUR` variants will display the currency value with the currency short name, while the `USDSymbol` and `EURSymbol` variants will display the currency value with the currency symbol. For example: `CurrencyFormat::USD(20.5)` will display `USD  20.5`, while `CurrencyFormat::USDSymbol(20.5)` will display `$  20.5`. The symbol or short name are alwats displayed on the left side of the column while the value with 2 digits will be displayed on the right side. |
| Distance     | `Kilometers`<br>`Meters`<br>`Centimeters`<br>`Millimeters`<br>`Inches`<br>`Feet`<br>`Yards`<br>`Miles`                                                                                                          | Renders a distance value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Volume       | `CubicMillimeters`<br>`CubicCentimeters`<br>`CubicMeters`<br>`CubicKilometers`<br>`Liters`<br>`Milliliters`<br>`Gallons`<br>`CubicFeet`<br>`CubicInches`<br>`CubicYards`<br>`CubicMiles`                        | Renders a volume value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Weight       | `Grams`<br>`Milligrams`<br>`Kilograms`<br>`Pounds`<br>`Tons`                                                                                                                                                    | Renders a weight value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Speed        | `KilometersPerHour`<br>`MetersPerHour`<br>`KilometersPerSecond`<br>`MetersPerSecond` <br> `MilesPerHour` <br> `MilesPerSecond` <br> `Knots` <br> `FeetPerSecond` <br> `Mach`                                    | Renders a speed value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

**Example**

Lets consider the following structure: `Student` with the following fields:
```rs
struct Student {
    name: String,
    grade: u8,
    stars: u8,
}
```
In order to use this structure in a ListView, the minimum implementation of the `ListItem` trait would be:

```rs
use appcui::listview::{ListItem, RenderMethod, NumericFormat, RatingFormat};

impl ListItem for Student {
    fn render_method(&self, column_index: u16) -> Option<RenderMethod> {
        match column_index {
            0 => Some(RenderMethod::Text(&self.name)),
            1 => Some(RenderMethod::UInt64(self.grade as u64, NumericFormat::Normal)),
            2 => Some(RenderMethod::Rating(self.stars as u32, RatingFormat::Stars(5))),
            _ => None,
        }
    }
}
```
For this implementation to work, the columns would have to be added when the listview is created (e.g. `listview!("class:Student, d:c, columns:[{&Name,20,left},{&Grade,5,center},{&Stars,5,center}]")`).
However, you can also add them programatically by using the `add_column` method or by overriding the `column` method from the `ListItem` trait, like in the following example:

```rs
impl ListItem for Student {
    fn columns_count() -> u16 { 3 }
    fn column(index: u16) -> Column { 
        match index {
            0 => Column::new("&Name", 20, TextAlignment::Left),
            1 => Column::new("&Grade", 5, TextAlignment::Center),
            2 => Column::new("&Stars", 5, TextAlignment::Center),
            _ => Column::new("", 10, TextAlignment::Left),
        }
    }
    fn render_method(&self, column_index: u16) -> Option<RenderMethod> {...}
}
```
Notice that in this case, we have to specify the number of columns that are displayed in the listview by using the `columns_count()` method.

If you want all of the columns to be sortable, you will have to override the `compare` method from the `ListItem` trait. This method has to return an `Ordering` value that indicates the order of the two items. 
```rs
impl ListItem for Student {
    fn columns_count() -> u16 { 3 }
    fn column(index: u16) -> Column {...}
    fn render_method(&self, column_index: u16) -> Option<RenderMethod> {...}
    fn compare(&self, other: &Self, column_index: u16) -> Ordering {
        match column_index {
            0 => self.name.cmp(other.name),
            1 => self.grade.cmp(&other.grade),
            2 => self.stars.cmp(&other.stars),
            _ => Ordering::Equal,
        }
    }    
}
```

Alternatively, you can use the `LisItem` derive macro to automatically implement the `ListItem` trait for a structure. The macro has to be combined with the `#[Column(...)]` attribute that has to be added to each field of the structure that has to be displayed in the listview. The `#[Column(...)]` attribute has the following parameters:

| Parameter        | Type   | Required | Default value | Description                                                                                                                                                                                                    |
| ---------------- | ------ | -------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name` or `text` | String | **Yes**  | N/A           | The name of the column. This name will be displayed in the header of the column.                                                                                                                               |
| `width` or `w`   | u16    | **No**   | 10            | The width of the column.                                                                                                                                                                                       |
| `align` or `a`   | Align  | **No**   | Left          | The alignment of the column (one of `Left` (or `l`), `Right` (or `r`)) and `Center` (or `c`)                                                                                                                   |
| `render` or `r`  | Render | **No**   | N/A           | The render method for the column. If not provided it will be automatically identified based of the field type                                                                                                  |
| `format` or `f`  | Format | **No**   | various ...   | The format of the render method. If not provided it will be defaulted to different variants based on the renderer type                                                                                         |
| `index` or `idx` | u16    | **No**   | N/A           | The index of the column. This is used to determine the order of the columns. Indexes starts with value `1` or `0` and have o be unique. If not provided, the next free index will be allocated for the column. |

If the `render` parameter is not provided, the render method will be automatically identified based on the field type. The following field types are supported:

| Field type                | Render method | Default variant |
| ------------------------- | ------------- | --------------- |
| `&str`                    | Text          |                 |
| `String`                  | Text          |                 |
| `i8`, `i16`, `i32`, `i64` | Int64         | Normal          |
| `u8`, `u16`, `u32`, `u64` | UInt64        | Normal          |
| `f32`, `f64`              | Float         | Normal          |
| `bool`                    | Bool          | CheckmarkMinus  |
| `NaiveDateTime`           | DateTime      | Normal          |
| `NaiveTime`               | Time          | Normal          |
| `NaiveDate`               | Date          | Full            |
| `Duration`                | Duration      | Auto            |
| `Status`                  | Status        | Graphical       |


This means that the previous `Student` structure can be rewritten as follows:

```rs
#[derive(ListItem)]
struct Student {
    #[Column(name: "&Name", width: 20, align: Left)]
    name: String,
    #[Column(name: "&Grade", width: 5, align: Center)]
    grade: u8,
    #[Column(name: "&Stars", width: 5, align: Center, render: Rating, format: Stars)]
    stars: u8, 
}
```

### Custom filtering

The filtering mechanism takes the string from the search bar and tries to see if any of the fields that are displayed contain that string (ignoring the case).
While this method will be good enough for most cases, there might be scearious where you want to implement a custom filtering algorithm.

For example, lets consider that we want to filter the student based on the name that **starts with** the specified text written in the search bar.
In this case, we have to implement the `matches` method from the `ListItem` trait:

```rs
impl ListItem for Student {
    fn matches(&self, text: &str) -> bool {
        self.name.starts_with(text)
    }       
}
```

We will also need to make sure that the `CustomFilter` flag is set when creating the listview:

```rs
let lv = listview!("class:Student, d:c, flags: CustomFilter");
```


### Custom rendering 

If you want to have a custom rendering for the items in the listview, you can use the `RenderMethod::Custom` variant. This variant will trigger the `paint` method from the `ListItem` trait. 
It is important to notice that you don't need to implement the `paint` method for all fields (only for the ones where the response from the `render_method` method is `RenderMethod::Custom`).

In the next example, we will atempt to print the grade differently based on the value of the grade. If the grade is greater than 5, we will print the grade in green, otherwise in red.

```rs
impl ListItem for Student {
    fn render_method(&self, column_index: u16) -> Option<RenderMethod> {
        match column_index {
            0 => Some(RenderMethod::Text(&self.name)),
            1 => Some(RenderMethod::Custom)),
            2 => Some(RenderMethod::Rating(self.stars as u32, RatingFormat::Stars(5))),
            _ => None,
        }
    } 
    fn paint(&self, column_index: u32, width: u16, surface: &mut Surface, theme: &Theme, attr: Option<CharAttribute>) {
        if column_index == 1 {
            // the grade column
            let color = if self.grade > 5 { Color::Green } else { Color::Red };
            // if the attr is provided, we will use it, otherwise we 
            // will use the color variable (Green or Red)
            let a = attr.unwrap_or(CharAttribute::with_fore_color(color));
            // prepare a string with the grade
            // normally this is not indicated as it would allocate memory 
            // everytime the paint method is called
            let t = format!("{}", self.grade);
            // print the string in the surface
            surface.write_string(0, 0, &t, a, false);
        }
    }      
}
```

