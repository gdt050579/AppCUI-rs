mod structures;
mod macros;
#[cfg(test)]
mod tests;
// stock themes
mod default;
mod dark_gray;
mod light;
mod high_contrast;


pub use structures::Theme;
pub use structures::Themes;
pub use structures::MenuTheme;