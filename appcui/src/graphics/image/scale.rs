
/// The scale of the image.
/// 
/// This enum defines the different scaling methods available for images.
/// Each variant represents a specific scaling factor:
/// 
/// - `NoScale`: No scaling (1:1)   
/// - `Scale50`: 50% scaling (2:1)
/// - `Scale33`: 33% scaling (3:1)
/// - `Scale25`: 25% scaling (4:1)
/// - `Scale20`: 20% scaling (5:1)
/// - `Scale10`: 10% scaling (10:1)
/// - `Scale5`: 5% scaling (20:1)   
/// 
/// The `Scale` enum is used to select the scaling method for an image. 
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Scale {
    NoScale = 1,
    Scale50 = 2,
    Scale33 = 3,
    Scale25 = 4,
    Scale20 = 5,
    Scale10 = 10,
    Scale5 = 20,
}