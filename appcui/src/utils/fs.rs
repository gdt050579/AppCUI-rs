
mod navigator;
mod root;
mod entry;
mod nav_simulator;

#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="linux")]
mod unix;
#[cfg(target_os="macos")]
mod unix;



#[cfg(test)]
mod tests;

pub(crate) use navigator::Navigator;
#[cfg(test)]
pub(crate) use nav_simulator::NavSimulator;
pub(crate) use root::Root;
pub(crate) use root::RootType;
pub(crate) use entry::Entry;
pub(crate) use entry::EntryType;


#[cfg(target_os="windows")]
use windows::get_os_roots;
#[cfg(target_os="linux")]
use unix::get_os_roots;
#[cfg(target_os="macos")]
use unix::get_os_roots;
