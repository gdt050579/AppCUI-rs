
mod navigator;
mod root;
mod entry;
mod nav_simulator;

#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="linux")]
mod unix;

#[cfg(test)]
mod tests;

pub(crate) use navigator::Navigator;
pub(crate) use nav_simulator::NavSimulator;
pub(crate) use root::Root;
pub(crate) use entry::Entry;

#[cfg(target_os="windows")]
pub(self) use windows::get_os_roots;
#[cfg(target_os="linux")]
pub(self) use unix::get_os_roots;
pub(crate) use entry::EntryType;
