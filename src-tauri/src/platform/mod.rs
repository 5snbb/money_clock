//! Platform adaptation layer.
//!
//! Each supported OS gets one adapter implementing [`PlatformAdapter`]. The
//! rest of the backend asks [`current`] for the adapter instead of branching on
//! compile-time target flags, keeping OS differences in a single place.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
mod traits;
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
mod unsupported;
#[cfg(target_os = "windows")]
mod windows;

pub use traits::{PlatformAdapter, PlatformCapabilities, PlatformKind, WindowDefaults};

#[cfg(target_os = "windows")]
static WINDOWS: windows::WindowsAdapter = windows::WindowsAdapter;
#[cfg(target_os = "macos")]
static MACOS: macos::MacosAdapter = macos::MacosAdapter;
#[cfg(target_os = "linux")]
static LINUX: linux::LinuxAdapter = linux::LinuxAdapter;
#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
static UNSUPPORTED: unsupported::UnsupportedAdapter = unsupported::UnsupportedAdapter;

/// Returns the adapter compiled for the current target OS.
pub fn current() -> &'static dyn PlatformAdapter {
    #[cfg(target_os = "windows")]
    {
        &WINDOWS
    }

    #[cfg(target_os = "macos")]
    {
        &MACOS
    }

    #[cfg(target_os = "linux")]
    {
        &LINUX
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        &UNSUPPORTED
    }
}
