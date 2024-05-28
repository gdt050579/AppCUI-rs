mod desktop;
mod empty_desktop;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::desktop::Desktop;
pub(crate) use self::empty_desktop::EmptyDesktop;
pub use self::desktop::ArrangeWindowsMethod;