
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
#[cfg(test)]
pub(crate) use nav_simulator::NavSimulator;
pub(crate) use root::Root;
pub(crate) use root::RootType;
pub(crate) use entry::Entry;
pub(crate) use entry::EntryType;


#[cfg(target_os="windows")]
use windows::get_os_roots;
#[cfg(target_os="windows")]
use windows::get_os_absolute_path;
#[cfg(target_os="windows")]
use windows::get_os_separator;
#[cfg(target_os="windows")]
use windows::is_fs_root;
#[cfg(target_family="unix")]
use unix::get_os_roots;
#[cfg(target_family="unix")]
use unix::get_os_absolute_path;
#[cfg(target_family="unix")]
use unix::get_os_separator;
#[cfg(target_family="unix")]
use unix::is_fs_root;
