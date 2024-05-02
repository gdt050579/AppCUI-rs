mod desktop;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::desktop::Desktop;
pub use self::desktop::ArrangeWindowsMethod;