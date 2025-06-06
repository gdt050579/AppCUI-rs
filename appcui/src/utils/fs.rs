mod entry;
mod nav_simulator;
mod navigator;
mod root;

#[cfg(target_os = "linux")]
mod unix;
#[cfg(all(target_arch = "wasm32", wasm_unix))]
mod unix;
#[cfg(target_os = "macos")]
mod unix;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(all(target_arch = "wasm32", wasm_windows))]
mod windows;

#[cfg(test)]
mod tests;

pub(crate) use entry::Entry;
pub(crate) use entry::EntryType;
#[cfg(test)]
pub(crate) use nav_simulator::NavSimulator;
pub(crate) use navigator::Navigator;
pub(crate) use root::Root;
pub(crate) use root::RootType;

#[cfg(any(target_os = "linux", wasm_unix))]
use unix::get_os_roots;
#[cfg(target_os = "macos")]
use unix::get_os_roots;
#[cfg(any(target_os = "windows", wasm_windows))]
use windows::get_os_roots;
