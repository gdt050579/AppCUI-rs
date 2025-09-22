//! Image handling and rendering module for AppCUI.
//! 
//! This module provides types and functions for creating, manipulating,
//! and rendering images within the AppCUI framework. The primary components are:
//! 
//! - [`Image`]: A structure representing a raster image with RGBA pixels
//! - [`Pixel`]: Represents an RGBA pixel with 8-bit color channels
//! - [`Scale`]: Enumeration of scaling methods for image rendering
//! - [`RendererType`]: Different rendering methods for displaying images in a terminal
//! 
//! # Examples
//! 
//! Creating an image from a string representation:
//! 
//! ```rust
//! use appcui::prelude::*;
//! use std::str::FromStr;
//! 
//! // Create a 3x2 image with specific colors
//! let image_str = "|RGB| |YWr|";
//! let img = Image::from_str(image_str).unwrap();
//! ```
//! 
//! Creating an empty image and manipulating pixels:
//! 
//! ```rust
//! use appcui::prelude::*;
//! 
//! // Create a 100x100 image
//! let mut img = Image::new(100, 100).unwrap();
//! 
//! // Set a red pixel at position (10, 20)
//! img.set_pixel(10, 20, Pixel::with_rgb(255, 0, 0));
//! 
//! // Get the pixel at position (10, 20)
//! if let Some(pixel) = img.pixel(10, 20) {
//!     assert_eq!(pixel.red, 255);
//!     assert_eq!(pixel.green, 0);
//!     assert_eq!(pixel.blue, 0);
//! }
//! ```

mod image;
mod pixel;
mod scale;
mod color_schema;
mod character_set;
mod render_options;
mod bit_tile;
#[cfg(test)]
mod tests;

pub use scale::Scale;
pub use pixel::Pixel;
pub use image::Image;
pub use character_set::CharacterSet;
pub use color_schema::ColorSchema;
pub use render_options::{ RenderOptions, RenderOptionsBuilder };
pub use self::bit_tile::BitTile;